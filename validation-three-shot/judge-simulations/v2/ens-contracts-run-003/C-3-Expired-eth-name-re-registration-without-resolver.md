# C-3 Judge Simulation

Status: Complete

## Simulated Judge Decision

- Decision: Accepted With Downgrade
- Severity: Medium
- Confidence: Medium
- Estimated payout: 10,000 USDC-equivalent, based on the fixed Medium smart contract reward

## Rationale

The affected code and assets are in scope. `ETHRegistrarController`, `BaseRegistrarImplementation`, and `ENSRegistry` are listed in the ENS smart contract scope, and the report's deployed addresses match the repository's mainnet deployment artifacts for the controller, registrar, and registry.

The core behavior is real. In `ETHRegistrarController.register`, the `registration.resolver == address(0)` branch only calls `base.register(...)`. `BaseRegistrarImplementation._register` then burns any expired ERC721 token, mints the new token to the buyer, and calls `ens.setSubnodeOwner(baseNode, bytes32(id), owner)`. `ENSRegistry.setSubnodeOwner` updates only `records[subnode].owner`; resolver and TTL are changed only by `setRecord`, `setSubnodeRecord`, `setResolver`, or `_setResolverAndTTL`. I did not find a surrounding guard, modifier, expiry transition, or controller check that clears a prior resolver during no-resolver re-registration.

Reachability is also plausible for an unprivileged attacker. A prior `.eth` registrant can set a resolver contract they control, let the name expire past the 90-day grace period, and wait for a later buyer to re-register the same available label with `resolver == address(0)`. After that registration, the buyer owns both the registrar ERC721 and the ENS registry node, but `ENSRegistry.resolver(node)` can still point to the prior owner's resolver. If that resolver uses resolver-owner authorization, such as `OwnedResolver`, the prior owner can continue changing the records served by the live resolver address until the new owner replaces or clears the resolver.

The embedded PoC is directionally strong and portable in form: it uses a local mainnet fork, fixed deployed ENS addresses, Anvil-funded local accounts, commit/reveal registration, expiry/grace/premium time travel, and assertions for buyer ownership plus stale attacker resolver control. I did not execute it in this pass because `MAINNET_RPC_URL` is not set in the worker environment and the repository test script runs compilation, which could mutate local build artifacts. Static review of the PoC shows it proves the claimed state divergence rather than merely call success.

I would not accept the submitted Critical severity as written. The exact Critical row claimed, "Unintended alteration of what the NFT represents," is arguable but not a clean Immunefi fit here. The registrar NFT still represents the same `.eth` label, ownership transfers to the buyer, and the buyer has registry authority to set a new resolver immediately. The demonstrated damage is stale attacker-controlled resolution for buyers who use the supported zero-resolver path and have not yet replaced the resolver. That is material user/protocol griefing and can lead to identity/content spoofing or payment redirection risk, but the report does not prove direct theft, permanent freezing, unauthorized minting, or an irreversible NFT-payload alteration. The best exact in-scope row is Medium smart contract griefing.

## Likely Judge Questions Or Objections

- Does `resolver == address(0)` mean "clear the resolver" or simply "do not configure resolver state"?
- ENS registry owner, resolver, and TTL are intentionally separate fields; why is preserving resolver across owner changes not expected registry behavior?
- The new owner can call `setResolver` or `setRecord` immediately after registration, so what is the concrete loss window and material value at risk?
- Is the official ENS app or common registration flow likely to use the zero-resolver path for valuable names, or does it normally set the Public Resolver?
- Does resolver-controlled ENS resolution qualify as "what the NFT represents" under the Critical row, or is that row meant for ERC721 metadata/art/content rather than registry configuration?
- The PoC proves stale resolver control, but it does not prove an actual direct theft or permanent state loss.

## Recommended Pre-Submission Edits

- Reframe the primary requested severity as Medium griefing unless stronger bounty-specific evidence is added for the Critical NFT-representation row.
- Add a concise explanation of why `resolver == address(0)` should be interpreted as "no resolver after registration," supported by controller parameter docs and fresh-name behavior.
- Show the exact mitigation available to the new owner, then explain why the interim attacker-controlled resolution window is still a compensable impact.
- Add a UniversalResolver or normal ENS resolution assertion to the PoC so the stale resolver is shown through the same path applications use.
- Include the precise local-fork command and note that it requires `MAINNET_RPC_URL`, uses only a local Anvil fork, and does not broadcast live transactions.
