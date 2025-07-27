```markdown
# Public Disclosure of Known Issues

Bug reports for publicly disclosed bugs are **not eligible** for a reward.

**None**

---

## Is this an upgrade of an existing system?

**No** – from the security-reviewer’s perspective, this should be treated as a fresh, stand-alone system.

---

## What ERC20 / ERC721 / ERC777 / ERC1155 token standards are supported?

- **ERC-20**  
  - The native stake/reward token `$PLUME` functions like an ERC-20 (plus an “ETH-style” `0xEeee...` sentinel for native transfers).  
  - Any ERC-20 can be added as a reward token via `addRewardToken`.
- **Not supported:**  
  - ERC-721  
  - ERC-777  
  - ERC-1155  

The staking contracts themselves do not include direct support for NFT or multi-token standards.

---

## What emergency actions may you want to use as a reason to downgrade an otherwise valid bug report?

- The bug requires privileged roles (`TIMELOCK_ROLE` / `ADMIN_ROLE`) to misconfigure a contract in a way that is already disallowed by policy.  
- The issue is a **gas optimization** or **low-severity DOS** that does not cause loss of funds or permanent unavailability.  
- Attacks that rely on an external system (e.g., the treasury implementation, oracle feeds, validators’ off-chain behavior) operating maliciously within their allowed privileges.  
- Issues that only affect **test/script code** or **out-of-scope directories**.  
- Findings that depend on the administrator intentionally performing malicious actions.

---

## What addresses would you consider any bug report requiring their involvement to be out of scope, even if they exceed the privileges attributed to them?

The addresses holding the following roles are considered **trusted**, and their actions, when performed within the defined capabilities of their roles, are **out of scope**:

- **ADMIN_ROLE / TIMELOCK_ROLE**: The system's highest-level administrators.  
- **VALIDATOR_ROLE**: The role responsible for adding and removing validators from the set.  
- **REWARD_MANAGER_ROLE**: The role responsible for managing reward tokens and their emission rates.  
- **l2AdminAddress (Validator Admin)**: The address that manages a specific validator's commission and other settings.

---

## What external dependencies are there?

The project has the following key external dependencies:

- **solidstate-solidity**: Used for the core Diamond Proxy architecture.  
- **@openzeppelin/contracts-upgradeable**: Used for standard, secure, and upgradeable components like `ReentrancyGuardUpgradeable` and `SafeERC20`.
```

