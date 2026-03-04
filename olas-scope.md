## Publicly known issues

_Anything included in this section is considered a publicly known issue and is therefore ineligible for awards._

The known issues (some of them intended by design) that are not in scope for this audit are outlined in the following documents:

- https://github.com/valory-xyz/autonolas-governance/blob/v1.2.5-pre-external-audit/docs/Vulnerabilities_list_governance.pdf 
- https://github.com/valory-xyz/autonolas-registries/blob/v1.3.2-pre-external-audit/docs/Vulnerabilities_list_registries.pdf
- https://github.com/valory-xyz/autonolas-tokenomics/blob/v1.4.2-pre-external-audit/docs/Vulnerabilities_list_tokenomics.pdf

### Vulnerabilities_list_tokenomics - ALL Issues OUT OF SCOPE

Involved contracts and level of the bugs
The present document describes issues affecting Tokenomics contracts
Vulnerabilities
1. depositServiceDonationsETH function (services state)
Severity: Low
The following function is implemented in the Treasury contract:
function depositServiceDonationsETH(uint256[] memory serviceIds, uint256[]
memory amounts) external payable
This service donating function calls another function from the Tokenomics contract that
ultimately results in calling the internal function
trackServiceDonations(). The latter one_
checks whether agent and component Ids of each of the passed service Id exist, and if
not, reverts with the ServiceNeverDeployed() error. The error arises from the fact that the
service was never deployed, and its underlying component and agent Ids were not
assigned (the assignment of underlying component and/or agent Ids to a service happens
during the deployment of the service itself).
However, after a specific service is deployed at least once and then terminated, it can be
updated and re-deployed again. In particular, the service can be updated with a different
set of agent Ids, making the donation distribution setup invalid for the following reason. If
this updated service receives a donation before it is re-deployed, the donation will be
distributed between its old component and agent Ids owners and not the new ones.
Therefore, donating to an updated service before its redeployment can affect the correct
distribution of rewards in the Tokenomics contract. We recommend not to donate when a
service is not in the Deployed or TerminatedBonded state (e.g. any service with
serviceIds[i] not in Deployed or TerminatedBonded state must not be passed as input
parameters to the function depositServiceDonationsETH). The state of the service can
be easily checked via the ServiceRegistry contract view function getService(uint256
serviceId).
2. depositServiceDonationsETH function (OLAS incentives)
Severity: Informative
The following function is implemented in the Treasury contract:
function depositServiceDonationsETH(uint256[] memory serviceIds, uint256[]
memory amounts) external payable
If a DAO member, holding the veOLAS threshold1
, uses this method to donate ETH to a
specific service, or if the service owner is a DAO member holding the veOLAS threshold2
,
the owners of the agents and components referenced in that service are entitled to
receive a share of the donation and OLAS top-ups generated through inflation.
While the current approach encourages service registration and donations through the
utilization of all available OLAS each epoch, this might be utilized in a counter-intended
1 Currently, the threshold for participation is set at 10000 veOLAS, and adjustments to this
threshold can be made through a governance voting process.2 Currently, the threshold for participation is set at 10000 veOLAS, and adjustments to this
threshold can be made through a governance voting process.
way by malicious donators or malicious service-owners. If a donator (or the
service-owner) owns all the underlying components and agents, meets the sufficient
veOLAS requirement, and makes only a small donation to their service, they could accrue
a significant number of OLAS tokens through inflation top-ups at a low cost. This behavior
may yield considerable gains initially but becomes less profitable as more major players
utilize the protocol, leading to more donations being distributed among multiple services
and stakeholders.
3. deposit method
Severity: High
In the depository contracts, the following method is implemented:
function deposit(uint256 productId, uint256 tokenAmount) external
This method allows users to deposit tokens, acquiring OLAS tokens at a discounted rate.
A potential concern can arise ten years after OLAS token launch in the case of an epoch
crossing into year intervals. In this scenario, a portion of OLAS becomes mintable only in
the eleventh year, as a result of the 1 billion fixed supply constraint for the initial ten years.
The creation of bonding programs with payouts leading to exceeding the total OLAS
supply mintable before ten years and the bonder’s depositing the full amount expecting
these payouts lead to a silent return in the OLAS mint() method and not a revert. This
results in successful product deposit and a consequent loss of OLAS payouts for
bonders.
To address this, a more specific check for epoch crossing year intervals can be integrated
into the tokenomics checkpoint() method. In the absence of redeploying a new
contract, it is recommended to carefully propose the creation of bonding programs at the
end of the tenth year. These programs should be structured ensuring that the payouts are
designed to keep the total amount of OLAS minted below 1 billion OLAS before the
ten-year mark. This precautionary measure prevents eventual lost OLAS payouts.
4. checkpoint method - cross-year
Severity: Informative
In the tokenomics contracts, the following method is implemented:
function checkpoint() external
This method allows users to deposit tokens, acquiring OLAS tokens at a discounted rate.
A potential concern may arise in the event of an epoch crossing into year intervals, where
a portion of OLAS larger than the year inflation limit becomes mintable.
The creation of bonding programs with payouts leading to an excess of the total OLAS
mintable before the specified year and the bonder depositing the full amount may result in
an amount of minted OLAS exceeding the year inflation limit. It's crucial to note that, at
most, only the amount reserved for the remaining time of the epoch from the following
year can be minted.
To address this, a more specific check for epoch crossing year intervals can be integrated
into the tokenomics checkpoint() method. In the absence of redeploying a new
contract, it is recommended to carefully propose the creation of bonding programs for
epoch-crossing years. These programs should be structured to ensure that the payouts
are designed in a manner that keeps the total amount of OLAS minted below the year
inflation limit.
5. Treasury Fund Token Management
Severity: Informative
By design, within the Treasury contract, there is currently no mechanism in place to
facilitate the removal of tokens other than ETH that have not been added to the Treasury
through the treasury depositTokenForOLAS() method.
Therefore, we strongly recommend refraining from transferring funds directly to the
Treasury contract that does not adhere to the established tokenomics logic. This
precautionary measure will help prevent potential freezing of funds within the Treasury
contract
6. Encoded inflation schedule
Severity: Informative
If donors in a given epoch fail to meet the veOLAS threshold for donating ETH to specific
services within 10 years of OLAS token creation, the reserved OLAS inflation for top-ups
remains inactive. Although accounted for in the inflation schedule of that epoch, that
amount is essentially deducted from the inflation schedule. For instance, if x OLAS were
accounted for in the inflation for top-ups during the inaugural tokenomics epoch but no
donator meets the veOLAS threshold, these top-ups cannot be utilized for subsequent
epochs encoded in the 10-year inflation schedule.
A similar scenario can occur when OLAS top-ups and staking incentives are distributed.
Due to the natural rounding behavior of Solidity and the division involved in calculating
top-ups and staking emissions, it's possible that the actual sum of OLAS allocated to
owners of agents and components referenced in donated services and the calculated
staking emissions might be slightly less than the exact amount that can be extracted from
the encoded inflation schedule in the tokenomics contract. In such cases, the difference
between the exact amount and the actually allocated amount for top-up and staking is
implicitly deducted from the inflation schedule.
This deferred inflation isn't lost; rather, it's postponed, as the OLAS token ensures that no
more than 1 billion tokens are minted within a decade, with no more than 2% of the supply
cap being minted annually, starting from 1 billion.
7. Withheld tokens
Severity: Informative
The TargetStakingDispenser contract on L2 withholds some staking emissions sent by L1
(see the section “Verification on staking contract enabled by StakingVerifier” here for
details on the tokens withheld by the TargetStakingDispenser).
To prevent L1 from sending new emissions while there are still withheld emissions on the
TargetStakingDispenser, we need to ensure regular synchronizations between L1 and L2.
Specifically, if there is demand for emissions for a specific contract on L2, and L1 is
synchronized with the withheld amount on the TargetStakingDispenser, L1 will only send a
message without minting or sending new emissions to the L2 target contract until the
withheld amount is fully utilized and additional demand arises.
Additionally, if there is no new demand for emissions from the L2 target dispenser and a
withheld amount remains, the DAO can initiate a new staking campaign to utilize the
withheld amount.
Finally, the DAO can employ the combination of the functions migrate(),
syncWithheldAmount(), processDataMaintenance(),
updateWithheldAmountMaintenance() to transfer and update balance of the withheld
tokens to a DAO-controlled account.
8. changeManagers function (specifically - voteWeighting)
Severity: Informative
The following function is implemented in the Dispenser contract:
function changeManagers(address
address
voteWeighting) external
tokenomics, address
treasury,
_
_
_
The purpose of this function is to change core tokenomics contract addresses. However,
when the Vote Weighting contract address is changed, if not all the staking incentives are
claimed, those can be lost. The idea is to force claim all the staking incentives before the
voteWeighting is updated. More details here.
9. claimStakingIncentives /
calculateStakingIncentivesBatch functions
_
Severity: Low
Following functions is implemented in the Dispenser contract:
function claimStakingIncentives( uint256 numClaimedEpochs,
uint256 chainId, bytes32 stakingTarget, bytes memory bridgePayload )
external payable
function
calculateStakingIncentivesBatch(uint256
_
numClaimedEpochs, uint256[] memory chainIds, bytes32[][] memory
stakingTargets ) internal returns (uint256[] memory totalAmounts,
uint256[][] memory stakingIncentives, uint256[] memory
transferAmounts)
The purpose of these functions is to calculate staking incentives and returns according to
the staking target provided. However, these functions do not account for the fact that the
amount of OLAS previously sent to L2 and communicated as not used and available for
re-usage (withheldAmount) should also be subtracted from the staking incentives
amount in favor of amounts returned back to Tokenomics. This means that staking
incentives amounts that are reused from withheld ones are calculated as subject to
inflation used, whereas in fact that part of inflation is untouched. Ultimately it results in
spending less inflation throughout the inflation period for the amount of funds that were
minted but withheld on L2 target dispenser contracts as over-excessive.
Note that the inflation amount is not returned to Tokenomics due to withheldAmount
reuse is never minted, meaning there is no loss of funds, just the inflation miscalculation
lowering its yearly mint possibility. In the absence of redeploying a new contract, the DAO
might act to adjust the inflation numbers in a distant timeline consolidating information
about all the withheld amounts across chains.
10. migrate function
Severity: Low
The following function is implemented in the TargetDispenserL2 contract:
function migrate(address newL2TargetDispenser) external
The purpose of this function is to migrate all the funds to a new L2TargetDispenser
address. However, this function does not check if the current withheldAmount value is
zero before migrating, essentially having the possibility to lose the inflation information for
not sending additional funds to L2.
In order to avoid the loss of withheldAmount, the DAO is advised to update the value
with the updateWithheldAmountMaintenance() function call right after the
TargetDispenser migration procedure is complete.
11.
sendMessage function
_
Severity: Low
The following function is implemented in the OptimismDepositProcessorL1 contract:
function
sendMessage(address[] memory targets, uint256[] memory
_
stakingIncentives, bytes memory bridgePayload, uint256
transferAmount, bytes32 batchHash) internal override returns (uint256
sequence, uint256 leftovers)
This function forms required data to send tokens and messages to L2 in all the optimism
deposit processor related contracts. A user-controlled gas limit is decoded as a uint256,
which is later truncated to uint32 when passed to the CrossDomainMessenger. If a user
supplies a payload with a value exceeding type(uint32).max, the truncation produces a
much smaller gas limit than intended, bypassing the protocol’s minimum gas check.
Although this action does not result in loss of funds (which are sent separately), it could
deliberately pass a smaller amount of gas such that a corresponding function on L2
reverts. This can then be corrected via the processDataMaintenance() function. In the
absence of contract re-deployment, users are advised to pass a sufficient amount of gas,
or just have it set to zero, such that the fallback value takes care of it.

###  Vulnerabilities_list_registries - ALL Issues OUT OF SCOPE

Involved contracts and level of the bugs
The present document aims to point out some vulnerabilities in the autonolas-registry
contracts.
Vulnerabilities
1. tokenURI function
Severity: Low
The following function is implemented in the GenericRegistry contract:
function tokenURI(uint256 unitId) public view virtual override
returns (string memory)
This function is defined by the EIP-721 standard. The standard states that the function is
supposed to throw if unitId is not a valid NFT. However, in our contract, the function does
not revert if the unitId is out of bounds, but just returns the value of a string with the
defined prefix and 64 zeros derived from a zero bytes32 value.
Therefore, we recommend checking the return value of this view function, and if the last
64 symbols are zero, consider it to be an invalid NFT. Also one might use the exists()
function to preliminary check if the requested NFT Id exists.
2. create function
Severity: Low
The following function is implemented in the GnosisSafeMultisig contract:
function create(address[] memory owners, uint256 threshold, bytes
memory data) external returns (address multisig)
This function creates a Safe service multisig when the service is deployed. Since
Autonolas protocol follows an optimistic design, none of the fields for the Safe multisig
creation are restricted. This way, the service owner might pass the payload field as they
feel fit for the purposes of the service multisig. That said, any possible malicious behavior
can also be embedded in the payload value.
In the event of the intended malicious multisig creation, the Autonolas protocol is not
affected, however, accounts interacting with the corresponding service might bear
eventual consequences of such a setup.
We strongly recommend not abusing the payload field of the service multisig when
deploying the service to perform any malicious actions.
3. update function (zero bonds)
Severity: Low
The following function is implemented in the ServiceRegistry and ServiceRegistryL2
contracts:
function update(address serviceOwner, bytes32 configHash, uint32[]
memory agentIds, uint32 threshold, uint256 serviceId) external
returns (bool success)
This function allows updating a service in a pre-registration state in a CRUD way. E.g. if
there is a need to remove agentIds[i] from the canonical agents making up the
service, then it is sufficient to call this function and update it in such a way that a
corresponding slots field is set to zero, i.e., agentParam[i].slots=0, also adjusting
the threshold.
When an agent slot is non-zero, and an operator can register an agent instance for that
slot, it is necessary that the corresponding agent bond is non-zero. In the current
implementation, there is no check for agent bonds to be different from zero if the
corresponding agent slot is non-zero. This vulnerability would enable an operator to
register an agent instance without the corresponding security bond. Hence, the operator
would not be affected by any possible slashing condition if the total operator bond is
equal to zero.
This vulnerability is addressed for the ServiceRegistry contract and ServiceRegistryL2 by
adding the zero-value check on the service manager level. Specifically, serviceManager
contract handles the check before calling the original serviceRegistry’s update() method.
See
https://github.com/valory-xyz/autonolas-registries/blob/main/test/ServiceManagerToken.j
s#L326-L333C25 for a test proving that the issue is resolved.
In absence of redeploying a new manager for the ServiceRegistryL2 contract on other
chains, we recommend that service owners assign a zero-value to agent bonds only if the
corresponding agent slot is zero.
4. update function (replacing agent Ids)
Severity: Low
The following function is implemented in the ServiceRegistry and ServiceRegistryL2
contract:
function update(address serviceOwner, bytes32 configHash, uint32[]
memory agentIds, uint32 threshold, uint256 serviceId) external
returns (bool success)
As described earlier, this function allows updating a service in a pre-registration state in a
CRUD way. However, considering that there is no possible direct damage to the protocol
and to save on transaction gas costs, the function is implemented via an optimistic
approach.
Specifically, the service owner might not specify that some of the agent Ids of the
previous setup must be taken out of the system (by setting corresponding slots variable
to zero). This means that operators are able to register agent instances specifying
non-declared service agent Ids (as those were deliberately left in the corresponding map
from the previous setup). This might lead to deploying the service on agent Ids from the
previous setup, declaring that they actually run on current ones (as retrieved via the
getService() view function).
We strongly recommend not abusing the update() function in order to deploy the service
to perform any malicious actions by using undeclared agent Ids, since this behavior is
easily spotted off-chain.
5. drain function
Severity: Informative
The following function is implemented in the ServiceRegistryTokenUtility contract:
function drain(address token) external returns (uint256 amount)
The primary purpose of this function is to allow the removal of slashed tokens, other than
chain-native tokens, from the contract.
By design, in the current setup of the Treasury contract, there is currently no mechanism
in place to facilitate the removal of tokens other than ETH that have not been added to the
Treasury through the treasury depositTokenForOLAS() method. Therefore, we strongly
advise against assigning the drainer role to the Treasury contract for
ServiceRegistryTokenUtility contract deployed on Ethereum.
6.
checkTokenStakingDeposit function
_
Severity: Informative
The following function is implemented in the ServiceRegistryTokenUtility contract:
function
checkTokenStakingDeposit(uint256 serviceId, uint256
_
stakingDeposit, uint32[] memory) internal view virtual
The primary purpose of this function is to ensure that the service owner's security
deposit and the operator bonds are correctly configured. Specifically, it checks that the
service owner's security deposit ( ) and the 𝑠𝑒𝑐𝑢𝑟𝑖𝑡𝑦𝐷𝑒𝑝𝑜𝑠𝑖𝑡 𝑏𝑜𝑛𝑑
for each operator are
greater than or equal to . Given that 𝑚𝑖𝑛𝑆𝑡𝑎𝑘𝑖𝑛𝑔𝐷𝑒𝑝𝑜𝑠𝑖𝑡 𝑠𝑒𝑐𝑢𝑟𝑖𝑡𝑦𝐷𝑒𝑝𝑜𝑠𝑖𝑡
is defined as the
maximum among the operator bonds ( ), when equals
𝑚𝑎𝑥𝑏𝑜𝑛𝑑{𝑏𝑜𝑛𝑑} 𝑚𝑖𝑛𝑆𝑡𝑎𝑘𝑖𝑛𝑔𝐷𝑒𝑝𝑜𝑠𝑖𝑡
𝑠𝑒𝑐𝑢𝑟𝑖𝑡𝑦𝐷𝑒𝑝𝑜𝑠𝑖𝑡
, the following relationship holds:
𝑚𝑖𝑛𝑆𝑡𝑎𝑘𝑖𝑛𝑔𝐷𝑒𝑝𝑜𝑠𝑖𝑡 = 𝑠𝑒𝑐𝑢𝑟𝑖𝑡𝑦𝐷𝑒𝑝𝑜𝑠𝑖𝑡 >= 𝑏𝑜𝑛𝑑 >= 𝑚𝑖𝑛𝑆𝑡𝑎𝑘𝑖𝑛𝑔𝐷𝑒𝑝𝑜𝑠𝑖𝑡
This ensures that 𝑠𝑒𝑐𝑢𝑟𝑖𝑡𝑦𝐷𝑒𝑝𝑜𝑠𝑖𝑡 = 𝑚𝑖𝑛𝑆𝑡𝑎𝑘𝑖𝑛𝑔𝐷𝑒𝑝𝑜𝑠𝑖𝑡 = 𝑏𝑜𝑛𝑑
for each operator bond.
It's important to note that the service registry and service registry utility tokens do not
enforce this requirement at the service level.
If one attempts to stake a service with a equal to 𝑠𝑒𝑐𝑢𝑟𝑖𝑡𝑦𝐷𝑒𝑝𝑜𝑠𝑖𝑡 𝑚𝑖𝑛𝑆𝑡𝑎𝑘𝑖𝑛𝑔𝐷𝑒𝑝𝑜𝑠𝑖𝑡
and
operator bonds that differ (e.g., 𝑏𝑜𝑛𝑑[𝑖] > 𝑏𝑜𝑛𝑑[𝑖]
), it is recommended to terminate and
update the service configuration to ensure compatibility with the staking logic.
7.
isRatio function
_
Severity: Informative
The following function is implemented in the StakingActivityChecker contract:
function isRatioPass(uint256[] memory curNonces, uint256[] memory
lastNonces, uint256 ts)
This function checks if the service multisig liveness ratio meets the defined threshold.
The provided implementation serves as an illustrative example, and we highlight that
multisig nonces are not tamper-resistant (cf. InternalAudit4 for more details on this). It is
therefore recommended to extend the basic isRatioPass() functionality in the
StakingActivityChecker to verify whether specific on-chain actions occur within
designated time frames. For a tamper-resistant check on on-chain activity, you can
consider the one implemented in MechActivityChecker.sol in this repository.
Additionally, the protocol optimistically assumes that the StakingActivityChecker contract
used for deploying staking instances is implemented with a correct logic. Therefore,
unless unexpected behavior such as reverts or non-boolean returns occur, the contract's
results will be considered accurate. However, this optimistic assumption can be exploited
by malicious users. For instance, malicious users could deploy multiple contracts with
flawed activity checks that always return true. They could then vote for these contracts,
causing the OLAS amount to be distributed to all stakers, including those without activity.
Conversely, malicious users could deploy contracts with incorrect liveness checks that
always return false, leading to a situation where the OLAS amount is sent, but funds
remain stuck in the staking contracts and cannot be recovered.
The following measures can be considered to mitigate eventual abuses:
1. Set a Sensible Threshold: The DAO needs to establish a sensible threshold to
enable staking emissions.
2. On-Chain Blacklist: Implement an on-chain blacklist that can be updated through
governance votes, allowing the community to monitor and exclude malicious
contracts.
3. Off-Chain Reputation System: Consider using an off-chain reputation system,
possibly leveraging oracles, to assess the trustworthiness of contracts.
8. stake function
Severity: Informative
The following function is implemented in the StakingFactory contract:
function stake(uint256 serviceId) external
The function stakes a specified service. However, if the service was evicted, it cannot be
staked again until it is explicitly unstaked.
9. unstake function
Severity: Informative
The following function is implemented in the StakingBase contract:
function unstake(uint256 serviceId) external returns (uint256 reward)
The function unstakes a previously staked service. If there are no available rewards left
on a staking contract, the service can be unstaked immediately at any time. However, if
there are even small funds deposited on the staking contract, the service will not be
unstaked. It is not considered to be a griefing attack, since the unstake time is
pre-defined via a minStakingDuration parameter. After that time, the service is
unstaked without any concern of zero or non-zero available rewards. When the service is
staked, it implicitly agrees to be staked for at least the minStakingDuration.
10. checkpoint function
Severity: High
The following function is implemented in the StakingBase contract:
function checkpoint() external returns (uint256[] memory, uint256[]
memory, uint256[] memory, uint256[] memory)
The function goes through all currently staked services and checks for their activity KPIs.
As designed, the function is O(n) in its complexity, where n is the number of staked
services. The actual staking limit is 500 services per staking contract. With the recent
Fusaka EVM upgrade being adopted, the hard cap on transaction gas limit is imposed.
Governance action to limit staking number of slots takes roughly a week. During this time,
if a staking contract is misconfigured and has more than 100 slots, there is a risk that
having more number of services staked results in a failure of a checkpoint transaction.
This scenario ultimately leads to all services being staked indefinitely. In time of waiting
for the proposal properly limiting the number of staking contract services, launchers of
staking contracts are urged to configure them with no more than 100 of staking slots.
11. deploy function
Severity: Informational
The following function is implemented in the ServiceRegistry and ServiceRegistyL2
contracts:
function deploy(address serviceOwner, uint256 serviceId, address
multisigImplementation, bytes memory data) external returns (address
multisig)
This function is responsible for deploying a service by creating a multisig instance
controlled by the set of service agent instances. When the deployment uses a
RecoveryModule-enabled multisig implementation, an additional module is installed to
allow the service multisig to recover ownership in the event that agent instance keys are
accidentally lost.
While this recovery mechanism allows re-obtaining control over the service multisig at the
protocol level, it does not guarantee recoverability for all downstream integrations. In
particular, certain interactions may still rely on the original multisig owner keys rather than
the recovered ownership state.
A notable example is the PolySafe (Gnosis Safe L2) integration used by Polymarket. In this
context, if the original multisig owner key is lost, there is no supported mechanism to
regain effective control over the multisig from Polymarket’s perspective, even if
ownership is recovered on-chain via the recovery module. As a consequence, critical
operations such as:
●
●
buying or selling conditional tokens
future earning Polymarket liquidity rewards
may become permanently inaccessible.
Therefore, although the recovery module provides resilience against agent key loss at the
service level, it does not fully mitigate the risk of loss of functionality for external systems
that bind permissions to the original multisig owner keys.
12. create function
Severity: Low
The following function is implemented in the PolySafeCreatorWithRecoveryModule
contract:
function create(address[] memory owners, uint256 threshold, bytes
memory data) external returns (address multisig)
This function creates PolySafe multisig when deploying the service. Since the data
payload in this function consists of signatures, there is a possibility to front-run this
transaction, and the original transaction will fail.
There is no financial benefit of such an attack. However, if this unlikely event takes place
and in order not to set up a separate service, users are advised to engage with the
GnosisSafeSameAddressMultisig contract’s create() function specific for PolySafes.
This contract function call just sets the already created multisig (that was created by front
running), and service deployment is successful.

### Vulnerabilities_list_governance - ALL Issues OUT OF SCOPE

Involved contracts and level of the bugs
The present document aims to point out some vulnerabilities in the contracts veOLAS,
buOLAS, and VoteWeighting. Some of these vulnerabilities may lead to critical1 bugs.
Vulnerabilities
1. getPastVotes function
Acknowledgments: This vulnerability was discovered thanks to howd4ys who kindly
reported it by participating in the Autonolas Immunefi Bug Program.
Severity: Low2
In the veOLAS contract, the following function is implemented:
1 The level of the bug is assigned by following the Immunefi classification2 Since no manipulation of governance voting can currently happen, this vulnerability identifies a
smart contract that fails to deliver promised returns but doesn’t lose value.
function getPastVotes(address account, uint256 blockNumber) public
view override returns (uint256 balance)
This function returns the voting power of the address account at a specific
blockNumber.
This function has an incorrect behavior when the input blockNumber is smaller than the
block number n where a lock was first created for the account.
Specifically, denoting by T the timestamp of the input blockNumber and T1 the
timestamp of the block n, the incorrect behavior arises because of the subtraction
veOLAS.sol#L680 that becomes an addition when blockTime=T is smaller than
uPoint.ts=T1. Denoting by T2 the endTime for the locking created for the account time T1, the calculation in veOLAS.sol#L680 provides the following value for the bias
bias=slope*(T2-T). However, the latter bias is bigger than the correct value for the
bias at the timestamp T1 which should be slope*(T2-T1).
at
We recommend using as an input parameter of the function a blockNumber bigger than
the block number n where a lock was first created for the account.
Note that the function getPastVotes(account,blockNumber) is used to weigh the
voting power of users that cast a vote on a governance proposal. Whether there is a
governance proposal and a user creates a lock after the beginning and no later than the
end of the voting period, due to this vulnerability of
getPastVotes(account,blockNumber), the user would be able to cast a voting power
bigger than its correct one. Note that the manipulation of the voting power has an upper
bound due to the fact that there is a limited number of blocks in a voting period and that
any locks last at least one week. Nevertheless, currently, there is no possibility of creating
new locks, so this issue cannot affect any governance voting.
The wrapped veOLAS (wveOLAS) contract wraps original veOLAS view functions and
serves as mitigating measures to address this issue.
2. balanceOfAt function
Severity: Low3
In the veOLAS contract, the following function is implemented:
3 This function is not currently used in any of the Autonolas on-chain contracts, thus this vulnerability
identifies a smart contract that fails to deliver promised returns but doesn’t lose value.
function balanceOfAt(address account, uint256 blockNumber) external
view returns (uint256 balance)
This function returns the actual balance of the address account at a specific
blockNumber.
This function has an incorrect behavior when the input blockNumber is smaller than the
block number n where a lock was first created for the account.
As for the previous vulnerability explanation ( getPastVotes()), the function
balanceOfAt()uses the same binary search algorithm followed by identical value
extraction, and thus returns a very first balance of a locked point, whereas it should return
zero.
We recommend using blockNumber input parameter value bigger than the block number
n where a lock was first created for the account.
The wrapped veOLAS (wveOLAS) contract wraps original veOLAS view functions and
serves as mitigating measures to address this issue.
3.
checkpoint function
_
Severity: Medium4
In the veOLAS contract, the following function is implemented:
function
checkpoint(address account, LockedBalance memory oldLocked,
_
LockedBalance memory newLocked, uint128 curSupply) internal
According to the article on medium.com, the declaration of a memory struct lastPoint and
its assignment to another memory struct leads to the pointer of the initial struct, and not
its deep copy, that can be observed in line 219. This leads to the incorrect calculations of
history points for the periods of time when there was no checkpoint() function called for
more than a week.
However, there is more to the specified issue that leads to following observations:
● If the contract has not created a user point during a specific week, then when
finally created, the internal checkpoint function writes a point in that week with the
4 When voting via veOLAS, the incorrect value is returned as a read-only value, thus this could
be declared as a Low severity. However, if there are consequences due to incorrect voting
failure, then it is a potential damage to the DAO members, and then the severity is Medium.
block number equal to the last created point but with a timestamp equal to the end
time of the week that has just passed. If no points were created for several weeks,
then the internal checkpoint function recreates a point for each week of inactivity
having the block number equal to the last created user point and the timestamp
equal to the end time of the end of each skipped week.
● Even if the checkpoint is called once a week, two points are created: one point
with a block number equal to the last created point but with a timestamp equal to
the end time of the week, and another one with an actual block number and
corresponding timestamp of the checkpoint call.
This behavior leads to the creation of supply points that have an incorrect block number
detached from the actual timestamp. This further leads to the scenario where all supply
points during the weeks of inactivity of veOLAS have the same block numbers as the first
point that triggered the checkpoint(). In other words, all the supply points that were
recreated at the end time of every week will not be correctly recovered during the block
number search (via the block number itself or the timestamp). Any historic lookups
between two supply points (not including points themselves) that were created
immediately before and immediately later the exact end of a week or that were created
with more than a week of inactivity will have an incorrect block number equal to the one
of a first point.
This might potentially affect the voting functionality. If the voting was performed during
the time that had to account for the inactivity weeks, immediately after the very last point
before the end time of a week, or immediately after an eventful point was created at a the
end time of a week, the weighted total supply (the overall number of votes) in the function
getPastTotalSupply() might return incorrect values (depending on the first point with the
same block number found via a binary search).
In the absence of deploying new contracts, we recommend running the analogue of the
cron scheduler / service that checks for the veOLAS activity during the week, and if there
was none, trigger a checkpoint() function call immediately before and immediately after
end time of each week. This way, all the supply points will be updated throughout the time
of the contract and, we increase the likelihood of having voting periods starting
immediately after and before effective points with different blocks timestamps (not in the
weekly time divider). As the protocol becomes more active, this issue will be minimized by
the participation of DAO members.
To minimize a possible impact, the service triggering the checkpoint() call must be
executed as close to the whole week of unix time as possible. Specifically, if the
checkpoint is called at least once a week, a possible deviation in the total voting supply
can only happen if the vote starts after the very last point of week (not in the weekly time
divider) or before the very first point of a week. The supply deviation factor depends on
the time difference between the very last weekly point (not in the weekly end divider) and
the very first point after the weekly end time divider point.
Therefore, calling checkpoints as closer to the end and the beginning of the week of unix
time as possible the supply deviation can be minimized. A probabilistic analysis of how
likely such a scenario can happen is out of the scope of this document. However, despite
its likelihood, it is worth mentioning that, even in such a scenario, there is no certainty that
the wrong point will be picked by the binary search and ultimately there is no certainty
that the issue is going to affect the expected result.
4. createLockFor function
Severity: Medium
In veOLAS and buOLAS contracts, the following function is implemented:
function createLockFor(address account, uint256 amount, uint256
unlockTime) external
This function allows anyone, even a smart contract, to create a lock for a third-party
account. If the third-party account has already a locked amount the call will be reverted.
If not and the OLAS amount provided as input is non-zero then a lock is created.
As a consequence, any third-party account can be forced into a long lock length (for a
maximum of 4 years for veOLAS and 10 years for buOLAS) by an attacker calling
`
createLockFor
`
with a very small amount of OLAS (i.e. 1/10 ** 18) and a max lock length.
An attacker could use this to prevent locks over a given adversarially chosen interval by
front-running all locks in this manner. All accounts with an intent to lock for less than 4
years would be affected. We assign a low likelihood to this attack, as it is not
economically profitable for the attacker.
Indeed, the caller of the
`
createLockFor
` function can lock for third-party users only by
using its own OLAS tokens. So the mintable OLAS tokens can be temporarily frozen only
with an attacker's extensive cost.
In the buOLAS contract, there is also an extra guardrail that can be considered. If the
attack has been discovered, it is possible to invoke a governance vote to revoke the
unvested OLAS of the third-party account that has been forced in a long lock into
buOLAS. If the governance approves the revoke, the third-party account can call the
buOLAS withdraw function, and all non-vested OLAS tokens will be burned. When the
withdrawal function is called less than one year after the attack, all the contract status
can return to their original status before the attack has been made.
5. totalSupplyLockedAtT function
Severity: Low
In the veOLAS contract, the following function is implemented:
function totalSupplyLockedAtT(uint256 ts) public view returns
(uint256)
The function is used solely by the totalSupplyLocked() function with the current
block.timestamp. By the original design, it is not intended to have a ts parameter
smaller than the current block.timestamp.
We recommend not to call this function for any external purposes. It is a view function
that is not currently used externally in any of Autonolas on-chain protocol contracts, and
thus does not affect any intended behavior.
The wrapped veOLAS (wveOLAS) contract wraps original veOLAS view functions and
serves as mitigating measures to address this issue.
6. getPastTotalSupply function
Severity: Low
In the veOLAS contract, the following function is implemented:
function getPastTotalSupply(uint256 blockNumber) external view
returns (uint256)
The function returns the voting power of a specified block number. However, by the
original implementation, the requested block number must be at least equal to the zero
supply point block number, or the block number of a contract deployment. Otherwise, the
function reverts instead of returning a zero value.
We recommend not to call this function with the input block number value less than a zero
supply point block number, since it is meaningless anyway as there must be no values
before the very first supply point is created in the contract.
7 . processMessageFromForeign function
Severity: Informative
In the HomeMediator contract, the following function is implemented:
function processMessageFromForeign(bytes memory data) external
The role of HomeMediator contract is to execute actions based on governance proposals
originating from Ethereum. This execution is rigorously bound to governance decisions,
with the validation of the message sender being restricted to the Timeloch address on
Ethereum.
In the current implementation, the processMessageFromForeign() method ensures
that the msg.sender aligns with the Ethereum Timelock address. However, it does not
enforce a verification of the source chainId to match Ethereum's chainId. This poses
no immediate issues as the arbitrary message bridge contract, facilitating communication
between Ethereum and Gnosis, exclusively processes requests from the Ethereum chain.
For future scenarios where the arbitrary message bridge contract might handle requests
from diverse chains (as outlined in the doc), it is recommended to improve the
implementation of HomeMediator's processMessageFromForeign() method by
incorporating a chainId check.
8. removeNominee function
Severity: Informative
In the VoteWeighting contract, the following function is implemented:
function removeNominee(bytes32 account, uint256 chainId) external
The removeNominee() function is designed to remove a nominee from the system. This
operation is restricted to the contract owner.
Whether the nominee exists, the nominee's current weight is then set to zero for the next
checkpoint time. The total weight sum is updated to reflect the removal of the nominee's
weight. If a dispenser contract is configured, the function calls the removeNominee
method on the dispenser to ensure this is aware of the removal.
If a nominee is removed and users have allocated non-zero weight to that nominee, the
associated voting power becomes orphaned. The contract doesn't automatically retrieve
or reallocate user voting power within the removeNominee() function itself. However,
users can reclaim their voting power using the
retrieveRemovedNomineeVotingPower() function. It's advisable for voters to update
a non-zero weight of their nominee to zero before the nominee's removal is expected to
happen or to reclaim their vote after the nominee's removal has occurred.
Additionally, when a nominee is removed, the last nominee in the set will take the place of
the removed nominee, changing its ID at the end of the removeNominee() call. It's
important to note that the same ID can correspond to different nominees at different
times, depending on removals.
9.
addNominee and removeNominee functions
_
Severity: Informative
In the VoteWeighting contract, the following functions are implemented:
- function
addNominee(Nominee memory nominee) internal
_
- function removeNominee(bytes32 account, uint256 chainId)
external
The removeNominee() function is designed to remove a nominee from the system, and
this operation is restricted to the contract owner. On the other hand,
addNominee()
_
adds a nominee to the system.
In both cases, if a Dispenser is configured, these functions respectively call the
addNominee() and removeNominee()methods on the dispenser contract. This ensures
that the dispenser is kept informed about any nominees being added or removed.
It's highly recommended for the deployer to set up a dispenser contract immediately after
deploying VoteWeighting and ensure that no nominees were added or removed before
that. This precaution helps prevent potential synchronization issues between nominees on
the VotingWeight and Dispenser contracts.
10. voteForNomineeWeights function
Severity: Informative
In the VoteWeighting contract, the following functions are implemented:
function voteForNomineeWeights(bytes32 account, uint256 chainId,
uint256 weight) public
The function is designed to allocate voting power for changing pool weights. Note that
following the original Curve implementation, the function reverts if the next time of
recording weights is equal to the end of veOLAS lock (reverts if nextTime >=
lockEnd). In case of equality, the voting account is not able to place their weights as
their veOLAS lock expires in a week. We consider this not to be an issue, as one week
of time results in a very low voting power addition, and the lock can always be extended
for longer in order to make a bigger weighting input.
Finally, all vulnerabilities that arise from misconfigured registration from users (e.g. component owners, agent owners, service owners, agents operators) or misuse of the registration logic (e.g. accidental locking of funds, loss of keys to control services, etc.).

# Overview

The audit encompasses parts of governance, tokenomics, and registries of the Olas protocol. Specifically:

- `autonolas-governance`: Contains the Autonolas OLAS token and the governance part of the on-chain protocol. Here, the audit focusses on L1 governance contracts, cross-chain contracts that extends L1 governance to multiple L2s via bridges, security guards ensuring only authorized operations execute on each chain from CM, and token burning.  
- `autonolas-tokenomics`: Contains the tokenomics part of Autonolas onchain-protocol. Specifically, the audit focusses on the logic used to update infation in tokenomics, cross-chain staking distribution system for L2 chains,  and a system that combines protocol-owned liquidity, algorithmic position optimization, and cross-chain buyback-and-burn mechanisms to manage protocol-owned-liquidity and treasury assets across multiple chains.
- `autonolas-registries`: Contains the Autonolas component / agent / service registries part of the on-chain protocol. The focus of the audit here is the service registry and management system that combines service lifecycle management via manager contract, multisig wallet creation with recovery mechanisms, activity-based staking rewards, and metadata management.

# Additional context

## Areas of concern (where to focus for bugs)

Any vulnerability that relies on one of the following attack vectors or a combination thereof is very relevant for this contest:

- Re-entrancy
- Integer Overflows / Underflows
- Access Control Issues
- Price Oracle Manipulation

## Main invariants

The code is huge and very sparse to describe the invariants briefly here; the following docs can be used instead:

- [Autonolas whitepaper](https://www.autonolas.network/documents/whitepaper/Whitepaper%20v1.0.pdf)

The following are relevant for governance-related contracts:

- [Summary of governance model](https://github.com/valory-xyz/autonolas-governance/blob/main/docs/Governance_process.pdf)
- [Cross-chain governance design](https://github.com/valory-xyz/autonolas-governance/blob/main/docs/governace_bridge.pdf)
- [guardCM_modular_approach.pdf](https://github.com/valory-xyz/autonolas-governance/blob/main/docs/guardCM_modular_approach.pdf)

The following are relevant for registries-related contracts:

- [Summary of registries design](https://github.com/valory-xyz/autonolas-registries/blob/main/docs/AgentServicesFunctionality.pdf)
- [Definitions and data structures](https://github.com/valory-xyz/autonolas-registries/blob/main/docs/definitions.md)
- [Services FSM](https://github.com/valory-xyz/autonolas-registries/blob/main/docs/FSM.md)

The following are relevant for tokenomics-related contract:

- [Token Inflation Update](https://github.com/valory-xyz/autonolas-tokenomics/blob/main/docs/Update_tokenomics_inflation.pdf)
- [PoL Management](https://github.com/valory-xyz/autonolas-aip/blob/042c70f23312cea9b82dff2c0bc4363b307d2be4/content/aips/aip-7/core-aip-ultrasound-pol.md)
- [Summary of tokenomics model](https://github.com/valory-xyz/autonolas-tokenomics/blob/main/docs/Autonolas_tokenomics_audit.pdf)
- [Autonolas tokenomics paper](https://www.autonolas.network/documents/whitepaper/Autonolas_Tokenomics_Core_Technical_Document.pdf)
- [Olas staking whitepaper](https://staking.olas.network/poaa-whitepaper.pdf)
- [Olas staking smart contracts](https://github.com/valory-xyz/autonolas-registries/blob/main/docs/StakingSmartContracts.pdf).

## All trusted roles in the protocol

The DAO is always considered trusted and to behave sensibly.
