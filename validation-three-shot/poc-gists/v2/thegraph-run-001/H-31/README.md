# The Graph PoC: Unsigned receiverDestination

Repository: `graphprotocol-contracts`
Package: `packages/horizon`
Test path: `packages/horizon/test/UnsignedRavPayoutDestinationPoC.t.sol`

Run from the `graphprotocol-contracts` repo root:

```bash
cd packages/horizon && forge test --match-path test/UnsignedRavPayoutDestinationPoC.t.sol --fork-url "$ARBITRUM_RPC_URL" --fork-block-number 460062146
```

Prerequisites:
- Foundry
- Repository dependencies installed
- `ARBITRUM_RPC_URL` set to an Arbitrum One RPC endpoint

Expected result:
The test passes and proves the same signed RAV can redirect 19,800,000 GRT to the attacker, empty escrow, and leave provider balance/stake unchanged.
