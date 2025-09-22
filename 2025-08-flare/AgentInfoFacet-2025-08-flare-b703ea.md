










### Storage layout (ERC20) 

```text
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string

```
### Storage layout (AgentOwnerRegistry) 

```text
manager address
whitelist mapping(address => bool)
workToMgmtAddress mapping(address => address)
mgmtToWorkAddress mapping(address => address)
agentName mapping(address => string)
agentDescription mapping(address => string)
agentIconUrl mapping(address => string)
agentTouUrl mapping(address => string)

```




#### AgentInfoFacet._getFullCollateral(address,Collateral.Kind) [PRIVATE]
```slithir
_agentVault_1(address) := phi(['_agentVault_1', '_agentVault_1'])
_kind_1(Collateral.Kind) := phi(['REF_631', 'REF_632'])
 agent = Agent.get(_agentVault)
TMP_1561(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_1561'])(Agent.State) := TMP_1561(Agent.State)
 collateral = AgentCollateral.singleCollateralData(agent,_kind)
TMP_1562(Collateral.Data) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.singleCollateralData(Agent.State,Collateral.Kind), arguments:["agent_1 (-> ['TMP_1561'])", '_kind_1'] 
collateral_1(Collateral.Data) := TMP_1562(Collateral.Data)
 collateral.fullCollateral
REF_639(uint256) -> collateral_1.fullCollateral
RETURN REF_639
```

#### AgentInfoFacet._getMinCollateralRatioBIPS(address,Collateral.Kind) [PRIVATE]
```slithir
_agentVault_1(address) := phi(['_agentVault_1', '_agentVault_1'])
_kind_1(Collateral.Kind) := phi(['REF_636', 'REF_635'])
 agent = Agent.get(_agentVault)
TMP_1563(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_1563'])(Agent.State) := TMP_1563(Agent.State)
 (None,sysMinCR) = AgentCollateral.mintingMinCollateralRatio(agent,_kind)
TUPLE_17(uint256,uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.mintingMinCollateralRatio(Agent.State,Collateral.Kind), arguments:["agent_1 (-> ['TMP_1563'])", '_kind_1'] 
sysMinCR_1(uint256)= UNPACK TUPLE_17 index: 1 
 sysMinCR
RETURN sysMinCR_1
```
#### AgentInfoFacet.getAgentFullPoolCollateral(address) [EXTERNAL]
```slithir
 _getFullCollateral(_agentVault,Collateral.Kind.POOL)
REF_632(Collateral.Kind) -> Kind.POOL
TMP_1556(uint256) = INTERNAL_CALL, AgentInfoFacet._getFullCollateral(address,Collateral.Kind)(_agentVault_1,REF_632)
RETURN TMP_1556
```
#### AgentInfoFacet.getAgentFullVaultCollateral(address) [EXTERNAL]
```slithir
 _getFullCollateral(_agentVault,Collateral.Kind.VAULT)
REF_631(Collateral.Kind) -> Kind.VAULT
TMP_1555(uint256) = INTERNAL_CALL, AgentInfoFacet._getFullCollateral(address,Collateral.Kind)(_agentVault_1,REF_631)
RETURN TMP_1555
```
#### AgentInfoFacet.getAgentInfo(address) [EXTERNAL]
```slithir
 agent = Agent.getAllowDestroyed(_agentVault)
TMP_1522(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.getAllowDestroyed(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_1522'])(Agent.State) := TMP_1522(Agent.State)
 collateralData = AgentCollateral.combinedData(agent)
TMP_1523(Collateral.CombinedData) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.combinedData(Agent.State), arguments:["agent_1 (-> ['TMP_1522'])"] 
collateralData_1(Collateral.CombinedData) := TMP_1523(Collateral.CombinedData)
 collateral = agent.getVaultCollateral()
TMP_1524(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getVaultCollateral(Agent.State), arguments:["agent_1 (-> ['TMP_1522'])"] 
collateral_1 (-> ['TMP_1524'])(CollateralTypeInt.Data) := TMP_1524(CollateralTypeInt.Data)
 poolCollateral = agent.getPoolCollateral()
TMP_1525(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getPoolCollateral(Agent.State), arguments:["agent_1 (-> ['TMP_1522'])"] 
poolCollateral_1 (-> ['TMP_1525'])(CollateralTypeInt.Data) := TMP_1525(CollateralTypeInt.Data)
 collateralPool = agent.collateralPool
REF_524(IICollateralPool) -> agent_1 (-> ['TMP_1522']).collateralPool
collateralPool_1(IICollateralPool) := REF_524(IICollateralPool)
 cr = Liquidation.getCollateralRatiosBIPS(agent)
TMP_1526(Liquidation.CRData) = LIBRARY_CALL, dest:Liquidation, function:Liquidation.getCollateralRatiosBIPS(Agent.State), arguments:["agent_1 (-> ['TMP_1522'])"] 
cr_1(Liquidation.CRData) := TMP_1526(Liquidation.CRData)
 _info.status = Agents.getAgentStatus(agent)
REF_526(AgentInfo.Status) -> _info_0.status
TMP_1527(AgentInfo.Status) = LIBRARY_CALL, dest:Agents, function:Agents.getAgentStatus(Agent.State), arguments:["agent_1 (-> ['TMP_1522'])"] 
_info_1(AgentInfo.Info) := phi(['_info_0'])
REF_526(AgentInfo.Status) (->_info_1) := TMP_1527(AgentInfo.Status)
 _info.ownerManagementAddress = agent.ownerManagementAddress
REF_528(address) -> _info_1.ownerManagementAddress
REF_529(address) -> agent_1 (-> ['TMP_1522']).ownerManagementAddress
_info_2(AgentInfo.Info) := phi(['_info_1'])
REF_528(address) (->_info_2) := REF_529(address)
 _info.ownerWorkAddress = Agents.getWorkAddress(agent)
REF_530(address) -> _info_2.ownerWorkAddress
TMP_1528(address) = LIBRARY_CALL, dest:Agents, function:Agents.getWorkAddress(Agent.State), arguments:["agent_1 (-> ['TMP_1522'])"] 
_info_3(AgentInfo.Info) := phi(['_info_2'])
REF_530(address) (->_info_3) := TMP_1528(address)
 _info.collateralPool = address(collateralPool)
REF_532(address) -> _info_3.collateralPool
TMP_1529 = CONVERT collateralPool_1 to address
_info_4(AgentInfo.Info) := phi(['_info_3'])
REF_532(address) (->_info_4) := TMP_1529(address)
 _info.collateralPoolToken = address(collateralPool.poolToken())
REF_533(address) -> _info_4.collateralPoolToken
TMP_1530(ICollateralPoolToken) = HIGH_LEVEL_CALL, dest:collateralPool_1(IICollateralPool), function:poolToken, arguments:[]  
TMP_1531 = CONVERT TMP_1530 to address
_info_5(AgentInfo.Info) := phi(['_info_4'])
REF_533(address) (->_info_5) := TMP_1531(address)
 _info.underlyingAddressString = agent.underlyingAddressString
REF_535(string) -> _info_5.underlyingAddressString
REF_536(string) -> agent_1 (-> ['TMP_1522']).underlyingAddressString
_info_6(AgentInfo.Info) := phi(['_info_5'])
REF_535(string) (->_info_6) := REF_536(string)
 _info.publiclyAvailable = agent.availableAgentsPos != 0
REF_537(bool) -> _info_6.publiclyAvailable
REF_538(uint32) -> agent_1 (-> ['TMP_1522']).availableAgentsPos
TMP_1532(bool) = REF_538 != 0
_info_7(AgentInfo.Info) := phi(['_info_6'])
REF_537(bool) (->_info_7) := TMP_1532(bool)
 _info.vaultCollateralToken = collateral.token
REF_539(IERC20) -> _info_7.vaultCollateralToken
REF_540(IERC20) -> collateral_1 (-> ['TMP_1524']).token
_info_8(AgentInfo.Info) := phi(['_info_7'])
REF_539(IERC20) (->_info_8) := REF_540(IERC20)
 _info.feeBIPS = agent.feeBIPS
REF_541(uint256) -> _info_8.feeBIPS
REF_542(uint16) -> agent_1 (-> ['TMP_1522']).feeBIPS
_info_9(AgentInfo.Info) := phi(['_info_8'])
REF_541(uint256) (->_info_9) := REF_542(uint16)
 _info.poolFeeShareBIPS = agent.poolFeeShareBIPS
REF_543(uint256) -> _info_9.poolFeeShareBIPS
REF_544(uint16) -> agent_1 (-> ['TMP_1522']).poolFeeShareBIPS
_info_10(AgentInfo.Info) := phi(['_info_9'])
REF_543(uint256) (->_info_10) := REF_544(uint16)
 _info.mintingVaultCollateralRatioBIPS = Math.max(agent.mintingVaultCollateralRatioBIPS,collateral.minCollateralRatioBIPS)
REF_545(uint256) -> _info_10.mintingVaultCollateralRatioBIPS
REF_547(uint32) -> agent_1 (-> ['TMP_1522']).mintingVaultCollateralRatioBIPS
REF_548(uint32) -> collateral_1 (-> ['TMP_1524']).minCollateralRatioBIPS
TMP_1533(uint256) = LIBRARY_CALL, dest:Math, function:Math.max(uint256,uint256), arguments:['REF_547', 'REF_548'] 
_info_11(AgentInfo.Info) := phi(['_info_10'])
REF_545(uint256) (->_info_11) := TMP_1533(uint256)
 _info.mintingPoolCollateralRatioBIPS = Math.max(agent.mintingPoolCollateralRatioBIPS,poolCollateral.minCollateralRatioBIPS)
REF_549(uint256) -> _info_11.mintingPoolCollateralRatioBIPS
REF_551(uint32) -> agent_1 (-> ['TMP_1522']).mintingPoolCollateralRatioBIPS
REF_552(uint32) -> poolCollateral_1 (-> ['TMP_1525']).minCollateralRatioBIPS
TMP_1534(uint256) = LIBRARY_CALL, dest:Math, function:Math.max(uint256,uint256), arguments:['REF_551', 'REF_552'] 
_info_12(AgentInfo.Info) := phi(['_info_11'])
REF_549(uint256) (->_info_12) := TMP_1534(uint256)
 _info.freeCollateralLots = collateralData.freeCollateralLots(agent)
REF_553(uint256) -> _info_12.freeCollateralLots
TMP_1535(uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.freeCollateralLots(Collateral.CombinedData,Agent.State), arguments:['collateralData_1', "agent_1 (-> ['TMP_1522'])"] 
_info_13(AgentInfo.Info) := phi(['_info_12'])
REF_553(uint256) (->_info_13) := TMP_1535(uint256)
 _info.totalVaultCollateralWei = collateralData.agentCollateral.fullCollateral
REF_555(uint256) -> _info_13.totalVaultCollateralWei
REF_556(Collateral.Data) -> collateralData_1.agentCollateral
REF_557(uint256) -> REF_556.fullCollateral
_info_14(AgentInfo.Info) := phi(['_info_13'])
REF_555(uint256) (->_info_14) := REF_557(uint256)
 _info.freeVaultCollateralWei = collateralData.agentCollateral.freeCollateralWei(agent)
REF_558(uint256) -> _info_14.freeVaultCollateralWei
REF_559(Collateral.Data) -> collateralData_1.agentCollateral
TMP_1536(uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.freeCollateralWei(Collateral.Data,Agent.State), arguments:['REF_559', "agent_1 (-> ['TMP_1522'])"] 
_info_15(AgentInfo.Info) := phi(['_info_14'])
REF_558(uint256) (->_info_15) := TMP_1536(uint256)
 _info.vaultCollateralRatioBIPS = cr.vaultCR
REF_561(uint256) -> _info_15.vaultCollateralRatioBIPS
REF_562(uint256) -> cr_1.vaultCR
_info_16(AgentInfo.Info) := phi(['_info_15'])
REF_561(uint256) (->_info_16) := REF_562(uint256)
 _info.poolWNatToken = poolCollateral.token
REF_563(IERC20) -> _info_16.poolWNatToken
REF_564(IERC20) -> poolCollateral_1 (-> ['TMP_1525']).token
_info_17(AgentInfo.Info) := phi(['_info_16'])
REF_563(IERC20) (->_info_17) := REF_564(IERC20)
 _info.totalPoolCollateralNATWei = collateralData.poolCollateral.fullCollateral
REF_565(uint256) -> _info_17.totalPoolCollateralNATWei
REF_566(Collateral.Data) -> collateralData_1.poolCollateral
REF_567(uint256) -> REF_566.fullCollateral
_info_18(AgentInfo.Info) := phi(['_info_17'])
REF_565(uint256) (->_info_18) := REF_567(uint256)
 _info.freePoolCollateralNATWei = collateralData.poolCollateral.freeCollateralWei(agent)
REF_568(uint256) -> _info_18.freePoolCollateralNATWei
REF_569(Collateral.Data) -> collateralData_1.poolCollateral
TMP_1537(uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.freeCollateralWei(Collateral.Data,Agent.State), arguments:['REF_569', "agent_1 (-> ['TMP_1522'])"] 
_info_19(AgentInfo.Info) := phi(['_info_18'])
REF_568(uint256) (->_info_19) := TMP_1537(uint256)
 _info.poolCollateralRatioBIPS = cr.poolCR
REF_571(uint256) -> _info_19.poolCollateralRatioBIPS
REF_572(uint256) -> cr_1.poolCR
_info_20(AgentInfo.Info) := phi(['_info_19'])
REF_571(uint256) (->_info_20) := REF_572(uint256)
 _info.totalAgentPoolTokensWei = collateralData.agentPoolTokens.fullCollateral
REF_573(uint256) -> _info_20.totalAgentPoolTokensWei
REF_574(Collateral.Data) -> collateralData_1.agentPoolTokens
REF_575(uint256) -> REF_574.fullCollateral
_info_21(AgentInfo.Info) := phi(['_info_20'])
REF_573(uint256) (->_info_21) := REF_575(uint256)
 _info.freeAgentPoolTokensWei = collateralData.agentPoolTokens.freeCollateralWei(agent)
REF_576(uint256) -> _info_21.freeAgentPoolTokensWei
REF_577(Collateral.Data) -> collateralData_1.agentPoolTokens
TMP_1538(uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.freeCollateralWei(Collateral.Data,Agent.State), arguments:['REF_577', "agent_1 (-> ['TMP_1522'])"] 
_info_22(AgentInfo.Info) := phi(['_info_21'])
REF_576(uint256) (->_info_22) := TMP_1538(uint256)
 _info.announcedVaultCollateralWithdrawalWei = agent.withdrawalAnnouncement(Collateral.Kind.VAULT).amountWei
REF_579(uint256) -> _info_22.announcedVaultCollateralWithdrawalWei
REF_581(Collateral.Kind) -> Kind.VAULT
TMP_1539(Agent.WithdrawalAnnouncement) = LIBRARY_CALL, dest:Agents, function:Agents.withdrawalAnnouncement(Agent.State,Collateral.Kind), arguments:["agent_1 (-> ['TMP_1522'])", 'REF_581'] 
REF_582(uint128) -> TMP_1539.amountWei
_info_23(AgentInfo.Info) := phi(['_info_22'])
REF_579(uint256) (->_info_23) := REF_582(uint128)
 _info.announcedPoolTokensWithdrawalWei = agent.withdrawalAnnouncement(Collateral.Kind.AGENT_POOL).amountWei
REF_583(uint256) -> _info_23.announcedPoolTokensWithdrawalWei
REF_585(Collateral.Kind) -> Kind.AGENT_POOL
TMP_1540(Agent.WithdrawalAnnouncement) = LIBRARY_CALL, dest:Agents, function:Agents.withdrawalAnnouncement(Agent.State,Collateral.Kind), arguments:["agent_1 (-> ['TMP_1522'])", 'REF_585'] 
REF_586(uint128) -> TMP_1540.amountWei
_info_24(AgentInfo.Info) := phi(['_info_23'])
REF_583(uint256) (->_info_24) := REF_586(uint128)
 _info.mintedUBA = Conversion.convertAmgToUBA(agent.mintedAMG)
REF_587(uint256) -> _info_24.mintedUBA
REF_589(uint64) -> agent_1 (-> ['TMP_1522']).mintedAMG
TMP_1541(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['REF_589'] 
_info_25(AgentInfo.Info) := phi(['_info_24'])
REF_587(uint256) (->_info_25) := TMP_1541(uint256)
 _info.reservedUBA = Conversion.convertAmgToUBA(agent.reservedAMG)
REF_590(uint256) -> _info_25.reservedUBA
REF_592(uint64) -> agent_1 (-> ['TMP_1522']).reservedAMG
TMP_1542(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['REF_592'] 
_info_26(AgentInfo.Info) := phi(['_info_25'])
REF_590(uint256) (->_info_26) := TMP_1542(uint256)
 _info.redeemingUBA = Conversion.convertAmgToUBA(agent.redeemingAMG)
REF_593(uint256) -> _info_26.redeemingUBA
REF_595(uint64) -> agent_1 (-> ['TMP_1522']).redeemingAMG
TMP_1543(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['REF_595'] 
_info_27(AgentInfo.Info) := phi(['_info_26'])
REF_593(uint256) (->_info_27) := TMP_1543(uint256)
 _info.poolRedeemingUBA = Conversion.convertAmgToUBA(agent.poolRedeemingAMG)
REF_596(uint256) -> _info_27.poolRedeemingUBA
REF_598(uint64) -> agent_1 (-> ['TMP_1522']).poolRedeemingAMG
TMP_1544(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['REF_598'] 
_info_28(AgentInfo.Info) := phi(['_info_27'])
REF_596(uint256) (->_info_28) := TMP_1544(uint256)
 _info.dustUBA = Conversion.convertAmgToUBA(agent.dustAMG)
REF_599(uint256) -> _info_28.dustUBA
REF_601(uint64) -> agent_1 (-> ['TMP_1522']).dustAMG
TMP_1545(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['REF_601'] 
_info_29(AgentInfo.Info) := phi(['_info_28'])
REF_599(uint256) (->_info_29) := TMP_1545(uint256)
 _info.liquidationStartTimestamp = agent.liquidationStartedAt
REF_602(uint256) -> _info_29.liquidationStartTimestamp
REF_603(uint64) -> agent_1 (-> ['TMP_1522']).liquidationStartedAt
_info_30(AgentInfo.Info) := phi(['_info_29'])
REF_602(uint256) (->_info_30) := REF_603(uint64)
 (_info.liquidationPaymentFactorVaultBIPS,_info.liquidationPaymentFactorPoolBIPS,_info.maxLiquidationAmountUBA) = _getLiquidationFactorsAndMaxAmount(agent,cr)
REF_604(uint256) -> _info_30.liquidationPaymentFactorVaultBIPS
REF_605(uint256) -> _info_30.liquidationPaymentFactorPoolBIPS
REF_606(uint256) -> _info_30.maxLiquidationAmountUBA
TUPLE_15(uint256,uint256,uint256) = INTERNAL_CALL, AgentInfoFacet._getLiquidationFactorsAndMaxAmount(Agent.State,Liquidation.CRData)(agent_1 (-> ['TMP_1522']),cr_1)
REF_604(uint256)= UNPACK TUPLE_15 index: 0 
REF_605(uint256)= UNPACK TUPLE_15 index: 1 
REF_606(uint256)= UNPACK TUPLE_15 index: 2 
 _info.underlyingBalanceUBA = agent.underlyingBalanceUBA
REF_607(int256) -> _info_30.underlyingBalanceUBA
REF_608(int128) -> agent_1 (-> ['TMP_1522']).underlyingBalanceUBA
_info_31(AgentInfo.Info) := phi(['_info_30'])
REF_607(int256) (->_info_31) := REF_608(int128)
 _info.requiredUnderlyingBalanceUBA = UnderlyingBalance.requiredUnderlyingUBA(agent)
REF_609(uint256) -> _info_31.requiredUnderlyingBalanceUBA
TMP_1546(uint256) = LIBRARY_CALL, dest:UnderlyingBalance, function:UnderlyingBalance.requiredUnderlyingUBA(Agent.State), arguments:["agent_1 (-> ['TMP_1522'])"] 
_info_32(AgentInfo.Info) := phi(['_info_31'])
REF_609(uint256) (->_info_32) := TMP_1546(uint256)
 _info.freeUnderlyingBalanceUBA = _info.underlyingBalanceUBA - _info.requiredUnderlyingBalanceUBA.toInt256()
REF_611(int256) -> _info_32.freeUnderlyingBalanceUBA
REF_612(int256) -> _info_32.underlyingBalanceUBA
REF_613(uint256) -> _info_32.requiredUnderlyingBalanceUBA
TMP_1547(int256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt256(uint256), arguments:['REF_613'] 
TMP_1548(int256) = REF_612 (c)- TMP_1547
_info_33(AgentInfo.Info) := phi(['_info_32'])
REF_611(int256) (->_info_33) := TMP_1548(int256)
 _info.announcedUnderlyingWithdrawalId = agent.announcedUnderlyingWithdrawalId
REF_615(uint256) -> _info_33.announcedUnderlyingWithdrawalId
REF_616(uint64) -> agent_1 (-> ['TMP_1522']).announcedUnderlyingWithdrawalId
_info_34(AgentInfo.Info) := phi(['_info_33'])
REF_615(uint256) (->_info_34) := REF_616(uint64)
 _info.buyFAssetByAgentFactorBIPS = agent.buyFAssetByAgentFactorBIPS
REF_617(uint256) -> _info_34.buyFAssetByAgentFactorBIPS
REF_618(uint16) -> agent_1 (-> ['TMP_1522']).buyFAssetByAgentFactorBIPS
_info_35(AgentInfo.Info) := phi(['_info_34'])
REF_617(uint256) (->_info_35) := REF_618(uint16)
 _info.poolExitCollateralRatioBIPS = agent.collateralPool.exitCollateralRatioBIPS()
REF_619(uint256) -> _info_35.poolExitCollateralRatioBIPS
REF_620(IICollateralPool) -> agent_1 (-> ['TMP_1522']).collateralPool
TMP_1549(uint32) = HIGH_LEVEL_CALL, dest:REF_620(IICollateralPool), function:exitCollateralRatioBIPS, arguments:[]  
_info_36(AgentInfo.Info) := phi(['_info_35'])
REF_619(uint256) (->_info_36) := TMP_1549(uint32)
 _info.redemptionPoolFeeShareBIPS = agent.redemptionPoolFeeShareBIPS
REF_622(uint256) -> _info_36.redemptionPoolFeeShareBIPS
REF_623(uint16) -> agent_1 (-> ['TMP_1522']).redemptionPoolFeeShareBIPS
_info_37(AgentInfo.Info) := phi(['_info_36'])
REF_622(uint256) (->_info_37) := REF_623(uint16)
 _info
RETURN _info_37
```
#### AgentInfoFacet.getAgentLiquidationFactorsAndMaxAmount(address) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_1557(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_1557'])(Agent.State) := TMP_1557(Agent.State)
 cr = Liquidation.getCollateralRatiosBIPS(agent)
TMP_1558(Liquidation.CRData) = LIBRARY_CALL, dest:Liquidation, function:Liquidation.getCollateralRatiosBIPS(Agent.State), arguments:["agent_1 (-> ['TMP_1557'])"] 
cr_1(Liquidation.CRData) := TMP_1558(Liquidation.CRData)
 _getLiquidationFactorsAndMaxAmount(agent,cr)
TUPLE_16(uint256,uint256,uint256) = INTERNAL_CALL, AgentInfoFacet._getLiquidationFactorsAndMaxAmount(Agent.State,Liquidation.CRData)(agent_1 (-> ['TMP_1557']),cr_1)
RETURN TUPLE_16
 (_liquidationPaymentFactorVaultBIPS,_liquidationPaymentFactorPoolBIPS,_maxLiquidationAmountUBA)
```
#### AgentInfoFacet.getAgentMinPoolCollateralRatioBIPS(address) [EXTERNAL]
```slithir
 _getMinCollateralRatioBIPS(_agentVault,Collateral.Kind.POOL)
REF_635(Collateral.Kind) -> Kind.POOL
TMP_1559(uint256) = INTERNAL_CALL, AgentInfoFacet._getMinCollateralRatioBIPS(address,Collateral.Kind)(_agentVault_1,REF_635)
RETURN TMP_1559
```
#### AgentInfoFacet.getAgentMinVaultCollateralRatioBIPS(address) [EXTERNAL]
```slithir
 _getMinCollateralRatioBIPS(_agentVault,Collateral.Kind.VAULT)
REF_636(Collateral.Kind) -> Kind.VAULT
TMP_1560(uint256) = INTERNAL_CALL, AgentInfoFacet._getMinCollateralRatioBIPS(address,Collateral.Kind)(_agentVault_1,REF_636)
RETURN TMP_1560
```
#### AgentInfoFacet.getAgentVaultCollateralToken(address) [EXTERNAL]
```slithir
 Agent.get(_agentVault).getVaultCollateral().token
TMP_1553(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
TMP_1554(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getVaultCollateral(Agent.State), arguments:['TMP_1553'] 
REF_630(IERC20) -> TMP_1554.token
RETURN REF_630
```
#### AgentInfoFacet.getAgentVaultOwner(address) [EXTERNAL]
```slithir
 Agent.getAllowDestroyed(_agentVault).ownerManagementAddress
TMP_1552(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.getAllowDestroyed(address), arguments:['_agentVault_1'] 
REF_627(address) -> TMP_1552.ownerManagementAddress
RETURN REF_627
 _ownerManagementAddress
```
#### AgentInfoFacet.getAllAgents(uint256,uint256) [EXTERNAL]
```slithir
 Agents.getAllAgents(_start,_end)
TUPLE_14(address[],uint256) = LIBRARY_CALL, dest:Agents, function:Agents.getAllAgents(uint256,uint256), arguments:['_start_1', '_end_1'] 
RETURN TUPLE_14
 (_agents,_totalLength)
```
#### AgentInfoFacet.getCollateralPool(address) [EXTERNAL]
```slithir
 address(Agent.getAllowDestroyed(_agentVault).collateralPool)
TMP_1550(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.getAllowDestroyed(address), arguments:['_agentVault_1'] 
REF_625(IICollateralPool) -> TMP_1550.collateralPool
TMP_1551 = CONVERT REF_625 to address
RETURN TMP_1551
```
#### AgentInfoFacet.isPoolTokenSuffixReserved(string) [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_1521(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_1521'])(AssetManagerState.State) := TMP_1521(AssetManagerState.State)
 state.reservedPoolTokenSuffixes[_suffix]
REF_518(mapping(string => bool)) -> state_1 (-> ['TMP_1521']).reservedPoolTokenSuffixes
REF_519(bool) -> REF_518[_suffix_1]
RETURN REF_519
```
#### AgentCollateral.singleCollateralData(Agent.State,Collateral.Kind) [INTERNAL]
```slithir
 _kind == Collateral.Kind.VAULT
REF_2815(Collateral.Kind) -> Kind.VAULT
TMP_4300(bool) = _kind_1 == REF_2815
CONDITION TMP_4300
 agentVaultCollateralData(_agent)
TMP_4301(Collateral.Data) = INTERNAL_CALL, AgentCollateral.agentVaultCollateralData(Agent.State)(_agent_1 (-> []))
RETURN TMP_4301
 _kind == Collateral.Kind.POOL
REF_2816(Collateral.Kind) -> Kind.POOL
TMP_4302(bool) = _kind_1 == REF_2816
CONDITION TMP_4302
 poolCollateralData(_agent)
TMP_4303(Collateral.Data) = INTERNAL_CALL, AgentCollateral.poolCollateralData(Agent.State)(_agent_1 (-> []))
RETURN TMP_4303
 agentsPoolTokensCollateralData(_agent,poolCollateralData(_agent))
TMP_4304(Collateral.Data) = INTERNAL_CALL, AgentCollateral.poolCollateralData(Agent.State)(_agent_1 (-> []))
TMP_4305(Collateral.Data) = INTERNAL_CALL, AgentCollateral.agentsPoolTokensCollateralData(Agent.State,Collateral.Data)(_agent_1 (-> []),TMP_4304)
RETURN TMP_4305
```
#### Agent.get(address) [INTERNAL]
```slithir
 agent = getWithoutCheck(_address)
TMP_5309(Agent.State) = INTERNAL_CALL, Agent.getWithoutCheck(address)(_address_1)
agent_1 (-> ['TMP_5309'])(Agent.State) := TMP_5309(Agent.State)
 status = agent.status
REF_3748(Agent.Status) -> agent_1 (-> ['TMP_5309']).status
status_1(Agent.Status) := REF_3748(Agent.Status)
 require(bool,error)(status != Agent.Status.EMPTY && status != Agent.Status.DESTROYED,revert InvalidAgentVaultAddress()())
REF_3749(Agent.Status) -> Status.EMPTY
TMP_5310(bool) = status_1 != REF_3749
REF_3750(Agent.Status) -> Status.DESTROYED
TMP_5311(bool) = status_1 != REF_3750
TMP_5312(bool) = TMP_5310 && TMP_5311
TMP_5313(None) = SOLIDITY_CALL revert InvalidAgentVaultAddress()()
TMP_5314(None) = SOLIDITY_CALL require(bool,error)(TMP_5312,TMP_5313)
 agent
RETURN agent_1 (-> ['TMP_5309'])
```
#### Conversion.convertAmgToUBA(uint64) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4637(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4637'])(AssetManagerSettings.Data) := TMP_4637(AssetManagerSettings.Data)
 uint256(_valueAMG) * settings.assetMintingGranularityUBA
TMP_4638 = CONVERT _valueAMG_1 to uint256
REF_3145(uint64) -> settings_1 (-> ['TMP_4637']).assetMintingGranularityUBA
TMP_4639(uint256) = TMP_4638 (c)* REF_3145
RETURN TMP_4639
```
#### Liquidation.maxLiquidationAmountAMG(Agent.State,uint256,uint256,Collateral.Kind) [INTERNAL]
```slithir
 _agent.status == Agent.Status.FULL_LIQUIDATION
REF_3274(Agent.Status) -> _agent_1 (-> []).status
REF_3275(Agent.Status) -> Status.FULL_LIQUIDATION
TMP_4761(bool) = REF_3274 == REF_3275
CONDITION TMP_4761
 _agent.mintedAMG
REF_3276(uint64) -> _agent_1 (-> []).mintedAMG
RETURN REF_3276
 targetRatioBIPS = _targetRatioBIPS(_agent,_collateralKind)
TMP_4762(uint256) = INTERNAL_CALL, Liquidation._targetRatioBIPS(Agent.State,Collateral.Kind)(_agent_1 (-> []),_collateralKind_1)
targetRatioBIPS_1(uint256) := TMP_4762(uint256)
 targetRatioBIPS <= _collateralRatioBIPS
TMP_4763(bool) = targetRatioBIPS_1 <= _collateralRatioBIPS_1
CONDITION TMP_4763
 0
RETURN 0
 _collateralRatioBIPS <= _factorBIPS
TMP_4764(bool) = _collateralRatioBIPS_1 <= _factorBIPS_1
CONDITION TMP_4764
 _agent.mintedAMG
REF_3277(uint64) -> _agent_1 (-> []).mintedAMG
RETURN REF_3277
 maxLiquidatedAMG = AgentCollateral.totalBackedAMG(_agent,_collateralKind).mulDivRoundUp(targetRatioBIPS - _collateralRatioBIPS,targetRatioBIPS - _factorBIPS)
TMP_4765(uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.totalBackedAMG(Agent.State,Collateral.Kind), arguments:['_agent_1 (-> [])', '_collateralKind_1'] 
TMP_4766(uint256) = targetRatioBIPS_1 (c)- _collateralRatioBIPS_1
TMP_4767(uint256) = targetRatioBIPS_1 (c)- _factorBIPS_1
TMP_4768(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDivRoundUp(uint256,uint256,uint256), arguments:['TMP_4765', 'TMP_4766', 'TMP_4767'] 
maxLiquidatedAMG_1(uint256) := TMP_4768(uint256)
 Math.min(maxLiquidatedAMG,_agent.mintedAMG)
REF_3281(uint64) -> _agent_1 (-> []).mintedAMG
TMP_4769(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['maxLiquidatedAMG_1', 'REF_3281'] 
RETURN TMP_4769
```
#### LiquidationPaymentStrategy.currentLiquidationFactorBIPS(Agent.State,uint256,uint256) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4783(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4783'])(AssetManagerSettings.Data) := TMP_4783(AssetManagerSettings.Data)
 step = _currentLiquidationStep(_agent)
TMP_4784(uint256) = INTERNAL_CALL, LiquidationPaymentStrategy._currentLiquidationStep(Agent.State)(_agent_1 (-> []))
step_1(uint256) := TMP_4784(uint256)
 factorBIPS = settings.liquidationCollateralFactorBIPS[step]
REF_3298(uint256[]) -> settings_1 (-> ['TMP_4783']).liquidationCollateralFactorBIPS
REF_3299(uint256) -> REF_3298[step_1]
factorBIPS_1(uint256) := REF_3299(uint256)
 _c1FactorBIPS = Math.min(settings.liquidationFactorVaultCollateralBIPS[step],factorBIPS)
REF_3301(uint256[]) -> settings_1 (-> ['TMP_4783']).liquidationFactorVaultCollateralBIPS
REF_3302(uint256) -> REF_3301[step_1]
TMP_4785(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['REF_3302', 'factorBIPS_1'] 
_c1FactorBIPS_1(uint256) := TMP_4785(uint256)
 vaultCollateral = _agent.getVaultCollateral()
TMP_4786(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getVaultCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
vaultCollateral_1 (-> ['TMP_4786'])(CollateralTypeInt.Data) := TMP_4786(CollateralTypeInt.Data)
 poolCollateral = _agent.getPoolCollateral()
TMP_4787(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getPoolCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
poolCollateral_1 (-> ['TMP_4787'])(CollateralTypeInt.Data) := TMP_4787(CollateralTypeInt.Data)
 ! vaultCollateral.isValid() && poolCollateral.isValid()
TMP_4788(bool) = LIBRARY_CALL, dest:CollateralTypes, function:CollateralTypes.isValid(CollateralTypeInt.Data), arguments:["vaultCollateral_1 (-> ['TMP_4786'])"] 
TMP_4789 = UnaryType.BANG TMP_4788 
TMP_4790(bool) = LIBRARY_CALL, dest:CollateralTypes, function:CollateralTypes.isValid(CollateralTypeInt.Data), arguments:["poolCollateral_1 (-> ['TMP_4787'])"] 
TMP_4791(bool) = TMP_4789 && TMP_4790
CONDITION TMP_4791
 _c1FactorBIPS = 0
_c1FactorBIPS_2(uint256) := 0(uint256)
 vaultCollateral.isValid() && ! poolCollateral.isValid()
TMP_4792(bool) = LIBRARY_CALL, dest:CollateralTypes, function:CollateralTypes.isValid(CollateralTypeInt.Data), arguments:["vaultCollateral_1 (-> ['TMP_4786'])"] 
TMP_4793(bool) = LIBRARY_CALL, dest:CollateralTypes, function:CollateralTypes.isValid(CollateralTypeInt.Data), arguments:["poolCollateral_1 (-> ['TMP_4787'])"] 
TMP_4794 = UnaryType.BANG TMP_4793 
TMP_4795(bool) = TMP_4792 && TMP_4794
CONDITION TMP_4795
 _c1FactorBIPS = factorBIPS
_c1FactorBIPS_3(uint256) := factorBIPS_1(uint256)
_c1FactorBIPS_4(uint256) := phi(['_c1FactorBIPS_3', '_c1FactorBIPS_1'])
_c1FactorBIPS_5(uint256) := phi(['_c1FactorBIPS_2', '_c1FactorBIPS_1'])
 _c1FactorBIPS > _vaultCR
TMP_4796(bool) = _c1FactorBIPS_5 > _vaultCR_1
CONDITION TMP_4796
 _c1FactorBIPS = _vaultCR
_c1FactorBIPS_6(uint256) := _vaultCR_1(uint256)
_c1FactorBIPS_7(uint256) := phi(['_c1FactorBIPS_6', '_c1FactorBIPS_1'])
 _poolFactorBIPS = factorBIPS - _c1FactorBIPS
TMP_4797(uint256) = factorBIPS_1 (c)- _c1FactorBIPS_7
_poolFactorBIPS_1(uint256) := TMP_4797(uint256)
 _poolFactorBIPS > _poolCR
TMP_4798(bool) = _poolFactorBIPS_1 > _poolCR_1
CONDITION TMP_4798
 _poolFactorBIPS = _poolCR
_poolFactorBIPS_2(uint256) := _poolCR_1(uint256)
 _c1FactorBIPS = Math.min(factorBIPS - _poolFactorBIPS,_vaultCR)
TMP_4799(uint256) = factorBIPS_1 (c)- _poolFactorBIPS_2
TMP_4800(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['TMP_4799', '_vaultCR_1'] 
_c1FactorBIPS_8(uint256) := TMP_4800(uint256)
_c1FactorBIPS_9(uint256) := phi(['_c1FactorBIPS_8', '_c1FactorBIPS_1'])
_poolFactorBIPS_3(uint256) := phi(['_poolFactorBIPS_2', '_poolFactorBIPS_1'])
 (_c1FactorBIPS,_poolFactorBIPS)
RETURN _c1FactorBIPS_9,_poolFactorBIPS_3
```
#### Math.max(uint256,uint256) [INTERNAL]
```slithir
 a > b
TMP_476(bool) = a_1 > b_1
CONDITION TMP_476
 a
RETURN a_1
 b
RETURN b_1
```
#### SafeCast.toUint64(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint64).max,SafeCast: value doesn't fit in 64 bits)
TMP_747(uint64) := 18446744073709551615(uint64)
TMP_748(bool) = value_1 <= TMP_747
TMP_749(None) = SOLIDITY_CALL require(bool,string)(TMP_748,SafeCast: value doesn't fit in 64 bits)
 uint64(value)
TMP_750 = CONVERT value_1 to uint64
RETURN TMP_750
```
#### AgentCollateral.mintingMinCollateralRatio(Agent.State,Collateral.Kind) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])', '_agent_1 (-> [])'])
_kind_1(Collateral.Kind) := phi(['REF_2871', 'REF_2849'])
 _kind == Collateral.Kind.AGENT_POOL
REF_2877(Collateral.Kind) -> Kind.AGENT_POOL
TMP_4353(bool) = _kind_1 == REF_2877
CONDITION TMP_4353
 mintingPoolHoldingsRequiredBIPS = Globals.getSettings().mintingPoolHoldingsRequiredBIPS
TMP_4354(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
REF_2879(uint32) -> TMP_4354.mintingPoolHoldingsRequiredBIPS
mintingPoolHoldingsRequiredBIPS_1(uint256) := REF_2879(uint32)
 _systemMinCollateralRatioBIPS = mintingPoolHoldingsRequiredBIPS
_systemMinCollateralRatioBIPS_4(uint256) := mintingPoolHoldingsRequiredBIPS_1(uint256)
 _mintingMinCollateralRatioBIPS = mintingPoolHoldingsRequiredBIPS
_mintingMinCollateralRatioBIPS_4(uint256) := mintingPoolHoldingsRequiredBIPS_1(uint256)
 _kind == Collateral.Kind.POOL
REF_2880(Collateral.Kind) -> Kind.POOL
TMP_4355(bool) = _kind_1 == REF_2880
CONDITION TMP_4355
 _systemMinCollateralRatioBIPS = _agent.getPoolCollateral().minCollateralRatioBIPS
TMP_4356(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getPoolCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
REF_2882(uint32) -> TMP_4356.minCollateralRatioBIPS
_systemMinCollateralRatioBIPS_2(uint256) := REF_2882(uint32)
 _mintingMinCollateralRatioBIPS = Math.max(_agent.mintingPoolCollateralRatioBIPS,_systemMinCollateralRatioBIPS)
REF_2884(uint32) -> _agent_1 (-> []).mintingPoolCollateralRatioBIPS
TMP_4357(uint256) = LIBRARY_CALL, dest:Math, function:Math.max(uint256,uint256), arguments:['REF_2884', '_systemMinCollateralRatioBIPS_2'] 
_mintingMinCollateralRatioBIPS_2(uint256) := TMP_4357(uint256)
 _systemMinCollateralRatioBIPS = _agent.getVaultCollateral().minCollateralRatioBIPS
TMP_4358(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getVaultCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
REF_2886(uint32) -> TMP_4358.minCollateralRatioBIPS
_systemMinCollateralRatioBIPS_1(uint256) := REF_2886(uint32)
 _mintingMinCollateralRatioBIPS = Math.max(_agent.mintingVaultCollateralRatioBIPS,_systemMinCollateralRatioBIPS)
REF_2888(uint32) -> _agent_1 (-> []).mintingVaultCollateralRatioBIPS
TMP_4359(uint256) = LIBRARY_CALL, dest:Math, function:Math.max(uint256,uint256), arguments:['REF_2888', '_systemMinCollateralRatioBIPS_1'] 
_mintingMinCollateralRatioBIPS_1(uint256) := TMP_4359(uint256)
_mintingMinCollateralRatioBIPS_3(uint256) := phi(['_mintingMinCollateralRatioBIPS_2', '_mintingMinCollateralRatioBIPS_1'])
_systemMinCollateralRatioBIPS_3(uint256) := phi(['_systemMinCollateralRatioBIPS_1', '_systemMinCollateralRatioBIPS_2'])
_mintingMinCollateralRatioBIPS_5(uint256) := phi(['_mintingMinCollateralRatioBIPS_0', '_mintingMinCollateralRatioBIPS_4'])
_systemMinCollateralRatioBIPS_5(uint256) := phi(['_systemMinCollateralRatioBIPS_0', '_systemMinCollateralRatioBIPS_4'])
 (_mintingMinCollateralRatioBIPS,_systemMinCollateralRatioBIPS)
RETURN _mintingMinCollateralRatioBIPS_5,_systemMinCollateralRatioBIPS_5
```
#### AgentCollateral.combinedData(Agent.State) [INTERNAL]
```slithir
 poolCollateral = poolCollateralData(_agent)
TMP_4296(Collateral.Data) = INTERNAL_CALL, AgentCollateral.poolCollateralData(Agent.State)(_agent_1 (-> []))
poolCollateral_1(Collateral.Data) := TMP_4296(Collateral.Data)
 Collateral.CombinedData({agentCollateral:agentVaultCollateralData(_agent),poolCollateral:poolCollateral,agentPoolTokens:agentsPoolTokensCollateralData(_agent,poolCollateral)})
TMP_4297(Collateral.Data) = INTERNAL_CALL, AgentCollateral.agentVaultCollateralData(Agent.State)(_agent_1 (-> []))
TMP_4298(Collateral.Data) = INTERNAL_CALL, AgentCollateral.agentsPoolTokensCollateralData(Agent.State,Collateral.Data)(_agent_1 (-> []),poolCollateral_1)
TMP_4299(Collateral.CombinedData) = new CombinedData(TMP_4297,poolCollateral_1,TMP_4298)
RETURN TMP_4299
```
#### AgentCollateral.freeCollateralLots(Collateral.CombinedData,Agent.State) [INTERNAL]
```slithir
 freeCollateralLotsOptionalFee(_data,_agent,true)
TMP_4322(uint256) = INTERNAL_CALL, AgentCollateral.freeCollateralLotsOptionalFee(Collateral.CombinedData,Agent.State,bool)(_data_1,_agent_1 (-> []),True)
RETURN TMP_4322
 _lots
```
#### AgentCollateral.freeCollateralWei(Collateral.Data,Agent.State) [INTERNAL]
```slithir
_data_1(Collateral.Data) := phi(['_data_1'])
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
 lockedCollateral = lockedCollateralWei(_data,_agent)
TMP_4332(uint256) = INTERNAL_CALL, AgentCollateral.lockedCollateralWei(Collateral.Data,Agent.State)(_data_1,_agent_1 (-> []))
lockedCollateral_1(uint256) := TMP_4332(uint256)
 MathUtils.subOrZero(_data.fullCollateral,lockedCollateral)
REF_2848(uint256) -> _data_1.fullCollateral
TMP_4333(uint256) = LIBRARY_CALL, dest:MathUtils, function:MathUtils.subOrZero(uint256,uint256), arguments:['REF_2848', 'lockedCollateral_1'] 
RETURN TMP_4333
```
#### Agents.getAgentStatus(Agent.State) [INTERNAL]
```slithir
 status = _agent.status
REF_2978(Agent.Status) -> _agent_1 (-> []).status
status_1(Agent.Status) := REF_2978(Agent.Status)
 status == Agent.Status.NORMAL
REF_2979(Agent.Status) -> Status.NORMAL
TMP_4471(bool) = status_1 == REF_2979
CONDITION TMP_4471
 AgentInfo.Status.NORMAL
REF_2980(AgentInfo.Status) -> Status.NORMAL
RETURN REF_2980
 status == Agent.Status.LIQUIDATION
REF_2981(Agent.Status) -> Status.LIQUIDATION
TMP_4472(bool) = status_1 == REF_2981
CONDITION TMP_4472
 AgentInfo.Status.LIQUIDATION
REF_2982(AgentInfo.Status) -> Status.LIQUIDATION
RETURN REF_2982
 status == Agent.Status.FULL_LIQUIDATION
REF_2983(Agent.Status) -> Status.FULL_LIQUIDATION
TMP_4473(bool) = status_1 == REF_2983
CONDITION TMP_4473
 AgentInfo.Status.FULL_LIQUIDATION
REF_2984(AgentInfo.Status) -> Status.FULL_LIQUIDATION
RETURN REF_2984
 status == Agent.Status.DESTROYING
REF_2985(Agent.Status) -> Status.DESTROYING
TMP_4474(bool) = status_1 == REF_2985
CONDITION TMP_4474
 AgentInfo.Status.DESTROYING
REF_2986(AgentInfo.Status) -> Status.DESTROYING
RETURN REF_2986
 assert(bool)(status == Agent.Status.DESTROYED)
REF_2987(Agent.Status) -> Status.DESTROYED
TMP_4475(bool) = status_1 == REF_2987
TMP_4476(None) = SOLIDITY_CALL assert(bool)(TMP_4475)
 AgentInfo.Status.DESTROYED
REF_2988(AgentInfo.Status) -> Status.DESTROYED
RETURN REF_2988
```
#### Agents.getPoolCollateral(Agent.State) [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4516(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4516'])(AssetManagerState.State) := TMP_4516(AssetManagerState.State)
 state.collateralTokens[_agent.poolCollateralIndex]
REF_3015(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4516']).collateralTokens
REF_3016(uint16) -> _agent_1 (-> []).poolCollateralIndex
REF_3017(CollateralTypeInt.Data) -> REF_3015[REF_3016]
RETURN REF_3017
```
#### Agents.getVaultCollateral(Agent.State) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
 state = AssetManagerState.get()
TMP_4510(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4510'])(AssetManagerState.State) := TMP_4510(AssetManagerState.State)
 state.collateralTokens[_agent.vaultCollateralIndex]
REF_3005(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4510']).collateralTokens
REF_3006(uint16) -> _agent_1 (-> []).vaultCollateralIndex
REF_3007(CollateralTypeInt.Data) -> REF_3005[REF_3006]
RETURN REF_3007
```
#### Agents.getWorkAddress(Agent.State) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])', '_agent_1 (-> [])'])
 Globals.getAgentOwnerRegistry().getWorkAddress(_agent.ownerManagementAddress)
TMP_4481(IAgentOwnerRegistry) = LIBRARY_CALL, dest:Globals, function:Globals.getAgentOwnerRegistry(), arguments:[] 
REF_2992(address) -> _agent_1 (-> []).ownerManagementAddress
TMP_4482(address) = HIGH_LEVEL_CALL, dest:TMP_4481(IAgentOwnerRegistry), function:getWorkAddress, arguments:['REF_2992']  
RETURN TMP_4482
```
#### Agents.withdrawalAnnouncement(Agent.State,Collateral.Kind) [INTERNAL]
```slithir
 assert(bool)(_kind != Collateral.Kind.POOL)
REF_3033(Collateral.Kind) -> Kind.POOL
TMP_4528(bool) = _kind_1 != REF_3033
TMP_4529(None) = SOLIDITY_CALL assert(bool)(TMP_4528)
 _kind == Collateral.Kind.VAULT
REF_3034(Collateral.Kind) -> Kind.VAULT
TMP_4530(bool) = _kind_1 == REF_3034
CONDITION TMP_4530
 _agent.vaultCollateralWithdrawalAnnouncement
REF_3035(Agent.WithdrawalAnnouncement) -> _agent_1 (-> []).vaultCollateralWithdrawalAnnouncement
RETURN REF_3035
 _agent.poolTokenWithdrawalAnnouncement
REF_3036(Agent.WithdrawalAnnouncement) -> _agent_1 (-> []).poolTokenWithdrawalAnnouncement
RETURN REF_3036
```
#### Liquidation.getCollateralRatiosBIPS(Agent.State) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
 (vaultCR,amgToC1WeiPrice) = getCollateralRatioBIPS(_agent,Collateral.Kind.VAULT)
REF_3268(Collateral.Kind) -> Kind.VAULT
TUPLE_55(uint256,uint256) = INTERNAL_CALL, Liquidation.getCollateralRatioBIPS(Agent.State,Collateral.Kind)(_agent_1 (-> []),REF_3268)
vaultCR_1(uint256)= UNPACK TUPLE_55 index: 0 
amgToC1WeiPrice_1(uint256)= UNPACK TUPLE_55 index: 1 
 (poolCR,amgToPoolWeiPrice) = getCollateralRatioBIPS(_agent,Collateral.Kind.POOL)
REF_3269(Collateral.Kind) -> Kind.POOL
TUPLE_56(uint256,uint256) = INTERNAL_CALL, Liquidation.getCollateralRatioBIPS(Agent.State,Collateral.Kind)(_agent_1 (-> []),REF_3269)
poolCR_1(uint256)= UNPACK TUPLE_56 index: 0 
amgToPoolWeiPrice_1(uint256)= UNPACK TUPLE_56 index: 1 
 CRData({vaultCR:vaultCR,poolCR:poolCR,amgToC1WeiPrice:amgToC1WeiPrice,amgToPoolWeiPrice:amgToPoolWeiPrice})
TMP_4757(Liquidation.CRData) = new CRData(vaultCR_1,poolCR_1,amgToC1WeiPrice_1,amgToPoolWeiPrice_1)
RETURN TMP_4757
```

#### Agent.getAllowDestroyed(address) [INTERNAL]
```slithir
 agent = getWithoutCheck(_address)
TMP_5315(Agent.State) = INTERNAL_CALL, Agent.getWithoutCheck(address)(_address_1)
agent_1 (-> ['TMP_5315'])(Agent.State) := TMP_5315(Agent.State)
 require(bool,error)(agent.status != Agent.Status.EMPTY,revert InvalidAgentVaultAddress()())
REF_3751(Agent.Status) -> agent_1 (-> ['TMP_5315']).status
REF_3752(Agent.Status) -> Status.EMPTY
TMP_5316(bool) = REF_3751 != REF_3752
TMP_5317(None) = SOLIDITY_CALL revert InvalidAgentVaultAddress()()
TMP_5318(None) = SOLIDITY_CALL require(bool,error)(TMP_5316,TMP_5317)
 agent
RETURN agent_1 (-> ['TMP_5315'])
```

#### Agents.getAllAgents(uint256,uint256) [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4462(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4462'])(AssetManagerState.State) := TMP_4462(AssetManagerState.State)
 _totalLength = state.allAgents.length
REF_2971(address[]) -> state_1 (-> ['TMP_4462']).allAgents
REF_2972 -> LENGTH REF_2971
_totalLength_1(uint256) := REF_2972(uint256)
 _end = Math.min(_end,_totalLength)
TMP_4463(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['_end_1', '_totalLength_1'] 
_end_2(uint256) := TMP_4463(uint256)
 _start = Math.min(_start,_end)
TMP_4464(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['_start_1', '_end_2'] 
_start_2(uint256) := TMP_4464(uint256)
 _agents = new address[](_end - _start)
TMP_4466(uint256) = _end_2 (c)- _start_2
TMP_4467(address[])  = new address[](TMP_4466)
_agents_1(address[]) = ['TMP_4467(address[])']
 i = _start
i_1(uint256) := _start_2(uint256)
 i < _end
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_4468(bool) = i_2 < _end_2
CONDITION TMP_4468
 _agents[i - _start] = state.allAgents[i]
TMP_4469(uint256) = i_2 (c)- _start_2
REF_2975(address) -> _agents_1[TMP_4469]
REF_2976(address[]) -> state_1 (-> ['TMP_4462']).allAgents
REF_2977(address) -> REF_2976[i_2]
_agents_2(address[]) := phi(['_agents_1'])
REF_2975(address) (->_agents_2) := REF_2977(address)
 i ++
TMP_4470(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 (_agents,_totalLength)
RETURN _agents_1,_totalLength_1
```
#### AssetManagerState.get() [INTERNAL]
```slithir
STATE_POSITION_1(bytes32) := phi(['STATE_POSITION_0'])
 position = STATE_POSITION
position_1(bytes32) := STATE_POSITION_1(bytes32)
 _state = position
_state_1 (-> ['position'])(AssetManagerState.State) := position_1(bytes32)
 _state
RETURN _state_1 (-> ['position'])
```
#### AgentCollateral.agentVaultCollateralData(Agent.State) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])', '_agent_1 (-> [])'])
 collateral = _agent.getVaultCollateral()
TMP_4306(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getVaultCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
collateral_1 (-> ['TMP_4306'])(CollateralTypeInt.Data) := TMP_4306(CollateralTypeInt.Data)
 Collateral.Data({kind:Collateral.Kind.VAULT,fullCollateral:collateral.token.balanceOf(_agent.vaultAddress()),amgToTokenWeiPrice:Conversion.currentAmgPriceInTokenWei(collateral)})
REF_2819(Collateral.Kind) -> Kind.VAULT
REF_2820(IERC20) -> collateral_1 (-> ['TMP_4306']).token
TMP_4307(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
TMP_4308(uint256) = HIGH_LEVEL_CALL, dest:REF_2820(IERC20), function:balanceOf, arguments:['TMP_4307']  
TMP_4309(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.currentAmgPriceInTokenWei(CollateralTypeInt.Data), arguments:["collateral_1 (-> ['TMP_4306'])"] 
TMP_4310(Collateral.Data) = new Data(REF_2819,TMP_4308,TMP_4309)
RETURN TMP_4310
```
#### AgentCollateral.agentsPoolTokensCollateralData(Agent.State,Collateral.Data) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])', '_agent_1 (-> [])'])
_poolCollateral_1(Collateral.Data) := phi(['poolCollateral_1', 'TMP_4304'])
 poolToken = _agent.collateralPool.poolToken()
REF_2830(IICollateralPool) -> _agent_1 (-> []).collateralPool
TMP_4315(ICollateralPoolToken) = HIGH_LEVEL_CALL, dest:REF_2830(IICollateralPool), function:poolToken, arguments:[]  
poolToken_1(IERC20) := TMP_4315(ICollateralPoolToken)
 agentPoolTokens = poolToken.balanceOf(_agent.vaultAddress())
TMP_4316(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
TMP_4317(uint256) = HIGH_LEVEL_CALL, dest:poolToken_1(IERC20), function:balanceOf, arguments:['TMP_4316']  
agentPoolTokens_1(uint256) := TMP_4317(uint256)
 totalPoolTokens = poolToken.totalSupply()
TMP_4318(uint256) = HIGH_LEVEL_CALL, dest:poolToken_1(IERC20), function:totalSupply, arguments:[]  
totalPoolTokens_1(uint256) := TMP_4318(uint256)
 Collateral.Data({kind:Collateral.Kind.AGENT_POOL,fullCollateral:agentPoolTokens,amgToTokenWeiPrice:amgToPoolTokenWeiPrice})
REF_2836(Collateral.Kind) -> Kind.AGENT_POOL
TMP_4319(Collateral.Data) = new Data(REF_2836,agentPoolTokens_1,amgToPoolTokenWeiPrice_3)
RETURN TMP_4319
 _poolCollateral.fullCollateral != 0
REF_2837(uint256) -> _poolCollateral_1.fullCollateral
TMP_4320(bool) = REF_2837 != 0
CONDITION TMP_4320
 amgToPoolTokenWeiPrice = _poolCollateral.amgToTokenWeiPrice.mulDiv(totalPoolTokens,_poolCollateral.fullCollateral)
REF_2838(uint256) -> _poolCollateral_1.amgToTokenWeiPrice
REF_2840(uint256) -> _poolCollateral_1.fullCollateral
TMP_4321(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['REF_2838', 'totalPoolTokens_1', 'REF_2840'] 
amgToPoolTokenWeiPrice_1(uint256) := TMP_4321(uint256)
 amgToPoolTokenWeiPrice = _poolCollateral.amgToTokenWeiPrice
REF_2841(uint256) -> _poolCollateral_1.amgToTokenWeiPrice
amgToPoolTokenWeiPrice_2(uint256) := REF_2841(uint256)
amgToPoolTokenWeiPrice_3(uint256) := phi(['amgToPoolTokenWeiPrice_1', 'amgToPoolTokenWeiPrice_2'])
```
#### AgentCollateral.poolCollateralData(Agent.State) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])', '_agent_1 (-> [])'])
 collateral = _agent.getPoolCollateral()
TMP_4311(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getPoolCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
collateral_1 (-> ['TMP_4311'])(CollateralTypeInt.Data) := TMP_4311(CollateralTypeInt.Data)
 Collateral.Data({kind:Collateral.Kind.POOL,fullCollateral:_agent.collateralPool.totalCollateral(),amgToTokenWeiPrice:Conversion.currentAmgPriceInTokenWei(collateral)})
REF_2826(Collateral.Kind) -> Kind.POOL
REF_2827(IICollateralPool) -> _agent_1 (-> []).collateralPool
TMP_4312(uint256) = HIGH_LEVEL_CALL, dest:REF_2827(IICollateralPool), function:totalCollateral, arguments:[]  
TMP_4313(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.currentAmgPriceInTokenWei(CollateralTypeInt.Data), arguments:["collateral_1 (-> ['TMP_4311'])"] 
TMP_4314(Collateral.Data) = new Data(REF_2826,TMP_4312,TMP_4313)
RETURN TMP_4314
```
#### Agent.getWithoutCheck(address) [INTERNAL]
```slithir
_address_1(address) := phi(['_address_1', '_address_1'])
AGENTS_POSITION_1(bytes32) := phi(['AGENTS_POSITION_0'])
 position = bytes32(uint256(AGENTS_POSITION) ^ (uint256(uint160(_address)) << 64))
TMP_5319 = CONVERT AGENTS_POSITION_1 to uint256
TMP_5320 = CONVERT _address_1 to uint160
TMP_5321 = CONVERT TMP_5320 to uint256
TMP_5322(uint256) = TMP_5321 << 64
TMP_5323(uint256) = TMP_5319 ^ TMP_5322
TMP_5324 = CONVERT TMP_5323 to bytes32
position_1(bytes32) := TMP_5324(bytes32)
 _agent = position
_agent_1 (-> ['position'])(Agent.State) := position_1(bytes32)
 _agent
RETURN _agent_1 (-> ['position'])
```
#### Globals.getSettings() [INTERNAL]
```slithir
ASSET_MANAGER_SETTINGS_POSITION_1(bytes32) := phi(['ASSET_MANAGER_SETTINGS_POSITION_0'])
 position = ASSET_MANAGER_SETTINGS_POSITION
position_1(bytes32) := ASSET_MANAGER_SETTINGS_POSITION_1(bytes32)
 _settings = position
_settings_1 (-> ['position'])(AssetManagerSettings.Data) := position_1(bytes32)
 _settings
RETURN _settings_1 (-> ['position'])
```

#### Liquidation._targetRatioBIPS(Agent.State,Collateral.Kind) [PRIVATE]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])', '_agent_1 (-> [])'])
_collateralKind_1(Collateral.Kind) := phi(['REF_3259', 'REF_3258', '_collateralKind_1'])
 collateral = _agent.getCollateral(_collateralKind)
TMP_4770(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getCollateral(Agent.State,Collateral.Kind), arguments:['_agent_1 (-> [])', '_collateralKind_1'] 
collateral_1 (-> ['TMP_4770'])(CollateralTypeInt.Data) := TMP_4770(CollateralTypeInt.Data)
 ! _agent.collateralUnderwater(_collateralKind)
TMP_4771(bool) = LIBRARY_CALL, dest:Agents, function:Agents.collateralUnderwater(Agent.State,Collateral.Kind), arguments:['_agent_1 (-> [])', '_collateralKind_1'] 
TMP_4772 = UnaryType.BANG TMP_4771 
CONDITION TMP_4772
 collateral.minCollateralRatioBIPS
REF_3284(uint32) -> collateral_1 (-> ['TMP_4770']).minCollateralRatioBIPS
RETURN REF_3284
 collateral.safetyMinCollateralRatioBIPS
REF_3285(uint32) -> collateral_1 (-> ['TMP_4770']).safetyMinCollateralRatioBIPS
RETURN REF_3285
```
#### SafePct.mulDivRoundUp(uint256,uint256,uint256) [INTERNAL]
```slithir
 resultRoundDown = mulDiv(x,y,z)
TMP_10531(uint256) = INTERNAL_CALL, SafePct.mulDiv(uint256,uint256,uint256)(x_1,y_1,z_1)
resultRoundDown_1(uint256) := TMP_10531(uint256)
 remainder = mulmod(uint256,uint256,uint256)(x,y,z)
TMP_10532(uint256) = SOLIDITY_CALL mulmod(uint256,uint256,uint256)(x_1,y_1,z_1)
remainder_1(uint256) := TMP_10532(uint256)
 remainder == 0
TMP_10533(bool) = remainder_1 == 0
CONDITION TMP_10533
 resultRoundDown
RETURN resultRoundDown_1
 resultRoundDown + 1
TMP_10534(uint256) = resultRoundDown_1 + 1
RETURN TMP_10534
```
#### Math.min(uint256,uint256) [INTERNAL]
```slithir
a_1(uint256) := phi(['result_8'])
b_1(uint256) := phi(['TMP_554'])
 a < b
TMP_477(bool) = a_1 < b_1
CONDITION TMP_477
 a
RETURN a_1
 b
RETURN b_1
```
#### CollateralTypes.isValid(CollateralTypeInt.Data) [INTERNAL]
```slithir
 _token.validUntil == 0 || _token.validUntil > block.timestamp
REF_3072(uint64) -> _token_1 (-> []).validUntil
TMP_4578(bool) = REF_3072 == 0
REF_3073(uint64) -> _token_1 (-> []).validUntil
TMP_4579(bool) = REF_3073 > block.timestamp
TMP_4580(bool) = TMP_4578 || TMP_4579
RETURN TMP_4580
```

#### AgentCollateral.freeCollateralLotsOptionalFee(Collateral.CombinedData,Agent.State,bool) [INTERNAL]
```slithir
_data_1(Collateral.CombinedData) := phi(['_data_1'])
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
 agentLots = freeSingleCollateralLots(_data.agentCollateral,_agent,_chargePoolFee)
REF_2842(Collateral.Data) -> _data_1.agentCollateral
TMP_4323(uint256) = INTERNAL_CALL, AgentCollateral.freeSingleCollateralLots(Collateral.Data,Agent.State,bool)(REF_2842,_agent_1 (-> []),_chargePoolFee_1)
agentLots_1(uint256) := TMP_4323(uint256)
 poolLots = freeSingleCollateralLots(_data.poolCollateral,_agent,_chargePoolFee)
REF_2843(Collateral.Data) -> _data_1.poolCollateral
TMP_4324(uint256) = INTERNAL_CALL, AgentCollateral.freeSingleCollateralLots(Collateral.Data,Agent.State,bool)(REF_2843,_agent_1 (-> []),_chargePoolFee_1)
poolLots_1(uint256) := TMP_4324(uint256)
 agentPoolTokenLots = freeSingleCollateralLots(_data.agentPoolTokens,_agent,_chargePoolFee)
REF_2844(Collateral.Data) -> _data_1.agentPoolTokens
TMP_4325(uint256) = INTERNAL_CALL, AgentCollateral.freeSingleCollateralLots(Collateral.Data,Agent.State,bool)(REF_2844,_agent_1 (-> []),_chargePoolFee_1)
agentPoolTokenLots_1(uint256) := TMP_4325(uint256)
 Math.min(agentLots,Math.min(poolLots,agentPoolTokenLots))
TMP_4326(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['poolLots_1', 'agentPoolTokenLots_1'] 
TMP_4327(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['agentLots_1', 'TMP_4326'] 
RETURN TMP_4327
 _lots
```
#### AgentCollateral.lockedCollateralWei(Collateral.Data,Agent.State) [INTERNAL]
```slithir
_data_1(Collateral.Data) := phi(['_data_1'])
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
 (mintingMinCollateralRatioBIPS,systemMinCollateralRatioBIPS) = mintingMinCollateralRatio(_agent,_data.kind)
REF_2849(Collateral.Kind) -> _data_1.kind
TUPLE_39(uint256,uint256) = INTERNAL_CALL, AgentCollateral.mintingMinCollateralRatio(Agent.State,Collateral.Kind)(_agent_1 (-> []),REF_2849)
mintingMinCollateralRatioBIPS_1(uint256)= UNPACK TUPLE_39 index: 0 
systemMinCollateralRatioBIPS_1(uint256)= UNPACK TUPLE_39 index: 1 
 backedAMG = uint256(_agent.reservedAMG) + uint256(_agent.mintedAMG)
REF_2850(uint64) -> _agent_1 (-> []).reservedAMG
TMP_4334 = CONVERT REF_2850 to uint256
REF_2851(uint64) -> _agent_1 (-> []).mintedAMG
TMP_4335 = CONVERT REF_2851 to uint256
TMP_4336(uint256) = TMP_4334 (c)+ TMP_4335
backedAMG_1(uint256) := TMP_4336(uint256)
 mintingCollateral = Conversion.convertAmgToTokenWei(backedAMG,_data.amgToTokenWeiPrice).mulBips(mintingMinCollateralRatioBIPS)
REF_2853(uint256) -> _data_1.amgToTokenWeiPrice
TMP_4337(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToTokenWei(uint256,uint256), arguments:['backedAMG_1', 'REF_2853'] 
TMP_4338(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['TMP_4337', 'mintingMinCollateralRatioBIPS_1'] 
mintingCollateral_1(uint256) := TMP_4338(uint256)
 redeemingCollateral = Conversion.convertAmgToTokenWei(redeemingAMG,_data.amgToTokenWeiPrice).mulBips(systemMinCollateralRatioBIPS)
REF_2856(uint256) -> _data_1.amgToTokenWeiPrice
TMP_4339(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToTokenWei(uint256,uint256), arguments:['redeemingAMG_3', 'REF_2856'] 
TMP_4340(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['TMP_4339', 'systemMinCollateralRatioBIPS_1'] 
redeemingCollateral_1(uint256) := TMP_4340(uint256)
 mintingCollateral + redeemingCollateral + announcedWithdrawal
TMP_4341(uint256) = mintingCollateral_1 (c)+ redeemingCollateral_1
TMP_4342(uint256) = TMP_4341 (c)+ announcedWithdrawal_3
RETURN TMP_4342
 _data.kind == Collateral.Kind.POOL
REF_2858(Collateral.Kind) -> _data_1.kind
REF_2859(Collateral.Kind) -> Kind.POOL
TMP_4343(bool) = REF_2858 == REF_2859
CONDITION TMP_4343
 redeemingAMG = _agent.poolRedeemingAMG
REF_2860(uint64) -> _agent_1 (-> []).poolRedeemingAMG
redeemingAMG_2(uint64) := REF_2860(uint64)
 redeemingAMG = _agent.redeemingAMG
REF_2861(uint64) -> _agent_1 (-> []).redeemingAMG
redeemingAMG_1(uint64) := REF_2861(uint64)
redeemingAMG_3(uint64) := phi(['redeemingAMG_1', 'redeemingAMG_2'])
 _data.kind != Collateral.Kind.POOL
REF_2862(Collateral.Kind) -> _data_1.kind
REF_2863(Collateral.Kind) -> Kind.POOL
TMP_4344(bool) = REF_2862 != REF_2863
CONDITION TMP_4344
 announcedWithdrawal = _agent.withdrawalAnnouncement(_data.kind).amountWei
REF_2865(Collateral.Kind) -> _data_1.kind
TMP_4345(Agent.WithdrawalAnnouncement) = LIBRARY_CALL, dest:Agents, function:Agents.withdrawalAnnouncement(Agent.State,Collateral.Kind), arguments:['_agent_1 (-> [])', 'REF_2865'] 
REF_2866(uint128) -> TMP_4345.amountWei
announcedWithdrawal_1(uint256) := REF_2866(uint128)
 announcedWithdrawal = 0
announcedWithdrawal_2(uint256) := 0(uint256)
announcedWithdrawal_3(uint256) := phi(['announcedWithdrawal_1', 'announcedWithdrawal_2'])
```
#### MathUtils.subOrZero(uint256,uint256) [INTERNAL]
```slithir
 _a > _b
TMP_10425(bool) = _a_1 > _b_1
CONDITION TMP_10425
 _a - _b
TMP_10426(uint256) = _a_1 (c)- _b_1
RETURN TMP_10426
 0
RETURN 0
```
#### Globals.getAgentOwnerRegistry() [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4737(AssetManagerSettings.Data) = INTERNAL_CALL, Globals.getSettings()()
settings_1 (-> ['TMP_4737'])(AssetManagerSettings.Data) := TMP_4737(AssetManagerSettings.Data)
 IAgentOwnerRegistry(settings.agentOwnerRegistry)
REF_3242(address) -> settings_1 (-> ['TMP_4737']).agentOwnerRegistry
TMP_4738 = CONVERT REF_3242 to IAgentOwnerRegistry
RETURN TMP_4738
```
#### AgentOwnerRegistry.getWorkAddress(address) [EXTERNAL]
```slithir
mgmtToWorkAddress_4(mapping(address => address)) := phi(['mgmtToWorkAddress_3', 'mgmtToWorkAddress_0', 'mgmtToWorkAddress_4'])
 mgmtToWorkAddress[_managementAddress]
REF_311(address) -> mgmtToWorkAddress_4[_managementAddress_1]
RETURN REF_311
```
#### Liquidation.getCollateralRatioBIPS(Agent.State,Collateral.Kind) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
_collateralKind_1(Collateral.Kind) := phi(['REF_3268', 'REF_3269'])
 (_data,_trustedData) = _collateralDataWithTrusted(_agent,_collateralKind)
TUPLE_57(Collateral.Data,Collateral.Data) = INTERNAL_CALL, Liquidation._collateralDataWithTrusted(Agent.State,Collateral.Kind)(_agent_1 (-> []),_collateralKind_1)
_data_1(Collateral.Data)= UNPACK TUPLE_57 index: 0 
_trustedData_1(Collateral.Data)= UNPACK TUPLE_57 index: 1 
 ratio = AgentCollateral.collateralRatioBIPS(_data,_agent)
TMP_4758(uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.collateralRatioBIPS(Collateral.Data,Agent.State), arguments:['_data_1', '_agent_1 (-> [])'] 
ratio_1(uint256) := TMP_4758(uint256)
 ratioTrusted = AgentCollateral.collateralRatioBIPS(_trustedData,_agent)
TMP_4759(uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.collateralRatioBIPS(Collateral.Data,Agent.State), arguments:['_trustedData_1', '_agent_1 (-> [])'] 
ratioTrusted_1(uint256) := TMP_4759(uint256)
 _amgToTokenWeiPrice = _data.amgToTokenWeiPrice
REF_3272(uint256) -> _data_1.amgToTokenWeiPrice
_amgToTokenWeiPrice_1(uint256) := REF_3272(uint256)
 _collateralRatioBIPS = Math.max(ratio,ratioTrusted)
TMP_4760(uint256) = LIBRARY_CALL, dest:Math, function:Math.max(uint256,uint256), arguments:['ratio_1', 'ratioTrusted_1'] 
_collateralRatioBIPS_1(uint256) := TMP_4760(uint256)
 (_collateralRatioBIPS,_amgToTokenWeiPrice)
RETURN _collateralRatioBIPS_1,_amgToTokenWeiPrice_1
```
#### Conversion.currentAmgPriceInTokenWei(CollateralTypeInt.Data) [INTERNAL]
```slithir
_token_1 (-> [])(CollateralTypeInt.Data) := phi(['_toToken_1 (-> [])', '_fromToken_1 (-> [])'])
 (_price,None,None) = currentAmgPriceInTokenWeiWithTs(_token,false)
TUPLE_44(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.currentAmgPriceInTokenWeiWithTs(CollateralTypeInt.Data,bool)(_token_1 (-> []),False)
_price_1(uint256)= UNPACK TUPLE_44 index: 0 
 _price
RETURN _price_1
```
#### ERC20.balanceOf(address) [PUBLIC]
```slithir
_balances_1(mapping(address => uint256)) := phi(['_balances_11', '_balances_1', '_balances_8', '_balances_5', '_balances_0'])
 _balances[account]
REF_73(uint256) -> _balances_1[account_1]
RETURN REF_73
```
#### Agent.vaultAddress(Agent.State) [INTERNAL]
```slithir
AGENTS_POSITION_2(bytes32) := phi(['AGENTS_POSITION_0'])
 position = _agent
position_1(bytes32) := _agent_1 (-> [])(Agent.State)
 address(uint160((uint256(position) ^ uint256(AGENTS_POSITION)) >> 64))
TMP_5325 = CONVERT position_1 to uint256
TMP_5326 = CONVERT AGENTS_POSITION_2 to uint256
TMP_5327(uint256) = TMP_5325 ^ TMP_5326
TMP_5328(uint256) = TMP_5327 >> 64
TMP_5329 = CONVERT TMP_5328 to uint160
TMP_5330 = CONVERT TMP_5329 to address
RETURN TMP_5330
```
#### ERC20.totalSupply() [PUBLIC]
```slithir
_totalSupply_1(uint256) := phi(['_totalSupply_0', '_totalSupply_4', '_totalSupply_7'])
 _totalSupply
RETURN _totalSupply_1
```
#### SafePct.mulDiv(uint256,uint256,uint256) [INTERNAL]
```slithir
x_1(uint256) := phi(['x_1', 'x_1'])
y_1(uint256) := phi(['y_1', 'y_1'])
z_1(uint256) := phi(['z_1', 'MAX_BIPS_1'])
 require(bool,error)(z > 0,revert DivisionByZero()())
TMP_10510(bool) = z_1 > 0
TMP_10511(None) = SOLIDITY_CALL revert DivisionByZero()()
TMP_10512(None) = SOLIDITY_CALL require(bool,error)(TMP_10510,TMP_10511)
 x == 0
TMP_10513(bool) = x_1 == 0
CONDITION TMP_10513
 0
RETURN 0
 xy = x * y
TMP_10514(uint256) = x_1 * y_1
xy_1(uint256) := TMP_10514(uint256)
 xy / x == y
TMP_10515(uint256) = xy_1 / x_1
TMP_10516(bool) = TMP_10515 == y_1
CONDITION TMP_10516
 xy / z
TMP_10517(uint256) = xy_1 / z_1
RETURN TMP_10517
 a = x / z
TMP_10518(uint256) = x_1 (c)/ z_1
a_1(uint256) := TMP_10518(uint256)
 b = x % z
TMP_10519(uint256) = x_1 % z_1
b_1(uint256) := TMP_10519(uint256)
 c = y / z
TMP_10520(uint256) = y_1 (c)/ z_1
c_1(uint256) := TMP_10520(uint256)
 d = y % z
TMP_10521(uint256) = y_1 % z_1
d_1(uint256) := TMP_10521(uint256)
 (a * c * z) + (a * d) + (b * c) + (b * d / z)
TMP_10522(uint256) = a_1 (c)* c_1
TMP_10523(uint256) = TMP_10522 (c)* z_1
TMP_10524(uint256) = a_1 (c)* d_1
TMP_10525(uint256) = TMP_10523 (c)+ TMP_10524
TMP_10526(uint256) = b_1 (c)* c_1
TMP_10527(uint256) = TMP_10525 (c)+ TMP_10526
TMP_10528(uint256) = b_1 (c)* d_1
TMP_10529(uint256) = TMP_10528 (c)/ z_1
TMP_10530(uint256) = TMP_10527 (c)+ TMP_10529
RETURN TMP_10530
```
#### Agents.collateralUnderwater(Agent.State,Collateral.Kind) [INTERNAL]
```slithir
 _kind == Collateral.Kind.VAULT
REF_3027(Collateral.Kind) -> Kind.VAULT
TMP_4521(bool) = _kind_1 == REF_3027
CONDITION TMP_4521
 (_agent.collateralsUnderwater & Agent.LF_VAULT) != 0
REF_3028(uint8) -> _agent_1 (-> []).collateralsUnderwater
REF_3029(uint8) -> Agent.LF_VAULT
TMP_4522(uint8) = REF_3028 & REF_3029
TMP_4523(bool) = TMP_4522 != 0
RETURN TMP_4523
 assert(bool)(_kind == Collateral.Kind.POOL)
REF_3030(Collateral.Kind) -> Kind.POOL
TMP_4524(bool) = _kind_1 == REF_3030
TMP_4525(None) = SOLIDITY_CALL assert(bool)(TMP_4524)
 (_agent.collateralsUnderwater & Agent.LF_POOL) != 0
REF_3031(uint8) -> _agent_1 (-> []).collateralsUnderwater
REF_3032(uint8) -> Agent.LF_POOL
TMP_4526(uint8) = REF_3031 & REF_3032
TMP_4527(bool) = TMP_4526 != 0
RETURN TMP_4527
```
#### Agents.getCollateral(Agent.State,Collateral.Kind) [INTERNAL]
```slithir
 assert(bool)(_kind != Collateral.Kind.AGENT_POOL)
REF_3018(Collateral.Kind) -> Kind.AGENT_POOL
TMP_4517(bool) = _kind_1 != REF_3018
TMP_4518(None) = SOLIDITY_CALL assert(bool)(TMP_4517)
 state = AssetManagerState.get()
TMP_4519(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4519'])(AssetManagerState.State) := TMP_4519(AssetManagerState.State)
 _kind == Collateral.Kind.VAULT
REF_3020(Collateral.Kind) -> Kind.VAULT
TMP_4520(bool) = _kind_1 == REF_3020
CONDITION TMP_4520
 state.collateralTokens[_agent.vaultCollateralIndex]
REF_3021(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4519']).collateralTokens
REF_3022(uint16) -> _agent_1 (-> []).vaultCollateralIndex
REF_3023(CollateralTypeInt.Data) -> REF_3021[REF_3022]
RETURN REF_3023
 state.collateralTokens[_agent.poolCollateralIndex]
REF_3024(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4519']).collateralTokens
REF_3025(uint16) -> _agent_1 (-> []).poolCollateralIndex
REF_3026(CollateralTypeInt.Data) -> REF_3024[REF_3025]
RETURN REF_3026
```
#### AgentCollateral.freeSingleCollateralLots(Collateral.Data,Agent.State,bool) [INTERNAL]
```slithir
_data_1(Collateral.Data) := phi(['REF_2842', 'REF_2844', 'REF_2843'])
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
_chargePoolFee_1(bool) := phi(['_chargePoolFee_1'])
 collateralWei = freeCollateralWei(_data,_agent)
TMP_4328(uint256) = INTERNAL_CALL, AgentCollateral.freeCollateralWei(Collateral.Data,Agent.State)(_data_1,_agent_1 (-> []))
collateralWei_1(uint256) := TMP_4328(uint256)
 lotWei = mintingLotCollateralWei(_data,_agent,_chargePoolFee)
TMP_4329(uint256) = INTERNAL_CALL, AgentCollateral.mintingLotCollateralWei(Collateral.Data,Agent.State,bool)(_data_1,_agent_1 (-> []),_chargePoolFee_1)
lotWei_1(uint256) := TMP_4329(uint256)
 lotWei != 0
TMP_4330(bool) = lotWei_1 != 0
CONDITION TMP_4330
 collateralWei / lotWei
TMP_4331(uint256) = collateralWei_1 (c)/ lotWei_1
RETURN TMP_4331
 0
RETURN 0
```
#### Conversion.convertAmgToTokenWei(uint256,uint256) [INTERNAL]
```slithir
AMG_TOKEN_WEI_PRICE_SCALE_1(uint256) := phi(['AMG_TOKEN_WEI_PRICE_SCALE_0'])
 _valueAMG.mulDiv(_amgToTokenWeiPrice,AMG_TOKEN_WEI_PRICE_SCALE)
TMP_4674(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_valueAMG_1', '_amgToTokenWeiPrice_1', 'AMG_TOKEN_WEI_PRICE_SCALE_1'] 
RETURN TMP_4674
```
#### SafePct.mulBips(uint256,uint256) [INTERNAL]
```slithir
MAX_BIPS_1(uint256) := phi(['MAX_BIPS_2', 'MAX_BIPS_0'])
 mulDiv(x,y,MAX_BIPS)
TMP_10535(uint256) = INTERNAL_CALL, SafePct.mulDiv(uint256,uint256,uint256)(x_1,y_1,MAX_BIPS_1)
RETURN TMP_10535
```
#### AgentCollateral.collateralRatioBIPS(Collateral.Data,Agent.State) [INTERNAL]
```slithir
 totalAMG = totalBackedAMG(_agent,_data.kind)
REF_2897(Collateral.Kind) -> _data_1.kind
TMP_4370(uint256) = INTERNAL_CALL, AgentCollateral.totalBackedAMG(Agent.State,Collateral.Kind)(_agent_1 (-> []),REF_2897)
totalAMG_1(uint256) := TMP_4370(uint256)
 backingTokenWei = Conversion.convertAmgToTokenWei(totalAMG,_data.amgToTokenWeiPrice)
REF_2899(uint256) -> _data_1.amgToTokenWeiPrice
TMP_4371(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToTokenWei(uint256,uint256), arguments:['totalAMG_1', 'REF_2899'] 
backingTokenWei_1(uint256) := TMP_4371(uint256)
 backingTokenWei == 0
TMP_4372(bool) = backingTokenWei_1 == 0
CONDITION TMP_4372
 1e10
RETURN 10000000000
 _data.fullCollateral.mulDiv(SafePct.MAX_BIPS,backingTokenWei)
REF_2900(uint256) -> _data_1.fullCollateral
REF_2902(uint256) -> SafePct.MAX_BIPS
TMP_4373(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['REF_2900', 'REF_2902', 'backingTokenWei_1'] 
RETURN TMP_4373
```
#### Liquidation._collateralDataWithTrusted(Agent.State,Collateral.Kind) [PRIVATE]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
_kind_1(Collateral.Kind) := phi(['_collateralKind_1'])
 collateral = _agent.getCollateral(_kind)
TMP_4773(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getCollateral(Agent.State,Collateral.Kind), arguments:['_agent_1 (-> [])', '_kind_1'] 
collateral_1 (-> ['TMP_4773'])(CollateralTypeInt.Data) := TMP_4773(CollateralTypeInt.Data)
 fullCollateral = _getCollateralAmount(_agent,_kind,collateral)
TMP_4774(uint256) = INTERNAL_CALL, Liquidation._getCollateralAmount(Agent.State,Collateral.Kind,CollateralTypeInt.Data)(_agent_1 (-> []),_kind_1,collateral_1 (-> ['TMP_4773']))
fullCollateral_1(uint256) := TMP_4774(uint256)
 (price,trusted) = Conversion.currentAmgPriceInTokenWeiWithTrusted(collateral)
TUPLE_58(uint256,uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.currentAmgPriceInTokenWeiWithTrusted(CollateralTypeInt.Data), arguments:["collateral_1 (-> ['TMP_4773'])"] 
price_1(uint256)= UNPACK TUPLE_58 index: 0 
trusted_1(uint256)= UNPACK TUPLE_58 index: 1 
 _data = Collateral.Data({kind:_kind,fullCollateral:fullCollateral,amgToTokenWeiPrice:price})
TMP_4775(Collateral.Data) = new Data(_kind_1,fullCollateral_1,price_1)
_data_1(Collateral.Data) := TMP_4775(Collateral.Data)
 _trustedData = Collateral.Data({kind:_kind,fullCollateral:fullCollateral,amgToTokenWeiPrice:trusted})
TMP_4776(Collateral.Data) = new Data(_kind_1,fullCollateral_1,trusted_1)
_trustedData_1(Collateral.Data) := TMP_4776(Collateral.Data)
 (_data,_trustedData)
RETURN _data_1,_trustedData_1
```
