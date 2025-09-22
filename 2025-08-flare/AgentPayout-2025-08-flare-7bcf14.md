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
### Storage layout (AgentVault) 

```text
assetManager IIAssetManager
initialized bool
__usedTokens IERC20[]
__tokenUseFlags mapping(IERC20 => uint256)
__internalWithdrawal bool
destroyed bool

```





### Storage layout (CollateralPoolToken) 

```text
collateralPool address
tokenName string
tokenSymbol string
timelocksByAccount mapping(address => CollateralPoolToken.TimelockQueue)
ignoreTimelocked bool
initialized bool

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
#### AgentPayout.payoutFromVault(Agent.State,address,uint256) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
_receiver_1(address) := phi(['_receiver_1'])
_amountWei_1(uint256) := phi(['amount_1'])
 collateral = Agents.getVaultCollateral(_agent)
TMP_4380(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getVaultCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
collateral_1 (-> ['TMP_4380'])(CollateralTypeInt.Data) := TMP_4380(CollateralTypeInt.Data)
 vault = IIAgentVault(_agent.vaultAddress())
TMP_4381(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
TMP_4382 = CONVERT TMP_4381 to IIAgentVault
vault_1(IIAgentVault) := TMP_4382(IIAgentVault)
 _amountPaid = Math.min(_amountWei,collateral.token.balanceOf(address(vault)))
REF_2911(IERC20) -> collateral_1 (-> ['TMP_4380']).token
TMP_4383 = CONVERT vault_1 to address
TMP_4384(uint256) = HIGH_LEVEL_CALL, dest:REF_2911(IERC20), function:balanceOf, arguments:['TMP_4383']  
TMP_4385(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['_amountWei_1', 'TMP_4384'] 
_amountPaid_1(uint256) := TMP_4385(uint256)
 vault.payout(collateral.token,_receiver,_amountPaid)
REF_2914(IERC20) -> collateral_1 (-> ['TMP_4380']).token
HIGH_LEVEL_CALL, dest:vault_1(IIAgentVault), function:payout, arguments:['REF_2914', '_receiver_1', '_amountPaid_1']  
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
#### Agents.convertUSD5ToVaultCollateralWei(Agent.State,uint256) [INTERNAL]
```slithir
 Conversion.convertFromUSD5(_amountUSD5,getVaultCollateral(_agent))
TMP_4511(CollateralTypeInt.Data) = INTERNAL_CALL, Agents.getVaultCollateral(Agent.State)(_agent_1 (-> []))
TMP_4512(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertFromUSD5(uint256,CollateralTypeInt.Data), arguments:['_amountUSD5_1', 'TMP_4511'] 
RETURN TMP_4512
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
#### ERC20.balanceOf(address) [PUBLIC]
```slithir
_balances_1(mapping(address => uint256)) := phi(['_balances_11', '_balances_1', '_balances_5', '_balances_8', '_balances_0'])
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
#### AgentVault.payout(IERC20,address,uint256) [EXTERNAL]
```slithir
 _token.safeTransfer(_recipient,_amount)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['_token_1', '_recipient_1', '_amount_1'] 
 onlyAssetManager()
MODIFIER_CALL, AgentVault.onlyAssetManager()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### Conversion.convertFromUSD5(uint256,CollateralTypeInt.Data) [INTERNAL]
```slithir
 bytes(_token.tokenFtsoSymbol).length == 0
REF_3158(string) -> _token_1 (-> []).tokenFtsoSymbol
TMP_4655 = CONVERT REF_3158 to bytes
REF_3159 -> LENGTH TMP_4655
TMP_4656(bool) = REF_3159 == 0
CONDITION TMP_4656
 _amountUSD5
RETURN _amountUSD5_1
 (tokenPrice,None,tokenFtsoDec) = readFtsoPrice(_token.tokenFtsoSymbol,false)
REF_3160(string) -> _token_1 (-> []).tokenFtsoSymbol
TUPLE_47(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.readFtsoPrice(string,bool)(REF_3160,False)
tokenPrice_1(uint256)= UNPACK TUPLE_47 index: 0 
tokenFtsoDec_1(uint256)= UNPACK TUPLE_47 index: 2 
 expPlus = _token.decimals + tokenFtsoDec - 5
REF_3161(uint8) -> _token_1 (-> []).decimals
TMP_4657(uint8) = REF_3161 (c)+ tokenFtsoDec_1
TMP_4658(uint8) = TMP_4657 (c)- 5
expPlus_1(uint256) := TMP_4658(uint8)
 _amountUSD5.mulDiv(10 ** expPlus,tokenPrice)
TMP_4659(uint256) = 10 (c)** expPlus_1
TMP_4660(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_amountUSD5_1', 'TMP_4659', 'tokenPrice_1'] 
RETURN TMP_4660
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
#### SafeERC20.safeTransfer(IERC20,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeWithSelector(token.transfer.selector,to,value))
REF_86(bytes4) (->None) := 2835717307(bytes4)
TMP_243(bytes) = SOLIDITY_CALL abi.encodeWithSelector()(REF_86,to_1,value_1)
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_243)
```
#### Conversion.readFtsoPrice(string,bool) [INTERNAL]
```slithir
_symbol_1(string) := phi(['REF_3163', 'REF_3166', 'REF_3160'])
_fromTrustedProviders_1(bool) := phi(['_fromTrustedProviders_1'])
 settings = Globals.getSettings()
TMP_4663(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4663'])(AssetManagerSettings.Data) := TMP_4663(AssetManagerSettings.Data)
 priceReader = IPriceReader(settings.priceReader)
REF_3169(address) -> settings_1 (-> ['TMP_4663']).priceReader
TMP_4664 = CONVERT REF_3169 to IPriceReader
priceReader_1(IPriceReader) := TMP_4664(IPriceReader)
 _fromTrustedProviders
CONDITION _fromTrustedProviders_1
 priceReader.getPriceFromTrustedProviders(_symbol)
TUPLE_50(uint256,uint256,uint256) = HIGH_LEVEL_CALL, dest:priceReader_1(IPriceReader), function:getPriceFromTrustedProviders, arguments:['_symbol_1']  
RETURN TUPLE_50
 priceReader.getPrice(_symbol)
TUPLE_51(uint256,uint256,uint256) = HIGH_LEVEL_CALL, dest:priceReader_1(IPriceReader), function:getPrice, arguments:['_symbol_1']  
RETURN TUPLE_51
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

#### CollateralPool._totalVirtualFees() [INTERNAL]
```slithir
totalFAssetFeeDebt_1(int256) := phi(['totalFAssetFeeDebt_0', 'totalFAssetFeeDebt_5', 'totalFAssetFeeDebt_3'])
totalFAssetFees_7(uint256) := phi(['totalFAssetFees_3', 'totalFAssetFees_0', 'totalFAssetFees_14', 'totalFAssetFees_12', 'totalFAssetFees_6', 'totalFAssetFees_4', 'totalFAssetFees_10'])
 virtualFees = totalFAssetFees.toInt256() + totalFAssetFeeDebt
TMP_6274(int256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt256(uint256), arguments:['totalFAssetFees_7'] 
TMP_6275(int256) = TMP_6274 (c)+ totalFAssetFeeDebt_1
virtualFees_1(int256) := TMP_6275(int256)
 virtualFees.toUint256()
TMP_6276(uint256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint256(int256), arguments:['virtualFees_1'] 
RETURN TMP_6276
```
#### ERC20._burn(address,uint256) [INTERNAL]
```slithir
_balances_9(mapping(address => uint256)) := phi(['_balances_11', '_balances_1', '_balances_5', '_balances_8', '_balances_0'])
_totalSupply_5(uint256) := phi(['_totalSupply_0', '_totalSupply_4', '_totalSupply_7'])
 require(bool,string)(account != address(0),ERC20: burn from the zero address)
TMP_216 = CONVERT 0 to address
TMP_217(bool) = account_1 != TMP_216
TMP_218(None) = SOLIDITY_CALL require(bool,string)(TMP_217,ERC20: burn from the zero address)
 _beforeTokenTransfer(account,address(0),amount)
TMP_219 = CONVERT 0 to address
INTERNAL_CALL, ERC20._beforeTokenTransfer(address,address,uint256)(account_1,TMP_219,amount_1)
 accountBalance = _balances[account]
REF_80(uint256) -> _balances_10[account_1]
accountBalance_1(uint256) := REF_80(uint256)
 require(bool,string)(accountBalance >= amount,ERC20: burn amount exceeds balance)
TMP_221(bool) = accountBalance_1 >= amount_1
TMP_222(None) = SOLIDITY_CALL require(bool,string)(TMP_221,ERC20: burn amount exceeds balance)
 _balances[account] = accountBalance - amount
REF_81(uint256) -> _balances_10[account_1]
TMP_223(uint256) = accountBalance_1 - amount_1
_balances_11(mapping(address => uint256)) := phi(['_balances_10'])
REF_81(uint256) (->_balances_11) := TMP_223(uint256)
 _totalSupply -= amount
_totalSupply_7(uint256) = _totalSupply_6 - amount_1
 Transfer(account,address(0),amount)
TMP_224 = CONVERT 0 to address
Emit Transfer(account_1,TMP_224,amount_1)
 _afterTokenTransfer(account,address(0),amount)
TMP_226 = CONVERT 0 to address
INTERNAL_CALL, ERC20._afterTokenTransfer(address,address,uint256)(account_1,TMP_226,amount_1)
```
#### SafeERC20._callOptionalReturn(IERC20,bytes) [PRIVATE]
```slithir
token_1(IERC20) := phi(['token_1', 'token_1', 'token_1', 'token_1', 'token_1', 'token_1'])
data_1(bytes) := phi(['TMP_243', 'TMP_245', 'TMP_265', 'TMP_253', 'TMP_258', 'approvalCall_1', 'TMP_270'])
 returndata = address(token).functionCall(data,SafeERC20: low-level call failed)
TMP_279 = CONVERT token_1 to address
TMP_280(bytes) = LIBRARY_CALL, dest:Address, function:Address.functionCall(address,bytes,string), arguments:['TMP_279', 'data_1', 'SafeERC20: low-level call failed'] 
returndata_1(bytes) := TMP_280(bytes)
 require(bool,string)(returndata.length == 0 || abi.decode(returndata,(bool)),SafeERC20: ERC20 operation did not succeed)
REF_112 -> LENGTH returndata_1
TMP_281(bool) = REF_112 == 0
TMP_282(bool) = SOLIDITY_CALL abi.decode()(returndata_1,bool)
TMP_283(bool) = TMP_281 || TMP_282
TMP_284(None) = SOLIDITY_CALL require(bool,string)(TMP_283,SafeERC20: ERC20 operation did not succeed)
```
