
#### RedemptionQueue.createRedemptionTicket(RedemptionQueue.State,address,uint64) [INTERNAL]
```slithir
 agent = _state.agents[_agentVault]
REF_3775(mapping(address => RedemptionQueue.AgentQueue)) -> _state_1 (-> []).agents
REF_3776(RedemptionQueue.AgentQueue) -> REF_3775[_agentVault_1]
agent_1 (-> ['_state'])(RedemptionQueue.AgentQueue) := REF_3776(RedemptionQueue.AgentQueue)
 ticketId = ++ _state.newTicketId
REF_3777(uint64) -> _state_1 (-> []).newTicketId
_state_2 (-> [])(RedemptionQueue.State) := phi(['_state_1 (-> [])'])
REF_3777(-> _state_2 (-> [])) = REF_3777 (c)+ 1
ticketId_1(uint64) := REF_3777(uint64)
 _state.tickets[ticketId] = Ticket({agentVault:_agentVault,valueAMG:_valueAMG,prev:_state.lastTicketId,next:0,prevForAgent:agent.lastTicketId,nextForAgent:0})
REF_3778(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_2 (-> []).tickets
REF_3779(RedemptionQueue.Ticket) -> REF_3778[ticketId_1]
REF_3780(uint64) -> _state_2 (-> []).lastTicketId
REF_3781(uint64) -> agent_1 (-> ['_state']).lastTicketId
TMP_5399(RedemptionQueue.Ticket) = new Ticket(_agentVault_1,_valueAMG_1,REF_3780,0,REF_3781,0)
_state_3 (-> [])(RedemptionQueue.State) := phi(['_state_2 (-> [])'])
REF_3779(RedemptionQueue.Ticket) (->_state_3 (-> [])) := TMP_5399(RedemptionQueue.Ticket)
 _state.firstTicketId == 0
REF_3782(uint64) -> _state_3 (-> []).firstTicketId
TMP_5400(bool) = REF_3782 == 0
CONDITION TMP_5400
 assert(bool)(_state.lastTicketId == 0)
REF_3783(uint64) -> _state_3 (-> []).lastTicketId
TMP_5401(bool) = REF_3783 == 0
TMP_5402(None) = SOLIDITY_CALL assert(bool)(TMP_5401)
 _state.firstTicketId = ticketId
REF_3784(uint64) -> _state_3 (-> []).firstTicketId
_state_4 (-> [])(RedemptionQueue.State) := phi(['_state_3 (-> [])'])
REF_3784(uint64) (->_state_4 (-> [])) := ticketId_1(uint64)
 assert(bool)(_state.lastTicketId != 0)
REF_3785(uint64) -> _state_3 (-> []).lastTicketId
TMP_5403(bool) = REF_3785 != 0
TMP_5404(None) = SOLIDITY_CALL assert(bool)(TMP_5403)
 _state.tickets[_state.lastTicketId].next = ticketId
REF_3786(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_3 (-> []).tickets
REF_3787(uint64) -> _state_3 (-> []).lastTicketId
REF_3788(RedemptionQueue.Ticket) -> REF_3786[REF_3787]
REF_3789(uint64) -> REF_3788.next
_state_5 (-> [])(RedemptionQueue.State) := phi(['_state_3 (-> [])'])
REF_3789(uint64) (->_state_5 (-> [])) := ticketId_1(uint64)
_state_6 (-> [])(RedemptionQueue.State) := phi(['_state_4 (-> [])', '_state_5 (-> [])'])
 _state.lastTicketId = ticketId
REF_3790(uint64) -> _state_6 (-> []).lastTicketId
_state_7 (-> [])(RedemptionQueue.State) := phi(['_state_6 (-> [])'])
REF_3790(uint64) (->_state_7 (-> [])) := ticketId_1(uint64)
 agent.firstTicketId == 0
REF_3791(uint64) -> agent_1 (-> ['_state']).firstTicketId
TMP_5405(bool) = REF_3791 == 0
CONDITION TMP_5405
 assert(bool)(agent.lastTicketId == 0)
REF_3792(uint64) -> agent_1 (-> ['_state']).lastTicketId
TMP_5406(bool) = REF_3792 == 0
TMP_5407(None) = SOLIDITY_CALL assert(bool)(TMP_5406)
 agent.firstTicketId = ticketId
REF_3793(uint64) -> agent_1 (-> ['_state']).firstTicketId
agent_2 (-> ['_state'])(RedemptionQueue.AgentQueue) := phi(["agent_1 (-> ['_state'])"])
REF_3793(uint64) (->agent_2 (-> ['_state'])) := ticketId_1(uint64)
_state_9 (-> ['_state'])(RedemptionQueue.State) := phi(["agent_2 (-> ['_state'])"])
 assert(bool)(agent.lastTicketId != 0)
REF_3794(uint64) -> agent_1 (-> ['_state']).lastTicketId
TMP_5408(bool) = REF_3794 != 0
TMP_5409(None) = SOLIDITY_CALL assert(bool)(TMP_5408)
 _state.tickets[agent.lastTicketId].nextForAgent = ticketId
REF_3795(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_7 (-> []).tickets
REF_3796(uint64) -> agent_1 (-> ['_state']).lastTicketId
REF_3797(RedemptionQueue.Ticket) -> REF_3795[REF_3796]
REF_3798(uint64) -> REF_3797.nextForAgent
_state_8 (-> [])(RedemptionQueue.State) := phi(['_state_7 (-> [])'])
REF_3798(uint64) (->_state_8 (-> [])) := ticketId_1(uint64)
agent_3 (-> ['_state'])(RedemptionQueue.AgentQueue) := phi(["agent_2 (-> ['_state'])", "agent_1 (-> ['_state'])"])
 agent.lastTicketId = ticketId
REF_3799(uint64) -> agent_3 (-> ['_state']).lastTicketId
agent_4 (-> ['_state'])(RedemptionQueue.AgentQueue) := phi(["agent_3 (-> ['_state'])"])
REF_3799(uint64) (->agent_4 (-> ['_state'])) := ticketId_1(uint64)
_state_10 (-> ['_state'])(RedemptionQueue.State) := phi(["agent_4 (-> ['_state'])"])
 ticketId
RETURN ticketId_1
```
#### RedemptionQueue.deleteRedemptionTicket(RedemptionQueue.State,uint64) [INTERNAL]
```slithir
 ticket = _state.tickets[_ticketId]
REF_3800(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_1 (-> []).tickets
REF_3801(RedemptionQueue.Ticket) -> REF_3800[_ticketId_1]
ticket_1 (-> ['_state'])(RedemptionQueue.Ticket) := REF_3801(RedemptionQueue.Ticket)
 assert(bool)(ticket.agentVault != address(0))
REF_3802(address) -> ticket_1 (-> ['_state']).agentVault
TMP_5410 = CONVERT 0 to address
TMP_5411(bool) = REF_3802 != TMP_5410
TMP_5412(None) = SOLIDITY_CALL assert(bool)(TMP_5411)
 agent = _state.agents[ticket.agentVault]
REF_3803(mapping(address => RedemptionQueue.AgentQueue)) -> _state_1 (-> []).agents
REF_3804(address) -> ticket_1 (-> ['_state']).agentVault
REF_3805(RedemptionQueue.AgentQueue) -> REF_3803[REF_3804]
agent_1 (-> ['_state'])(RedemptionQueue.AgentQueue) := REF_3805(RedemptionQueue.AgentQueue)
 ticket.prev == 0
REF_3806(uint64) -> ticket_1 (-> ['_state']).prev
TMP_5413(bool) = REF_3806 == 0
CONDITION TMP_5413
 assert(bool)(_ticketId == _state.firstTicketId)
REF_3807(uint64) -> _state_1 (-> []).firstTicketId
TMP_5414(bool) = _ticketId_1 == REF_3807
TMP_5415(None) = SOLIDITY_CALL assert(bool)(TMP_5414)
 _state.firstTicketId = ticket.next
REF_3808(uint64) -> _state_1 (-> []).firstTicketId
REF_3809(uint64) -> ticket_1 (-> ['_state']).next
_state_3 (-> [])(RedemptionQueue.State) := phi(['_state_1 (-> [])'])
REF_3808(uint64) (->_state_3 (-> [])) := REF_3809(uint64)
 assert(bool)(_ticketId != _state.firstTicketId)
REF_3810(uint64) -> _state_1 (-> []).firstTicketId
TMP_5416(bool) = _ticketId_1 != REF_3810
TMP_5417(None) = SOLIDITY_CALL assert(bool)(TMP_5416)
 _state.tickets[ticket.prev].next = ticket.next
REF_3811(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_1 (-> []).tickets
REF_3812(uint64) -> ticket_1 (-> ['_state']).prev
REF_3813(RedemptionQueue.Ticket) -> REF_3811[REF_3812]
REF_3814(uint64) -> REF_3813.next
REF_3815(uint64) -> ticket_1 (-> ['_state']).next
_state_2 (-> [])(RedemptionQueue.State) := phi(['_state_1 (-> [])'])
REF_3814(uint64) (->_state_2 (-> [])) := REF_3815(uint64)
_state_4 (-> [])(RedemptionQueue.State) := phi(['_state_3 (-> [])', '_state_2 (-> [])'])
 ticket.next == 0
REF_3816(uint64) -> ticket_1 (-> ['_state']).next
TMP_5418(bool) = REF_3816 == 0
CONDITION TMP_5418
 assert(bool)(_ticketId == _state.lastTicketId)
REF_3817(uint64) -> _state_4 (-> []).lastTicketId
TMP_5419(bool) = _ticketId_1 == REF_3817
TMP_5420(None) = SOLIDITY_CALL assert(bool)(TMP_5419)
 _state.lastTicketId = ticket.prev
REF_3818(uint64) -> _state_4 (-> []).lastTicketId
REF_3819(uint64) -> ticket_1 (-> ['_state']).prev
_state_6 (-> [])(RedemptionQueue.State) := phi(['_state_4 (-> [])'])
REF_3818(uint64) (->_state_6 (-> [])) := REF_3819(uint64)
 assert(bool)(_ticketId != _state.lastTicketId)
REF_3820(uint64) -> _state_4 (-> []).lastTicketId
TMP_5421(bool) = _ticketId_1 != REF_3820
TMP_5422(None) = SOLIDITY_CALL assert(bool)(TMP_5421)
 _state.tickets[ticket.next].prev = ticket.prev
REF_3821(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_4 (-> []).tickets
REF_3822(uint64) -> ticket_1 (-> ['_state']).next
REF_3823(RedemptionQueue.Ticket) -> REF_3821[REF_3822]
REF_3824(uint64) -> REF_3823.prev
REF_3825(uint64) -> ticket_1 (-> ['_state']).prev
_state_5 (-> [])(RedemptionQueue.State) := phi(['_state_4 (-> [])'])
REF_3824(uint64) (->_state_5 (-> [])) := REF_3825(uint64)
_state_7 (-> [])(RedemptionQueue.State) := phi(['_state_5 (-> [])', '_state_6 (-> [])'])
 ticket.prevForAgent == 0
REF_3826(uint64) -> ticket_1 (-> ['_state']).prevForAgent
TMP_5423(bool) = REF_3826 == 0
CONDITION TMP_5423
 assert(bool)(_ticketId == agent.firstTicketId)
REF_3827(uint64) -> agent_1 (-> ['_state']).firstTicketId
TMP_5424(bool) = _ticketId_1 == REF_3827
TMP_5425(None) = SOLIDITY_CALL assert(bool)(TMP_5424)
 agent.firstTicketId = ticket.nextForAgent
REF_3828(uint64) -> agent_1 (-> ['_state']).firstTicketId
REF_3829(uint64) -> ticket_1 (-> ['_state']).nextForAgent
agent_2 (-> ['_state'])(RedemptionQueue.AgentQueue) := phi(["agent_1 (-> ['_state'])"])
REF_3828(uint64) (->agent_2 (-> ['_state'])) := REF_3829(uint64)
_state_12 (-> ['_state'])(RedemptionQueue.State) := phi(["agent_2 (-> ['_state'])"])
 assert(bool)(_ticketId != agent.firstTicketId)
REF_3830(uint64) -> agent_1 (-> ['_state']).firstTicketId
TMP_5426(bool) = _ticketId_1 != REF_3830
TMP_5427(None) = SOLIDITY_CALL assert(bool)(TMP_5426)
 _state.tickets[ticket.prevForAgent].nextForAgent = ticket.nextForAgent
REF_3831(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_7 (-> []).tickets
REF_3832(uint64) -> ticket_1 (-> ['_state']).prevForAgent
REF_3833(RedemptionQueue.Ticket) -> REF_3831[REF_3832]
REF_3834(uint64) -> REF_3833.nextForAgent
REF_3835(uint64) -> ticket_1 (-> ['_state']).nextForAgent
_state_8 (-> [])(RedemptionQueue.State) := phi(['_state_7 (-> [])'])
REF_3834(uint64) (->_state_8 (-> [])) := REF_3835(uint64)
_state_9 (-> [])(RedemptionQueue.State) := phi(['_state_8 (-> [])', '_state_1 (-> [])'])
agent_3 (-> ['_state'])(RedemptionQueue.AgentQueue) := phi(["agent_1 (-> ['_state'])", "agent_2 (-> ['_state'])"])
 ticket.nextForAgent == 0
REF_3836(uint64) -> ticket_1 (-> ['_state']).nextForAgent
TMP_5428(bool) = REF_3836 == 0
CONDITION TMP_5428
 assert(bool)(_ticketId == agent.lastTicketId)
REF_3837(uint64) -> agent_3 (-> ['_state']).lastTicketId
TMP_5429(bool) = _ticketId_1 == REF_3837
TMP_5430(None) = SOLIDITY_CALL assert(bool)(TMP_5429)
 agent.lastTicketId = ticket.prevForAgent
REF_3838(uint64) -> agent_3 (-> ['_state']).lastTicketId
REF_3839(uint64) -> ticket_1 (-> ['_state']).prevForAgent
agent_4 (-> ['_state'])(RedemptionQueue.AgentQueue) := phi(["agent_3 (-> ['_state'])"])
REF_3838(uint64) (->agent_4 (-> ['_state'])) := REF_3839(uint64)
_state_13 (-> ['_state'])(RedemptionQueue.State) := phi(["agent_4 (-> ['_state'])"])
 assert(bool)(_ticketId != agent.lastTicketId)
REF_3840(uint64) -> agent_3 (-> ['_state']).lastTicketId
TMP_5431(bool) = _ticketId_1 != REF_3840
TMP_5432(None) = SOLIDITY_CALL assert(bool)(TMP_5431)
 _state.tickets[ticket.nextForAgent].prevForAgent = ticket.prevForAgent
REF_3841(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_9 (-> []).tickets
REF_3842(uint64) -> ticket_1 (-> ['_state']).nextForAgent
REF_3843(RedemptionQueue.Ticket) -> REF_3841[REF_3842]
REF_3844(uint64) -> REF_3843.prevForAgent
REF_3845(uint64) -> ticket_1 (-> ['_state']).prevForAgent
_state_10 (-> [])(RedemptionQueue.State) := phi(['_state_9 (-> [])'])
REF_3844(uint64) (->_state_10 (-> [])) := REF_3845(uint64)
_state_11 (-> [])(RedemptionQueue.State) := phi(['_state_10 (-> [])', '_state_1 (-> [])'])
 delete _state.tickets[_ticketId]
REF_3846(mapping(uint64 => RedemptionQueue.Ticket)) -> _state_11 (-> []).tickets
REF_3847(RedemptionQueue.Ticket) -> REF_3846[_ticketId_1]
REF_3846 = delete REF_3847
```

