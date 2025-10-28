# Interface Implementation Index

**Repository**: 2025-10-covenant
**Commit**: d5ebe4
**Total Interfaces Found**: 40

---

## Table of Contents

- [IACLManager](#iaclmanager)
- [IAToken](#iatoken)
- [IAaveIncentivesController](#iaaveincentivescontroller)
- [IAaveOracle](#iaaveoracle)
- [ICovenant](#icovenant)
- [ICovenantPriceOracle](#icovenantpriceoracle)
- [ICreditDelegationToken](#icreditdelegationtoken)
- [IDataProvider](#idataprovider)
- [IDefaultInterestRateStrategy](#idefaultinterestratestrategy)
- [IDelegationToken](#idelegationtoken)
- [IERC20](#ierc20)
- [IERC20](#ierc20)
- [IERC20Detailed](#ierc20detailed)
- [IERC20Errors](#ierc20errors)
- [IERC20Metadata](#ierc20metadata)
- [IERC20WithPermit](#ierc20withpermit)
- [IFlashLoanReceiver](#iflashloanreceiver)
- [IFlashLoanSimpleReceiver](#iflashloansimplereceiver)
- [IInitializableAToken](#iinitializableatoken)
- [IInitializableDebtToken](#iinitializabledebttoken)
- [IL2Pool](#il2pool)
- [ILatentSwapLEX](#ilatentswaplex)
- [ILiquidExchangeModel](#iliquidexchangemodel)
- [IPool](#ipool)
- [IPoolAddressesProvider](#ipooladdressesprovider)
- [IPoolAddressesProviderRegistry](#ipooladdressesproviderregistry)
- [IPoolConfigurator](#ipoolconfigurator)
- [IPoolDataProvider](#ipooldataprovider)
- [IPriceOracle](#ipriceoracle)
- [IPriceOracle](#ipriceoracle)
- [IPriceOracle](#ipriceoracle)
- [IPriceOracleGetter](#ipriceoraclegetter)
- [IPriceOracleSentinel](#ipriceoraclesentinel)
- [IReserveInterestRateStrategy](#ireserveinterestratestrategy)
- [IScaledBalanceToken](#iscaledbalancetoken)
- [ISequencerOracle](#isequenceroracle)
- [IStableDebtToken](#istabledebttoken)
- [ISynthToken](#isynthtoken)
- [ITokenData](#itokendata)
- [IVariableDebtToken](#ivariabledebttoken)

---

## Interface Details

### IACLManager

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/IACLManager.sol`

**Implementation Count**: 1

**Implementations**:

- **ACLManager**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/configuration/ACLManager.sol`
  - Location: External Library

---

### IAToken

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/IAToken.sol`

**Implementation Count**: 4

**Implementations**:

- **AToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/AToken.sol`
  - Location: External Library
- **DelegationAwareAToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/DelegationAwareAToken.sol`
  - Location: External Library
- **MockAToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/upgradeability/MockAToken.sol`
  - Location: External Library
- **MockATokenRepayment**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/tokens/MockATokenRepayment.sol`
  - Location: External Library

---

### IAaveIncentivesController

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/IAaveIncentivesController.sol`

**Implementation Count**: 1

**Implementations**:

- **MockIncentivesController**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/helpers/MockIncentivesController.sol`
  - Location: External Library

---

### IAaveOracle

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/IAaveOracle.sol`

**Implementation Count**: 1

**Implementations**:

- **AaveOracle**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/misc/AaveOracle.sol`
  - Location: External Library

---

### ICovenant

**Interface File**: `2025-10-covenant/src/interfaces/ICovenant.sol`

**Implementation Count**: 1

**Implementations**:

- **Covenant**
  - File: `2025-10-covenant/src/Covenant.sol`
  - Location: Source Code

---

### ICovenantPriceOracle

**Interface File**: `2025-10-covenant/src/curators/interfaces/ICovenantPriceOracle.sol`

**Implementation Count**: 2

**Implementations**:

- **BaseAdapter**
  - File: `2025-10-covenant/src/curators/oracles/BaseAdapter.sol`
  - Location: Source Code
- **CrossAdapter**
  - File: `2025-10-covenant/src/curators/oracles/CrossAdapter.sol`
  - Location: Source Code

---

### ICreditDelegationToken

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/ICreditDelegationToken.sol`

**Implementation Count**: 5

**Implementations**:

- **DebtTokenBase**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/base/DebtTokenBase.sol`
  - Location: External Library
- **MockStableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/upgradeability/MockStableDebtToken.sol`
  - Location: External Library
- **MockVariableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/upgradeability/MockVariableDebtToken.sol`
  - Location: External Library
- **StableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/StableDebtToken.sol`
  - Location: External Library
- **VariableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/VariableDebtToken.sol`
  - Location: External Library

---

### IDataProvider

**Interface File**: `2025-10-covenant/src/periphery/interfaces/IDataProvider.sol`

**Implementation Count**: 1

**Implementations**:

- **DataProvider**
  - File: `2025-10-covenant/src/periphery/DataProvider.sol`
  - Location: Source Code

---

### IDefaultInterestRateStrategy

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/IDefaultInterestRateStrategy.sol`

**Implementation Count**: 1

**Implementations**:

- **DefaultReserveInterestRateStrategy**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/pool/DefaultReserveInterestRateStrategy.sol`
  - Location: External Library

---

### IDelegationToken

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/IDelegationToken.sol`

**Implementation Count**: 1

**Implementations**:

- **MintableDelegationERC20**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/tokens/MintableDelegationERC20.sol`
  - Location: External Library

---

### IERC20

**Interface File**: `2025-10-covenant/lib/openzeppelin-contracts/contracts/token/ERC20/IERC20.sol`

**Implementation Count**: 3

**Implementations**:

- **SynthToken**
  - File: `2025-10-covenant/src/synths/SynthToken.sol`
  - Location: Source Code
- **SynthToken**
  - File: `2025-10-covenant/src/synths/SynthToken.sol`
  - Location: Source Code
- **SynthToken**
  - File: `2025-10-covenant/src/synths/SynthToken.sol`
  - Location: Source Code

---

### IERC20

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/dependencies/openzeppelin/contracts/IERC20.sol`

**Implementation Count**: 16

**Implementations**:

- **AToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/AToken.sol`
  - Location: External Library
- **AToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/AToken.sol`
  - Location: External Library
- **DelegationAwareAToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/DelegationAwareAToken.sol`
  - Location: External Library
- **DelegationAwareAToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/DelegationAwareAToken.sol`
  - Location: External Library
- **IncentivizedERC20**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/base/IncentivizedERC20.sol`
  - Location: External Library
- **MintableERC20**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/tokens/MintableERC20.sol`
  - Location: External Library
- **MintableIncentivizedERC20**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/base/MintableIncentivizedERC20.sol`
  - Location: External Library
- **MockAToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/upgradeability/MockAToken.sol`
  - Location: External Library
- **MockAToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/upgradeability/MockAToken.sol`
  - Location: External Library
- **MockATokenRepayment**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/tokens/MockATokenRepayment.sol`
  - Location: External Library
- **MockATokenRepayment**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/tokens/MockATokenRepayment.sol`
  - Location: External Library
- **MockStableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/upgradeability/MockStableDebtToken.sol`
  - Location: External Library
- **MockVariableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/upgradeability/MockVariableDebtToken.sol`
  - Location: External Library
- **ScaledBalanceTokenBase**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/base/ScaledBalanceTokenBase.sol`
  - Location: External Library
- **StableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/StableDebtToken.sol`
  - Location: External Library
- **VariableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/VariableDebtToken.sol`
  - Location: External Library

---

### IERC20Detailed

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/dependencies/openzeppelin/contracts/IERC20Detailed.sol`

**Implementation Count**: 11

**Implementations**:

- **AToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/AToken.sol`
  - Location: External Library
- **DelegationAwareAToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/DelegationAwareAToken.sol`
  - Location: External Library
- **IncentivizedERC20**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/base/IncentivizedERC20.sol`
  - Location: External Library
- **MintableIncentivizedERC20**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/base/MintableIncentivizedERC20.sol`
  - Location: External Library
- **MockAToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/upgradeability/MockAToken.sol`
  - Location: External Library
- **MockATokenRepayment**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/tokens/MockATokenRepayment.sol`
  - Location: External Library
- **MockStableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/upgradeability/MockStableDebtToken.sol`
  - Location: External Library
- **MockVariableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/upgradeability/MockVariableDebtToken.sol`
  - Location: External Library
- **ScaledBalanceTokenBase**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/base/ScaledBalanceTokenBase.sol`
  - Location: External Library
- **StableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/StableDebtToken.sol`
  - Location: External Library
- **VariableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/VariableDebtToken.sol`
  - Location: External Library

---

### IERC20Errors

**Interface File**: `2025-10-covenant/lib/openzeppelin-contracts/contracts/interfaces/draft-IERC6093.sol`

**Implementation Count**: 1

**Implementations**:

- **SynthToken**
  - File: `2025-10-covenant/src/synths/SynthToken.sol`
  - Location: Source Code

---

### IERC20Metadata

**Interface File**: `2025-10-covenant/lib/openzeppelin-contracts/contracts/token/ERC20/extensions/IERC20Metadata.sol`

**Implementation Count**: 1

**Implementations**:

- **SynthToken**
  - File: `2025-10-covenant/src/synths/SynthToken.sol`
  - Location: Source Code

---

### IERC20WithPermit

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/IERC20WithPermit.sol`

**Implementation Count**: 1

**Implementations**:

- **MintableERC20**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/tokens/MintableERC20.sol`
  - Location: External Library

---

### IFlashLoanReceiver

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/flashloan/interfaces/IFlashLoanReceiver.sol`

**Implementation Count**: 2

**Implementations**:

- **FlashLoanReceiverBase**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/flashloan/base/FlashLoanReceiverBase.sol`
  - Location: External Library
- **MockFlashLoanReceiver**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/flashloan/MockFlashLoanReceiver.sol`
  - Location: External Library

---

### IFlashLoanSimpleReceiver

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/flashloan/interfaces/IFlashLoanSimpleReceiver.sol`

**Implementation Count**: 2

**Implementations**:

- **FlashLoanSimpleReceiverBase**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/flashloan/base/FlashLoanSimpleReceiverBase.sol`
  - Location: External Library
- **MockFlashLoanSimpleReceiver**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/flashloan/MockSimpleFlashLoanReceiver.sol`
  - Location: External Library

---

### IInitializableAToken

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/IInitializableAToken.sol`

**Implementation Count**: 4

**Implementations**:

- **AToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/AToken.sol`
  - Location: External Library
- **DelegationAwareAToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/DelegationAwareAToken.sol`
  - Location: External Library
- **MockAToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/upgradeability/MockAToken.sol`
  - Location: External Library
- **MockATokenRepayment**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/tokens/MockATokenRepayment.sol`
  - Location: External Library

---

### IInitializableDebtToken

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/IInitializableDebtToken.sol`

**Implementation Count**: 4

**Implementations**:

- **MockStableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/upgradeability/MockStableDebtToken.sol`
  - Location: External Library
- **MockVariableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/upgradeability/MockVariableDebtToken.sol`
  - Location: External Library
- **StableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/StableDebtToken.sol`
  - Location: External Library
- **VariableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/VariableDebtToken.sol`
  - Location: External Library

---

### IL2Pool

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/IL2Pool.sol`

**Implementation Count**: 2

**Implementations**:

- **L2Pool**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/pool/L2Pool.sol`
  - Location: External Library
- **MockL2Pool**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/helpers/MockL2Pool.sol`
  - Location: External Library

---

### ILatentSwapLEX

**Interface File**: `2025-10-covenant/src/lex/latentswap/interfaces/ILatentSwapLEX.sol`

**Implementation Count**: 1

**Implementations**:

- **LatentSwapLEX**
  - File: `2025-10-covenant/src/lex/latentswap/LatentSwapLEX.sol`
  - Location: Source Code

---

### ILiquidExchangeModel

**Interface File**: `2025-10-covenant/src/interfaces/ILiquidExchangeModel.sol`

**Implementation Count**: 1

**Implementations**:

- **LatentSwapLEX**
  - File: `2025-10-covenant/src/lex/latentswap/LatentSwapLEX.sol`
  - Location: Source Code

---

### IPool

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/IPool.sol`

**Implementation Count**: 4

**Implementations**:

- **L2Pool**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/pool/L2Pool.sol`
  - Location: External Library
- **MockL2Pool**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/helpers/MockL2Pool.sol`
  - Location: External Library
- **MockPoolInherited**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/helpers/MockPool.sol`
  - Location: External Library
- **Pool**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/pool/Pool.sol`
  - Location: External Library

---

### IPoolAddressesProvider

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/IPoolAddressesProvider.sol`

**Implementation Count**: 1

**Implementations**:

- **PoolAddressesProvider**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/configuration/PoolAddressesProvider.sol`
  - Location: External Library

---

### IPoolAddressesProviderRegistry

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/IPoolAddressesProviderRegistry.sol`

**Implementation Count**: 1

**Implementations**:

- **PoolAddressesProviderRegistry**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/configuration/PoolAddressesProviderRegistry.sol`
  - Location: External Library

---

### IPoolConfigurator

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/IPoolConfigurator.sol`

**Implementation Count**: 1

**Implementations**:

- **PoolConfigurator**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/pool/PoolConfigurator.sol`
  - Location: External Library

---

### IPoolDataProvider

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/IPoolDataProvider.sol`

**Implementation Count**: 1

**Implementations**:

- **AaveProtocolDataProvider**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/misc/AaveProtocolDataProvider.sol`
  - Location: External Library

---

### IPriceOracle

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/IPriceOracle.sol`

**Implementation Count**: 1

**Implementations**:

- **PriceOracle**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/oracle/PriceOracle.sol`
  - Location: External Library

---

### IPriceOracle

**Interface File**: `2025-10-covenant/src/interfaces/IPriceOracle.sol`

**Implementation Count**: 1

**Implementations**:

- **CovenantCurator**
  - File: `2025-10-covenant/src/curators/CovenantCurator.sol`
  - Location: Source Code

---

### IPriceOracle

**Interface File**: `2025-10-covenant/lib/euler-price-oracle/src/interfaces/IPriceOracle.sol`

**Implementation Count**: 22

**Implementations**:

- **BaseAdapter**
  - File: `2025-10-covenant/lib/euler-price-oracle/src/adapter/BaseAdapter.sol`
  - Location: External Library
- **BaseAdapter**
  - File: `2025-10-covenant/src/curators/oracles/BaseAdapter.sol`
  - Location: Source Code
- **ChainlinkInfrequentOracle**
  - File: `2025-10-covenant/lib/euler-price-oracle/src/adapter/chainlink/ChainlinkInfrequentOracle.sol`
  - Location: External Library
- **ChainlinkOracle**
  - File: `2025-10-covenant/lib/euler-price-oracle/src/adapter/chainlink/ChainlinkOracle.sol`
  - Location: External Library
- **ChainlinkOracle**
  - File: `2025-10-covenant/src/curators/oracles/chainlink/ChainlinkOracle.sol`
  - Location: Source Code
- **ChronicleOracle**
  - File: `2025-10-covenant/lib/euler-price-oracle/src/adapter/chronicle/ChronicleOracle.sol`
  - Location: External Library
- **CrossAdapter**
  - File: `2025-10-covenant/lib/euler-price-oracle/src/adapter/CrossAdapter.sol`
  - Location: External Library
- **CrossAdapter**
  - File: `2025-10-covenant/src/curators/oracles/CrossAdapter.sol`
  - Location: Source Code
- **CurveEMAOracle**
  - File: `2025-10-covenant/lib/euler-price-oracle/src/adapter/curve/CurveEMAOracle.sol`
  - Location: External Library
- **EulerRouter**
  - File: `2025-10-covenant/lib/euler-price-oracle/src/EulerRouter.sol`
  - Location: External Library
- **FixedRateOracle**
  - File: `2025-10-covenant/lib/euler-price-oracle/src/adapter/fixed/FixedRateOracle.sol`
  - Location: External Library
- **IdleTranchesOracle**
  - File: `2025-10-covenant/lib/euler-price-oracle/src/adapter/idle/IdleTranchesOracle.sol`
  - Location: External Library
- **LidoFundamentalOracle**
  - File: `2025-10-covenant/lib/euler-price-oracle/src/adapter/lido/LidoFundamentalOracle.sol`
  - Location: External Library
- **LidoOracle**
  - File: `2025-10-covenant/lib/euler-price-oracle/src/adapter/lido/LidoOracle.sol`
  - Location: External Library
- **OndoOracle**
  - File: `2025-10-covenant/lib/euler-price-oracle/src/adapter/ondo/OndoOracle.sol`
  - Location: External Library
- **PendleOracle**
  - File: `2025-10-covenant/lib/euler-price-oracle/src/adapter/pendle/PendleOracle.sol`
  - Location: External Library
- **PendleUniversalOracle**
  - File: `2025-10-covenant/lib/euler-price-oracle/src/adapter/pendle/PendleUniversalOracle.sol`
  - Location: External Library
- **PythOracle**
  - File: `2025-10-covenant/lib/euler-price-oracle/src/adapter/pyth/PythOracle.sol`
  - Location: External Library
- **PythOracle**
  - File: `2025-10-covenant/src/curators/oracles/pyth/PythOracle.sol`
  - Location: Source Code
- **RateProviderOracle**
  - File: `2025-10-covenant/lib/euler-price-oracle/src/adapter/rate/RateProviderOracle.sol`
  - Location: External Library
- **RedstoneCoreOracle**
  - File: `2025-10-covenant/lib/euler-price-oracle/src/adapter/redstone/RedstoneCoreOracle.sol`
  - Location: External Library
- **UniswapV3Oracle**
  - File: `2025-10-covenant/lib/euler-price-oracle/src/adapter/uniswap/UniswapV3Oracle.sol`
  - Location: External Library

---

### IPriceOracleGetter

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/IPriceOracleGetter.sol`

**Implementation Count**: 1

**Implementations**:

- **AaveOracle**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/misc/AaveOracle.sol`
  - Location: External Library

---

### IPriceOracleSentinel

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/IPriceOracleSentinel.sol`

**Implementation Count**: 1

**Implementations**:

- **PriceOracleSentinel**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/configuration/PriceOracleSentinel.sol`
  - Location: External Library

---

### IReserveInterestRateStrategy

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/IReserveInterestRateStrategy.sol`

**Implementation Count**: 1

**Implementations**:

- **DefaultReserveInterestRateStrategy**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/pool/DefaultReserveInterestRateStrategy.sol`
  - Location: External Library

---

### IScaledBalanceToken

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/IScaledBalanceToken.sol`

**Implementation Count**: 13

**Implementations**:

- **AToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/AToken.sol`
  - Location: External Library
- **AToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/AToken.sol`
  - Location: External Library
- **DelegationAwareAToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/DelegationAwareAToken.sol`
  - Location: External Library
- **DelegationAwareAToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/DelegationAwareAToken.sol`
  - Location: External Library
- **MockAToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/upgradeability/MockAToken.sol`
  - Location: External Library
- **MockAToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/upgradeability/MockAToken.sol`
  - Location: External Library
- **MockATokenRepayment**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/tokens/MockATokenRepayment.sol`
  - Location: External Library
- **MockATokenRepayment**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/tokens/MockATokenRepayment.sol`
  - Location: External Library
- **MockVariableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/upgradeability/MockVariableDebtToken.sol`
  - Location: External Library
- **MockVariableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/upgradeability/MockVariableDebtToken.sol`
  - Location: External Library
- **ScaledBalanceTokenBase**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/base/ScaledBalanceTokenBase.sol`
  - Location: External Library
- **VariableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/VariableDebtToken.sol`
  - Location: External Library
- **VariableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/VariableDebtToken.sol`
  - Location: External Library

---

### ISequencerOracle

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/ISequencerOracle.sol`

**Implementation Count**: 1

**Implementations**:

- **SequencerOracle**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/oracle/SequencerOracle.sol`
  - Location: External Library

---

### IStableDebtToken

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/IStableDebtToken.sol`

**Implementation Count**: 2

**Implementations**:

- **MockStableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/upgradeability/MockStableDebtToken.sol`
  - Location: External Library
- **StableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/StableDebtToken.sol`
  - Location: External Library

---

### ISynthToken

**Interface File**: `2025-10-covenant/src/interfaces/ISynthToken.sol`

**Implementation Count**: 1

**Implementations**:

- **SynthToken**
  - File: `2025-10-covenant/src/synths/SynthToken.sol`
  - Location: Source Code

---

### ITokenData

**Interface File**: `2025-10-covenant/src/lex/latentswap/interfaces/ITokenData.sol`

**Implementation Count**: 2

**Implementations**:

- **LatentSwapLEX**
  - File: `2025-10-covenant/src/lex/latentswap/LatentSwapLEX.sol`
  - Location: Source Code
- **TokenData**
  - File: `2025-10-covenant/src/lex/latentswap/libraries/TokenData.sol`
  - Location: Source Code

---

### IVariableDebtToken

**Interface File**: `2025-10-covenant/lib/aave-v3-core/contracts/interfaces/IVariableDebtToken.sol`

**Implementation Count**: 2

**Implementations**:

- **MockVariableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/mocks/upgradeability/MockVariableDebtToken.sol`
  - Location: External Library
- **VariableDebtToken**
  - File: `2025-10-covenant/lib/aave-v3-core/contracts/protocol/tokenization/VariableDebtToken.sol`
  - Location: External Library

---

## Statistics

- **Total Interfaces**: 40
- **Total Implementations**: 123
- **Average Implementations per Interface**: 3.08

**Implementations by Location**:

- Source Code (`/src/`): 37
- External Libraries (`/lib/`): 86
- Tests (`/test/`): 0
- Other: 0

