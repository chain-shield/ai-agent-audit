# Private Client Audit Intake

Use this form to capture protocol-specific audit context that cannot be reliably inferred from code alone. Leave a field blank if it does not apply.

## 1. Audit Target

### 1. Protocol / project name

Answer:

### 2. Repo URL and exact commit / tag / branch to audit

Repo URL:

Commit / tag / branch:

### 3. Are there multiple repos or packages in scope?

Only list what should be audited.

Answer:

### 4. Any non-standard setup needed to build or test?

Only mention things not obvious from the repo. Do not include secrets.

Answer:

## 2. Scope

### 5. What contracts, folders, or modules are in scope?

| Path | Contract / module | Notes |
| --- | --- | --- |
|  |  |  |

### 6. What is explicitly out of scope?

Examples: mocks, scripts, old versions, periphery, external dependencies.

Answer:

### 7. Should deployment scripts, upgrade scripts, tests, or off-chain services be reviewed?

Check all that apply:

- [ ] Deployment scripts
- [ ] Upgrade scripts
- [ ] Tests
- [ ] Off-chain services
- [ ] None of the above

Notes:

### 8. Are third-party dependencies in scope beyond your usage of them?

Choose one:

- [ ] No
- [ ] Yes, selected dependencies
- [ ] Unsure

Selected dependencies / notes:

## 3. Protocol Intent

### 9. What are the protocol's most important user flows?

Examples: deposit, withdraw, borrow, liquidate, stake, claim, bridge, vote.

Answer:

### 10. What are the most security-critical flows?

Where would a bug be most damaging?

Answer:

### 11. What should never happen?

List protocol invariants or forbidden states in plain English.

Answer:

### 12. What edge cases are intentionally allowed?

Examples: delayed withdrawals, paused actions, dust loss, temporary imbalance.

Answer:

### 13. Any protocol behavior that looks risky in code but is intentional?

Answer:

## 4. Assets And Integrations

### 14. What assets can the protocol custody, mint, burn, freeze, or otherwise control?

| Asset | Chain | Role in protocol | Expected token behavior |
| --- | --- | --- | --- |
|  |  |  |  |

### 15. Which token behaviors are supported?

Check all that apply:

- [ ] Standard ERC20
- [ ] USDT-like ERC20
- [ ] Fee-on-transfer
- [ ] Rebasing
- [ ] ERC777 / token hooks
- [ ] ERC4626
- [ ] ERC721
- [ ] ERC1155
- [ ] Native ETH
- [ ] Other

Notes:

### 16. Which token behaviors are explicitly unsupported?

Answer:

### 17. What external protocols, bridges, or messaging systems does this rely on?

| Dependency | Used for | Trust / availability assumption |
| --- | --- | --- |
|  |  |  |

### 18. What oracle or price sources are used?

| Oracle | Used for | Expected freshness | Fallback behavior |
| --- | --- | --- | --- |
|  |  |  |  |

### 19. Are keepers, relayers, bots, or off-chain services required for safety or liveness?

Answer:

## 5. Roles And Trust

### 20. List privileged roles and what each can do

| Role | Powers | Current / expected holder | Timelock / multisig? |
| --- | --- | --- | --- |
|  |  |  |  |

### 21. Which privileged roles should auditors treat as trusted?

| Role | Trusted? | Why |
| --- | --- | --- |
|  |  |  |

### 22. Which privileged-role actions should still be considered valid audit surface?

Examples: unsafe upgrades, missing timelocks, unrestricted sweeps, weak role separation, dangerous parameter changes.

Answer:

### 23. Can contracts be upgraded? If yes, what is the intended upgrade process?

Answer:

### 24. What emergency controls exist and what are their intended limits?

Examples: pause, unpause, freeze, rescue tokens, disable markets.

Answer:

## 6. Known Risks

### 25. Known issues, accepted risks, or areas intentionally deferred

| Issue | Affected area | Accepted / deferred? | Reason |
| --- | --- | --- | --- |
|  |  |  |  |

### 26. Prior audits or internal reviews

Add file links, URLs, or short notes.

Answer:

### 27. Any areas you are especially worried about?

Answer:

### 28. Any areas you do not want auditors to spend time on?

Answer:

## 7. Reporting Preferences

### 29. Should Low / QA / Info findings be included?

Choose one:

- [ ] Yes
- [ ] No

Notes:

### 30. Any severity edge cases we should handle differently from the default private-audit rubric?

Only mention exceptions, not full severity definitions.

Answer:

### 31. Preferred output format

Check all that apply:

- [ ] Markdown
- [ ] PDF
- [ ] GitHub issues
- [ ] CSV summary
- [ ] Executive summary

Notes:

### 32. Anything else auditors should know?

Answer:
