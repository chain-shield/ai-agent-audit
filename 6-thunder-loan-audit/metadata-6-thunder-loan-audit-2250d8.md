
## PROTOCOL OVERVIEW:

**ThunderLoan Protocol**

ThunderLoan is an up-gradable flash-loan marketplace inspired by Aave.
Liquidity providers deposit whitelisted ERC-20 tokens into per-asset “AssetToken” vaults. When you deposit, the contract mints interest-bearing AssetTokens that track an ever-increasing exchange-rate; your balance always represents your share of the underlying pool.

Borrowers can, in a single transaction, borrow any available token with no collateral, receive it in their own contract, perform arbitrary logic, and must repay *amount + flat fee* before the tx ends. ThunderLoan immediately transfers the fee to the relevant AssetToken and bumps the exchange-rate, streaming yield to every depositor.

Core contracts
• **ThunderLoan (UUPS-upgradeable)** – orchestrates deposits, redemptions, flash-loans, manages token whitelist, and sets the global fee.
• **AssetToken** – ERC-20 wrapper that holds the underlying funds and enforces mint/burn/actions from ThunderLoan only.
• **OracleUpgradeable** – reads TSwap pools to price tokens in WETH for fair fee calculation.
• **ERC1967Proxy** – enables future upgrades to ThunderLoan logic via owner-authorized implementation changes.

Security notes: funds live in AssetToken contracts, not the logic contract; owner can list tokens, adjust fee, and upgrade implementation, so governance keys must be trusted.


## SUMMARY OF FILE: 6-thunder-loan-audit/script/DeployThunderLoan.s.sol
### DeployThunderLoan (Script)
Solidity v0.8.20.  This is a Foundry `Script` used exclusively for deployment and holds no user funds.  Trust is limited to the deployer EOAs that broadcast the transaction.  The script creates a fresh ThunderLoan implementation contract and immediately wraps it in an `ERC1967Proxy` to enable upgradeability.  No state is stored in this script; all user assets and admin rights live in ThunderLoan / the proxy.

Storage Variables
- *None*: Script keeps no persistent state.

Public / External Functions
1. **run() public** — non-payable, no modifiers.
   Natspec: Executes the broadcasted deployment sequence.  It starts an on-chain broadcast, deploys the ThunderLoan implementation, deploys an ERC1967Proxy pointing at that implementation with empty initializer data, and then stops broadcasting.  Intended to be called by Foundry during `forge script` execution.

No other external / public functions are present.

Because this contract is only a scripting helper, upgrade / admin risk resides in the `ThunderLoan` owner configured elsewhere, not here.


## SUMMARY OF FILE: 6-thunder-loan-audit/src/upgradedProtocol/ThunderLoanUpgraded.sol
### ThunderLoanUpgraded.sol

**Purpose (≤250 words)**  
Upgradable flash-loan pool where users deposit ERC20 assets and receive interest-bearing `AssetToken`s. Flash borrowers can borrow any listed token within one transaction and must return the amount plus a protocol fee. The fee is immediately streamed to depositors by increasing the `AssetToken` exchange-rate. Contract inherits `OwnableUpgradeable`, `UUPSUpgradeable`, and a trusted `OracleUpgradeable` (TSwap price oracle) to value borrowings. Funds reside in the individual `AssetToken` contracts; `ThunderLoanUpgraded` only orchestrates flows.

**Trust / Permissions**  
• Owner: add/remove allowed tokens, change flat fee, and authorize implementation upgrades.  
• Oracle: off-chain price accuracy required for correct fee.  
• Users: supply/withdraw own funds; interact permissionlessly.  
Centralization risk if owner misbehaves or oracle manipulated.

**Storage Variables**  
• `s_tokenToAssetToken` – mapping token→asset wrapper  
• `s_flashLoanFee` – flat fee (18-dec. WETH)  
• `FEE_PRECISION` – 1e18 constant  
• `s_currentlyFlashLoaning` – re-entrancy guard per token

**External/Public Functions**  
1. `initialize(address tswap)` external initializer nonpayable – sets owner, oracle, fee.  
2. `deposit(IERC20 token,uint256 amount)` external nonpayable – mint `AssetToken`s; updates rate with fee.  
3. `redeem(IERC20 token,uint256 amtAsset)` external nonpayable – burn asset, transfer underlying.  
4. `flashloan(address recv,IERC20 token,uint256 amt,bytes params)` external nonpayable – issues loan, calls `IFlashLoanReceiver.executeOperation`, asserts repayment+fee.  
5. `repay(IERC20 token,uint256 amount)` public nonpayable – helper for borrower to return funds during loan.  
6. `setAllowedToken(IERC20 token,bool allowed)` external onlyOwner nonpayable returns `AssetToken` – whitelist or revoke token and deploy wrapper.  
7. `getCalculatedFee(IERC20 token,uint256 amount)` public view – quote fee.  
8. `updateFlashLoanFee(uint256 newFee)` external onlyOwner nonpayable – change fee (≤1e18).  
9. `isAllowedToken(IERC20 token)` public view – check whitelist.  
10. `getAssetFromToken(IERC20 token)` public view.  
11. `isCurrentlyFlashLoaning(IERC20 token)` public view.  
12. `getFee()` external view – current flat fee.

**NatSpec (≤50 words each)**  
• initialize – one-time initializer; wires oracle and default fee.  
• deposit – transfer underlying, mint wrapper, distribute fee.  
• redeem – burn wrapper; withdraw proportional underlying.  
• flashloan – lend token, insure repayment+fee within same tx.  
• repay – borrower utility to send funds back.  
• setAllowedToken – owner whitelists token & deploys `AssetToken` wrapper.  
• getCalculatedFee – view fee in underlying units.  
• updateFlashLoanFee – owner can tweak flat fee.  
• isAllowedToken / getAssetFromToken / isCurrentlyFlashLoaning / getFee – pure views for UI & guards.


## SUMMARY OF FILE: 6-thunder-loan-audit/src/protocol/AssetToken.sol
### AssetToken (≈210 words)

Purpose & Trust Model
- Interest-bearing ERC20 that tracks deposits in ThunderLoan. One AssetToken is redeemable for `exchangeRate` units of the underlying ERC20.  
- Only the ThunderLoan contract (single trusted admin) may mint, burn, move underlying or update rates; holders rely on ThunderLoan’s honesty but retain custody of their AssetTokens.

Primary Storage
- `IERC20 i_underlying` – underlying asset address.
- `address i_thunderLoan` – authorised ThunderLoan controller.
- `uint256 s_exchangeRate` – underlying per AssetToken.
- `uint256 constant EXCHANGE_RATE_PRECISION` – 1e18, maths scalar.
- `uint256 constant STARTING_EXCHANGE_RATE` – 1e18, initial rate.

External / Public API
1. `constructor(address thunderLoan, IERC20 underlying, string assetName, string assetSymbol) nonpayable`  
   ▸ Deploys token, sets immutable controller & underlying, seed exchange rate. Reverts on zero addresses.

2. `mint(address to, uint256 amount) external onlyThunderLoan` nonpayable  
   ▸ ThunderLoan mints AssetTokens to `to` when users deposit.

3. `burn(address account, uint256 amount) external onlyThunderLoan` nonpayable  
   ▸ ThunderLoan burns AssetTokens on withdrawal.

4. `transferUnderlyingTo(address to, uint256 amount) external onlyThunderLoan` nonpayable  
   ▸ Sends underlying tokens from the contract to recipient, used during withdrawals or flash-loan execution.

5. `updateExchangeRate(uint256 fee) external onlyThunderLoan` nonpayable  
   ▸ Adjusts `s_exchangeRate` upwards after collecting flash-loan fee; reverts if new rate ≤ old rate.

6. `getExchangeRate() external view returns (uint256)`  
   ▸ Read current exchange rate (scaled 1e18).

7. `getUnderlying() external view returns (IERC20)`  
   ▸ Returns the underlying token address.



## SUMMARY OF FILE: 6-thunder-loan-audit/src/protocol/ThunderLoan.sol
### ThunderLoan.sol (v0.8.20)

Flash-loan & liquidity pool core.  Liquidity providers deposit ERC20s and receive interest-bearing AssetTokens.  Borrowers take zero-collateral flash-loans that must be repaid (plus fee) in the same tx.  Owner can list tokens, tune fee and upgrade via UUPS.

Trust / fund flow
* User funds are held in freshly deployed AssetToken contracts (one per ERC20).
* Protocol owner (EOA) can add/remove supported tokens and change fee; cannot move user funds directly but keeps powerful control.
* OracleUpgradeable prices set external fee reference.

Major entrypoints
```solidity
initialize(address tswap) external initializer
deposit(IERC20 token,uint256 amount) external
redeem(IERC20 token,uint256 assetAmt) external
flashloan(address receiver,IERC20 token,uint256 amt,bytes params) external
repay(IERC20 token,uint256 amt) public
setAllowedToken(IERC20 token,bool allowed) external onlyOwner
updateFlashLoanFee(uint256 newFee) external onlyOwner
// views
getCalculatedFee(...) public view
isAllowedToken(...) public view
getAssetFromToken(...) public view
isCurrentlyFlashLoaning(...) public view
getFee() external view
getFeePrecision() external view
```

Storage variables
| Name | Type | Description (≤50 chars) |
|------|------|-------------------------|
| s_tokenToAssetToken | mapping(IERC20⇒AssetToken) | ERC20 → wrapper AssetToken |
| s_feePrecision | uint256 | 1e18 denominator |
| s_flashLoanFee | uint256 | fee ratio (18 dec) |
| s_currentlyFlashLoaning | mapping(IERC20⇒bool) | re-entrancy guard per token |

Public/External function details
1. initialize – external, initializer, nonpayable.  Sets owner, Oracle, fee 0.3%.
2. deposit – external, revertIfZero, revertIfNotAllowedToken.  Mints AssetTokens and updates exchange rate with fee.
3. redeem – external, same modifiers.  Burns AssetTokens, transfers underlying.
4. flashloan – external, same mods, emits FlashLoan.  Sends tokens to receiver and enforces payback+fee.
5. repay – public, nonpayable.  Allows borrower to push tokens during loan.
6. setAllowedToken – external onlyOwner.  Deploys/tears down AssetToken wrappers.
7. getCalculatedFee – view pure math price×rate.
8. updateFlashLoanFee – external onlyOwner.  Bounds new fee ≤ precision.
9-13. remaining view helpers expose state.




## SUMMARY OF FILE: 6-thunder-loan-audit/src/protocol/OracleUpgradeable.sol
### `OracleUpgradeable` (upgradeable library-style contract)

Purpose & Trust Model:
- Lightweight on-chain oracle wrapper that converts any ERC-20 price into WETH terms by querying a TSwap pool fetched from an `IPoolFactory`.
- Holds **no user funds**. Single privileged action is the once-only initializer that sets the factory address when the proxy is deployed. After that it is read-only.
- Depends on external `PoolFactory` and `TSwapPool` contracts to return honest prices.

Storage Variables:
1. `address s_poolFactory` – Address of PoolFactory that maps tokens → swap pool. (private)

Public / External API:

| Function signature | State mutability | Visibility / modifiers | NatSpec (<50w) |
| --- | --- | --- | --- |
| `getPriceInWeth(address token) → uint256` | view | public | Returns the current price of 1 pool token denominated in WETH by calling the token’s associated TSwap pool. |
| `getPrice(address token) → uint256` | view | external | Convenience alias that forwards to `getPriceInWeth` for interface uniformity. |
| `getPoolFactoryAddress() → address` | view | external | Exposes the stored PoolFactory address so external contracts/users can verify the data-source being used. |

Initializers (internal, not part of API): `__Oracle_init`, `__Oracle_init_unchained`—only callable during proxy setup to store the factory address.


## Main List of Files in Project

src/protocol/ThunderLoan.sol


 ## DOCUMENTATION: 

 ### report.md

# Aderyn Analysis Report

This report was generated by [Aderyn](https://github.com/Cyfrin/aderyn), a static analysis tool built by [Cyfrin](https://cyfrin.io), a blockchain security company. This report is not a substitute for manual audit or security review. It should not be relied upon for any purpose other than to assist in the identification of potential security vulnerabilities.
# Table of Contents

- [Summary](#summary)
  - [Files Summary](#files-summary)
  - [Files Details](#files-details)
  - [Issue Summary](#issue-summary)
- [Medium Issues](#medium-issues)
  - [M-1: Centralization Risk for trusted owners](#M-1)
  - [M-2: Using `ERC721::_mint()` can be dangerous](#M-2)
- [NC Issues](#nc-issues)
  - [NC-1: Missing checks for `address(0)` when assigning values to address state variables](#NC-1)
  - [NC-2: Functions not used internally could be marked external](#NC-2)
  - [NC-3: Constants should be defined and used instead of literals](#NC-3)
  - [NC-4: Event is missing `indexed` fields](#NC-4)


# Summary

## Files Summary

| Key | Value |
| --- | --- |
| .sol Files | 8 |
| Total nSLOC | 103 |


## Files Details

| Filepath | nSLOC |
| --- | --- |
| src/interfaces/IFlashLoanReceiver.sol | 13 |
| src/protocol/AssetToken.sol | 9 |
| src/interfaces/IThunderLoan.sol | 4 |
| src/interfaces/ITSwapPool.sol | 4 |
| src/protocol/ThunderLoan.sol | 23 |
| src/protocol/OracleUpgradeable.sol | 23 |
| src/upgradedProtocol/ThunderLoanUpgraded.sol | 23 |
| src/interfaces/IPoolFactory.sol | 4 |
| **Total** | **103** |


## Issue Summary

| Category | No. of Issues |
| --- | --- |
| Critical | 0 |
| High | 0 |
| Medium | 2 |
| Low | 0 |
| NC | 4 |


# Medium Issues

<a name="M-1"></a>
## M-1: Centralization Risk for trusted owners

Contracts have owners with privileged rights to perform admin tasks and need to be trusted to not perform malicious updates or drain funds.

- Found in src/protocol/ThunderLoan.sol: Line: 239
- Found in src/protocol/ThunderLoan.sol: Line: 265
- Found in src/protocol/ThunderLoan.sol: Line: 292
- Found in src/upgradedProtocol/ThunderLoanUpgraded.sol: Line: 235
- Found in src/upgradedProtocol/ThunderLoanUpgraded.sol: Line: 261
- Found in src/upgradedProtocol/ThunderLoanUpgraded.sol: Line: 284


<a name="M-2"></a>
## M-2: Using `ERC721::_mint()` can be dangerous

Using `ERC721::_mint()` can mint ERC721 tokens to addresses which don't support ERC721 tokens. Use `_safeMint()` instead of `_mint()` for ERC721.

- Found in src/protocol/AssetToken.sol: Line: 69


# NC Issues

<a name="NC-1"></a>
## NC-1: Missing checks for `address(0)` when assigning values to address state variables

Assigning values to address state variables without checking for `address(0)`.

- Found in src/protocol/OracleUpgradeable.sol: Line: 16


<a name="NC-2"></a>
## NC-2: Functions not used internally could be marked external



- Found in src/protocol/ThunderLoan.sol: Line: 280
- Found in src/upgradedProtocol/ThunderLoanUpgraded.sol: Line: 276
- Found in src/protocol/ThunderLoan.sol: Line: 272
- Found in src/upgradedProtocol/ThunderLoanUpgraded.sol: Line: 268
- Found in src/upgradedProtocol/ThunderLoanUpgraded.sol: Line: 272
- Found in src/protocol/ThunderLoan.sol: Line: 231
- Found in src/protocol/ThunderLoan.sol: Line: 276
- Found in src/upgradedProtocol/ThunderLoanUpgraded.sol: Line: 227


<a name="NC-3"></a>
## NC-3: Constants should be defined and used instead of literals



- Found in src/protocol/ThunderLoan.sol: Line: 144
- Found in src/protocol/ThunderLoan.sol: Line: 145
- Found in src/upgradedProtocol/ThunderLoanUpgraded.sol: Line: 144


<a name="NC-4"></a>
## NC-4: Event is missing `indexed` fields

Index event fields make the field more quickly accessible to off-chain tools that parse events. However, note that each index field costs extra gas during emission, so it's not necessarily best to index the maximum allowed per event (three fields). Each event should use three indexed fields if there are three or more fields, and gas usage is not particularly of concern for the events in question. If there are fewer than three fields, all of the fields should be indexed.

- Found in src/protocol/ThunderLoan.sol: Line: 106
- Found in src/protocol/ThunderLoan.sol: Line: 107
- Found in src/protocol/ThunderLoan.sol: Line: 110
- Found in src/protocol/AssetToken.sol: Line: 31
- Found in src/upgradedProtocol/ThunderLoanUpgraded.sol: Line: 105
- Found in src/upgradedProtocol/ThunderLoanUpgraded.sol: Line: 106
- Found in src/protocol/ThunderLoan.sol: Line: 105
- Found in src/upgradedProtocol/ThunderLoanUpgraded.sol: Line: 107
- Found in src/upgradedProtocol/ThunderLoanUpgraded.sol: Line: 110




### README.md

# Thunder Loan

<br/>
<p align="center">
<img src="./thunder-loan.svg" width="700" alt="thunder-loans">
</p>
<br/>


*A flash loan protocol based on [Aave](https://aave.com/) and [Compound](https://compound.finance/).*

You can learn more about how [Aave works](https://www.youtube.com/watch?v=dTCwssZ116A) at a high level from [this video](https://www.youtube.com/watch?v=dTCwssZ116A).


- [Thunder Loan](#thunder-loan)
- [About](#about)
- [Getting Started](#getting-started)
  - [Requirements](#requirements)
  - [Quickstart](#quickstart)
- [Usage](#usage)
  - [Testing](#testing)
    - [Test Coverage](#test-coverage)
- [Audit Scope Details](#audit-scope-details)
  - [Roles](#roles)
  - [Known Issues](#known-issues)

# About 

The ⚡️ThunderLoan⚡️ protocol is meant to do the following:

1. Give users a way to create flash loans
2. Give liquidity providers a way to earn money off their capital

Liquidity providers can `deposit` assets into `ThunderLoan` and be given `AssetTokens` in return. These `AssetTokens` gain interest over time depending on how often people take out flash loans!

What is a flash loan? 

A flash loan is a loan that exists for exactly 1 transaction. A user can borrow any amount of assets from the protocol as long as they pay it back in the same transaction. If they don't pay it back, the transaction reverts and the loan is cancelled.

Users additionally have to pay a small fee to the protocol depending on how much money they borrow. To calculate the fee, we're using the famous on-chain TSwap price oracle.

We are planning to upgrade from the current `ThunderLoan` contract to the `ThunderLoanUpgraded` contract. Please include this upgrade in scope of a security review. 

# Getting Started

## Requirements

- [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
  - You'll know you did it right if you can run `git --version` and you see a response like `git version x.x.x`
- [foundry](https://getfoundry.sh/)
  - You'll know you did it right if you can run `forge --version` and you see a response like `forge 0.2.0 (816e00b 2023-03-16T00:05:26.396218Z)`

## Quickstart

```
git clone https://github.com/Cyfrin/6-thunder-loan-audit
cd 6-thunder-loan-audit
make 
```

# Usage

## Testing

```
forge test
```

### Test Coverage

```
forge coverage
```

and for coverage based testing: 

```
forge coverage --report debug
```

# Audit Scope Details

- Commit Hash: 8803f851f6b37e99eab2e94b4690c8b70e26b3f6
- In Scope:
```
#-- interfaces
|   #-- IFlashLoanReceiver.sol
|   #-- IPoolFactory.sol
|   #-- ITSwapPool.sol
|   #-- IThunderLoan.sol
#-- protocol
|   #-- AssetToken.sol
|   #-- OracleUpgradeable.sol
|   #-- ThunderLoan.sol
#-- upgradedProtocol
    #-- ThunderLoanUpgraded.sol
```
- Solc Version: 0.8.20
- Chain(s) to deploy contract to: Ethereum
- ERC20s:
  - USDC 
  - DAI
  - LINK
  - WETH

## Roles

- Owner: The owner of the protocol who has the power to upgrade the implementation. 
- Liquidity Provider: A user who deposits assets into the protocol to earn interest. 
- User: A user who takes out flash loans from the protocol.

## Known Issues

- We are aware that `getCalculatedFee` can result in 0 fees for very small flash loans. We are OK with that. There is some small rounding errors when it comes to low fees
- We are aware that the first depositor gets an unfair advantage in assetToken distribution. We will be making a large initial deposit to mitigate this, and this is a known issue
- We are aware that "weird" ERC20s break the protocol, including fee-on-transfer, rebasing, and ERC-777 tokens. The owner will vet any additional tokens before adding them to the protocol. 


 ## CONFIG FILES: 

 ### foundry.toml

[profile.default]
src = "src"
out = "out"
libs = ["lib"]
remappings = [
    '@openzeppelin/contracts=lib/openzeppelin-contracts/contracts',
    '@openzeppelin/contracts-upgradeable=lib/openzeppelin-contracts-upgradeable/contracts',
]
solc = '0.8.20'

[fmt]
bracket_spacing = true
int_types = "long"
line_length = 120
multiline_func_header = "all"
number_underscore = "thousands"
quote_style = "double"
tab_width = 4
wrap_comments = true


# See more config options https://github.com/foundry-rs/foundry/blob/master/crates/config/README.md#all-options


