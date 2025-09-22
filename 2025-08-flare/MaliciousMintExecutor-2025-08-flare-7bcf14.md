
### Storage layout (MaliciousMintExecutor) 

```text
liquidationStartedTs uint256
reserved uint256
minted uint256
poolCR uint256
vaultCR uint256

```
#### MaliciousMintExecutor.constructor(address,address,address,address) [PUBLIC]
```slithir
 diamond = _diamond
diamond_1(address) := _diamond_1(address)
 agentVault = _agentVault
agentVault_1(address) := _agentVault_1(address)
 minter = _minter
minter_1(address) := _minter_1(address)
 fasset = _fasset
fasset_1(address) := _fasset_1(address)
```
#### MaliciousMintExecutor.fallback() [EXTERNAL]
```slithir
 proceed()
INTERNAL_CALL, MaliciousMintExecutor.proceed()()
```
#### MaliciousMintExecutor.mint(IPayment.Proof,uint256) [EXTERNAL]
```slithir
diamond_2(address) := phi(['diamond_0', 'diamond_3', 'diamond_13', 'diamond_1'])
 IAssetManager(diamond).executeMinting(_proof,_collateralReservationId)
TMP_5497 = CONVERT diamond_2 to IAssetManager
HIGH_LEVEL_CALL, dest:TMP_5497(IAssetManager), function:executeMinting, arguments:['_proof_1', '_collateralReservationId_1']  
diamond_3(address) := phi(['diamond_13', 'diamond_2', 'diamond_3', 'diamond_1'])
```

#### IAssetManager.executeMinting(IPayment.Proof,uint256) [EXTERNAL]
```slithir

```
#### IAssetManager.getAgentInfo(address) [EXTERNAL]
```slithir

```
#### IAssetManager.liquidate(address,uint256) [EXTERNAL]
```slithir

```
#### IAssetManager.startLiquidation(address) [EXTERNAL]
```slithir

```
