
#### VirtualProtocolDAO._propose(address[],uint256[],bytes[],string,address) [INTERNAL]
```slithir
targets_1(address[]) := phi(['targets_1'])
values_1(uint256[]) := phi(['values_1'])
calldatas_1(bytes[]) := phi(['calldatas_1'])
description_1(string) := phi(['description_1'])
proposer_1(address) := phi(['proposer_1'])
 super._propose(targets,values,calldatas,description,proposer)
TMP_10268(uint256) = INTERNAL_CALL, GovernorStorage._propose(address[],uint256[],bytes[],string,address)(targets_1,values_1,calldatas_1,description_1,proposer_1)
RETURN TMP_10268
```
#### VirtualProtocolDAO.constructor(IVotes,uint48,uint32,uint256,uint256) [PUBLIC]
```slithir
 Governor(VirtualProtocol)
INTERNAL_CALL, Governor.constructor(string)(VirtualProtocol)
 GovernorSettings(initialVotingDelay,initialVotingPeriod,initialProposalThreshold)
INTERNAL_CALL, GovernorSettings.constructor(uint48,uint32,uint256)(initialVotingDelay_1,initialVotingPeriod_1,initialProposalThreshold_1)
 GovernorVotes(token)
INTERNAL_CALL, GovernorVotes.constructor(IVotes)(token_1)
 GovernorVotesQuorumFraction(initialQuorumNumerator)
INTERNAL_CALL, GovernorVotesQuorumFraction.constructor(uint256)(initialQuorumNumerator_1)
```
#### VirtualProtocolDAO.proposalThreshold() [PUBLIC]
```slithir
 super.proposalThreshold()
TMP_10266(uint256) = INTERNAL_CALL, GovernorSettings.proposalThreshold()()
RETURN TMP_10266
```
#### VirtualProtocolDAO.propose(address[],uint256[],bytes[],string) [PUBLIC]
```slithir
 super.propose(targets,values,calldatas,description)
TMP_10267(uint256) = INTERNAL_CALL, Governor.propose(address[],uint256[],bytes[],string)(targets_1,values_1,calldatas_1,description_1)
RETURN TMP_10267
```
#### VirtualProtocolDAO.quorum(uint256) [PUBLIC]
```slithir
blockNumber_1(uint256) := phi(['TMP_10219'])
 super.quorum(blockNumber)
TMP_10269(uint256) = INTERNAL_CALL, GovernorVotesQuorumFraction.quorum(uint256)(blockNumber_1)
RETURN TMP_10269
```
#### VirtualProtocolDAO.quorumDenominator() [PUBLIC]
```slithir
 10000
RETURN 10000
```
#### VirtualGenesisDAO.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 BALLOT_TYPEHASH = keccak256(bytes)(Ballot(uint256 proposalId,uint8 support,address voter,uint256 nonce))
 EXTENDED_BALLOT_TYPEHASH = keccak256(bytes)(ExtendedBallot(uint256 proposalId,uint8 support,address voter,uint256 nonce,string reason,bytes params))
 DEFAULT_ADMIN_ROLE = 0x00
 EXECUTOR_ROLE = keccak256(bytes)(EXECUTOR_ROLE)
role_1(bytes32) := phi(['EXECUTOR_ROLE_1', 'TMP_9582', 'TMP_9575', 'TMP_9577', 'TMP_9580'])
 _checkRole(role)
INTERNAL_CALL, AccessControl._checkRole(bytes32)(role_1)
 _checkGovernance()
INTERNAL_CALL, Governor._checkGovernance()()
```
#### VirtualProtocolDAO.votingDelay() [PUBLIC]
```slithir
 super.votingDelay()
TMP_10264(uint256) = INTERNAL_CALL, GovernorSettings.votingDelay()()
RETURN TMP_10264
```
#### VirtualProtocolDAO.votingPeriod() [PUBLIC]
```slithir
 super.votingPeriod()
TMP_10265(uint256) = INTERNAL_CALL, GovernorSettings.votingPeriod()()
RETURN TMP_10265
```
