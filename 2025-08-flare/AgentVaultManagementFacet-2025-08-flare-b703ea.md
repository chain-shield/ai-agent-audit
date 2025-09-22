

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





#### AgentVaultManagementFacet._createCollateralPool(IIAssetManager,address,AgentSettings.Data) [PRIVATE]
```slithir
_assetManager_1(IIAssetManager) := phi(['assetManager_1'])
_agentVault_1(address) := phi(['TMP_1770'])
_settings_1(AgentSettings.Data) := phi(['_settings_1'])
 globalSettings = Globals.getSettings()
TMP_1845(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
globalSettings_1 (-> ['TMP_1845'])(AssetManagerSettings.Data) := TMP_1845(AssetManagerSettings.Data)
 collateralPoolFactory = IICollateralPoolFactory(globalSettings.collateralPoolFactory)
REF_864(address) -> globalSettings_1 (-> ['TMP_1845']).collateralPoolFactory
TMP_1846 = CONVERT REF_864 to IICollateralPoolFactory
collateralPoolFactory_1(IICollateralPoolFactory) := TMP_1846(IICollateralPoolFactory)
 poolTokenFactory = IICollateralPoolTokenFactory(globalSettings.collateralPoolTokenFactory)
REF_865(address) -> globalSettings_1 (-> ['TMP_1845']).collateralPoolTokenFactory
TMP_1847 = CONVERT REF_865 to IICollateralPoolTokenFactory
poolTokenFactory_1(IICollateralPoolTokenFactory) := TMP_1847(IICollateralPoolTokenFactory)
 collateralPool = collateralPoolFactory.create(_assetManager,_agentVault,_settings)
TMP_1848(IICollateralPool) = HIGH_LEVEL_CALL, dest:collateralPoolFactory_1(IICollateralPoolFactory), function:create, arguments:['_assetManager_1', '_agentVault_1', '_settings_1']  
collateralPool_1(IICollateralPool) := TMP_1848(IICollateralPool)
 poolToken = poolTokenFactory.create(collateralPool,globalSettings.poolTokenSuffix,_settings.poolTokenSuffix)
REF_868(string) -> globalSettings_1 (-> ['TMP_1845']).poolTokenSuffix
REF_869(string) -> _settings_1.poolTokenSuffix
TMP_1849(address) = HIGH_LEVEL_CALL, dest:poolTokenFactory_1(IICollateralPoolTokenFactory), function:create, arguments:['collateralPool_1', 'REF_868', 'REF_869']  
poolToken_1(address) := TMP_1849(address)
 collateralPool.setPoolToken(poolToken)
HIGH_LEVEL_CALL, dest:collateralPool_1(IICollateralPool), function:setPoolToken, arguments:['poolToken_1']  
 collateralPool
RETURN collateralPool_1
```
#### AgentVaultManagementFacet._emitAgentVaultCreated(address,address,IICollateralPool,string,AgentSettings.Data) [PRIVATE]
```slithir
_ownerManagementAddress_1(address) := phi(['ownerManagementAddress_1'])
_agentVault_1(address) := phi(['TMP_1779'])
_collateralPool_1(IICollateralPool) := phi(['REF_802'])
_underlyingAddress_1(string) := phi(['REF_803'])
_settings_1(AgentSettings.Data) := phi(['_settings_1'])
 data.collateralPool = address(_collateralPool)
REF_878(address) -> data_0.collateralPool
TMP_1880 = CONVERT _collateralPool_1 to address
data_1(IAssetManagerEvents.AgentVaultCreationData) := phi(['data_0'])
REF_878(address) (->data_1) := TMP_1880(address)
 data.collateralPoolToken = address(_collateralPool.poolToken())
REF_879(address) -> data_1.collateralPoolToken
TMP_1881(ICollateralPoolToken) = HIGH_LEVEL_CALL, dest:_collateralPool_1(IICollateralPool), function:poolToken, arguments:[]  
TMP_1882 = CONVERT TMP_1881 to address
data_2(IAssetManagerEvents.AgentVaultCreationData) := phi(['data_1'])
REF_879(address) (->data_2) := TMP_1882(address)
 data.vaultCollateralToken = address(_settings.vaultCollateralToken)
REF_881(address) -> data_2.vaultCollateralToken
REF_882(IERC20) -> _settings_1.vaultCollateralToken
TMP_1883 = CONVERT REF_882 to address
data_3(IAssetManagerEvents.AgentVaultCreationData) := phi(['data_2'])
REF_881(address) (->data_3) := TMP_1883(address)
 data.poolWNatToken = address(_collateralPool.wNat())
REF_883(address) -> data_3.poolWNatToken
TMP_1884(IWNat) = HIGH_LEVEL_CALL, dest:_collateralPool_1(IICollateralPool), function:wNat, arguments:[]  
TMP_1885 = CONVERT TMP_1884 to address
data_4(IAssetManagerEvents.AgentVaultCreationData) := phi(['data_3'])
REF_883(address) (->data_4) := TMP_1885(address)
 data.underlyingAddress = _underlyingAddress
REF_885(string) -> data_4.underlyingAddress
data_5(IAssetManagerEvents.AgentVaultCreationData) := phi(['data_4'])
REF_885(string) (->data_5) := _underlyingAddress_1(string)
 data.feeBIPS = _settings.feeBIPS
REF_886(uint256) -> data_5.feeBIPS
REF_887(uint256) -> _settings_1.feeBIPS
data_6(IAssetManagerEvents.AgentVaultCreationData) := phi(['data_5'])
REF_886(uint256) (->data_6) := REF_887(uint256)
 data.poolFeeShareBIPS = _settings.poolFeeShareBIPS
REF_888(uint256) -> data_6.poolFeeShareBIPS
REF_889(uint256) -> _settings_1.poolFeeShareBIPS
data_7(IAssetManagerEvents.AgentVaultCreationData) := phi(['data_6'])
REF_888(uint256) (->data_7) := REF_889(uint256)
 data.mintingVaultCollateralRatioBIPS = _settings.mintingVaultCollateralRatioBIPS
REF_890(uint256) -> data_7.mintingVaultCollateralRatioBIPS
REF_891(uint256) -> _settings_1.mintingVaultCollateralRatioBIPS
data_8(IAssetManagerEvents.AgentVaultCreationData) := phi(['data_7'])
REF_890(uint256) (->data_8) := REF_891(uint256)
 data.mintingPoolCollateralRatioBIPS = _settings.mintingPoolCollateralRatioBIPS
REF_892(uint256) -> data_8.mintingPoolCollateralRatioBIPS
REF_893(uint256) -> _settings_1.mintingPoolCollateralRatioBIPS
data_9(IAssetManagerEvents.AgentVaultCreationData) := phi(['data_8'])
REF_892(uint256) (->data_9) := REF_893(uint256)
 data.buyFAssetByAgentFactorBIPS = _settings.buyFAssetByAgentFactorBIPS
REF_894(uint256) -> data_9.buyFAssetByAgentFactorBIPS
REF_895(uint256) -> _settings_1.buyFAssetByAgentFactorBIPS
data_10(IAssetManagerEvents.AgentVaultCreationData) := phi(['data_9'])
REF_894(uint256) (->data_10) := REF_895(uint256)
 data.poolExitCollateralRatioBIPS = _settings.poolExitCollateralRatioBIPS
REF_896(uint256) -> data_10.poolExitCollateralRatioBIPS
REF_897(uint256) -> _settings_1.poolExitCollateralRatioBIPS
data_11(IAssetManagerEvents.AgentVaultCreationData) := phi(['data_10'])
REF_896(uint256) (->data_11) := REF_897(uint256)
 data.redemptionPoolFeeShareBIPS = _settings.redemptionPoolFeeShareBIPS
REF_898(uint256) -> data_11.redemptionPoolFeeShareBIPS
REF_899(uint256) -> _settings_1.redemptionPoolFeeShareBIPS
data_12(IAssetManagerEvents.AgentVaultCreationData) := phi(['data_11'])
REF_898(uint256) (->data_12) := REF_899(uint256)
 IAssetManagerEvents.AgentVaultCreated(_ownerManagementAddress,_agentVault,data)
Emit AgentVaultCreated(_ownerManagementAddress_1,_agentVault_1,data_12)
```
#### AgentVaultManagementFacet._getManagementAddress(address) [PRIVATE]
```slithir
_ownerAddress_1(address) := phi(['msg.sender'])
 ownerManagementAddress = Globals.getAgentOwnerRegistry().getManagementAddress(_ownerAddress)
TMP_1887(IAgentOwnerRegistry) = LIBRARY_CALL, dest:Globals, function:Globals.getAgentOwnerRegistry(), arguments:[] 
TMP_1888(address) = HIGH_LEVEL_CALL, dest:TMP_1887(IAgentOwnerRegistry), function:getManagementAddress, arguments:['_ownerAddress_1']  
ownerManagementAddress_1(address) := TMP_1888(address)
 ownerManagementAddress != address(0)
TMP_1889 = CONVERT 0 to address
TMP_1890(bool) = ownerManagementAddress_1 != TMP_1889
CONDITION TMP_1890
 ownerManagementAddress
RETURN ownerManagementAddress_1
 _ownerAddress
RETURN _ownerAddress_1
```
#### AgentVaultManagementFacet._reserveAndValidatePoolTokenSuffix(string) [PRIVATE]
```slithir
_suffix_1(string) := phi(['REF_749'])
MIN_SUFFIX_LEN_1(uint256) := phi(['MIN_SUFFIX_LEN_0'])
MAX_SUFFIX_LEN_1(uint256) := phi(['MAX_SUFFIX_LEN_0'])
 state = AssetManagerState.get()
TMP_1851(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_1851'])(AssetManagerState.State) := TMP_1851(AssetManagerState.State)
 require(bool,error)(! state.reservedPoolTokenSuffixes[_suffix],revert SuffixReserved()())
REF_872(mapping(string => bool)) -> state_1 (-> ['TMP_1851']).reservedPoolTokenSuffixes
REF_873(bool) -> REF_872[_suffix_1]
TMP_1852 = UnaryType.BANG REF_873 
TMP_1853(None) = SOLIDITY_CALL revert SuffixReserved()()
TMP_1854(None) = SOLIDITY_CALL require(bool,error)(TMP_1852,TMP_1853)
 state.reservedPoolTokenSuffixes[_suffix] = true
REF_874(mapping(string => bool)) -> state_1 (-> ['TMP_1851']).reservedPoolTokenSuffixes
REF_875(bool) -> REF_874[_suffix_1]
state_2 (-> ['TMP_1851'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_1851'])"])
REF_875(bool) (->state_2 (-> ['TMP_1851'])) := True(bool)
TMP_1851(AssetManagerState.State) := phi(["state_2 (-> ['TMP_1851'])"])
 suffixb = bytes(_suffix)
TMP_1855 = CONVERT _suffix_1 to bytes
suffixb_1(bytes) := TMP_1855(bytes)
 len = suffixb.length
REF_876 -> LENGTH suffixb_1
len_1(uint256) := REF_876(uint256)
 require(bool,error)(len >= MIN_SUFFIX_LEN,revert SuffixInvalidFormat()())
TMP_1856(bool) = len_1 >= MIN_SUFFIX_LEN_1
TMP_1857(None) = SOLIDITY_CALL revert SuffixInvalidFormat()()
TMP_1858(None) = SOLIDITY_CALL require(bool,error)(TMP_1856,TMP_1857)
 require(bool,error)(len <= MAX_SUFFIX_LEN,revert SuffixInvalidFormat()())
TMP_1859(bool) = len_1 <= MAX_SUFFIX_LEN_1
TMP_1860(None) = SOLIDITY_CALL revert SuffixInvalidFormat()()
TMP_1861(None) = SOLIDITY_CALL require(bool,error)(TMP_1859,TMP_1860)
 i = 0
i_1(uint256) := 0(uint256)
 i < len
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_1862(bool) = i_2 < len_1
CONDITION TMP_1862
 ch = suffixb[i]
REF_877(None) -> suffixb_1[i_2]
ch_1(bytes1) := REF_877(None)
 require(bool,error)((ch >= A && ch <= Z) || (ch >= 0 && ch <= 9) || (i > 0 && i < len - 1 && ch == -),revert SuffixInvalidFormat()())
TMP_1863(bool) = ch_1 >= A
TMP_1864(bool) = ch_1 <= Z
TMP_1865(bool) = TMP_1863 && TMP_1864
TMP_1866(bool) = ch_1 >= 0
TMP_1867(bool) = ch_1 <= 9
TMP_1868(bool) = TMP_1866 && TMP_1867
TMP_1869(bool) = TMP_1865 || TMP_1868
TMP_1870(bool) = i_2 > 0
TMP_1871(uint256) = len_1 (c)- 1
TMP_1872(bool) = i_2 < TMP_1871
TMP_1873(bool) = TMP_1870 && TMP_1872
TMP_1874(bool) = ch_1 == -
TMP_1875(bool) = TMP_1873 && TMP_1874
TMP_1876(bool) = TMP_1869 || TMP_1875
TMP_1877(None) = SOLIDITY_CALL revert SuffixInvalidFormat()()
TMP_1878(None) = SOLIDITY_CALL require(bool,error)(TMP_1876,TMP_1877)
 i ++
TMP_1879(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
```
#### AgentVaultManagementFacet._upgradeAgentVaultAndPool(address) [PRIVATE]
```slithir
_agentVault_1(address) := phi(['_agentVault_1', 'REF_849'])
 settings = Globals.getSettings()
TMP_1826(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_1826'])(AssetManagerSettings.Data) := TMP_1826(AssetManagerSettings.Data)
 collateralPool = Agent.get(_agentVault).collateralPool
TMP_1827(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
REF_852(IICollateralPool) -> TMP_1827.collateralPool
collateralPool_1(ICollateralPool) := REF_852(IICollateralPool)
 collateralPoolToken = collateralPool.poolToken()
TMP_1828(ICollateralPoolToken) = HIGH_LEVEL_CALL, dest:collateralPool_1(ICollateralPool), function:poolToken, arguments:[]  
collateralPoolToken_1(ICollateralPoolToken) := TMP_1828(ICollateralPoolToken)
 _upgradeContract(IIAgentVaultFactory(settings.agentVaultFactory),_agentVault)
REF_854(address) -> settings_1 (-> ['TMP_1826']).agentVaultFactory
TMP_1829 = CONVERT REF_854 to IIAgentVaultFactory
INTERNAL_CALL, AgentVaultManagementFacet._upgradeContract(IUpgradableContractFactory,address)(TMP_1829,_agentVault_1)
 _upgradeContract(IICollateralPoolFactory(settings.collateralPoolFactory),address(collateralPool))
REF_855(address) -> settings_1 (-> ['TMP_1826']).collateralPoolFactory
TMP_1831 = CONVERT REF_855 to IICollateralPoolFactory
TMP_1832 = CONVERT collateralPool_1 to address
INTERNAL_CALL, AgentVaultManagementFacet._upgradeContract(IUpgradableContractFactory,address)(TMP_1831,TMP_1832)
 _upgradeContract(IICollateralPoolTokenFactory(settings.collateralPoolTokenFactory),address(collateralPoolToken))
REF_856(address) -> settings_1 (-> ['TMP_1826']).collateralPoolTokenFactory
TMP_1834 = CONVERT REF_856 to IICollateralPoolTokenFactory
TMP_1835 = CONVERT collateralPoolToken_1 to address
INTERNAL_CALL, AgentVaultManagementFacet._upgradeContract(IUpgradableContractFactory,address)(TMP_1834,TMP_1835)
```
#### AgentVaultManagementFacet._upgradeContract(IUpgradableContractFactory,address) [PRIVATE]
```slithir
_factory_1(IUpgradableContractFactory) := phi(['TMP_1834', 'TMP_1829', 'TMP_1831'])
_proxyAddress_1(address) := phi(['_agentVault_1', 'TMP_1835', 'TMP_1832'])
 proxy = IUpgradableProxy(_proxyAddress)
TMP_1837 = CONVERT _proxyAddress_1 to IUpgradableProxy
proxy_1(IUpgradableProxy) := TMP_1837(IUpgradableProxy)
 newImplementation = _factory.implementation()
TMP_1838(address) = HIGH_LEVEL_CALL, dest:_factory_1(IUpgradableContractFactory), function:implementation, arguments:[]  
newImplementation_1(address) := TMP_1838(address)
 currentImplementation = proxy.implementation()
TMP_1839(address) = HIGH_LEVEL_CALL, dest:proxy_1(IUpgradableProxy), function:implementation, arguments:[]  
currentImplementation_1(address) := TMP_1839(address)
 currentImplementation != newImplementation
TMP_1840(bool) = currentImplementation_1 != newImplementation_1
CONDITION TMP_1840
 initCall = _factory.upgradeInitCall(_proxyAddress)
TMP_1841(bytes) = HIGH_LEVEL_CALL, dest:_factory_1(IUpgradableContractFactory), function:upgradeInitCall, arguments:['_proxyAddress_1']  
initCall_1(bytes) := TMP_1841(bytes)
 initCall.length > 0
REF_860 -> LENGTH initCall_1
TMP_1842(bool) = REF_860 > 0
CONDITION TMP_1842
 proxy.upgradeToAndCall(newImplementation,initCall)
HIGH_LEVEL_CALL, dest:proxy_1(IUpgradableProxy), function:upgradeToAndCall, arguments:['newImplementation_1', 'initCall_1']  
 proxy.upgradeTo(newImplementation)
HIGH_LEVEL_CALL, dest:proxy_1(IUpgradableProxy), function:upgradeTo, arguments:['newImplementation_1']
```
#### AgentVaultManagementFacet.announceDestroyAgent(address) [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_1783(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_1783'])(AssetManagerSettings.Data) := TMP_1783(AssetManagerSettings.Data)
 agent = Agent.get(_agentVault)
TMP_1784(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_1784'])(Agent.State) := TMP_1784(Agent.State)
 require(bool,error)(agent.availableAgentsPos == 0,revert AgentStillAvailable()())
REF_806(uint32) -> agent_1 (-> ['TMP_1784']).availableAgentsPos
TMP_1785(bool) = REF_806 == 0
TMP_1786(None) = SOLIDITY_CALL revert AgentStillAvailable()()
TMP_1787(None) = SOLIDITY_CALL require(bool,error)(TMP_1785,TMP_1786)
 require(bool,error)(agent.totalBackedAMG() == 0,revert AgentStillActive()())
TMP_1788(uint64) = LIBRARY_CALL, dest:Agents, function:Agents.totalBackedAMG(Agent.State), arguments:["agent_1 (-> ['TMP_1784'])"] 
TMP_1789(bool) = TMP_1788 == 0
TMP_1790(None) = SOLIDITY_CALL revert AgentStillActive()()
TMP_1791(None) = SOLIDITY_CALL require(bool,error)(TMP_1789,TMP_1790)
 agent.status != Agent.Status.DESTROYING
REF_808(Agent.Status) -> agent_1 (-> ['TMP_1784']).status
REF_809(Agent.Status) -> Status.DESTROYING
TMP_1792(bool) = REF_808 != REF_809
CONDITION TMP_1792
 agent.status = Agent.Status.DESTROYING
REF_810(Agent.Status) -> agent_1 (-> ['TMP_1784']).status
REF_811(Agent.Status) -> Status.DESTROYING
agent_2 (-> ['TMP_1784'])(Agent.State) := phi(["agent_1 (-> ['TMP_1784'])"])
REF_810(Agent.Status) (->agent_2 (-> ['TMP_1784'])) := REF_811(Agent.Status)
TMP_1784(Agent.State) := phi(["agent_2 (-> ['TMP_1784'])"])
 destroyAllowedAt = block.timestamp + settings.withdrawalWaitMinSeconds
REF_812(uint64) -> settings_1 (-> ['TMP_1783']).withdrawalWaitMinSeconds
TMP_1793(uint256) = block.timestamp (c)+ REF_812
destroyAllowedAt_1(uint256) := TMP_1793(uint256)
 agent.destroyAllowedAt = destroyAllowedAt.toUint64()
REF_813(uint64) -> agent_2 (-> ['TMP_1784']).destroyAllowedAt
TMP_1794(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['destroyAllowedAt_1'] 
agent_3 (-> ['TMP_1784'])(Agent.State) := phi(["agent_2 (-> ['TMP_1784'])"])
REF_813(uint64) (->agent_3 (-> ['TMP_1784'])) := TMP_1794(uint64)
TMP_1784(Agent.State) := phi(["agent_3 (-> ['TMP_1784'])"])
 IAssetManagerEvents.AgentDestroyAnnounced(_agentVault,destroyAllowedAt)
Emit AgentDestroyAnnounced(_agentVault_1,destroyAllowedAt_1)
agent_4 (-> ['TMP_1784'])(Agent.State) := phi(["agent_3 (-> ['TMP_1784'])", "agent_1 (-> ['TMP_1784'])"])
 agent.destroyAllowedAt
REF_816(uint64) -> agent_4 (-> ['TMP_1784']).destroyAllowedAt
RETURN REF_816
 onlyAgentVaultOwner(_agentVault)
MODIFIER_CALL, AssetManagerBase.onlyAgentVaultOwner(address)(_agentVault_1)
 _destroyAllowedAt
```
#### AgentVaultManagementFacet.createAgentVault(IAddressValidity.Proof,AgentSettings.Data) [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_1742(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_1742'])(AssetManagerState.State) := TMP_1742(AssetManagerState.State)
 _reserveAndValidatePoolTokenSuffix(_settings.poolTokenSuffix)
REF_749(string) -> _settings_1.poolTokenSuffix
INTERNAL_CALL, AgentVaultManagementFacet._reserveAndValidatePoolTokenSuffix(string)(REF_749)
 ownerManagementAddress = _getManagementAddress(msg.sender)
TMP_1744(address) = INTERNAL_CALL, AgentVaultManagementFacet._getManagementAddress(address)(msg.sender)
ownerManagementAddress_1(address) := TMP_1744(address)
 Agents.requireWhitelisted(ownerManagementAddress)
LIBRARY_CALL, dest:Agents, function:Agents.requireWhitelisted(address), arguments:['ownerManagementAddress_1'] 
 TransactionAttestation.verifyAddressValidity(_addressProof)
LIBRARY_CALL, dest:TransactionAttestation, function:TransactionAttestation.verifyAddressValidity(IAddressValidity.Proof), arguments:['_addressProof_1'] 
 avb = _addressProof.data.responseBody
REF_752(IAddressValidity.Response) -> _addressProof_1.data
REF_753(IAddressValidity.ResponseBody) -> REF_752.responseBody
avb_1(IAddressValidity.ResponseBody) := REF_753(IAddressValidity.ResponseBody)
 require(bool,error)(avb.isValid,revert AddressInvalid()())
REF_754(bool) -> avb_1.isValid
TMP_1747(None) = SOLIDITY_CALL revert AddressInvalid()()
TMP_1748(None) = SOLIDITY_CALL require(bool,error)(REF_754,TMP_1747)
 require(bool,error)(avb.standardAddressHash != CoreVaultClient.coreVaultUnderlyingAddressHash(),revert AddressUsedByCoreVault()())
REF_755(bytes32) -> avb_1.standardAddressHash
TMP_1749(bytes32) = LIBRARY_CALL, dest:CoreVaultClient, function:CoreVaultClient.coreVaultUnderlyingAddressHash(), arguments:[] 
TMP_1750(bool) = REF_755 != TMP_1749
TMP_1751(None) = SOLIDITY_CALL revert AddressUsedByCoreVault()()
TMP_1752(None) = SOLIDITY_CALL require(bool,error)(TMP_1750,TMP_1751)
 assetManager = IIAssetManager(address(this))
TMP_1753 = CONVERT this to address
TMP_1754 = CONVERT TMP_1753 to IIAssetManager
assetManager_1(IIAssetManager) := TMP_1754(IIAssetManager)
 agentVaultFactory = IIAgentVaultFactory(Globals.getSettings().agentVaultFactory)
TMP_1755(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
REF_758(address) -> TMP_1755.agentVaultFactory
TMP_1756 = CONVERT REF_758 to IIAgentVaultFactory
agentVaultFactory_1(IIAgentVaultFactory) := TMP_1756(IIAgentVaultFactory)
 agentVault = agentVaultFactory.create(assetManager)
TMP_1757(IIAgentVault) = HIGH_LEVEL_CALL, dest:agentVaultFactory_1(IIAgentVaultFactory), function:create, arguments:['assetManager_1']  
agentVault_1(IIAgentVault) := TMP_1757(IIAgentVault)
 agent = Agent.getWithoutCheck(address(agentVault))
TMP_1758 = CONVERT agentVault_1 to address
TMP_1759(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.getWithoutCheck(address), arguments:['TMP_1758'] 
agent_1 (-> ['TMP_1759'])(Agent.State) := TMP_1759(Agent.State)
 assert(bool)(agent.status == Agent.Status.EMPTY)
REF_761(Agent.Status) -> agent_1 (-> ['TMP_1759']).status
REF_762(Agent.Status) -> Status.EMPTY
TMP_1760(bool) = REF_761 == REF_762
TMP_1761(None) = SOLIDITY_CALL assert(bool)(TMP_1760)
 agent.status = Agent.Status.NORMAL
REF_763(Agent.Status) -> agent_1 (-> ['TMP_1759']).status
REF_764(Agent.Status) -> Status.NORMAL
agent_2 (-> ['TMP_1759'])(Agent.State) := phi(["agent_1 (-> ['TMP_1759'])"])
REF_763(Agent.Status) (->agent_2 (-> ['TMP_1759'])) := REF_764(Agent.Status)
TMP_1759(Agent.State) := phi(["agent_2 (-> ['TMP_1759'])"])
 agent.ownerManagementAddress = ownerManagementAddress
REF_765(address) -> agent_2 (-> ['TMP_1759']).ownerManagementAddress
agent_3 (-> ['TMP_1759'])(Agent.State) := phi(["agent_2 (-> ['TMP_1759'])"])
REF_765(address) (->agent_3 (-> ['TMP_1759'])) := ownerManagementAddress_1(address)
TMP_1759(Agent.State) := phi(["agent_3 (-> ['TMP_1759'])"])
 agent.setVaultCollateral(_settings.vaultCollateralToken)
REF_767(IERC20) -> _settings_1.vaultCollateralToken
LIBRARY_CALL, dest:AgentUpdates, function:AgentUpdates.setVaultCollateral(Agent.State,IERC20), arguments:["agent_3 (-> ['TMP_1759'])", 'REF_767'] 
 agent.poolCollateralIndex = state.poolCollateralIndex
REF_768(uint16) -> agent_3 (-> ['TMP_1759']).poolCollateralIndex
REF_769(uint16) -> state_1 (-> ['TMP_1742']).poolCollateralIndex
agent_4 (-> ['TMP_1759'])(Agent.State) := phi(["agent_3 (-> ['TMP_1759'])"])
REF_768(uint16) (->agent_4 (-> ['TMP_1759'])) := REF_769(uint16)
TMP_1759(Agent.State) := phi(["agent_4 (-> ['TMP_1759'])"])
 agent.setMintingVaultCollateralRatioBIPS(_settings.mintingVaultCollateralRatioBIPS)
REF_771(uint256) -> _settings_1.mintingVaultCollateralRatioBIPS
LIBRARY_CALL, dest:AgentUpdates, function:AgentUpdates.setMintingVaultCollateralRatioBIPS(Agent.State,uint256), arguments:["agent_4 (-> ['TMP_1759'])", 'REF_771'] 
 agent.setMintingPoolCollateralRatioBIPS(_settings.mintingPoolCollateralRatioBIPS)
REF_773(uint256) -> _settings_1.mintingPoolCollateralRatioBIPS
LIBRARY_CALL, dest:AgentUpdates, function:AgentUpdates.setMintingPoolCollateralRatioBIPS(Agent.State,uint256), arguments:["agent_4 (-> ['TMP_1759'])", 'REF_773'] 
 agent.setFeeBIPS(_settings.feeBIPS)
REF_775(uint256) -> _settings_1.feeBIPS
LIBRARY_CALL, dest:AgentUpdates, function:AgentUpdates.setFeeBIPS(Agent.State,uint256), arguments:["agent_4 (-> ['TMP_1759'])", 'REF_775'] 
 agent.setPoolFeeShareBIPS(_settings.poolFeeShareBIPS)
REF_777(uint256) -> _settings_1.poolFeeShareBIPS
LIBRARY_CALL, dest:AgentUpdates, function:AgentUpdates.setPoolFeeShareBIPS(Agent.State,uint256), arguments:["agent_4 (-> ['TMP_1759'])", 'REF_777'] 
 agent.setBuyFAssetByAgentFactorBIPS(_settings.buyFAssetByAgentFactorBIPS)
REF_779(uint256) -> _settings_1.buyFAssetByAgentFactorBIPS
LIBRARY_CALL, dest:AgentUpdates, function:AgentUpdates.setBuyFAssetByAgentFactorBIPS(Agent.State,uint256), arguments:["agent_4 (-> ['TMP_1759'])", 'REF_779'] 
 state.underlyingAddressOwnership.claimAndTransfer(address(agentVault),avb.standardAddressHash)
REF_780(UnderlyingAddressOwnership.State) -> state_1 (-> ['TMP_1742']).underlyingAddressOwnership
TMP_1768 = CONVERT agentVault_1 to address
REF_782(bytes32) -> avb_1.standardAddressHash
LIBRARY_CALL, dest:UnderlyingAddressOwnership, function:UnderlyingAddressOwnership.claimAndTransfer(UnderlyingAddressOwnership.State,address,bytes32), arguments:['REF_780', 'TMP_1768', 'REF_782'] 
 agent.underlyingAddressString = avb.standardAddress
REF_783(string) -> agent_4 (-> ['TMP_1759']).underlyingAddressString
REF_784(string) -> avb_1.standardAddress
agent_5 (-> ['TMP_1759'])(Agent.State) := phi(["agent_4 (-> ['TMP_1759'])"])
REF_783(string) (->agent_5 (-> ['TMP_1759'])) := REF_784(string)
TMP_1759(Agent.State) := phi(["agent_5 (-> ['TMP_1759'])"])
 agent.underlyingAddressHash = avb.standardAddressHash
REF_785(bytes32) -> agent_5 (-> ['TMP_1759']).underlyingAddressHash
REF_786(bytes32) -> avb_1.standardAddressHash
agent_6 (-> ['TMP_1759'])(Agent.State) := phi(["agent_5 (-> ['TMP_1759'])"])
REF_785(bytes32) (->agent_6 (-> ['TMP_1759'])) := REF_786(bytes32)
TMP_1759(Agent.State) := phi(["agent_6 (-> ['TMP_1759'])"])
 agent.underlyingBlockAtCreation = state.currentUnderlyingBlock
REF_787(uint64) -> agent_6 (-> ['TMP_1759']).underlyingBlockAtCreation
REF_788(uint64) -> state_1 (-> ['TMP_1742']).currentUnderlyingBlock
agent_7 (-> ['TMP_1759'])(Agent.State) := phi(["agent_6 (-> ['TMP_1759'])"])
REF_787(uint64) (->agent_7 (-> ['TMP_1759'])) := REF_788(uint64)
TMP_1759(Agent.State) := phi(["agent_7 (-> ['TMP_1759'])"])
 agent.collateralPool = _createCollateralPool(assetManager,address(agentVault),_settings)
REF_789(IICollateralPool) -> agent_7 (-> ['TMP_1759']).collateralPool
TMP_1770 = CONVERT agentVault_1 to address
TMP_1771(IICollateralPool) = INTERNAL_CALL, AgentVaultManagementFacet._createCollateralPool(IIAssetManager,address,AgentSettings.Data)(assetManager_1,TMP_1770,_settings_1)
agent_8 (-> ['TMP_1759'])(Agent.State) := phi(["agent_7 (-> ['TMP_1759'])"])
REF_789(IICollateralPool) (->agent_8 (-> ['TMP_1759'])) := TMP_1771(IICollateralPool)
TMP_1759(Agent.State) := phi(["agent_8 (-> ['TMP_1759'])"])
 agent.setPoolExitCollateralRatioBIPS(_settings.poolExitCollateralRatioBIPS)
REF_791(uint256) -> _settings_1.poolExitCollateralRatioBIPS
LIBRARY_CALL, dest:AgentUpdates, function:AgentUpdates.setPoolExitCollateralRatioBIPS(Agent.State,uint256), arguments:["agent_8 (-> ['TMP_1759'])", 'REF_791'] 
 agent.setRedemptionPoolFeeShareBIPS(_settings.redemptionPoolFeeShareBIPS)
REF_793(uint256) -> _settings_1.redemptionPoolFeeShareBIPS
LIBRARY_CALL, dest:AgentUpdates, function:AgentUpdates.setRedemptionPoolFeeShareBIPS(Agent.State,uint256), arguments:["agent_8 (-> ['TMP_1759'])", 'REF_793'] 
 agent.allAgentsPos = state.allAgents.length.toUint32()
REF_794(uint32) -> agent_8 (-> ['TMP_1759']).allAgentsPos
REF_795(address[]) -> state_1 (-> ['TMP_1742']).allAgents
REF_796 -> LENGTH REF_795
TMP_1774(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['REF_796'] 
agent_9 (-> ['TMP_1759'])(Agent.State) := phi(["agent_8 (-> ['TMP_1759'])"])
REF_794(uint32) (->agent_9 (-> ['TMP_1759'])) := TMP_1774(uint32)
TMP_1759(Agent.State) := phi(["agent_9 (-> ['TMP_1759'])"])
 state.allAgents.push(address(agentVault))
REF_798(address[]) -> state_1 (-> ['TMP_1742']).allAgents
TMP_1775 = CONVERT agentVault_1 to address
REF_800 -> LENGTH REF_798
TMP_1777(uint256) := REF_800(uint256)
TMP_1778(uint256) = TMP_1777 (c)+ 1
state_2 (-> ['TMP_1742'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_1742'])"])
REF_800(uint256) (->state_3 (-> ['TMP_1742'])) := TMP_1778(uint256)
REF_801(address) -> REF_798[TMP_1777]
state_3 (-> ['TMP_1742'])(AssetManagerState.State) := phi(["state_2 (-> ['TMP_1742'])"])
REF_801(address) (->state_3 (-> ['TMP_1742'])) := TMP_1775(address)
TMP_1742(AssetManagerState.State) := phi(["state_3 (-> ['TMP_1742'])"])
 _emitAgentVaultCreated(ownerManagementAddress,address(agentVault),agent.collateralPool,avb.standardAddress,_settings)
TMP_1779 = CONVERT agentVault_1 to address
REF_802(IICollateralPool) -> agent_9 (-> ['TMP_1759']).collateralPool
REF_803(string) -> avb_1.standardAddress
INTERNAL_CALL, AgentVaultManagementFacet._emitAgentVaultCreated(address,address,IICollateralPool,string,AgentSettings.Data)(ownerManagementAddress_1,TMP_1779,REF_802,REF_803,_settings_1)
 address(agentVault)
TMP_1781 = CONVERT agentVault_1 to address
RETURN TMP_1781
 onlyAttached()
MODIFIER_CALL, AssetManagerBase.onlyAttached()()
 _agentVault
```
#### AgentVaultManagementFacet.destroyAgent(address,address) [EXTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_1797(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_1797'])(AssetManagerState.State) := TMP_1797(AssetManagerState.State)
 agent = Agent.get(_agentVault)
TMP_1798(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_1798'])(Agent.State) := TMP_1798(Agent.State)
 require(bool,error)(agent.status == Agent.Status.DESTROYING,revert DestroyNotAnnounced()())
REF_819(Agent.Status) -> agent_1 (-> ['TMP_1798']).status
REF_820(Agent.Status) -> Status.DESTROYING
TMP_1799(bool) = REF_819 == REF_820
TMP_1800(None) = SOLIDITY_CALL revert DestroyNotAnnounced()()
TMP_1801(None) = SOLIDITY_CALL require(bool,error)(TMP_1799,TMP_1800)
 require(bool,error)(block.timestamp > agent.destroyAllowedAt,revert DestroyNotAllowedYet()())
REF_821(uint64) -> agent_1 (-> ['TMP_1798']).destroyAllowedAt
TMP_1802(bool) = block.timestamp > REF_821
TMP_1803(None) = SOLIDITY_CALL revert DestroyNotAllowedYet()()
TMP_1804(None) = SOLIDITY_CALL require(bool,error)(TMP_1802,TMP_1803)
 assert(bool)(agent.totalBackedAMG() == 0)
TMP_1805(uint64) = LIBRARY_CALL, dest:Agents, function:Agents.totalBackedAMG(Agent.State), arguments:["agent_1 (-> ['TMP_1798'])"] 
TMP_1806(bool) = TMP_1805 == 0
TMP_1807(None) = SOLIDITY_CALL assert(bool)(TMP_1806)
 agent.collateralPool.destroy(_recipient)
REF_823(IICollateralPool) -> agent_1 (-> ['TMP_1798']).collateralPool
HIGH_LEVEL_CALL, dest:REF_823(IICollateralPool), function:destroy, arguments:['_recipient_1']  
 IIAgentVault(_agentVault).destroy()
TMP_1809 = CONVERT _agentVault_1 to IIAgentVault
HIGH_LEVEL_CALL, dest:TMP_1809(IIAgentVault), function:destroy, arguments:[]  
 ind = agent.allAgentsPos
REF_826(uint32) -> agent_1 (-> ['TMP_1798']).allAgentsPos
ind_1(uint256) := REF_826(uint32)
 ind + 1 < state.allAgents.length
TMP_1811(uint256) = ind_1 (c)+ 1
REF_827(address[]) -> state_1 (-> ['TMP_1797']).allAgents
REF_828 -> LENGTH REF_827
TMP_1812(bool) = TMP_1811 < REF_828
CONDITION TMP_1812
 state.allAgents[ind] = state.allAgents[state.allAgents.length - 1]
REF_829(address[]) -> state_1 (-> ['TMP_1797']).allAgents
REF_830(address) -> REF_829[ind_1]
REF_831(address[]) -> state_1 (-> ['TMP_1797']).allAgents
REF_832(address[]) -> state_1 (-> ['TMP_1797']).allAgents
REF_833 -> LENGTH REF_832
TMP_1813(uint256) = REF_833 (c)- 1
REF_834(address) -> REF_831[TMP_1813]
state_2 (-> ['TMP_1797'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_1797'])"])
REF_830(address) (->state_2 (-> ['TMP_1797'])) := REF_834(address)
TMP_1797(AssetManagerState.State) := phi(["state_2 (-> ['TMP_1797'])"])
 movedAgent = Agent.get(state.allAgents[ind])
REF_836(address[]) -> state_2 (-> ['TMP_1797']).allAgents
REF_837(address) -> REF_836[ind_1]
TMP_1814(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['REF_837'] 
movedAgent_1 (-> ['TMP_1814'])(Agent.State) := TMP_1814(Agent.State)
 movedAgent.allAgentsPos = uint32(ind)
REF_838(uint32) -> movedAgent_1 (-> ['TMP_1814']).allAgentsPos
TMP_1815 = CONVERT ind_1 to uint32
movedAgent_2 (-> ['TMP_1814'])(Agent.State) := phi(["movedAgent_1 (-> ['TMP_1814'])"])
REF_838(uint32) (->movedAgent_2 (-> ['TMP_1814'])) := TMP_1815(uint32)
TMP_1814(Agent.State) := phi(["movedAgent_2 (-> ['TMP_1814'])"])
state_3 (-> ['TMP_1797'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_1797'])", "state_2 (-> ['TMP_1797'])"])
 state.allAgents.pop()
REF_839(address[]) -> state_3 (-> ['TMP_1797']).allAgents
REF_841 -> LENGTH REF_839
TMP_1817(uint256) = REF_841 (c)- 1
REF_842(address) -> REF_839[TMP_1817]
REF_839 = delete REF_842 
REF_843 -> LENGTH REF_839
state_4 (-> ['TMP_1797'])(AssetManagerState.State) := phi(["state_3 (-> ['TMP_1797'])"])
REF_843(uint256) (->state_4 (-> ['TMP_1797'])) := TMP_1817(uint256)
TMP_1797(AssetManagerState.State) := phi(["state_4 (-> ['TMP_1797'])"])
 agent.status = Agent.Status.DESTROYED
REF_844(Agent.Status) -> agent_1 (-> ['TMP_1798']).status
REF_845(Agent.Status) -> Status.DESTROYED
agent_2 (-> ['TMP_1798'])(Agent.State) := phi(["agent_1 (-> ['TMP_1798'])"])
REF_844(Agent.Status) (->agent_2 (-> ['TMP_1798'])) := REF_845(Agent.Status)
TMP_1798(Agent.State) := phi(["agent_2 (-> ['TMP_1798'])"])
 IAssetManagerEvents.AgentDestroyed(_agentVault)
Emit AgentDestroyed(_agentVault_1)
 onlyAgentVaultOwner(_agentVault)
MODIFIER_CALL, AssetManagerBase.onlyAgentVaultOwner(address)(_agentVault_1)
```

#### AgentVaultManagementFacet.upgradeAgentVaultAndPool(address) [EXTERNAL]
```slithir
 _upgradeAgentVaultAndPool(_agentVault)
INTERNAL_CALL, AgentVaultManagementFacet._upgradeAgentVaultAndPool(address)(_agentVault_1)
 onlyAgentVaultOwner(_agentVault)
MODIFIER_CALL, AssetManagerBase.onlyAgentVaultOwner(address)(_agentVault_1)
```
#### AgentVaultManagementFacet.upgradeAgentVaultsAndPools(uint256,uint256) [EXTERNAL]
```slithir
 (_agents,None) = Agents.getAllAgents(_start,_end)
TUPLE_19(address[],uint256) = LIBRARY_CALL, dest:Agents, function:Agents.getAllAgents(uint256,uint256), arguments:['_start_1', '_end_1'] 
_agents_1(address[])= UNPACK TUPLE_19 index: 0 
 i = 0
i_1(uint256) := 0(uint256)
 i < _agents.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_848 -> LENGTH _agents_1
TMP_1822(bool) = i_2 < REF_848
CONDITION TMP_1822
 _upgradeAgentVaultAndPool(_agents[i])
REF_849(address) -> _agents_1[i_2]
INTERNAL_CALL, AgentVaultManagementFacet._upgradeAgentVaultAndPool(address)(REF_849)
 i ++
TMP_1824(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 onlyAssetManagerController()
MODIFIER_CALL, AssetManagerBase.onlyAssetManagerController()()
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
#### IICollateralPool.setPoolToken(address) [EXTERNAL]
```slithir

```


#### IICollateralPool.wNat() [EXTERNAL]
```slithir

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
#### AgentOwnerRegistry.getManagementAddress(address) [EXTERNAL]
```slithir
workToMgmtAddress_5(mapping(address => address)) := phi(['workToMgmtAddress_5', 'workToMgmtAddress_2', 'workToMgmtAddress_3', 'workToMgmtAddress_0', 'workToMgmtAddress_4'])
 workToMgmtAddress[_workAddress]
REF_312(address) -> workToMgmtAddress_5[_workAddress_1]
RETURN REF_312
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
#### CollateralPool.poolToken() [EXTERNAL]
```slithir
token_4(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_0', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
 token
RETURN token_4
```
#### IUpgradableContractFactory.implementation() [EXTERNAL]
```slithir

```

#### IUpgradableProxy.implementation() [EXTERNAL]
```slithir

```
#### IUpgradableProxy.upgradeTo(address) [EXTERNAL]
```slithir

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
#### AgentUpdates.setBuyFAssetByAgentFactorBIPS(Agent.State,uint256) [INTERNAL]
```slithir
 require(bool,error)(_buyFAssetByAgentFactorBIPS <= SafePct.MAX_BIPS,revert ValueTooHigh()())
REF_2961(uint256) -> SafePct.MAX_BIPS
TMP_4436(bool) = _buyFAssetByAgentFactorBIPS_1 <= REF_2961
TMP_4437(None) = SOLIDITY_CALL revert ValueTooHigh()()
TMP_4438(None) = SOLIDITY_CALL require(bool,error)(TMP_4436,TMP_4437)
 require(bool,error)(_buyFAssetByAgentFactorBIPS >= 9000,revert ValueTooLow()())
TMP_4439(bool) = _buyFAssetByAgentFactorBIPS_1 >= 9000
TMP_4440(None) = SOLIDITY_CALL revert ValueTooLow()()
TMP_4441(None) = SOLIDITY_CALL require(bool,error)(TMP_4439,TMP_4440)
 _agent.buyFAssetByAgentFactorBIPS = _buyFAssetByAgentFactorBIPS.toUint16()
REF_2962(uint16) -> _agent_1 (-> []).buyFAssetByAgentFactorBIPS
TMP_4442(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['_buyFAssetByAgentFactorBIPS_1'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2962(uint16) (->_agent_2 (-> [])) := TMP_4442(uint16)
```
#### AgentUpdates.setFeeBIPS(Agent.State,uint256) [INTERNAL]
```slithir
 require(bool,error)(_feeBIPS <= SafePct.MAX_BIPS,revert FeeTooHigh()())
REF_2952(uint256) -> SafePct.MAX_BIPS
TMP_4424(bool) = _feeBIPS_1 <= REF_2952
TMP_4425(None) = SOLIDITY_CALL revert FeeTooHigh()()
TMP_4426(None) = SOLIDITY_CALL require(bool,error)(TMP_4424,TMP_4425)
 _agent.feeBIPS = _feeBIPS.toUint16()
REF_2953(uint16) -> _agent_1 (-> []).feeBIPS
TMP_4427(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['_feeBIPS_1'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2953(uint16) (->_agent_2 (-> [])) := TMP_4427(uint16)
```
#### AgentUpdates.setMintingPoolCollateralRatioBIPS(Agent.State,uint256) [INTERNAL]
```slithir
 collateral = Agents.getPoolCollateral(_agent)
TMP_4419(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getPoolCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
collateral_1 (-> ['TMP_4419'])(CollateralTypeInt.Data) := TMP_4419(CollateralTypeInt.Data)
 require(bool,error)(_mintingPoolCollateralRatioBIPS >= collateral.minCollateralRatioBIPS,revert CollateralRatioTooSmall()())
REF_2949(uint32) -> collateral_1 (-> ['TMP_4419']).minCollateralRatioBIPS
TMP_4420(bool) = _mintingPoolCollateralRatioBIPS_1 >= REF_2949
TMP_4421(None) = SOLIDITY_CALL revert CollateralRatioTooSmall()()
TMP_4422(None) = SOLIDITY_CALL require(bool,error)(TMP_4420,TMP_4421)
 _agent.mintingPoolCollateralRatioBIPS = _mintingPoolCollateralRatioBIPS.toUint32()
REF_2950(uint32) -> _agent_1 (-> []).mintingPoolCollateralRatioBIPS
TMP_4423(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['_mintingPoolCollateralRatioBIPS_1'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2950(uint32) (->_agent_2 (-> [])) := TMP_4423(uint32)
```
#### AgentUpdates.setMintingVaultCollateralRatioBIPS(Agent.State,uint256) [INTERNAL]
```slithir
 collateral = Agents.getVaultCollateral(_agent)
TMP_4414(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getVaultCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
collateral_1 (-> ['TMP_4414'])(CollateralTypeInt.Data) := TMP_4414(CollateralTypeInt.Data)
 require(bool,error)(_mintingVaultCollateralRatioBIPS >= collateral.minCollateralRatioBIPS,revert CollateralRatioTooSmall()())
REF_2945(uint32) -> collateral_1 (-> ['TMP_4414']).minCollateralRatioBIPS
TMP_4415(bool) = _mintingVaultCollateralRatioBIPS_1 >= REF_2945
TMP_4416(None) = SOLIDITY_CALL revert CollateralRatioTooSmall()()
TMP_4417(None) = SOLIDITY_CALL require(bool,error)(TMP_4415,TMP_4416)
 _agent.mintingVaultCollateralRatioBIPS = _mintingVaultCollateralRatioBIPS.toUint32()
REF_2946(uint32) -> _agent_1 (-> []).mintingVaultCollateralRatioBIPS
TMP_4418(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['_mintingVaultCollateralRatioBIPS_1'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2946(uint32) (->_agent_2 (-> [])) := TMP_4418(uint32)
```

#### AgentUpdates.setPoolFeeShareBIPS(Agent.State,uint256) [INTERNAL]
```slithir
 require(bool,error)(_poolFeeShareBIPS <= SafePct.MAX_BIPS,revert ValueTooHigh()())
REF_2955(uint256) -> SafePct.MAX_BIPS
TMP_4428(bool) = _poolFeeShareBIPS_1 <= REF_2955
TMP_4429(None) = SOLIDITY_CALL revert ValueTooHigh()()
TMP_4430(None) = SOLIDITY_CALL require(bool,error)(TMP_4428,TMP_4429)
 _agent.poolFeeShareBIPS = _poolFeeShareBIPS.toUint16()
REF_2956(uint16) -> _agent_1 (-> []).poolFeeShareBIPS
TMP_4431(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['_poolFeeShareBIPS_1'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2956(uint16) (->_agent_2 (-> [])) := TMP_4431(uint16)
```
#### AgentUpdates.setRedemptionPoolFeeShareBIPS(Agent.State,uint256) [INTERNAL]
```slithir
 require(bool,error)(_redemptionPoolFeeShareBIPS <= SafePct.MAX_BIPS,revert ValueTooHigh()())
REF_2958(uint256) -> SafePct.MAX_BIPS
TMP_4432(bool) = _redemptionPoolFeeShareBIPS_1 <= REF_2958
TMP_4433(None) = SOLIDITY_CALL revert ValueTooHigh()()
TMP_4434(None) = SOLIDITY_CALL require(bool,error)(TMP_4432,TMP_4433)
 _agent.redemptionPoolFeeShareBIPS = _redemptionPoolFeeShareBIPS.toUint16()
REF_2959(uint16) -> _agent_1 (-> []).redemptionPoolFeeShareBIPS
TMP_4435(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['_redemptionPoolFeeShareBIPS_1'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2959(uint16) (->_agent_2 (-> [])) := TMP_4435(uint16)
```
#### AgentUpdates.setVaultCollateral(Agent.State,IERC20) [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4401(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4401'])(AssetManagerState.State) := TMP_4401(AssetManagerState.State)
 tokenIndex = CollateralTypes.getIndex(CollateralType.Class.VAULT,_token)
REF_2933(CollateralType.Class) -> Class.VAULT
TMP_4402(uint256) = LIBRARY_CALL, dest:CollateralTypes, function:CollateralTypes.getIndex(CollateralType.Class,IERC20), arguments:['REF_2933', '_token_1'] 
tokenIndex_1(uint256) := TMP_4402(uint256)
 collateral = state.collateralTokens[tokenIndex]
REF_2934(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4401']).collateralTokens
REF_2935(CollateralTypeInt.Data) -> REF_2934[tokenIndex_1]
collateral_1 (-> ['state'])(CollateralTypeInt.Data) := REF_2935(CollateralTypeInt.Data)
 assert(bool)(collateral.collateralClass == CollateralType.Class.VAULT)
REF_2936(CollateralType.Class) -> collateral_1 (-> ['state']).collateralClass
REF_2937(CollateralType.Class) -> Class.VAULT
TMP_4403(bool) = REF_2936 == REF_2937
TMP_4404(None) = SOLIDITY_CALL assert(bool)(TMP_4403)
 require(bool,error)(collateral.validUntil == 0,revert CollateralDeprecated()())
REF_2938(uint64) -> collateral_1 (-> ['state']).validUntil
TMP_4405(bool) = REF_2938 == 0
TMP_4406(None) = SOLIDITY_CALL revert CollateralDeprecated()()
TMP_4407(None) = SOLIDITY_CALL require(bool,error)(TMP_4405,TMP_4406)
 _agent.vaultCollateralIndex = tokenIndex.toUint16()
REF_2939(uint16) -> _agent_1 (-> []).vaultCollateralIndex
TMP_4408(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['tokenIndex_1'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2939(uint16) (->_agent_2 (-> [])) := TMP_4408(uint16)
 switchCollateralData = AgentCollateral.agentVaultCollateralData(_agent)
TMP_4409(Collateral.Data) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.agentVaultCollateralData(Agent.State), arguments:['_agent_2 (-> [])'] 
switchCollateralData_1(Collateral.Data) := TMP_4409(Collateral.Data)
 crBIPS = AgentCollateral.collateralRatioBIPS(switchCollateralData,_agent)
TMP_4410(uint256) = LIBRARY_CALL, dest:AgentCollateral, function:AgentCollateral.collateralRatioBIPS(Collateral.Data,Agent.State), arguments:['switchCollateralData_1', '_agent_2 (-> [])'] 
crBIPS_1(uint256) := TMP_4410(uint256)
 require(bool,error)(crBIPS >= collateral.minCollateralRatioBIPS,revert NotEnoughCollateral()())
REF_2943(uint32) -> collateral_1 (-> ['state']).minCollateralRatioBIPS
TMP_4411(bool) = crBIPS_1 >= REF_2943
TMP_4412(None) = SOLIDITY_CALL revert NotEnoughCollateral()()
TMP_4413(None) = SOLIDITY_CALL require(bool,error)(TMP_4411,TMP_4412)
```
#### Agents.requireWhitelisted(address) [INTERNAL]
```slithir
_ownerManagementAddress_1(address) := phi(['REF_2996'])
 require(bool,error)(Globals.getAgentOwnerRegistry().isWhitelisted(_ownerManagementAddress),revert AgentNotWhitelisted()())
TMP_4488(IAgentOwnerRegistry) = LIBRARY_CALL, dest:Globals, function:Globals.getAgentOwnerRegistry(), arguments:[] 
TMP_4489(bool) = HIGH_LEVEL_CALL, dest:TMP_4488(IAgentOwnerRegistry), function:isWhitelisted, arguments:['_ownerManagementAddress_1']  
TMP_4490(None) = SOLIDITY_CALL revert AgentNotWhitelisted()()
TMP_4491(None) = SOLIDITY_CALL require(bool,error)(TMP_4489,TMP_4490)
```
#### CoreVaultClient.coreVaultUnderlyingAddressHash() [INTERNAL]
```slithir
 state = getState()
TMP_4713(CoreVaultClient.State) = INTERNAL_CALL, CoreVaultClient.getState()()
state_1 (-> ['TMP_4713'])(CoreVaultClient.State) := TMP_4713(CoreVaultClient.State)
 address(state.coreVaultManager) == address(0)
REF_3212(IICoreVaultManager) -> state_1 (-> ['TMP_4713']).coreVaultManager
TMP_4714 = CONVERT REF_3212 to address
TMP_4715 = CONVERT 0 to address
TMP_4716(bool) = TMP_4714 == TMP_4715
CONDITION TMP_4716
 bytes32(0)
TMP_4717 = CONVERT 0 to bytes32
RETURN TMP_4717
 state.coreVaultManager.coreVaultAddressHash()
REF_3213(IICoreVaultManager) -> state_1 (-> ['TMP_4713']).coreVaultManager
TMP_4718(bytes32) = HIGH_LEVEL_CALL, dest:REF_3213(IICoreVaultManager), function:coreVaultAddressHash, arguments:[]  
RETURN TMP_4718
```
#### TransactionAttestation.verifyAddressValidity(IAddressValidity.Proof) [INTERNAL]
```slithir
 _settings = Globals.getSettings()
TMP_5269(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
_settings_1 (-> ['TMP_5269'])(AssetManagerSettings.Data) := TMP_5269(AssetManagerSettings.Data)
 fdcVerification = IFdcVerification(_settings.fdcVerification)
REF_3694(address) -> _settings_1 (-> ['TMP_5269']).fdcVerification
TMP_5270 = CONVERT REF_3694 to IFdcVerification
fdcVerification_1(IFdcVerification) := TMP_5270(IFdcVerification)
 require(bool,error)(_proof.data.sourceId == _settings.chainId,revert InvalidChain()())
REF_3695(IAddressValidity.Response) -> _proof_1.data
REF_3696(bytes32) -> REF_3695.sourceId
REF_3697(bytes32) -> _settings_1 (-> ['TMP_5269']).chainId
TMP_5271(bool) = REF_3696 == REF_3697
TMP_5272(None) = SOLIDITY_CALL revert InvalidChain()()
TMP_5273(None) = SOLIDITY_CALL require(bool,error)(TMP_5271,TMP_5272)
 require(bool,error)(fdcVerification.verifyAddressValidity(_proof),revert AddressValidityNotProven()())
TMP_5274(bool) = HIGH_LEVEL_CALL, dest:fdcVerification_1(IFdcVerification), function:verifyAddressValidity, arguments:['_proof_1']  
TMP_5275(None) = SOLIDITY_CALL revert AddressValidityNotProven()()
TMP_5276(None) = SOLIDITY_CALL require(bool,error)(TMP_5274,TMP_5275)
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

#### SafeCast.toUint32(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint32).max,SafeCast: value doesn't fit in 32 bits)
TMP_767(uint32) := 4294967295(uint32)
TMP_768(bool) = value_1 <= TMP_767
TMP_769(None) = SOLIDITY_CALL require(bool,string)(TMP_768,SafeCast: value doesn't fit in 32 bits)
 uint32(value)
TMP_770 = CONVERT value_1 to uint32
RETURN TMP_770
```

#### IICollateralPool.destroy(address) [EXTERNAL]
```slithir

```
#### IIAgentVault.destroy() [EXTERNAL]
```slithir

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
#### SafeCast.toUint16(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint16).max,SafeCast: value doesn't fit in 16 bits)
TMP_777(uint16) := 65535(uint16)
TMP_778(bool) = value_1 <= TMP_777
TMP_779(None) = SOLIDITY_CALL require(bool,string)(TMP_778,SafeCast: value doesn't fit in 16 bits)
 uint16(value)
TMP_780 = CONVERT value_1 to uint16
RETURN TMP_780
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
#### IICollateralPool.setExitCollateralRatioBIPS(uint256) [EXTERNAL]
```slithir

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
#### CollateralTypes.getIndex(CollateralType.Class,IERC20) [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4569(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4569'])(AssetManagerState.State) := TMP_4569(AssetManagerState.State)
 index = state.collateralTokenIndex[_tokenKey(_collateralClass,_token)]
REF_3067(mapping(bytes32 => uint256)) -> state_1 (-> ['TMP_4569']).collateralTokenIndex
TMP_4570(bytes32) = INTERNAL_CALL, CollateralTypes._tokenKey(CollateralType.Class,IERC20)(_collateralClass_1,_token_1)
REF_3068(uint256) -> REF_3067[TMP_4570]
index_1(uint256) := REF_3068(uint256)
 require(bool,error)(index > 0,revert UnknownToken()())
TMP_4571(bool) = index_1 > 0
TMP_4572(None) = SOLIDITY_CALL revert UnknownToken()()
TMP_4573(None) = SOLIDITY_CALL require(bool,error)(TMP_4571,TMP_4572)
 index - 1
TMP_4574(uint256) = index_1 (c)- 1
RETURN TMP_4574
```
#### AgentOwnerRegistry.isWhitelisted(address) [PUBLIC]
```slithir
_address_1(address) := phi(['msg.sender'])
whitelist_1(mapping(address => bool)) := phi(['whitelist_2', 'whitelist_1', 'whitelist_4', 'whitelist_5', 'whitelist_0', 'whitelist_3'])
 whitelist[_address]
REF_313(bool) -> whitelist_1[_address_1]
RETURN REF_313
```
#### CoreVaultClient.getState() [INTERNAL]
```slithir
STATE_POSITION_1(bytes32) := phi(['STATE_POSITION_0'])
 position = STATE_POSITION
position_1(bytes32) := STATE_POSITION_1(bytes32)
 _state = position
_state_1 (-> ['position'])(CoreVaultClient.State) := position_1(bytes32)
 _state
RETURN _state_1 (-> ['position'])
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

#### Conversion.convertAmgToTokenWei(uint256,uint256) [INTERNAL]
```slithir
AMG_TOKEN_WEI_PRICE_SCALE_1(uint256) := phi(['AMG_TOKEN_WEI_PRICE_SCALE_0'])
 _valueAMG.mulDiv(_amgToTokenWeiPrice,AMG_TOKEN_WEI_PRICE_SCALE)
TMP_4674(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_valueAMG_1', '_amgToTokenWeiPrice_1', 'AMG_TOKEN_WEI_PRICE_SCALE_1'] 
RETURN TMP_4674
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

