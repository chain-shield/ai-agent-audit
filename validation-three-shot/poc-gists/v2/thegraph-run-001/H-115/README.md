# The Graph PoC: Unsigned RAV payment parameters

Repository: `graphprotocol-contracts`
Package: `packages/horizon`
Test path: `packages/horizon/test/UnsignedRavPaymentParamsPoC.t.sol`

Run from the `graphprotocol-contracts` repo root:

```bash
cd packages/horizon && forge test --match-path test/UnsignedRavPaymentParamsPoC.t.sol --fork-url "$ARBITRUM_RPC_URL" --fork-block-number 460064183
```

Prerequisites:
- Foundry
- Repository dependencies installed
- `ARBITRUM_RPC_URL` set to an Arbitrum One RPC endpoint

Expected result:
The test passes and proves the same signed RAV can be collected with attacker-selected payment parameters, draining 20,000,000 GRT from escrow while the intended service provider receives zero.
