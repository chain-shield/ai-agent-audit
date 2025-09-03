

#### OperatorHub.approveOperatorPerps(address,uint256) [EXTERNAL]
```slithir
perpManager_4(IOperatorPanel) := phi(['perpManager_0', 'perpManager_1', 'perpManager_3', 'perpManager_5', 'perpManager_7'])
 perpManager.approveOperator(msg.sender,operator,roles)
HIGH_LEVEL_CALL, dest:perpManager_4(IOperatorPanel), function:approveOperator, arguments:['msg.sender', 'operator_1', 'roles_1']  
perpManager_5(IOperatorPanel) := phi(['perpManager_4', 'perpManager_1', 'perpManager_3', 'perpManager_5', 'perpManager_7'])
```
#### OperatorHub.approveOperatorSpot(address,uint256) [EXTERNAL]
```slithir
accountManager_4(IOperatorPanel) := phi(['accountManager_1', 'accountManager_3', 'accountManager_0', 'accountManager_5', 'accountManager_7'])
 accountManager.approveOperator(msg.sender,operator,roles)
HIGH_LEVEL_CALL, dest:accountManager_4(IOperatorPanel), function:approveOperator, arguments:['msg.sender', 'operator_1', 'roles_1']  
accountManager_5(IOperatorPanel) := phi(['accountManager_1', 'accountManager_3', 'accountManager_4', 'accountManager_5', 'accountManager_7'])
```
#### OperatorHub.constructor(IViewPort,IAccountManager) [PUBLIC]
```slithir
 perpManager = IOperatorPanel(address(perpManager_))
TMP_9823 = CONVERT perpManager__1 to address
TMP_9824 = CONVERT TMP_9823 to IOperatorPanel
perpManager_1(IOperatorPanel) := TMP_9824(IOperatorPanel)
 accountManager = IOperatorPanel(address(accountManager_))
TMP_9825 = CONVERT accountManager__1 to address
TMP_9826 = CONVERT TMP_9825 to IOperatorPanel
accountManager_1(IOperatorPanel) := TMP_9826(IOperatorPanel)
```
#### IOperatorHub.disapproveOperatorPerps(address,uint256) [EXTERNAL]
```slithir

```
#### OperatorHub.disapproveOperatorSpot(address,uint256) [EXTERNAL]
```slithir
accountManager_6(IOperatorPanel) := phi(['accountManager_1', 'accountManager_3', 'accountManager_0', 'accountManager_5', 'accountManager_7'])
 accountManager.disapproveOperator(msg.sender,operator,roles)
HIGH_LEVEL_CALL, dest:accountManager_6(IOperatorPanel), function:disapproveOperator, arguments:['msg.sender', 'operator_1', 'roles_1']  
accountManager_7(IOperatorPanel) := phi(['accountManager_1', 'accountManager_3', 'accountManager_6', 'accountManager_5', 'accountManager_7'])
```
#### OperatorHub.getRoleApprovalsPerps(address,address) [EXTERNAL]
```slithir
perpManager_2(IOperatorPanel) := phi(['perpManager_0', 'perpManager_1', 'perpManager_3', 'perpManager_5', 'perpManager_7'])
 perpManager.getOperatorRoleApprovals(account,operator)
TMP_9828(uint256) = HIGH_LEVEL_CALL, dest:perpManager_2(IOperatorPanel), function:getOperatorRoleApprovals, arguments:['account_1', 'operator_1']  
perpManager_3(IOperatorPanel) := phi(['perpManager_1', 'perpManager_3', 'perpManager_2', 'perpManager_5', 'perpManager_7'])
RETURN TMP_9828
 roles
```
#### OperatorHub.getRoleApprovalsSpot(address,address) [EXTERNAL]
```slithir
accountManager_2(IOperatorPanel) := phi(['accountManager_1', 'accountManager_3', 'accountManager_0', 'accountManager_5', 'accountManager_7'])
 accountManager.getOperatorRoleApprovals(account,operator)
TMP_9827(uint256) = HIGH_LEVEL_CALL, dest:accountManager_2(IOperatorPanel), function:getOperatorRoleApprovals, arguments:['account_1', 'operator_1']  
accountManager_3(IOperatorPanel) := phi(['accountManager_1', 'accountManager_3', 'accountManager_2', 'accountManager_5', 'accountManager_7'])
RETURN TMP_9827
 roles
```
#### IOperatorPanel.approveOperator(address,address,uint256) [EXTERNAL]
```slithir

```
#### IOperatorPanel.disapproveOperator(address,address,uint256) [EXTERNAL]
```slithir

```
#### IOperatorPanel.getOperatorRoleApprovals(address,address) [EXTERNAL]
```slithir

```
