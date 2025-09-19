
### Storage layout (CollateralPoolToken) 

```text
collateralPool address
tokenName string
tokenSymbol string
timelocksByAccount mapping(address => CollateralPoolToken.TimelockQueue)
ignoreTimelocked bool
initialized bool

```



### Storage layout (AgentVault) 

```text
assetManager IIAssetManager
initialized bool
__usedTokens IERC20[]
__tokenUseFlags mapping(IERC20 => uint256)
__internalWithdrawal bool
destroyed bool

```





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



### Storage layout (CollateralPool) 

```text
agentVault address
assetManager IIAssetManager
fAsset IFAsset
token IICollateralPoolToken
wNat IWNat
exitCollateralRatioBIPS uint32
__topupCollateralRatioBIPS uint32
__topupTokenPriceFactorBIPS uint16
internalWithdrawal bool
initialized bool
_fAssetFeeDebtOf mapping(address => int256)
totalFAssetFeeDebt int256
totalFAssetFees uint256
totalCollateral uint256

```



#### RedemptionDefaults._replaceFailedVaultPaymentWithPool(Agent.State,Redemption.Request,uint256,uint256) [PRIVATE]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
_request_1 (-> [])(Redemption.Request) := phi(['_request_1 (-> [])'])
_paidC1Wei_1(uint256) := phi(['paidC1Wei_1'])
_paidPoolWei_1(uint256) := phi(['paidPoolWei_1'])
 cd = AgentCollateral.combinedData(_agent)
TMP_4869(Collateral.CombinedData) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.combinedData(Agent.State), arguments:['_agent_1 (-> [])'] 
cd_1(Collateral.CombinedData) := TMP_4869(Collateral.CombinedData)
 poolTokenEquiv = _paidC1Wei.mulDiv(cd.agentPoolTokens.amgToTokenWeiPrice,cd.agentCollateral.amgToTokenWeiPrice)
REF_3384(Collateral.Data) -> cd_1.agentPoolTokens
REF_3385(uint256) -> REF_3384.amgToTokenWeiPrice
REF_3386(Collateral.Data) -> cd_1.agentCollateral
REF_3387(uint256) -> REF_3386.amgToTokenWeiPrice
TMP_4870(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_paidC1Wei_1', 'REF_3385', 'REF_3387'] 
poolTokenEquiv_1(uint256) := TMP_4870(uint256)
 requiredPoolTokensForRemainder = uint256(_agent.reservedAMG + _agent.mintedAMG + _agent.redeemingAMG - _request.valueAMG).mulDiv(cd.agentPoolTokens.amgToTokenWeiPrice,Conversion.AMG_TOKEN_WEI_PRICE_SCALE).mulBips(Globals.getSettings().mintingPoolHoldingsRequiredBIPS)
REF_3388(uint64) -> _agent_1 (-> []).reservedAMG
REF_3389(uint64) -> _agent_1 (-> []).mintedAMG
TMP_4871(uint64) = REF_3388 (c)+ REF_3389
REF_3390(uint64) -> _agent_1 (-> []).redeemingAMG
TMP_4872(uint64) = TMP_4871 (c)+ REF_3390
REF_3391(uint64) -> _request_1 (-> []).valueAMG
TMP_4873(uint64) = TMP_4872 (c)- REF_3391
TMP_4874 = CONVERT TMP_4873 to uint256
REF_3393(Collateral.Data) -> cd_1.agentPoolTokens
REF_3394(uint256) -> REF_3393.amgToTokenWeiPrice
REF_3395(uint256) -> Conversion.AMG_TOKEN_WEI_PRICE_SCALE
TMP_4875(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['TMP_4874', 'REF_3394', 'REF_3395'] 
TMP_4876(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
REF_3398(uint32) -> TMP_4876.mintingPoolHoldingsRequiredBIPS
TMP_4877(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['TMP_4875', 'REF_3398'] 
requiredPoolTokensForRemainder_1(uint256) := TMP_4877(uint256)
 require(bool,error)(requiredPoolTokensForRemainder + poolTokenEquiv <= cd.agentPoolTokens.fullCollateral,revert NotEnoughAgentPoolTokensToCoverFailedVaultPayment()())
TMP_4878(uint256) = requiredPoolTokensForRemainder_1 (c)+ poolTokenEquiv_1
REF_3399(Collateral.Data) -> cd_1.agentPoolTokens
REF_3400(uint256) -> REF_3399.fullCollateral
TMP_4879(bool) = TMP_4878 <= REF_3400
TMP_4880(None) = SOLIDITY_CALL revert NotEnoughAgentPoolTokensToCoverFailedVaultPayment()()
TMP_4881(None) = SOLIDITY_CALL require(bool,error)(TMP_4879,TMP_4880)
 poolWeiEquiv = _paidC1Wei.mulDiv(cd.poolCollateral.amgToTokenWeiPrice,cd.agentCollateral.amgToTokenWeiPrice)
REF_3402(Collateral.Data) -> cd_1.poolCollateral
REF_3403(uint256) -> REF_3402.amgToTokenWeiPrice
REF_3404(Collateral.Data) -> cd_1.agentCollateral
REF_3405(uint256) -> REF_3404.amgToTokenWeiPrice
TMP_4882(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_paidC1Wei_1', 'REF_3403', 'REF_3405'] 
poolWeiEquiv_1(uint256) := TMP_4882(uint256)
 combinedPaidPoolWei = _paidPoolWei + poolWeiEquiv
TMP_4883(uint256) = _paidPoolWei_1 (c)+ poolWeiEquiv_1
combinedPaidPoolWei_1(uint256) := TMP_4883(uint256)
 require(bool,error)(combinedPaidPoolWei <= cd.poolCollateral.maxRedemptionCollateral(_agent,_request.valueAMG),revert NotEnoughPoolCollateralToCoverFailedVaultPayment()())
REF_3406(Collateral.Data) -> cd_1.poolCollateral
REF_3408(uint64) -> _request_1 (-> []).valueAMG
TMP_4884(uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.maxRedemptionCollateral(Collateral.Data,Agent.State,uint256), arguments:['REF_3406', '_agent_1 (-> [])', 'REF_3408'] 
TMP_4885(bool) = combinedPaidPoolWei_1 <= TMP_4884
TMP_4886(None) = SOLIDITY_CALL revert NotEnoughPoolCollateralToCoverFailedVaultPayment()()
TMP_4887(None) = SOLIDITY_CALL require(bool,error)(TMP_4885,TMP_4886)
 combinedPaidPoolWei
RETURN combinedPaidPoolWei_1
```
#### RedemptionDefaults.executeDefaultOrCancel(Agent.State,Redemption.Request,uint256) [INTERNAL]
```slithir
 assert(bool)(_request.status == Redemption.Status.ACTIVE)
REF_3363(Redemption.Status) -> _request_1 (-> []).status
REF_3364(Redemption.Status) -> Status.ACTIVE
TMP_4856(bool) = REF_3363 == REF_3364
TMP_4857(None) = SOLIDITY_CALL assert(bool)(TMP_4856)
 ! _request.transferToCoreVault
REF_3365(bool) -> _request_1 (-> []).transferToCoreVault
TMP_4858 = UnaryType.BANG REF_3365 
CONDITION TMP_4858
 (paidC1Wei,paidPoolWei) = _collateralAmountForRedemption(_agent,_request)
TUPLE_59(uint256,uint256) = INTERNAL_CALL, RedemptionDefaults._collateralAmountForRedemption(Agent.State,Redemption.Request)(_agent_1 (-> []),_request_1 (-> []))
paidC1Wei_1(uint256)= UNPACK TUPLE_59 index: 0 
paidPoolWei_1(uint256)= UNPACK TUPLE_59 index: 1 
 (successVault,None) = AgentPayout.tryPayoutFromVault(_agent,_request.redeemer,paidC1Wei)
REF_3367(address) -> _request_1 (-> []).redeemer
TUPLE_60(bool,uint256) = LIBRARY_CALL, dest:AgentPayout, function:AgentPayout.tryPayoutFromVault(Agent.State,address,uint256), arguments:['_agent_1 (-> [])', 'REF_3367', 'paidC1Wei_1'] 
successVault_1(bool)= UNPACK TUPLE_60 index: 0 
 ! successVault
TMP_4859 = UnaryType.BANG successVault_1 
CONDITION TMP_4859
 paidPoolWei = _replaceFailedVaultPaymentWithPool(_agent,_request,paidC1Wei,paidPoolWei)
TMP_4860(uint256) = INTERNAL_CALL, RedemptionDefaults._replaceFailedVaultPaymentWithPool(Agent.State,Redemption.Request,uint256,uint256)(_agent_1 (-> []),_request_1 (-> []),paidC1Wei_1,paidPoolWei_1)
paidPoolWei_2(uint256) := TMP_4860(uint256)
 paidC1Wei = 0
paidC1Wei_2(uint256) := 0(uint256)
paidC1Wei_3(uint256) := phi(['paidC1Wei_1', 'paidC1Wei_2'])
paidPoolWei_3(uint256) := phi(['paidPoolWei_1', 'paidPoolWei_2'])
 paidPoolWei > 0
TMP_4861(bool) = paidPoolWei_3 > 0
CONDITION TMP_4861
 AgentPayout.payoutFromPool(_agent,_request.redeemer,paidPoolWei,paidPoolWei)
REF_3369(address) -> _request_1 (-> []).redeemer
TMP_4862(uint256) = LIBRARY_CALL, dest:AgentPayout, function:AgentPayout.payoutFromPool(Agent.State,address,uint256,uint256), arguments:['_agent_1 (-> [])', 'REF_3369', 'paidPoolWei_3', 'paidPoolWei_3'] 
 AgentBacking.endRedeemingAssets(_agent,_request.valueAMG,_request.poolSelfClose)
REF_3371(uint64) -> _request_1 (-> []).valueAMG
REF_3372(bool) -> _request_1 (-> []).poolSelfClose
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.endRedeemingAssets(Agent.State,uint64,bool), arguments:['_agent_1 (-> [])', 'REF_3371', 'REF_3372'] 
 IAssetManagerEvents.RedemptionDefault(_agent.vaultAddress(),_request.redeemer,_redemptionRequestId,_request.underlyingValueUBA,paidC1Wei,paidPoolWei)
TMP_4864(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
REF_3375(address) -> _request_1 (-> []).redeemer
REF_3376(uint128) -> _request_1 (-> []).underlyingValueUBA
Emit RedemptionDefault(TMP_4864,REF_3375,_redemptionRequestId_1,REF_3376,paidC1Wei_3,paidPoolWei_3)
 IAssetManagerEvents.RedemptionDefault(_agent.vaultAddress(),_request.redeemer,_redemptionRequestId,_request.underlyingValueUBA,0,0)
TMP_4866(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
REF_3379(address) -> _request_1 (-> []).redeemer
REF_3380(uint128) -> _request_1 (-> []).underlyingValueUBA
Emit RedemptionDefault(TMP_4866,REF_3379,_redemptionRequestId_1,REF_3380,0,0)
 CoreVaultClient.cancelTransferToCoreVault(_agent,_request,_redemptionRequestId)
LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.cancelTransferToCoreVault(Agent.State,Redemption.Request,uint256), arguments:['_agent_1 (-> [])', '_request_1 (-> [])', '_redemptionRequestId_1']
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
#### AgentCollateral.maxRedemptionCollateral(Collateral.Data,Agent.State,uint256) [INTERNAL]
```slithir
 _valueAMG == 0
TMP_4360(bool) = _valueAMG_1 == 0
CONDITION TMP_4360
 0
RETURN 0
 assert(bool)(_valueAMG <= redeemingAMG)
TMP_4361(bool) = _valueAMG_1 <= redeemingAMG_3
TMP_4362(None) = SOLIDITY_CALL assert(bool)(TMP_4361)
 totalAMG = uint256(_agent.mintedAMG) + uint256(_agent.reservedAMG) + uint256(redeemingAMG)
REF_2889(uint64) -> _agent_1 (-> []).mintedAMG
TMP_4363 = CONVERT REF_2889 to uint256
REF_2890(uint64) -> _agent_1 (-> []).reservedAMG
TMP_4364 = CONVERT REF_2890 to uint256
TMP_4365(uint256) = TMP_4363 (c)+ TMP_4364
TMP_4366 = CONVERT redeemingAMG_3 to uint256
TMP_4367(uint256) = TMP_4365 (c)+ TMP_4366
totalAMG_1(uint256) := TMP_4367(uint256)
 _data.fullCollateral.mulDiv(_valueAMG,totalAMG)
REF_2891(uint256) -> _data_1.fullCollateral
TMP_4368(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['REF_2891', '_valueAMG_1', 'totalAMG_1'] 
RETURN TMP_4368
 _data.kind == Collateral.Kind.POOL
REF_2893(Collateral.Kind) -> _data_1.kind
REF_2894(Collateral.Kind) -> Kind.POOL
TMP_4369(bool) = REF_2893 == REF_2894
CONDITION TMP_4369
 redeemingAMG = _agent.poolRedeemingAMG
REF_2895(uint64) -> _agent_1 (-> []).poolRedeemingAMG
redeemingAMG_1(uint256) := REF_2895(uint64)
 redeemingAMG = _agent.redeemingAMG
REF_2896(uint64) -> _agent_1 (-> []).redeemingAMG
redeemingAMG_2(uint256) := REF_2896(uint64)
redeemingAMG_3(uint256) := phi(['redeemingAMG_1', 'redeemingAMG_2'])
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
#### Conversion.convertAmgToTokenWei(uint256,uint256) [INTERNAL]
```slithir
AMG_TOKEN_WEI_PRICE_SCALE_1(uint256) := phi(['AMG_TOKEN_WEI_PRICE_SCALE_0'])
 _valueAMG.mulDiv(_amgToTokenWeiPrice,AMG_TOKEN_WEI_PRICE_SCALE)
TMP_4674(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_valueAMG_1', '_amgToTokenWeiPrice_1', 'AMG_TOKEN_WEI_PRICE_SCALE_1'] 
RETURN TMP_4674
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
#### SafePct.mulBips(uint256,uint256) [INTERNAL]
```slithir
MAX_BIPS_1(uint256) := phi(['MAX_BIPS_2', 'MAX_BIPS_0'])
 mulDiv(x,y,MAX_BIPS)
TMP_10535(uint256) = INTERNAL_CALL, SafePct.mulDiv(uint256,uint256,uint256)(x_1,y_1,MAX_BIPS_1)
RETURN TMP_10535
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
#### AgentBacking.endRedeemingAssets(Agent.State,uint64,bool) [INTERNAL]
```slithir
 _agent.redeemingAMG = _agent.redeemingAMG - _valueAMG
REF_2785(uint64) -> _agent_1 (-> []).redeemingAMG
REF_2786(uint64) -> _agent_1 (-> []).redeemingAMG
TMP_4270(uint64) = REF_2786 (c)- _valueAMG_1
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2785(uint64) (->_agent_2 (-> [])) := TMP_4270(uint64)
 ! _poolSelfCloseRedemption
TMP_4271 = UnaryType.BANG _poolSelfCloseRedemption_1 
CONDITION TMP_4271
 _agent.poolRedeemingAMG = _agent.poolRedeemingAMG - _valueAMG
REF_2787(uint64) -> _agent_2 (-> []).poolRedeemingAMG
REF_2788(uint64) -> _agent_2 (-> []).poolRedeemingAMG
TMP_4272(uint64) = REF_2788 (c)- _valueAMG_1
_agent_3 (-> [])(Agent.State) := phi(['_agent_2 (-> [])'])
REF_2787(uint64) (->_agent_3 (-> [])) := TMP_4272(uint64)
```
#### AgentPayout.payoutFromPool(Agent.State,address,uint256,uint256) [INTERNAL]
```slithir
 poolBalance = _agent.collateralPool.totalCollateral()
REF_2922(IICollateralPool) -> _agent_1 (-> []).collateralPool
TMP_4394(uint256) = HIGH_LEVEL_CALL, dest:REF_2922(IICollateralPool), function:totalCollateral, arguments:[]  
poolBalance_1(uint256) := TMP_4394(uint256)
 _amountPaid = Math.min(_amountWei,poolBalance)
TMP_4395(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['_amountWei_1', 'poolBalance_1'] 
_amountPaid_1(uint256) := TMP_4395(uint256)
 _agentResponsibilityWei = Math.min(_agentResponsibilityWei,_amountPaid)
TMP_4396(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['_agentResponsibilityWei_1', '_amountPaid_1'] 
_agentResponsibilityWei_2(uint256) := TMP_4396(uint256)
 _agent.collateralPool.payout(_receiver,_amountPaid,_agentResponsibilityWei)
REF_2926(IICollateralPool) -> _agent_1 (-> []).collateralPool
HIGH_LEVEL_CALL, dest:REF_2926(IICollateralPool), function:payout, arguments:['_receiver_1', '_amountPaid_1', '_agentResponsibilityWei_2']  
 _amountPaid
RETURN _amountPaid_1
```
#### AgentPayout.tryPayoutFromVault(Agent.State,address,uint256) [INTERNAL]
```slithir
 collateral = Agents.getVaultCollateral(_agent)
TMP_4387(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getVaultCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
collateral_1 (-> ['TMP_4387'])(CollateralTypeInt.Data) := TMP_4387(CollateralTypeInt.Data)
 vault = IIAgentVault(_agent.vaultAddress())
TMP_4388(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
TMP_4389 = CONVERT TMP_4388 to IIAgentVault
vault_1(IIAgentVault) := TMP_4389(IIAgentVault)
 _amountPaid = Math.min(_amountWei,collateral.token.balanceOf(address(vault)))
REF_2918(IERC20) -> collateral_1 (-> ['TMP_4387']).token
TMP_4390 = CONVERT vault_1 to address
TMP_4391(uint256) = HIGH_LEVEL_CALL, dest:REF_2918(IERC20), function:balanceOf, arguments:['TMP_4390']  
TMP_4392(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['_amountWei_1', 'TMP_4391'] 
_amountPaid_1(uint256) := TMP_4392(uint256)
 vault.payout(collateral.token,_receiver,_amountPaid)
REF_2921(IERC20) -> collateral_1 (-> ['TMP_4387']).token
HIGH_LEVEL_CALL, dest:vault_1(IIAgentVault), function:payout, arguments:['REF_2921', '_receiver_1', '_amountPaid_1']  
 _success = true
_success_1(bool) := True(bool)
 _success = false
_success_3(bool) := False(bool)
 _amountPaid = 0
_amountPaid_3(uint256) := 0(uint256)
 (_success,_amountPaid)
_success_2(bool) := phi(['_success_1', '_success_3'])
_amountPaid_2(uint256) := phi(['_amountPaid_3', '_amountPaid_1'])
RETURN _success_2,_amountPaid_2
```
#### CoreVaultClient.cancelTransferToCoreVault(Agent.State,Redemption.Request,uint256) [INTERNAL]
```slithir
 Redemptions.releaseTransferToCoreVault(_redemptionRequestId,_request)
LIBRARY_CALL, dest:Redemptions, function:Redemptions.releaseTransferToCoreVault(uint256,Redemption.Request), arguments:['_redemptionRequestId_1', '_request_1 (-> [])'] 
 Redemptions.reCreateRedemptionTicket(_agent,_request)
LIBRARY_CALL, dest:Redemptions, function:Redemptions.reCreateRedemptionTicket(Agent.State,Redemption.Request), arguments:['_agent_1 (-> [])', '_request_1 (-> [])'] 
 ICoreVaultClient.TransferToCoreVaultDefaulted(_agent.vaultAddress(),_redemptionRequestId,_request.underlyingValueUBA)
TMP_4685(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
REF_3189(uint128) -> _request_1 (-> []).underlyingValueUBA
Emit TransferToCoreVaultDefaulted(TMP_4685,_redemptionRequestId_1,REF_3189)
 onlyEnabled()
MODIFIER_CALL, CoreVaultClient.onlyEnabled()()
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
_balances_1(mapping(address => uint256)) := phi(['_balances_11', '_balances_1', '_balances_5', '_balances_8', '_balances_0'])
 _balances[account]
REF_73(uint256) -> _balances_1[account_1]
RETURN REF_73
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
#### CollateralPool.payout(address,uint256,uint256) [EXTERNAL]
```slithir
agentVault_17(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_1', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_0', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
token_40(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_0', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalCollateral_14(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_0', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 agentTokenBalance = token.balanceOf(agentVault)
TMP_6206(uint256) = HIGH_LEVEL_CALL, dest:token_42(IICollateralPoolToken), function:balanceOf, arguments:['agentVault_19']  
agentVault_20(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_19', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
token_43(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_42', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalCollateral_17(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_35', 'totalCollateral_16', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
agentTokenBalance_1(uint256) := TMP_6206(uint256)
 slashedTokens = Math.min(maxSlashedTokens,agentTokenBalance)
TMP_6207(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['maxSlashedTokens_3', 'agentTokenBalance_1'] 
slashedTokens_1(uint256) := TMP_6207(uint256)
 slashedTokens > 0
TMP_6208(bool) = slashedTokens_1 > 0
CONDITION TMP_6208
 debtFAssetFeeShare = _tokensToVirtualFeeShare(slashedTokens)
TMP_6209(uint256) = INTERNAL_CALL, CollateralPool._tokensToVirtualFeeShare(uint256)(slashedTokens_1)
token_45(IICollateralPoolToken) := phi(['token_51'])
debtFAssetFeeShare_1(uint256) := TMP_6209(uint256)
 _deleteFAssetFeeDebt(agentVault,debtFAssetFeeShare)
INTERNAL_CALL, CollateralPool._deleteFAssetFeeDebt(address,uint256)(agentVault_22,debtFAssetFeeShare_1)
 token.burn(agentVault,slashedTokens,true)
HIGH_LEVEL_CALL, dest:token_46(IICollateralPoolToken), function:burn, arguments:['agentVault_23', 'slashedTokens_1', 'True']  
agentVault_24(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_23', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
token_47(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_46', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
 _transferWNatTo(_recipient,_amount)
INTERNAL_CALL, CollateralPool._transferWNatTo(address,uint256)(_recipient_1,_amount_1)
 CPPaidOut(_recipient,_amount,slashedTokens)
Emit CPPaidOut(_recipient_1,_amount_1,slashedTokens_1)
 onlyAssetManager()
MODIFIER_CALL, CollateralPool.onlyAssetManager()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 totalCollateral > 0
TMP_6216(bool) = totalCollateral_17 > 0
CONDITION TMP_6216
 maxSlashedTokens = token.totalSupply().mulDivRoundUp(_agentResponsibilityWei,totalCollateral)
TMP_6217(uint256) = HIGH_LEVEL_CALL, dest:token_43(IICollateralPoolToken), function:totalSupply, arguments:[]  
agentVault_21(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
token_44(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalCollateral_18(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
TMP_6218(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDivRoundUp(uint256,uint256,uint256), arguments:['TMP_6217', '_agentResponsibilityWei_1', 'totalCollateral_18'] 
maxSlashedTokens_1(uint256) := TMP_6218(uint256)
 maxSlashedTokens = agentTokenBalance
maxSlashedTokens_2(uint256) := agentTokenBalance_1(uint256)
maxSlashedTokens_3(uint256) := phi(['maxSlashedTokens_1', 'maxSlashedTokens_2'])
```
#### AgentVault.payout(IERC20,address,uint256) [EXTERNAL]
```slithir
 _token.safeTransfer(_recipient,_amount)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['_token_1', '_recipient_1', '_amount_1'] 
 onlyAssetManager()
MODIFIER_CALL, AgentVault.onlyAssetManager()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### Redemptions.reCreateRedemptionTicket(Agent.State,Redemption.Request) [INTERNAL]
```slithir
 AgentBacking.endRedeemingAssets(_agent,_request.valueAMG,_request.poolSelfClose)
REF_3574(uint64) -> _request_1 (-> []).valueAMG
REF_3575(bool) -> _request_1 (-> []).poolSelfClose
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.endRedeemingAssets(Agent.State,uint64,bool), arguments:['_agent_1 (-> [])', 'REF_3574', 'REF_3575'] 
 AgentBacking.createNewMinting(_agent,_request.valueAMG)
REF_3577(uint64) -> _request_1 (-> []).valueAMG
LIBRARY_CALL, dest:AgentBacking, function:AgentBacking.createNewMinting(Agent.State,uint64), arguments:['_agent_1 (-> [])', 'REF_3577']
```
#### Redemptions.releaseTransferToCoreVault(uint256,Redemption.Request) [INTERNAL]
```slithir
_redemptionRequestId_1(uint256) := phi(['_redemptionRequestId_1'])
_request_1 (-> [])(Redemption.Request) := phi(['_request_2 (-> [])'])
 _request.transferToCoreVault
REF_3580(bool) -> _request_1 (-> []).transferToCoreVault
CONDITION REF_3580
 agent = Agent.get(_request.agentVault)
REF_3582(address) -> _request_1 (-> []).agentVault
TMP_5040(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['REF_3582'] 
agent_1 (-> ['TMP_5040'])(Agent.State) := TMP_5040(Agent.State)
 agent.activeTransferToCoreVault == _redemptionRequestId
REF_3583(uint64) -> agent_1 (-> ['TMP_5040']).activeTransferToCoreVault
TMP_5041(bool) = REF_3583 == _redemptionRequestId_1
CONDITION TMP_5041
 agent.activeTransferToCoreVault = 0
REF_3584(uint64) -> agent_1 (-> ['TMP_5040']).activeTransferToCoreVault
agent_2 (-> ['TMP_5040'])(Agent.State) := phi(["agent_1 (-> ['TMP_5040'])"])
REF_3584(uint64) (->agent_2 (-> ['TMP_5040'])) := 0(uint256)
TMP_5040(Agent.State) := phi(["agent_2 (-> ['TMP_5040'])"])
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
#### Conversion.currentAmgPriceInTokenWeiWithTs(CollateralTypeInt.Data,bool) [INTERNAL]
```slithir
_token_1 (-> [])(CollateralTypeInt.Data) := phi(['_token_1 (-> [])', 'REF_3140', '_token_1 (-> [])'])
 (assetPrice,assetTs,assetFtsoDec) = readFtsoPrice(_token.assetFtsoSymbol,_fromTrustedProviders)
REF_3163(string) -> _token_1 (-> []).assetFtsoSymbol
TUPLE_48(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.readFtsoPrice(string,bool)(REF_3163,_fromTrustedProviders_1)
assetPrice_1(uint256)= UNPACK TUPLE_48 index: 0 
assetTs_1(uint256)= UNPACK TUPLE_48 index: 1 
assetFtsoDec_1(uint256)= UNPACK TUPLE_48 index: 2 
 _token.directPricePair
REF_3164(bool) -> _token_1 (-> []).directPricePair
CONDITION REF_3164
 price = calcAmgToTokenWeiPrice(_token.decimals,1,0,assetPrice,assetFtsoDec)
REF_3165(uint8) -> _token_1 (-> []).decimals
TMP_4661(uint256) = INTERNAL_CALL, Conversion.calcAmgToTokenWeiPrice(uint256,uint256,uint256,uint256,uint256)(REF_3165,1,0,assetPrice_1,assetFtsoDec_1)
price_1(uint256) := TMP_4661(uint256)
 (price,assetTs,assetTs)
RETURN price_1,assetTs_1,assetTs_1
 (tokenPrice,tokenTs,tokenFtsoDec) = readFtsoPrice(_token.tokenFtsoSymbol,_fromTrustedProviders)
REF_3166(string) -> _token_1 (-> []).tokenFtsoSymbol
TUPLE_49(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.readFtsoPrice(string,bool)(REF_3166,_fromTrustedProviders_1)
tokenPrice_1(uint256)= UNPACK TUPLE_49 index: 0 
tokenTs_1(uint256)= UNPACK TUPLE_49 index: 1 
tokenFtsoDec_1(uint256)= UNPACK TUPLE_49 index: 2 
 price_scope_0 = calcAmgToTokenWeiPrice(_token.decimals,tokenPrice,tokenFtsoDec,assetPrice,assetFtsoDec)
REF_3167(uint8) -> _token_1 (-> []).decimals
TMP_4662(uint256) = INTERNAL_CALL, Conversion.calcAmgToTokenWeiPrice(uint256,uint256,uint256,uint256,uint256)(REF_3167,tokenPrice_1,tokenFtsoDec_1,assetPrice_1,assetFtsoDec_1)
price_scope_0_1(uint256) := TMP_4662(uint256)
 (price_scope_0,assetTs,tokenTs)
RETURN price_scope_0_1,assetTs_1,tokenTs_1
```
#### ERC20.totalSupply() [PUBLIC]
```slithir
_totalSupply_1(uint256) := phi(['_totalSupply_0', '_totalSupply_4', '_totalSupply_7'])
 _totalSupply
RETURN _totalSupply_1
```
#### CollateralPool._deleteFAssetFeeDebt(address,uint256) [INTERNAL]
```slithir
_account_1(address) := phi(['agentVault_22', 'msg.sender'])
_fAssets_1(uint256) := phi(['debtFAssetFeeShare_1', '_fAssets_1', 'debtFAssetFeeShare_1', 'debtFAssetFeeShare_1'])
_fAssetFeeDebtOf_8(mapping(address => int256)) := phi(['_fAssetFeeDebtOf_5', '_fAssetFeeDebtOf_0', '_fAssetFeeDebtOf_2', '_fAssetFeeDebtOf_7', '_fAssetFeeDebtOf_4', '_fAssetFeeDebtOf_9', '_fAssetFeeDebtOf_10'])
totalFAssetFeeDebt_4(int256) := phi(['totalFAssetFeeDebt_0', 'totalFAssetFeeDebt_5', 'totalFAssetFeeDebt_3'])
 _fAssets == 0
TMP_6299(bool) = _fAssets_1 == 0
CONDITION TMP_6299
 fAssets = _fAssets.toInt256()
TMP_6300(int256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt256(uint256), arguments:['_fAssets_1'] 
fAssets_1(int256) := TMP_6300(int256)
 _fAssetFeeDebtOf[_account] -= fAssets
REF_4251(int256) -> _fAssetFeeDebtOf_8[_account_1]
_fAssetFeeDebtOf_9(mapping(address => int256)) := phi(['_fAssetFeeDebtOf_8'])
REF_4251(-> _fAssetFeeDebtOf_9) = REF_4251 (c)- fAssets_1
 totalFAssetFeeDebt -= fAssets
totalFAssetFeeDebt_5(int256) = totalFAssetFeeDebt_4 (c)- fAssets_1
 CPFeeDebtChanged(_account,_fAssetFeeDebtOf[_account])
REF_4252(int256) -> _fAssetFeeDebtOf_9[_account_1]
Emit CPFeeDebtChanged(_account_1,REF_4252)
```
#### CollateralPool._tokensToVirtualFeeShare(uint256) [INTERNAL]
```slithir
_tokens_1(uint256) := phi(['slashedTokens_1', '_tokenShare_1', 'tokens_1', '_tokenShare_1'])
token_50(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_0', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
 _tokens == 0
TMP_6224(bool) = _tokens_1 == 0
CONDITION TMP_6224
 0
RETURN 0
 totalPoolTokens = token.totalSupply()
TMP_6225(uint256) = HIGH_LEVEL_CALL, dest:token_50(IICollateralPoolToken), function:totalSupply, arguments:[]  
token_51(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_50', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalPoolTokens_1(uint256) := TMP_6225(uint256)
 assert(bool)(_tokens <= totalPoolTokens)
TMP_6226(bool) = _tokens_1 <= totalPoolTokens_1
TMP_6227(None) = SOLIDITY_CALL assert(bool)(TMP_6226)
 _totalVirtualFees().mulDiv(_tokens,totalPoolTokens)
TMP_6228(uint256) = INTERNAL_CALL, CollateralPool._totalVirtualFees()()
TMP_6229(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['TMP_6228', '_tokens_1', 'totalPoolTokens_1'] 
RETURN TMP_6229
```
#### CollateralPool._transferWNatTo(address,uint256) [INTERNAL]
```slithir
_to_1(address) := phi(['_recipient_1'])
_amount_1(uint256) := phi(['_amount_1'])
wNat_4(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_0', 'wNat_25', 'wNat_15', 'wNat_36'])
totalCollateral_32(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_0', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 _amount > 0
TMP_6307(bool) = _amount_1 > 0
CONDITION TMP_6307
 totalCollateral -= _amount
totalCollateral_33(uint256) = totalCollateral_32 (c)- _amount_1
 wNat.safeTransfer(_to,_amount)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['wNat_4', '_to_1', '_amount_1']
```
#### CollateralPoolToken.burn(address,uint256,bool) [EXTERNAL]
```slithir
 _ignoreTimelocked
CONDITION _ignoreTimelocked_1
 ignoreTimelocked = true
ignoreTimelocked_1(bool) := True(bool)
 _burn(_account,_amount)
INTERNAL_CALL, ERC20._burn(address,uint256)(_account_1,_amount_1)
 _ignoreTimelocked
CONDITION _ignoreTimelocked_1
 ignoreTimelocked = false
ignoreTimelocked_2(bool) := False(bool)
 onlyCollateralPool()
MODIFIER_CALL, CollateralPoolToken.onlyCollateralPool()()
```
#### SafeERC20.safeTransfer(IERC20,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeWithSelector(token.transfer.selector,to,value))
REF_86(bytes4) (->None) := 2835717307(bytes4)
TMP_243(bytes) = SOLIDITY_CALL abi.encodeWithSelector()(REF_86,to_1,value_1)
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_243)
```
#### AgentBacking.createNewMinting(Agent.State,uint64) [INTERNAL]
```slithir
 _agent.mintedAMG += _valueAMG
REF_2789(uint64) -> _agent_1 (-> []).mintedAMG
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2789(-> _agent_2 (-> [])) = REF_2789 (c)+ _valueAMG_1
 settings = Globals.getSettings()
TMP_4273(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4273'])(AssetManagerSettings.Data) := TMP_4273(AssetManagerSettings.Data)
 valueWithDustAMG = _agent.dustAMG + _valueAMG
REF_2791(uint64) -> _agent_2 (-> []).dustAMG
TMP_4274(uint64) = REF_2791 (c)+ _valueAMG_1
valueWithDustAMG_1(uint64) := TMP_4274(uint64)
 newDustAMG = valueWithDustAMG % settings.lotSizeAMG
REF_2792(uint64) -> settings_1 (-> ['TMP_4273']).lotSizeAMG
TMP_4275(uint64) = valueWithDustAMG_1 % REF_2792
newDustAMG_1(uint64) := TMP_4275(uint64)
 ticketValueAMG = valueWithDustAMG - newDustAMG
TMP_4276(uint64) = valueWithDustAMG_1 (c)- newDustAMG_1
ticketValueAMG_1(uint64) := TMP_4276(uint64)
 ticketValueAMG > 0
TMP_4277(bool) = ticketValueAMG_1 > 0
CONDITION TMP_4277
 createRedemptionTicket(_agent,ticketValueAMG)
INTERNAL_CALL, AgentBacking.createRedemptionTicket(Agent.State,uint64)(_agent_2 (-> []),ticketValueAMG_1)
 changeDust(_agent,newDustAMG)
INTERNAL_CALL, AgentBacking.changeDust(Agent.State,uint64)(_agent_2 (-> []),newDustAMG_1)
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
