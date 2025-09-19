
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





### Storage layout (WNatMock) 

```text
governanceVP IGovernanceVotePower
delegations mapping(address => WNatMock.Delegation[])
delegators mapping(address => EnumerableSet.AddressSet)

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
### Storage layout (AgentVault) 

```text
assetManager IIAssetManager
initialized bool
__usedTokens IERC20[]
__tokenUseFlags mapping(IERC20 => uint256)
__internalWithdrawal bool
destroyed bool

```






#### MintingDefaultsFacet.mintingPaymentDefault(IReferencedPaymentNonexistence.Proof,uint256) [EXTERNAL]
```slithir
 crt = Minting.getCollateralReservation(_crtId,true)
TMP_2977(CollateralReservation.Data) = LIBRARY_CALL, dest:Minting, function:Minting.getCollateralReservation(uint256,bool), arguments:['_crtId_1', 'True'] 
crt_1 (-> ['TMP_2977'])(CollateralReservation.Data) := TMP_2977(CollateralReservation.Data)
 require(bool,error)(! _proof.data.requestBody.checkSourceAddresses,revert SourceAddressesNotSupported()())
REF_1756(IReferencedPaymentNonexistence.Response) -> _proof_1.data
REF_1757(IReferencedPaymentNonexistence.RequestBody) -> REF_1756.requestBody
REF_1758(bool) -> REF_1757.checkSourceAddresses
TMP_2978 = UnaryType.BANG REF_1758 
TMP_2979(None) = SOLIDITY_CALL revert SourceAddressesNotSupported()()
TMP_2980(None) = SOLIDITY_CALL require(bool,error)(TMP_2978,TMP_2979)
 agent = Agent.get(crt.agentVault)
REF_1760(address) -> crt_1 (-> ['TMP_2977']).agentVault
TMP_2981(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['REF_1760'] 
agent_1 (-> ['TMP_2981'])(Agent.State) := TMP_2981(Agent.State)
 Agents.requireAgentVaultOwner(agent)
LIBRARY_CALL, dest:Agents, function:Agents.requireAgentVaultOwner(Agent.State), arguments:["agent_1 (-> ['TMP_2981'])"] 
 TransactionAttestation.verifyReferencedPaymentNonexistence(_proof)
LIBRARY_CALL, dest:TransactionAttestation, function:TransactionAttestation.verifyReferencedPaymentNonexistence(IReferencedPaymentNonexistence.Proof), arguments:['_proof_1'] 
 underlyingValueUBA = Conversion.convertAmgToUBA(crt.valueAMG)
REF_1764(uint64) -> crt_1 (-> ['TMP_2977']).valueAMG
TMP_2984(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['REF_1764'] 
underlyingValueUBA_1(uint256) := TMP_2984(uint256)
 require(bool,error)(_proof.data.requestBody.standardPaymentReference == PaymentReference.minting(_crtId) && _proof.data.requestBody.destinationAddressHash == agent.underlyingAddressHash && _proof.data.requestBody.amount == underlyingValueUBA + crt.underlyingFeeUBA,revert MintingNonPaymentMismatch()())
REF_1765(IReferencedPaymentNonexistence.Response) -> _proof_1.data
REF_1766(IReferencedPaymentNonexistence.RequestBody) -> REF_1765.requestBody
REF_1767(bytes32) -> REF_1766.standardPaymentReference
TMP_2985(bytes32) = LIBRARY_CALL, dest:PaymentReference, function:PaymentReference.minting(uint256), arguments:['_crtId_1'] 
TMP_2986(bool) = REF_1767 == TMP_2985
REF_1769(IReferencedPaymentNonexistence.Response) -> _proof_1.data
REF_1770(IReferencedPaymentNonexistence.RequestBody) -> REF_1769.requestBody
REF_1771(bytes32) -> REF_1770.destinationAddressHash
REF_1772(bytes32) -> agent_1 (-> ['TMP_2981']).underlyingAddressHash
TMP_2987(bool) = REF_1771 == REF_1772
TMP_2988(bool) = TMP_2986 && TMP_2987
REF_1773(IReferencedPaymentNonexistence.Response) -> _proof_1.data
REF_1774(IReferencedPaymentNonexistence.RequestBody) -> REF_1773.requestBody
REF_1775(uint256) -> REF_1774.amount
REF_1776(uint128) -> crt_1 (-> ['TMP_2977']).underlyingFeeUBA
TMP_2989(uint256) = underlyingValueUBA_1 (c)+ REF_1776
TMP_2990(bool) = REF_1775 == TMP_2989
TMP_2991(bool) = TMP_2988 && TMP_2990
TMP_2992(None) = SOLIDITY_CALL revert MintingNonPaymentMismatch()()
TMP_2993(None) = SOLIDITY_CALL require(bool,error)(TMP_2991,TMP_2992)
 require(bool,error)(_proof.data.responseBody.firstOverflowBlockNumber > crt.lastUnderlyingBlock && _proof.data.responseBody.firstOverflowBlockTimestamp > crt.lastUnderlyingTimestamp,revert MintingDefaultTooEarly()())
REF_1777(IReferencedPaymentNonexistence.Response) -> _proof_1.data
REF_1778(IReferencedPaymentNonexistence.ResponseBody) -> REF_1777.responseBody
REF_1779(uint64) -> REF_1778.firstOverflowBlockNumber
REF_1780(uint64) -> crt_1 (-> ['TMP_2977']).lastUnderlyingBlock
TMP_2994(bool) = REF_1779 > REF_1780
REF_1781(IReferencedPaymentNonexistence.Response) -> _proof_1.data
REF_1782(IReferencedPaymentNonexistence.ResponseBody) -> REF_1781.responseBody
REF_1783(uint64) -> REF_1782.firstOverflowBlockTimestamp
REF_1784(uint64) -> crt_1 (-> ['TMP_2977']).lastUnderlyingTimestamp
TMP_2995(bool) = REF_1783 > REF_1784
TMP_2996(bool) = TMP_2994 && TMP_2995
TMP_2997(None) = SOLIDITY_CALL revert MintingDefaultTooEarly()()
TMP_2998(None) = SOLIDITY_CALL require(bool,error)(TMP_2996,TMP_2997)
 require(bool,error)(_proof.data.requestBody.minimalBlockNumber <= crt.firstUnderlyingBlock,revert MintingNonPaymentProofWindowTooShort()())
REF_1785(IReferencedPaymentNonexistence.Response) -> _proof_1.data
REF_1786(IReferencedPaymentNonexistence.RequestBody) -> REF_1785.requestBody
REF_1787(uint64) -> REF_1786.minimalBlockNumber
REF_1788(uint64) -> crt_1 (-> ['TMP_2977']).firstUnderlyingBlock
TMP_2999(bool) = REF_1787 <= REF_1788
TMP_3000(None) = SOLIDITY_CALL revert MintingNonPaymentProofWindowTooShort()()
TMP_3001(None) = SOLIDITY_CALL require(bool,error)(TMP_2999,TMP_3000)
 reservedValueUBA = underlyingValueUBA + Minting.calculatePoolFeeUBA(agent,crt)
TMP_3002(uint256) = LIBRARY_CALL, dest:Minting, function:Minting.calculatePoolFeeUBA(Agent.State,CollateralReservation.Data), arguments:["agent_1 (-> ['TMP_2981'])", "crt_1 (-> ['TMP_2977'])"] 
TMP_3003(uint256) = underlyingValueUBA_1 (c)+ TMP_3002
reservedValueUBA_1(uint256) := TMP_3003(uint256)
 IAssetManagerEvents.MintingPaymentDefault(crt.agentVault,crt.minter,_crtId,reservedValueUBA)
REF_1791(address) -> crt_1 (-> ['TMP_2977']).agentVault
REF_1792(address) -> crt_1 (-> ['TMP_2977']).minter
Emit MintingPaymentDefault(REF_1791,REF_1792,_crtId_1,reservedValueUBA_1)
 totalFee = crt.reservationFeeNatWei + crt.executorFeeNatGWei * Conversion.GWEI
REF_1793(uint128) -> crt_1 (-> ['TMP_2977']).reservationFeeNatWei
REF_1794(uint64) -> crt_1 (-> ['TMP_2977']).executorFeeNatGWei
REF_1795(uint256) -> Conversion.GWEI
TMP_3005(uint64) = REF_1794 (c)* REF_1795
TMP_3006(uint128) = REF_1793 (c)+ TMP_3005
totalFee_1(uint256) := TMP_3006(uint128)
 Minting.releaseCollateralReservation(crt,CollateralReservation.Status.DEFAULTED)
REF_1797(CollateralReservation.Status) -> Status.DEFAULTED
LIBRARY_CALL, dest:Minting, function:Minting.releaseCollateralReservation(CollateralReservation.Data,CollateralReservation.Status), arguments:["crt_1 (-> ['TMP_2977'])", 'REF_1797'] 
 Minting.distributeCollateralReservationFee(agent,totalFee)
LIBRARY_CALL, dest:Minting, function:Minting.distributeCollateralReservationFee(Agent.State,uint256), arguments:["agent_1 (-> ['TMP_2981'])", 'totalFee_1'] 
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### MintingDefaultsFacet.unstickMinting(IConfirmedBlockHeightExists.Proof,uint256) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_3010(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_3010'])(AssetManagerSettings.Data) := TMP_3010(AssetManagerSettings.Data)
 crt = Minting.getCollateralReservation(_crtId,true)
TMP_3011(CollateralReservation.Data) = LIBRARY_CALL, dest:Minting, function:Minting.getCollateralReservation(uint256,bool), arguments:['_crtId_1', 'True'] 
crt_1 (-> ['TMP_3011'])(CollateralReservation.Data) := TMP_3011(CollateralReservation.Data)
 agent = Agent.get(crt.agentVault)
REF_1802(address) -> crt_1 (-> ['TMP_3011']).agentVault
TMP_3012(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['REF_1802'] 
agent_1 (-> ['TMP_3012'])(Agent.State) := TMP_3012(Agent.State)
 Agents.requireAgentVaultOwner(agent)
LIBRARY_CALL, dest:Agents, function:Agents.requireAgentVaultOwner(Agent.State), arguments:["agent_1 (-> ['TMP_3012'])"] 
 TransactionAttestation.verifyConfirmedBlockHeightExists(_proof)
LIBRARY_CALL, dest:TransactionAttestation, function:TransactionAttestation.verifyConfirmedBlockHeightExists(IConfirmedBlockHeightExists.Proof), arguments:['_proof_1'] 
 require(bool,error)(_proof.data.responseBody.lowestQueryWindowBlockNumber > crt.lastUnderlyingBlock && _proof.data.responseBody.lowestQueryWindowBlockTimestamp > crt.lastUnderlyingTimestamp && _proof.data.responseBody.lowestQueryWindowBlockTimestamp + settings.attestationWindowSeconds <= _proof.data.responseBody.blockTimestamp,revert CannotUnstickMintingYet()())
REF_1805(IConfirmedBlockHeightExists.Response) -> _proof_1.data
REF_1806(IConfirmedBlockHeightExists.ResponseBody) -> REF_1805.responseBody
REF_1807(uint64) -> REF_1806.lowestQueryWindowBlockNumber
REF_1808(uint64) -> crt_1 (-> ['TMP_3011']).lastUnderlyingBlock
TMP_3015(bool) = REF_1807 > REF_1808
REF_1809(IConfirmedBlockHeightExists.Response) -> _proof_1.data
REF_1810(IConfirmedBlockHeightExists.ResponseBody) -> REF_1809.responseBody
REF_1811(uint64) -> REF_1810.lowestQueryWindowBlockTimestamp
REF_1812(uint64) -> crt_1 (-> ['TMP_3011']).lastUnderlyingTimestamp
TMP_3016(bool) = REF_1811 > REF_1812
TMP_3017(bool) = TMP_3015 && TMP_3016
REF_1813(IConfirmedBlockHeightExists.Response) -> _proof_1.data
REF_1814(IConfirmedBlockHeightExists.ResponseBody) -> REF_1813.responseBody
REF_1815(uint64) -> REF_1814.lowestQueryWindowBlockTimestamp
REF_1816(uint64) -> settings_1 (-> ['TMP_3010']).attestationWindowSeconds
TMP_3018(uint64) = REF_1815 (c)+ REF_1816
REF_1817(IConfirmedBlockHeightExists.Response) -> _proof_1.data
REF_1818(IConfirmedBlockHeightExists.ResponseBody) -> REF_1817.responseBody
REF_1819(uint64) -> REF_1818.blockTimestamp
TMP_3019(bool) = TMP_3018 <= REF_1819
TMP_3020(bool) = TMP_3017 && TMP_3019
TMP_3021(None) = SOLIDITY_CALL revert CannotUnstickMintingYet()()
TMP_3022(None) = SOLIDITY_CALL require(bool,error)(TMP_3020,TMP_3021)
 Globals.getBurnAddress().transfer(crt.reservationFeeNatWei + crt.executorFeeNatGWei * Conversion.GWEI)
TMP_3023(address) = LIBRARY_CALL, dest:Globals, function:Globals.getBurnAddress(), arguments:[] 
REF_1822(uint128) -> crt_1 (-> ['TMP_3011']).reservationFeeNatWei
REF_1823(uint64) -> crt_1 (-> ['TMP_3011']).executorFeeNatGWei
REF_1824(uint256) -> Conversion.GWEI
TMP_3024(uint64) = REF_1823 (c)* REF_1824
TMP_3025(uint128) = REF_1822 (c)+ TMP_3024
Transfer dest:TMP_3023 value:TMP_3025
 amgToTokenWeiPrice = Conversion.currentAmgPriceInTokenWei(agent.vaultCollateralIndex)
REF_1826(uint16) -> agent_1 (-> ['TMP_3012']).vaultCollateralIndex
TMP_3027(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.currentAmgPriceInTokenWei(uint256), arguments:['REF_1826'] 
amgToTokenWeiPrice_1(uint256) := TMP_3027(uint256)
 reservedCollateral = Conversion.convertAmgToTokenWei(crt.valueAMG,amgToTokenWeiPrice)
REF_1828(uint64) -> crt_1 (-> ['TMP_3011']).valueAMG
TMP_3028(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToTokenWei(uint256,uint256), arguments:['REF_1828', 'amgToTokenWeiPrice_1'] 
reservedCollateral_1(uint256) := TMP_3028(uint256)
 burnedNatWei = _burnVaultCollateral(agent,reservedCollateral)
TMP_3029(uint256) = INTERNAL_CALL, MintingDefaultsFacet._burnVaultCollateral(Agent.State,uint256)(agent_1 (-> ['TMP_3012']),reservedCollateral_1)
burnedNatWei_1(uint256) := TMP_3029(uint256)
 reservedValueUBA = Conversion.convertAmgToUBA(crt.valueAMG) + Minting.calculatePoolFeeUBA(agent,crt)
REF_1830(uint64) -> crt_1 (-> ['TMP_3011']).valueAMG
TMP_3030(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['REF_1830'] 
TMP_3031(uint256) = LIBRARY_CALL, dest:Minting, function:Minting.calculatePoolFeeUBA(Agent.State,CollateralReservation.Data), arguments:["agent_1 (-> ['TMP_3012'])", "crt_1 (-> ['TMP_3011'])"] 
TMP_3032(uint256) = TMP_3030 (c)+ TMP_3031
reservedValueUBA_1(uint256) := TMP_3032(uint256)
 IAssetManagerEvents.CollateralReservationDeleted(crt.agentVault,crt.minter,_crtId,reservedValueUBA)
REF_1833(address) -> crt_1 (-> ['TMP_3011']).agentVault
REF_1834(address) -> crt_1 (-> ['TMP_3011']).minter
Emit CollateralReservationDeleted(REF_1833,REF_1834,_crtId_1,reservedValueUBA_1)
 Minting.releaseCollateralReservation(crt,CollateralReservation.Status.EXPIRED)
REF_1836(CollateralReservation.Status) -> Status.EXPIRED
LIBRARY_CALL, dest:Minting, function:Minting.releaseCollateralReservation(CollateralReservation.Data,CollateralReservation.Status), arguments:["crt_1 (-> ['TMP_3011'])", 'REF_1836'] 
 Transfers.transferNAT(address(msg.sender),msg.value - burnedNatWei)
TMP_3035 = CONVERT msg.sender to address
TMP_3036(uint256) = msg.value (c)- burnedNatWei_1
LIBRARY_CALL, dest:Transfers, function:Transfers.transferNAT(address,uint256), arguments:['TMP_3035', 'TMP_3036'] 
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### Agents.getOwnerPayAddress(Agent.State) [INTERNAL]
```slithir
 workAddress = getWorkAddress(_agent)
TMP_4483(address) = INTERNAL_CALL, Agents.getWorkAddress(Agent.State)(_agent_1 (-> []))
workAddress_1(address) := TMP_4483(address)
 workAddress != address(0)
TMP_4484 = CONVERT 0 to address
TMP_4485(bool) = workAddress_1 != TMP_4484
CONDITION TMP_4485
 address(workAddress)
TMP_4486 = CONVERT workAddress_1 to address
RETURN TMP_4486
 address(_agent.ownerManagementAddress)
REF_2993(address) -> _agent_1 (-> []).ownerManagementAddress
TMP_4487 = CONVERT REF_2993 to address
RETURN TMP_4487
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
#### Conversion.convert(uint256,CollateralTypeInt.Data,CollateralTypeInt.Data) [INTERNAL]
```slithir
 priceMul = currentAmgPriceInTokenWei(_toToken)
TMP_4652(uint256) = INTERNAL_CALL, Conversion.currentAmgPriceInTokenWei(CollateralTypeInt.Data)(_toToken_1 (-> []))
priceMul_1(uint256) := TMP_4652(uint256)
 priceDiv = currentAmgPriceInTokenWei(_fromToken)
TMP_4653(uint256) = INTERNAL_CALL, Conversion.currentAmgPriceInTokenWei(CollateralTypeInt.Data)(_fromToken_1 (-> []))
priceDiv_1(uint256) := TMP_4653(uint256)
 _amount.mulDiv(priceMul,priceDiv)
TMP_4654(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_amount_1', 'priceMul_1', 'priceDiv_1'] 
RETURN TMP_4654
```
#### Globals.getBurnAddress() [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4739(AssetManagerSettings.Data) = INTERNAL_CALL, Globals.getSettings()()
settings_1 (-> ['TMP_4739'])(AssetManagerSettings.Data) := TMP_4739(AssetManagerSettings.Data)
 settings.burnAddress
REF_3244(address) -> settings_1 (-> ['TMP_4739']).burnAddress
RETURN REF_3244
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
#### SafePct.mulBips(uint256,uint256) [INTERNAL]
```slithir
MAX_BIPS_1(uint256) := phi(['MAX_BIPS_2', 'MAX_BIPS_0'])
 mulDiv(x,y,MAX_BIPS)
TMP_10535(uint256) = INTERNAL_CALL, SafePct.mulDiv(uint256,uint256,uint256)(x_1,y_1,MAX_BIPS_1)
RETURN TMP_10535
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
#### Agents.requireAgentVaultOwner(Agent.State) [INTERNAL]
```slithir
 require(bool,error)(isOwner(_agent,msg.sender),revert OnlyAgentVaultOwner()())
TMP_4497(bool) = INTERNAL_CALL, Agents.isOwner(Agent.State,address)(_agent_1 (-> []),msg.sender)
TMP_4498(None) = SOLIDITY_CALL revert OnlyAgentVaultOwner()()
TMP_4499(None) = SOLIDITY_CALL require(bool,error)(TMP_4497,TMP_4498)
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
#### Minting.calculatePoolFeeUBA(Agent.State,CollateralReservation.Data) [INTERNAL]
```slithir
_agent_1 (-> ['TMP_4821'])(Agent.State) := phi(["agent_1 (-> ['TMP_4821'])"])
_crt_1 (-> [])(CollateralReservation.Data) := phi(['_crt_1 (-> [])'])
 storedPoolFeeShareBIPS = _crt.poolFeeShareBIPS
REF_3355(uint16) -> _crt_1 (-> []).poolFeeShareBIPS
storedPoolFeeShareBIPS_1(uint16) := REF_3355(uint16)
 _calculatePoolFeeUBA(_crt.underlyingFeeUBA,poolFeeShareBIPS)
REF_3356(uint128) -> _crt_1 (-> []).underlyingFeeUBA
TMP_4849(uint256) = INTERNAL_CALL, Minting._calculatePoolFeeUBA(uint256,uint16)(REF_3356,poolFeeShareBIPS_3)
RETURN TMP_4849
 storedPoolFeeShareBIPS > 0
TMP_4850(bool) = storedPoolFeeShareBIPS_1 > 0
CONDITION TMP_4850
 poolFeeShareBIPS = storedPoolFeeShareBIPS - 1
TMP_4851(uint16) = storedPoolFeeShareBIPS_1 (c)- 1
poolFeeShareBIPS_1(uint16) := TMP_4851(uint16)
 poolFeeShareBIPS = _agent.poolFeeShareBIPS
REF_3357(uint16) -> _agent_1 (-> ['TMP_4821']).poolFeeShareBIPS
poolFeeShareBIPS_2(uint16) := REF_3357(uint16)
poolFeeShareBIPS_3(uint16) := phi(['poolFeeShareBIPS_1', 'poolFeeShareBIPS_2'])
```
#### Minting.distributeCollateralReservationFee(Agent.State,uint256) [INTERNAL]
```slithir
 _fee == 0
TMP_4806(bool) = _fee_1 == 0
CONDITION TMP_4806
 poolFeeShare = _fee.mulBips(_agent.poolFeeShareBIPS)
REF_3317(uint16) -> _agent_1 (-> []).poolFeeShareBIPS
TMP_4807(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['_fee_1', 'REF_3317'] 
poolFeeShare_1(uint256) := TMP_4807(uint256)
 _agent.collateralPool.depositNat{value: poolFeeShare}()
REF_3318(IICollateralPool) -> _agent_1 (-> []).collateralPool
HIGH_LEVEL_CALL, dest:REF_3318(IICollateralPool), function:depositNat, arguments:[] value:poolFeeShare_1 
 Transfers.depositWNat(Globals.getWNat(),Agents.getOwnerPayAddress(_agent),_fee - poolFeeShare)
TMP_4809(IWNat) = LIBRARY_CALL, dest:Globals, function:Globals.getWNat(), arguments:[] 
TMP_4810(address) = LIBRARY_CALL, dest:Agents, function:Agents.getOwnerPayAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
TMP_4811(uint256) = _fee_1 (c)- poolFeeShare_1
LIBRARY_CALL, dest:Transfers, function:Transfers.depositWNat(IWNat,address,uint256), arguments:['TMP_4809', 'TMP_4810', 'TMP_4811']
```
#### Minting.getCollateralReservation(uint256,bool) [INTERNAL]
```slithir
 require(bool,error)(_crtId > 0,revert InvalidCrtId()())
TMP_4828(bool) = _crtId_1 > 0
TMP_4829(None) = SOLIDITY_CALL revert InvalidCrtId()()
TMP_4830(None) = SOLIDITY_CALL require(bool,error)(TMP_4828,TMP_4829)
 state = AssetManagerState.get()
TMP_4831(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4831'])(AssetManagerState.State) := TMP_4831(AssetManagerState.State)
 _crt = state.crts[_crtId]
REF_3343(mapping(uint256 => CollateralReservation.Data)) -> state_1 (-> ['TMP_4831']).crts
REF_3344(CollateralReservation.Data) -> REF_3343[_crtId_1]
_crt_1 (-> ['state'])(CollateralReservation.Data) := REF_3344(CollateralReservation.Data)
 require(bool,error)(_crt.valueAMG != 0,revert InvalidCrtId()())
REF_3345(uint64) -> _crt_1 (-> ['state']).valueAMG
TMP_4832(bool) = REF_3345 != 0
TMP_4833(None) = SOLIDITY_CALL revert InvalidCrtId()()
TMP_4834(None) = SOLIDITY_CALL require(bool,error)(TMP_4832,TMP_4833)
 _requireActive
CONDITION _requireActive_1
 require(bool,error)(_crt.status == CollateralReservation.Status.ACTIVE,revert InvalidCrtId()())
REF_3346(CollateralReservation.Status) -> _crt_1 (-> ['state']).status
REF_3347(CollateralReservation.Status) -> Status.ACTIVE
TMP_4835(bool) = REF_3346 == REF_3347
TMP_4836(None) = SOLIDITY_CALL revert InvalidCrtId()()
TMP_4837(None) = SOLIDITY_CALL require(bool,error)(TMP_4835,TMP_4836)
 _crt
RETURN _crt_1 (-> ['state'])
```
#### Minting.releaseCollateralReservation(CollateralReservation.Data,CollateralReservation.Status) [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4820(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4820'])(AssetManagerState.State) := TMP_4820(AssetManagerState.State)
 agent = Agent.get(_crt.agentVault)
REF_3334(address) -> _crt_1 (-> []).agentVault
TMP_4821(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['REF_3334'] 
agent_1 (-> ['TMP_4821'])(Agent.State) := TMP_4821(Agent.State)
 reservationAMG = _crt.valueAMG + Conversion.convertUBAToAmg(calculatePoolFeeUBA(agent,_crt))
REF_3335(uint64) -> _crt_1 (-> []).valueAMG
TMP_4822(uint256) = INTERNAL_CALL, Minting.calculatePoolFeeUBA(Agent.State,CollateralReservation.Data)(agent_1 (-> ['TMP_4821']),_crt_1 (-> []))
TMP_4823(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertUBAToAmg(uint256), arguments:['TMP_4822'] 
TMP_4824(uint64) = REF_3335 (c)+ TMP_4823
reservationAMG_1(uint64) := TMP_4824(uint64)
 agent.reservedAMG = agent.reservedAMG - reservationAMG
REF_3337(uint64) -> agent_1 (-> ['TMP_4821']).reservedAMG
REF_3338(uint64) -> agent_1 (-> ['TMP_4821']).reservedAMG
TMP_4825(uint64) = REF_3338 (c)- reservationAMG_1
agent_2 (-> ['TMP_4821'])(Agent.State) := phi(["agent_1 (-> ['TMP_4821'])"])
REF_3337(uint64) (->agent_2 (-> ['TMP_4821'])) := TMP_4825(uint64)
TMP_4821(Agent.State) := phi(["agent_2 (-> ['TMP_4821'])"])
 state.totalReservedCollateralAMG -= reservationAMG
REF_3339(uint64) -> state_1 (-> ['TMP_4820']).totalReservedCollateralAMG
state_2 (-> ['TMP_4820'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_4820'])"])
REF_3339(-> state_2 (-> ['TMP_4820'])) = REF_3339 (c)- reservationAMG_1
TMP_4820(AssetManagerState.State) := phi(["state_2 (-> ['TMP_4820'])"])
 assert(bool)(_status != CollateralReservation.Status.ACTIVE)
REF_3340(CollateralReservation.Status) -> Status.ACTIVE
TMP_4826(bool) = _status_1 != REF_3340
TMP_4827(None) = SOLIDITY_CALL assert(bool)(TMP_4826)
 _crt.status = _status
REF_3341(CollateralReservation.Status) -> _crt_1 (-> []).status
_crt_2 (-> [])(CollateralReservation.Data) := phi(['_crt_1 (-> [])'])
REF_3341(CollateralReservation.Status) (->_crt_2 (-> [])) := _status_1(CollateralReservation.Status)
```
#### TransactionAttestation.verifyReferencedPaymentNonexistence(IReferencedPaymentNonexistence.Proof) [INTERNAL]
```slithir
 _settings = Globals.getSettings()
TMP_5261(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
_settings_1 (-> ['TMP_5261'])(AssetManagerSettings.Data) := TMP_5261(AssetManagerSettings.Data)
 fdcVerification = IFdcVerification(_settings.fdcVerification)
REF_3688(address) -> _settings_1 (-> ['TMP_5261']).fdcVerification
TMP_5262 = CONVERT REF_3688 to IFdcVerification
fdcVerification_1(IFdcVerification) := TMP_5262(IFdcVerification)
 require(bool,error)(_proof.data.sourceId == _settings.chainId,revert InvalidChain()())
REF_3689(IReferencedPaymentNonexistence.Response) -> _proof_1.data
REF_3690(bytes32) -> REF_3689.sourceId
REF_3691(bytes32) -> _settings_1 (-> ['TMP_5261']).chainId
TMP_5263(bool) = REF_3690 == REF_3691
TMP_5264(None) = SOLIDITY_CALL revert InvalidChain()()
TMP_5265(None) = SOLIDITY_CALL require(bool,error)(TMP_5263,TMP_5264)
 require(bool,error)(fdcVerification.verifyReferencedPaymentNonexistence(_proof),revert NonPaymentNotProven()())
TMP_5266(bool) = HIGH_LEVEL_CALL, dest:fdcVerification_1(IFdcVerification), function:verifyReferencedPaymentNonexistence, arguments:['_proof_1']  
TMP_5267(None) = SOLIDITY_CALL revert NonPaymentNotProven()()
TMP_5268(None) = SOLIDITY_CALL require(bool,error)(TMP_5266,TMP_5267)
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
#### PaymentReference.minting(uint256) [INTERNAL]
```slithir
MAX_ID_1(uint256) := phi(['MAX_ID_0'])
MINTING_1(uint256) := phi(['MINTING_0'])
 assert(bool)(_id <= MAX_ID)
TMP_5345(bool) = _id_1 <= MAX_ID_1
TMP_5346(None) = SOLIDITY_CALL assert(bool)(TMP_5345)
 bytes32(_id | MINTING)
TMP_5347(uint256) = _id_1 | MINTING_1
TMP_5348 = CONVERT TMP_5347 to bytes32
RETURN TMP_5348
```
#### Conversion.convertAmgToTokenWei(uint256,uint256) [INTERNAL]
```slithir
AMG_TOKEN_WEI_PRICE_SCALE_1(uint256) := phi(['AMG_TOKEN_WEI_PRICE_SCALE_0'])
 _valueAMG.mulDiv(_amgToTokenWeiPrice,AMG_TOKEN_WEI_PRICE_SCALE)
TMP_4674(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_valueAMG_1', '_amgToTokenWeiPrice_1', 'AMG_TOKEN_WEI_PRICE_SCALE_1'] 
RETURN TMP_4674
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
#### TransactionAttestation.verifyConfirmedBlockHeightExists(IConfirmedBlockHeightExists.Proof) [INTERNAL]
```slithir
 _settings = Globals.getSettings()
TMP_5253(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
_settings_1 (-> ['TMP_5253'])(AssetManagerSettings.Data) := TMP_5253(AssetManagerSettings.Data)
 fdcVerification = IFdcVerification(_settings.fdcVerification)
REF_3682(address) -> _settings_1 (-> ['TMP_5253']).fdcVerification
TMP_5254 = CONVERT REF_3682 to IFdcVerification
fdcVerification_1(IFdcVerification) := TMP_5254(IFdcVerification)
 require(bool,error)(_proof.data.sourceId == _settings.chainId,revert InvalidChain()())
REF_3683(IConfirmedBlockHeightExists.Response) -> _proof_1.data
REF_3684(bytes32) -> REF_3683.sourceId
REF_3685(bytes32) -> _settings_1 (-> ['TMP_5253']).chainId
TMP_5255(bool) = REF_3684 == REF_3685
TMP_5256(None) = SOLIDITY_CALL revert InvalidChain()()
TMP_5257(None) = SOLIDITY_CALL require(bool,error)(TMP_5255,TMP_5256)
 require(bool,error)(fdcVerification.verifyConfirmedBlockHeightExists(_proof),revert BlockHeightNotProven()())
TMP_5258(bool) = HIGH_LEVEL_CALL, dest:fdcVerification_1(IFdcVerification), function:verifyConfirmedBlockHeightExists, arguments:['_proof_1']  
TMP_5259(None) = SOLIDITY_CALL revert BlockHeightNotProven()()
TMP_5260(None) = SOLIDITY_CALL require(bool,error)(TMP_5258,TMP_5259)
```
#### Transfers.transferNAT(address,uint256) [INTERNAL]
```slithir
TRANSFER_GAS_ALLOWANCE_1(uint256) := phi(['TRANSFER_GAS_ALLOWANCE_0', 'TRANSFER_GAS_ALLOWANCE_3', 'TRANSFER_GAS_ALLOWANCE_2'])
 _amount > 0
TMP_10536(bool) = _amount_1 > 0
CONDITION TMP_10536
 (success,None) = _recipient.call{gas: TRANSFER_GAS_ALLOWANCE,value: _amount}()
TUPLE_91(bool,bytes) = LOW_LEVEL_CALL, dest:_recipient_1, function:call, arguments:[''] value:_amount_1 gas:TRANSFER_GAS_ALLOWANCE_2
TRANSFER_GAS_ALLOWANCE_3(uint256) := phi(['TRANSFER_GAS_ALLOWANCE_3', 'TRANSFER_GAS_ALLOWANCE_2'])
success_1(bool)= UNPACK TUPLE_91 index: 0 
 require(bool,error)(success,revert TransferFailed()())
TMP_10537(None) = SOLIDITY_CALL revert TransferFailed()()
TMP_10538(None) = SOLIDITY_CALL require(bool,error)(success_1,TMP_10537)
 requireReentrancyGuard()
MODIFIER_CALL, Transfers.requireReentrancyGuard()()
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
#### SafeERC20.safeTransfer(IERC20,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeWithSelector(token.transfer.selector,to,value))
REF_86(bytes4) (->None) := 2835717307(bytes4)
TMP_243(bytes) = SOLIDITY_CALL abi.encodeWithSelector()(REF_86,to_1,value_1)
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_243)
```
#### Agents.isOwner(Agent.State,address) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['TMP_4493', '_agent_1 (-> [])'])
_address_1(address) := phi(['msg.sender'])
 _address == _agent.ownerManagementAddress || _address == getWorkAddress(_agent)
REF_2989(address) -> _agent_1 (-> []).ownerManagementAddress
TMP_4477(bool) = _address_1 == REF_2989
TMP_4478(address) = INTERNAL_CALL, Agents.getWorkAddress(Agent.State)(_agent_1 (-> []))
TMP_4479(bool) = _address_1 == TMP_4478
TMP_4480(bool) = TMP_4477 || TMP_4479
RETURN TMP_4480
```

#### Globals.getWNat() [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4731(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4731'])(AssetManagerState.State) := TMP_4731(AssetManagerState.State)
 IWNat(address(state.collateralTokens[state.poolCollateralIndex].token))
REF_3231(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4731']).collateralTokens
REF_3232(uint16) -> state_1 (-> ['TMP_4731']).poolCollateralIndex
REF_3233(CollateralTypeInt.Data) -> REF_3231[REF_3232]
REF_3234(IERC20) -> REF_3233.token
TMP_4732 = CONVERT REF_3234 to address
TMP_4733 = CONVERT TMP_4732 to IWNat
RETURN TMP_4733
```
#### CollateralPool.depositNat() [EXTERNAL]
```slithir
 _depositWNat()
INTERNAL_CALL, CollateralPool._depositWNat()()
 onlyAssetManager()
MODIFIER_CALL, CollateralPool.onlyAssetManager()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### Transfers.depositWNat(IWNat,address,uint256) [INTERNAL]
```slithir
 _amount > 0
TMP_10540(bool) = _amount_1 > 0
CONDITION TMP_10540
 _wNat.depositTo{value: _amount}(_recipient)
HIGH_LEVEL_CALL, dest:_wNat_1(IWNat), function:depositTo, arguments:['_recipient_1'] value:_amount_1
```
#### Conversion.convertUBAToAmg(uint256) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4640(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4640'])(AssetManagerSettings.Data) := TMP_4640(AssetManagerSettings.Data)
 SafeCast.toUint64(_valueUBA / settings.assetMintingGranularityUBA)
REF_3148(uint64) -> settings_1 (-> ['TMP_4640']).assetMintingGranularityUBA
TMP_4641(uint256) = _valueUBA_1 (c)/ REF_3148
TMP_4642(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['TMP_4641'] 
RETURN TMP_4642
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
#### Conversion.roundUBAToAmg(uint256) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4643(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4643'])(AssetManagerSettings.Data) := TMP_4643(AssetManagerSettings.Data)
 _valueUBA - (_valueUBA % settings.assetMintingGranularityUBA)
REF_3150(uint64) -> settings_1 (-> ['TMP_4643']).assetMintingGranularityUBA
TMP_4644(uint256) = _valueUBA_1 % REF_3150
TMP_4645(uint256) = _valueUBA_1 (c)- TMP_4644
RETURN TMP_4645
```
#### CollateralPool._depositWNat() [INTERNAL]
```slithir
agentVault_29(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_1', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_0', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_28(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_14', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_0', 'assetManager_30'])
wNat_7(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_0', 'wNat_25', 'wNat_15', 'wNat_36'])
totalCollateral_36(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_0', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 msg.value > 0
TMP_6312(bool) = msg.value > 0
CONDITION TMP_6312
 totalCollateral += msg.value
totalCollateral_37(uint256) = totalCollateral_36 (c)+ msg.value
 wNat.deposit{value: msg.value}()
HIGH_LEVEL_CALL, dest:wNat_7(IWNat), function:deposit, arguments:[] value:msg.value 
agentVault_30(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_29', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_29(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_28', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
wNat_8(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_7', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_25', 'wNat_15', 'wNat_36'])
 assetManager.updateCollateral(agentVault,wNat)
HIGH_LEVEL_CALL, dest:assetManager_29(IIAssetManager), function:updateCollateral, arguments:['agentVault_30', 'wNat_8']  
agentVault_31(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_30', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_30(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_29', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
wNat_9(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_8', 'wNat_28', 'wNat_43', 'wNat_25', 'wNat_15', 'wNat_36'])
```
#### WNatMock.depositTo(address) [PUBLIC]
```slithir
 _mint(_recipient,msg.value)
INTERNAL_CALL, ERC20._mint(address,uint256)(_recipient_1,msg.value)
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
#### Conversion.calcAmgToTokenWeiPrice(uint256,uint256,uint256,uint256,uint256) [INTERNAL]
```slithir
_tokenDecimals_1(uint256) := phi(['REF_3165', 'REF_3167'])
_tokenPrice_1(uint256) := phi(['tokenPrice_1'])
_tokenFtsoDecimals_1(uint256) := phi(['tokenFtsoDec_1'])
_assetPrice_1(uint256) := phi(['assetPrice_1'])
_assetFtsoDecimals_1(uint256) := phi(['assetFtsoDec_1'])
AMG_TOKEN_WEI_PRICE_SCALE_EXP_1(uint256) := phi(['AMG_TOKEN_WEI_PRICE_SCALE_EXP_0'])
 settings = Globals.getSettings()
TMP_4665(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4665'])(AssetManagerSettings.Data) := TMP_4665(AssetManagerSettings.Data)
 expPlus = _tokenDecimals + _tokenFtsoDecimals + AMG_TOKEN_WEI_PRICE_SCALE_EXP
TMP_4666(uint256) = _tokenDecimals_1 (c)+ _tokenFtsoDecimals_1
TMP_4667(uint256) = TMP_4666 (c)+ AMG_TOKEN_WEI_PRICE_SCALE_EXP_1
expPlus_1(uint256) := TMP_4667(uint256)
 expMinus = settings.assetMintingDecimals + _assetFtsoDecimals
REF_3173(uint8) -> settings_1 (-> ['TMP_4665']).assetMintingDecimals
TMP_4668(uint8) = REF_3173 (c)+ _assetFtsoDecimals_1
expMinus_1(uint256) := TMP_4668(uint8)
 assert(bool)(expPlus >= expMinus)
TMP_4669(bool) = expPlus_1 >= expMinus_1
TMP_4670(None) = SOLIDITY_CALL assert(bool)(TMP_4669)
 _assetPrice.mulDiv(10 ** (expPlus - expMinus),_tokenPrice)
TMP_4671(uint256) = expPlus_1 (c)- expMinus_1
TMP_4672(uint256) = 10 (c)** TMP_4671
TMP_4673(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_assetPrice_1', 'TMP_4672', '_tokenPrice_1'] 
RETURN TMP_4673
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
