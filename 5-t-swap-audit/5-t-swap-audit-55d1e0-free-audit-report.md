# 5 t swap audit - Findings Report
## Commit hash: 55d1e086ed0917fd055b14f63099c2342eb6b86a

## Protocol Overview 

**TSwap** is a lightweight Automated Market Maker (AMM) similar to Uniswap v1.

1. **Pool creation**  
• The `PoolFactory` contract holds the canonical WETH address and can spawn a new `TSwapPool` for any ERC-20 token that doesn’t yet have one. The factory records both token→pool and pool→token mappings for easy lookup.

2. **Pool mechanics**  
• Every `TSwapPool` manages two reserves: the chosen ERC-20 and WETH.  
• Liquidity providers deposit both assets proportionally via `deposit`, receiving ERC20 LP tokens (the pool itself is an ERC20) that represent their share.  
• Withdrawals burn LP tokens and return the underlying assets.  
• Swaps are executed with `swapExactInput` or `swapExactOutput`, using the constant-product invariant x*y=k and charging a 0.3 % fee (applied as a 997/1000 multiplier) that stays in the pool, enriching LPs.

3. **Deployment**  
A Foundry script (`DeployTSwap.t.sol`) deploys the factory. On mainnet it wires in canonical WETH; on local tests it can deploy a mock WETH token.

With ~370 SLOC and no admin keys, TSwap offers a simple, permissionless way to trade or provide liquidity between any ERC-20 and ETH.
## High Risk Findings
[H-1]. Accounting Invariant Violation issue found with High severity


### Number of Findings
- C: 0
- H: 1
- M: 0
- L: 0
- I: 0



