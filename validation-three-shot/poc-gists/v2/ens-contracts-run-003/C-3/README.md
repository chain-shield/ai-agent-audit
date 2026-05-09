# ENS C-3 PoC - expired .eth re-registration keeps stale resolver active

This secret gist supports the Immunefi submission for the ENS finding:

`Expired .eth re-registration without resolver leaves attacker-controlled resolver active`

## Repository

Use the ENS contracts repository at commit `91c966febd7b55494269df830fc6775f040b927b`.

## Save As

From the ENS repository root:

```bash
test/ExpiredNameNoResolverReRegistrationPoC.test.ts
```

## Run

```bash
MAINNET_RPC_URL=<mainnet rpc url> bun run test test/ExpiredNameNoResolverReRegistrationPoC.test.ts
```

## Expected Result

The single test passes on a local mainnet fork at block `25030187` and proves:

- The victim owns the re-registered `.eth` name and registry node.
- The registry resolver remains the prior owner-controlled resolver.
- The victim cannot update records inside that resolver.
- The prior owner can still mutate the live resolved address.

