# Accepted H/M Findings: The Wildcat Protocol

# [H-01] User could withdraw more than supposed to, forcing last user withdraw to fail

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-the-wildcat-protocol
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/final_report.html

Submitted by deadrxsezzz Within Wildcat, withdraw requests are put into batches. Users first queue their withdraws and whenever there’s sufficient liquidity, they’re filled at the current rate. Usually, withdraw requests are only executable after the expiry passes and then all users within the batch get a cut from the batch.normalizedAmountPaid proportional to the scaled amount they’ve requested a withdraw for.

uint128 newTotalWithdrawn = uint128 ( MathUtils.

mulDiv ( batch.

normalizedAmountPaid, status.

scaledAmount, batch.

scaledTotalAmount ) ); This makes sure that the sum of all withdraws doesn’t exceed the total batch.normalizedAmountPaid.

However, this invariant could be broken, if the market is closed as it allows for a batch’s withdraws to be executed, before all requests are added.

Consider the market is made of 3 lenders - Alice, Bob and Laurence.

Alice queues a larger withdraw with an expiry time 1 year in the future.

Market gets closed.

Alice executes her withdraw request at the current rate.

Bob makes queues multiple smaller requests. As they’re smaller, the normalized amount they represent suffers higher precision loss. Because they’re part of the whole batch, they also slightly lower the batch’s overall rate.

Bob executes his requests.

Laurence queues a withdraw for his entire amount. When he attempts to execute it, it will fail. This is because Alice has executed her withdraw at a higher rate than the current one and there’s now insufficient state.normalizedUnclaimedWithdrawals Note: marking this as High severity as it both could happen intentionally (attacker purposefully queuing numerous low-value withdraws to cause rounding down) and also with normal behaviour in high-value closed access markets where a user’s withdraw could easily be in the hundreds of thousands.

Also breaks core invariant:

The sum of all transfer amounts for withdrawal executions in a batch must be less than or equal to batch.normalizedAmountPaid

## Recommended Mitigation Steps

Although it’s not a clean fix, consider adding a addNormalizedUnclaimedRewards function which can only be called after a market is closed. It takes token from the user and increases the global variable state.normalizedUnclaimedRewards. The invariant would remain broken, but it will make sure no funds are permanently stuck.

laurenceday (Wildcat) confirmed and commented:

We’re going to have to dig into this, but we’re confirming. Thank you!

3docSec (judge) commented:

I am confirming as High, under the assumption that funds can’t be recovered (didn’t see a cancelWithdrawal or similar option).

laurenceday (Wildcat) commented:

Fixed by the mitigation for M-01:

wildcat-finance/v2-protocol@b25e528.

Medium Risk Findings (8)

# [M-01] Users are incentivized to not withdraw immediately after the market is closed

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-the-wildcat-protocol
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/final_report.html

Submitted by deadrxsezzz Within a withdraw batch, all users within said batch are paid equally - at the same rate, despite what exactly was the rate when each individual one created their withdraw.

While this usually is not a problem as it is a way to reward users who queue the withdrawal and start the expiry cooldown, it creates a problematic situation when the market is closed with an outstanding expiry batch.

The problem is that up until the expiry timestamp comes, all new withdraw requests are added to this old batch where the rate of the previous requests drags the overall withdraw rate down.

Consider the following scenario:

A withdraw batch is created and its expiry time is 1 year.

6 months in, the withdraw batch has half of the markets value in it and the market is closed. The current rate is 1.12 and the batch is currently filled at 1.06 Now users have two choices - to either withdraw their funds now at ~1.06 rate or wait 6 months to be able to withdraw their funds at 1.12 rate.

This creates a very unpleasant situation as the users have an incentive to hold their funds within the contract, despite not providing any value.

Looked from slightly different POV, these early withdraw requesters force everyone else to lock their funds for additional 6 months, for the APY they should’ve usually received for just holding up until now.

## Recommended Mitigation Steps

After closing a market and filling the current expiry, delete it from pendingWithdrawalExpiry. Introduce a closedExpiry variable so you later make sure a future expiry is not made at that same timestamp to avoid collision.

d1ll0n (Wildcat) confirmed and commented:

Thanks for this, good find! Will adopt the proposed solution and see if it fixes H-01.

laurenceday (Wildcat) commented:

Fixed with wildcat-finance/v2-protocol@b25e528.

# [M-02] FixedTermLoanHooks allow Borrower to update Annual Interest before end of the “Fixed Term Period”

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-the-wildcat-protocol
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/final_report.html

FixedTermLoanHooks allow Borrower to update Annual Interest before end of the “Fixed Term Period” Submitted by Infect3d, also found by 0xpiken and falconhoof Summary While the documentation states that in case of ‘fixed term’ market the APR cannot be changed until the term ends, nothing prevents this in FixedTermLoanHooks.

## Vulnerability details

In Wildcat markets, lenders know in advance how much APR the borrower will pay them. In order to allow lenders to exit the market swiftly, the market must always have at least a reserve ratio of the lender funds ready to be withdrawn.

If the borrower decides to reduce the APR, in order to allow lenders to ‘ragequit’, a new reserve ratio is calculated based on the variation of the APR as described in the link above.

Finally, is a market implement a fixed term (date until when withdrawals are not possible), it shouldn’t be able to reduce the APR, as this would allow the borrower to ‘rug’ the lenders by reducing the APR to 0% while they couldn’t do anything against that.

The issue here is that while lenders are (as expected) prevented to withdraw before end of term:

- https://github.com/code-423n4/2024-08-wildcat/blob/main/src/access/FixedTermLoanHooks.sol#L857-L859
File:

src / access / FixedTermLoanHooks.

sol 848:

function onQueueWithdrawal ( 849:

address lender, 850:

uint32 /* expiry */, 851:

uint /* scaledAmount */, 852:

MarketState calldata /* state */, 853:

bytes calldata hooksData 854: ) external override { 855:

HookedMarket memory market = _hookedMarkets [ msg.

sender ]; 856:

if (!

market.

isHooked ) revert NotHookedMarket (); 857:

if ( market.

fixedTermEndTime > block.

timestamp ) { 858:

revert WithdrawBeforeTermEnd (); 859: } this is not the case for the borrower setting the annual interest:

- https://github.com/code-423n4/2024-08-wildcat/blob/main/src/access/FixedTermLoanHooks.sol#L960-L978
File:

src / access / FixedTermLoanHooks.

sol 960:

function onSetAnnualInterestAndReserveRatioBips ( 961:

uint16 annualInterestBips, 962:

uint16 reserveRatioBips, 963:

MarketState calldata intermediateState, 964:

bytes calldata hooksData 965: ) 966:

public 967:

virtual 968:

override 969:

returns ( uint16 updatedAnnualInterestBips, uint16 updatedReserveRatioBips ) 970: { 971:

return 972:

super.

onSetAnnualInterestAndReserveRatioBips ( 973:

annualInterestBips, 974:

reserveRatioBips, 975:

intermediateState, 976:

hooksData 977: ); 978: } 979:

## Impact

Borrower can rug the lenders by reducing the APR while they cannot quit the market.

## Recommended Mitigation Steps

When FixedTermLoanHooks::onSetAnnualInterestAndReserveRatioBips is called, revert if market.fixedTermEndTime > block.timestamp.

laurenceday (Wildcat) disputed and commented via duplicate issue #23:

This is a valid finding, thank you - an embarrassing one for us at that, we clearly just missed this when writing the hook templates!

However, we’re a bit torn internally on whether this truly classifies as a High. We’ve definitely specified in documentation that this is a rug pull mechanic, but there are no funds directly or indirectly at risk here, unless you classify the potential of earning less than expected when you initially deposited as falling in that bucket.

So, we’re going to kick this one to the judge: does earning 10,000 on a 100,000 deposit rather than 15,000 count as funds at risk if there’s no way to ragequit for the period of time where that interest should accrue? Or is this more of a medium wherein protocol functionality is impacted?

It’s definitely a goof on our end, and we’re appreciative that the warden caught it, so thank you. With that said, we’re trying to be fair to you (the warden) while also being fair to everyone else that’s found things. This is a very gentle dispute for the judge to handle: sadly the ‘disagree with severity’ tag isn’t available to us anymore!

3docSec (judge) decreased severity to Medium and commented via duplicate issue #23:

Hi @laurenceday thanks for adding context.

“So, we’re going to kick this one to the judge: does earning 10,000 on a 100,000 deposit rather than 15,000 count as funds at risk if there’s no way to ragequit for the period of time where that interest should accrue? Or is this more of a medium wherein protocol functionality is impacted?” I consider this a Medium issue: because it’s only future “interest” gains that are at risk, I see this more like an availability issue where the lender’s funds are locked at conditions they didn’t necessarily sign up for; the problem is the locking (as you said if there was a ragequit option, it would be a different story).

I admit this is a subjective framing, but at the same time, it’s consistent with how severity is assessed in bug bounty programs, where missing out on future returns generally has lower severity than having present funds at risk.

# [M-03] Inconsistency across multiple repaying functions causing lender to pay extra fees

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-the-wildcat-protocol
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/final_report.html

Submitted by deadrxsezzz, also found by Bigsam, 0xNirix, Udsen, Takarez, and Infect3d Within functions such as repay and repayAndProcessUnpaidWithdrawalBatches, funds are first pulled from the user in order to use them towards the currently expired, but not yet unpaid batch, and then the updated state is fetched.

function repay ( uint256 amount ) external nonReentrant sphereXGuardExternal { if ( amount == 0 ) revert_NullRepayAmount (); asset.

safeTransferFrom ( msg.

sender, address ( this ), amount ); emit_DebtRepaid ( msg.

sender, amount ); MarketState memory state = _getUpdatedState (); if ( state.

isClosed ) revert_RepayToClosedMarket (); // Execute repay hook if enabled hooks.

onRepay ( amount, state, _runtimeConstant ( 0x24 )); _writeState ( state ); } However, this is not true for functions such as closeMarket, deposit, repayOutstandingDebt and repayDelinquentDebt, where the state is first fetched and only then funds are pulled, forcing borrower into higher fees.

function closeMarket () external onlyBorrower nonReentrant sphereXGuardExternal { MarketState memory state = _getUpdatedState (); // fetches updated state if ( state.

isClosed ) revert_MarketAlreadyClosed (); uint256 currentlyHeld = totalAssets (); uint256 totalDebts = state.

totalDebts (); if ( currentlyHeld < totalDebts ) { // Transfer remaining debts from borrower uint256 remainingDebt = totalDebts - currentlyHeld; _repay ( state, remainingDebt, 0x04 ); // pulls user funds currentlyHeld += remainingDebt; This inconsistency will cause borrowers to pay extra fees which they otherwise wouldn’t.

## Recommended Mitigation Steps

Always pull the funds first and refund later if needed.

d1ll0n (Wildcat) acknowledged and commented:

The listed functions which incur higher fees all require the current state of the market to accurately calculate relevant values to the transfer. Because of that, the transfer can’t happen until after the state is updated, and it would be expensive (and too large to fit in the contract size) to redo the withdrawal payments post-transfer.

For the repay functions this is more of an issue than the others, as that represents the borrower specifically taking action to repay their debts, whereas the other functions are actions by other parties (and thus we aren’t very concerned if they fail to cure the borrower’s delinquency for them). We may end up just removing these secondary repay functions.

laurenceday (Wildcat) commented:

Resolved by wildcat-finance/v2-protocol@e7afdc9.

# [M-04] FixedTermLoanHook looks at block.timestamp instead of expiry

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-the-wildcat-protocol
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/final_report.html

FixedTermLoanHook looks at block.timestamp instead of expiry Submitted by deadrxsezzz The idea of FixedTermLoanHook is to only allow for withdrawals after a certain term end time. However, the problem is that the current implementation does not look at the expiry, but instead at the block.timestamp.

function onQueueWithdrawal ( address lender, uint32 /* expiry */, uint /* scaledAmount */, MarketState calldata /* state */, bytes calldata hooksData ) external override { HookedMarket memory market = _hookedMarkets [ msg.

sender ]; if (!

market.

isHooked ) revert NotHookedMarket (); if ( market.

fixedTermEndTime > block.

timestamp ) { revert WithdrawBeforeTermEnd (); } This creates inconsistencies such as forcing users not only to wait until term’s end, but also having to wait an extra withdrawalBatchDuration before they’re able to withdraw their funds.

## Recommended Mitigation Steps

Check the expiry instead of block.timestamp.

d1ll0n (Wildcat) confirmed laurenceday (Wildcat) acknowledged and commented:

We’ve reflected on this a little bit, and decided that we want to turn this from a confirmed into an acknowledge.

The reasoning goes as follows:

Imagine that a fixed market has an expiry of December 30th, but there’s a withdrawal cycle of 7 days.

Presumably the borrower is anticipating [and may have structured things] such that they are expecting to be able to make full use of any credit extended to them until then, and not a day sooner.

Fixing this in the way suggested would permit people to place withdrawal requests on December 23rd, with the potential to tip a market into delinquent status (depending on grace period configuration) before the fixed duration has actually met.

Net-net we think it makes more sense to allow the market to revert back to a perpetual after that expiry and allow withdrawal requests to be processed per the conditions. The expectation here would be that the withdrawal cycle would actually be quite short.

Infect3d (warden) commented:

May I comment on this issue: can we really consider this a bug rather than a feature and a design improvement, also considering sponsor comment?

Expiry mechanism is known by borrower and lender, so if borrower wants lenders to be able to withdraw on time, he can simply configure fixedTermEndTime = value - withdrawalBatchDuration.

3docSec (judge) commented:

Hi @Infect3d - I agree we are close to an accepted trade-off territory. Here I lean on the sponsor who very transparently made it clear this trade-off is not something they had deliberately thought of.

Therefore, because the impact is compatible with Medium severity, “satisfactory Medium” plus “sponsor acknowledged” is a fair way of categorizing this finding.

# [M-05] Role providers can bypass intended restrictions and lower expiry set by other providers

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-the-wildcat-protocol
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/final_report.html

Submitted by deadrxsezzz, also found by 0x1771 and gesha17

- https://github.com/code-423n4/2024-08-wildcat/blob/main/src/access/FixedTermLoanHooks.sol#L413
If we look at the code comments, we’ll see that role providers can update a user’s credential only if at least one of the 3 is true:

the previous credential’s provider is no longer supported, OR the caller is the previous role provider, OR the new expiry is later than the current expiry /** * @dev Grants a role to an account by updating the account's status.

* Can only be called by an approved role provider.

* * If the account has an existing credential, it can only be updated if:

* - the previous credential's provider is no longer supported, OR * - the caller is the previous role provider, OR * - the new expiry is later than the current expiry */ function grantRole ( address account, uint32 roleGrantedTimestamp ) external { RoleProvider callingProvider = _roleProviders [ msg.

sender ]; if ( callingProvider.

isNull ()) revert ProviderNotFound (); _grantRole ( callingProvider, account, roleGrantedTimestamp ); } This means that a role provider should not be able to reduce a credential set by another role provider.

However, this could easily be bypassed by simply splitting the call into 2 separate ones:

First one to set the expiry slightly later than the currently set one. This would set the role provider to the new one.

Second call to reduce the expiry as much as they’d like. Since they’re the previous provider they can do that.

## Recommended Mitigation Steps

Fix is non-trivial.

d1ll0n (Wildcat) disputed and commented:

This is a useful note to be aware of, but I’d categorize it low/informational as role providers are inherently trusted entities. The likelihood and impact of this kind of attack are pretty minimal.

3docSec (judge) commented:

There are a few factors to be considered:

it is a valid privilege escalation vector the attacker has to be privileged already the attack can have a direct impact on a lender While the first two have me on the fence when choosing between Medium and Low severity, the third point is a tiebreaker towards Medium.

If we stick to the C4 severity categorization, I see a good fit with the Medium definition:

“the function of the protocol or its availability could be impacted […] with a hypothetical attack path with stated assumptions”

# [M-06] No lender is able to exit even after the market is closed

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-the-wildcat-protocol
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/final_report.html

Submitted by 0xpiken, also found by josephxander and 0xNirix When a borrower creates a market hooked by a fixed-term hook, all lenders are prohibited from withdrawing their assets from the market before the fixed-term time has elapsed.

The borrower can close the market at any time. However, fixedTermEndTime of the market is not updated, preventing lenders from withdrawing their assets if fixedTermEndTime has not yet elapsed.

Copy below codes to WildcatMarket.t.sol and run forge test —match-test test closeMarket BeforeFixedTermExpired:

function test_closeMarket_BeforeFixedTermExpired () external { //@audit-info deploy a FixedTermLoanHooks template address fixedTermHookTemplate = LibStoredInitCode.

deployInitCode ( type ( FixedTermLoanHooks ).

creationCode ); hooksFactory.

addHooksTemplate ( fixedTermHookTemplate, 'FixedTermLoanHooks', address ( 0 ), address ( 0 ), 0, 0 ); vm.

startPrank ( borrower ); //@audit-info borrower deploy a FixedTermLoanHooks hookInstance address hooksInstance = hooksFactory.

deployHooksInstance ( fixedTermHookTemplate, '' ); DeployMarketInputs memory parameters = DeployMarketInputs ({ asset:

address ( asset ), namePrefix:

'name', symbolPrefix:

'symbol', maxTotalSupply:

type ( uint128 ).

max, annualInterestBips:

1000, delinquencyFeeBips:

1000, withdrawalBatchDuration:

10000, reserveRatioBips:

10000, delinquencyGracePeriod:

10000, hooks:

EmptyHooksConfig.

setHooksAddress ( address ( hooksInstance )) }); //@audit-info borrower deploy a market hooked by a FixedTermLoanHooks hookInstance address market = hooksFactory.

deployMarket ( parameters, abi.

encode ( block.

timestamp + ( 365 days )), bytes32 ( uint ( 1 )), address ( 0 ), 0 ); vm.

stopPrank (); //@audit-info lenders can only withdraw their asset one year later assertEq ( FixedTermLoanHooks ( hooksInstance ).

getHookedMarket ( market ).

fixedTermEndTime, block.

timestamp + ( 365 days )); //@audit-info alice deposit 50K asset into market vm.

startPrank ( alice ); asset.

approve ( market, type ( uint ).

max ); WildcatMarket ( market ).

depositUpTo ( 50_000e18 ); vm.

stopPrank (); //@audit-info borrower close market in advance vm.

prank ( borrower ); WildcatMarket ( market ).

closeMarket (); //@audit-info the market is closed assertTrue ( WildcatMarket ( market ).

isClosed ()); //@audit-info however, alice can not withdraw her asset due to the unexpired fixed term.

vm.

expectRevert ( FixedTermLoanHooks.

WithdrawBeforeTermEnd.

selector ); vm.

prank ( alice ); WildcatMarket ( market ).

queueFullWithdrawal (); }

## Recommended Mitigation Steps

When a market hooked by a fixed-term hook is closed, fixedTermEndTime should be set to block.timestamp if it has not yet elapsed:

constructor(address _deployer, bytes memory /* args */) IHooks() { borrower = _deployer; // Allow deployer to grant roles with no expiry _roleProviders[_deployer] = encodeRoleProvider( type(uint32).max, _deployer, NotPullProviderIndex ); HooksConfig optionalFlags = encodeHooksConfig({ hooksAddress: address(0), useOnDeposit: true, useOnQueueWithdrawal: false, useOnExecuteWithdrawal: false, useOnTransfer: true, useOnBorrow: false, useOnRepay: false, useOnCloseMarket: false, useOnNukeFromOrbit: false, useOnSetMaxTotalSupply: false, useOnSetAnnualInterestAndReserveRatioBips: false, useOnSetProtocolFeeBips: false }); HooksConfig requiredFlags = EmptyHooksConfig.setFlag(Bit_Enabled_SetAnnualInterestAndReserveRatioBips)

+.setFlag(Bit_Enabled_CloseMarket);.setFlag(Bit_Enabled_QueueWithdrawal); config = encodeHooksDeploymentConfig(optionalFlags, requiredFlags); } function onCloseMarket( MarketState calldata /* state */, bytes calldata /* hooksData */ - ) external override {} + ) external override { + HookedMarket memory market = _hookedMarkets[msg.sender]; + if (!market.isHooked) revert NotHookedMarket(); + if (market.fixedTermEndTime > block.timestamp) { + _hookedMarkets[msg.sender].fixedTermEndTime = uint32(block.timestamp); + } laurenceday (Wildcat) confirmed and commented:

This is a great catch.

laurenceday (Wildcat) commented:

Fixed by wildcat-finance/v2-protocol@05958e3.

# [M-07] Role providers cannot be EOAs as stated in the documentation

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-the-wildcat-protocol
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/final_report.html

Submitted by pfapostol, also found by Infect3d Lines of code

- https://github.com/code-423n4/2024-08-wildcat/blob/fe746cc0fbedc4447a981a50e6ba4c95f98b9fe1/src/access/AccessControlHooks.sol#L220
- https://github.com/code-423n4/2024-08-wildcat/blob/fe746cc0fbedc4447a981a50e6ba4c95f98b9fe1/src/access/FixedTermLoanHooks.sol#L254

## Impact

The Documentation suggests that a role provider can be a “push” provider (one that “pushes” credentials into the hooks contract by calling grantRole ) and a “pull” provider (one that the hook calls via getCredential or validateCredential ).

The documentation also states that:

Role providers do not have to implement any of these functions - a role provider can be an EOA.

But in fact, only the initial deployer can be an EOA provider, since it is coded in the constructor. Any other EOA provider that the borrower tries to add via addRoleProvider will fail because it does not implement the interface.

## Recommended Mitigation Steps

Replace the interface call with a low-level call and check if the user implements the interface in order to be a pull provider:

( bool succes, bytes memory data ) = providerAddress.

call ( abi.

encodeWithSelector ( IRoleProvider.

isPullProvider.

selector )); bool isPullProvider; if ( succes && data.

length == 0x20 ) { isPullProvider = abi.

decode ( data, ( bool )); } else { isPullProvider = false; } With this code all logic works as expected, for EOA providers pullProviderIndex is set to type(uint24).max, for contracts - depending on the result of calling isPullProvider:

Traces:

[ 141487 ] AuditMarket::test_PoC_EOA_provider() ├─ [ 0 ] VM::startPrank(BORROWER1: [ 0xB193AC639A896a0B7a0B334a97f0095cD87427f2 ]) │ └─ ← [ Return ] ├─ [ 30181 ] AccessControlHooks::addRoleProvider(RoleProvider: [ 0x2e234DAe75C793f67A35089C9d99245E1C58470b ], 2592000 [ 2.592e6 ]) │ ├─ [ 2275 ] RoleProvider::isPullProvider() │ │ └─ ← [ Return ] false │ ├─ emit RoleProviderAdded(providerAddress: RoleProvider: [ 0x2e234DAe75C793f67A35089C9d99245E1C58470b ], timeToLive:

2592000 [ 2.592e6 ], pullProviderIndex:

16777215 [ 1.677e7 ]) │ └─ ← [ Stop ] ├─ [ 74541 ] AccessControlHooks::addRoleProvider(RoleProvider: [ 0xF62849F9A0B5Bf2913b396098F7c7019b51A820a ], 2592000 [ 2.592e6 ]) │ ├─ [ 2275 ] RoleProvider::isPullProvider() │ │ └─ ← [ Return ] true │ ├─ emit RoleProviderAdded(providerAddress: RoleProvider: [ 0xF62849F9A0B5Bf2913b396098F7c7019b51A820a ], timeToLive:

2592000 [ 2.592e6 ], pullProviderIndex:

0 ) │ └─ ← [ Stop ] ├─ [ 27653 ] AccessControlHooks::addRoleProvider(EOA_PROVIDER1: [ 0x6aAfF89c996cAa2BD28408f735Ba7A441276B03F ], 2592000 [ 2.592e6 ]) │ ├─ [ 0 ] EOA_PROVIDER1::isPullProvider() │ │ └─ ← [ Stop ] │ ├─ emit RoleProviderAdded(providerAddress: EOA_PROVIDER1: [ 0x6aAfF89c996cAa2BD28408f735Ba7A441276B03F ], timeToLive:

2592000 [ 2.592e6 ], pullProviderIndex:

16777215 [ 1.677e7 ]) │ └─ ← [ Stop ] └─ ← [ Stop ] laurenceday (Wildcat) commented:

Not really a medium in that it doesn’t ‘matter’ for the most part: this is sort of a documentation issue in that we’d never really expect an EOA that wasn’t the borrower (which is an EOA provider) to be a role provider.

It’s vanishingly unlikely that a borrower is going to add some random arbiter that they don’t control - possible that they add another address that THEY control but in that case they might as well use the one that’s known to us.

Disputing, but with a light touch: we consider this a useful QA.

3docSec (judge) commented:

Thanks for the context. If we ignore the documentation, the fact that the initial role provider is the borrower, and the NotPullProviderIndex value that is used in this case makes it clear that the intention is allowing for EOAs to be there.

While not the most requested feature, it’s something a borrower may want to do, and given the above, may reasonably expect to see working. For this reason, I think a Medium is reasonable because we have marginal I admit, but still tangible, availability impact.

# [M-08] AccessControlHooks onQueueWithdrawal() does not check if market is hooked which could lead to unexpected errors such as temporary DoS

- **Contest:** The Wildcat Protocol
- **Slug:** 2024-08-the-wildcat-protocol
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-the-wildcat-protocol
- **Source snapshot:** competitions/2024-08-the-wildcat-protocol/final_report.html

AccessControlHooks onQueueWithdrawal() does not check if market is hooked which could lead to unexpected errors such as temporary DoS Submitted by gesha17, also found by falconhoof, kutugu, and Infect3d

## Impact

The onQueueWithdrawal() function does not check if the caller is a hooked market, meaning anyone can call the function and attempt to verify credentials on a lender. This results in calls to registered pull providers with arbitrary hookData, which could lead to potential issues such as abuse of credentials that are valid for a short term, e.g. 1 block.

## Recommended Mitigation Steps

Require the caller to be a registered hooked market, same as onQueueWithdrawal() in FixedTermloanHooks 3docSec (judge) commented via duplicate issue #83:

I find this group compatible with the Medium severity for the following reasons:

access to a lender’s signature is very feasible in the frontrunning scenario depicted in this finding the hypothesis on validateCredential isn’t really a speculation but rather a very reasonable implementation, one that was also assumed in the previous audit (finding number 2).

laurenceday (Wildcat) acknowledged and commented:

We don’t consider this a real issue, in that we’ve always wanted it to be possible for anyone to call the validate function to poke a credential update. This finding assumes that you have the signature someone else would be using as a credential and generally relies on a specific implementation of the provider that doesn’t actually exist, so there’s no need to check isHooked.

It’s been upgraded to a Medium, and we’re not going to argue with this at this stage. As such, we’re acknowledging rather than confirming or disputing simply to put a cap on the report.
