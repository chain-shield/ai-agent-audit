# 4 puppy raffle audit - Findings Report
## Commit hash: 3ff0f0bfddf25fd0c160fe57388fa6ff2e0f0960

##Findings by Status


Finding Status: Valid


[H-1]. Reentrancy in refund() allows draining contract balance via CEI violation
**Derived From** : Reentrancy via Checks-Effects-Interactions Violation in refund()
Finding Status: Valid
Finding Complexity: 0
Privilege: Permissionless


[M-2]. DoS via unbounded nested loop in enterRaffle
**Derived From** : Nested loop in enterRaffle causes Denial of Service
Finding Status: Valid
Finding Complexity: 0
Privilege: Permissionless


[M-3]. Strict equality check in withdrawFees allows DoS via forced ETH
**Derived From** : Strict Balance Check Lockup in withdrawFees
Finding Status: Valid
Finding Complexity: 0
Privilege: Permissionless


[H-4]. Insolvency in selectWinner due to accounting mismatch with refunds
**Derived From** : Prize Pool Accounting Mismatch due to Refunds
Finding Status: Valid
Finding Complexity: 0
Privilege: Permissionless


[M-5]. Integer Overflow and Truncation in totalFees Accounting locks fees
**Derived From** : Unsafe casting and overflow of totalFees bricks fee withdrawal
Finding Status: Valid
Finding Complexity: 0
Privilege: Permissionless


[M-6]. Weak Randomness allows manipulation of winner selection
**Derived From** : Weak PRNG using block.difficulty/timestamp allows winner manipulation
Finding Status: Valid
Finding Complexity: 0
Privilege: Permissionless


[H-7]. Missing totalSupply() causes selectWinner to revert
**Derived From** : Missing totalSupply implementation prevents minting
Finding Status: Valid
Finding Complexity: 0
Privilege: Permissionless

### Number of Findings
- C: 0
- H: 3
- M: 4
- L: 0
- I: 0

##Findings by Status
