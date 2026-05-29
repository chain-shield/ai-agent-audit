# Benchmark Ground Truth: Coded Estate Invitational

## Accepted H/M Findings

# Accepted H/M Findings: Coded Estate Invitational

# [H-01] Attakers can steal the funds from long-term reservation

- **Contest:** Coded Estate Invitational
- **Slug:** 2024-10-coded-estate-invitational
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-10-coded-estate-invitational
- **Source snapshot:** competitions/2024-10-coded-estate-invitational/final_report.html

Submitted by Ch_301 In this protocol NFT owner can set the NFT in sale even if it is still under active rent by triggering execute.rs#setlistforsell() which could set token.sell.auto_approve to a true value (means anyone can directly be approved and this will open multiple doors for attackers).

Users can call execute.rs#setbidtobuy() and send the necessary amount to gain approval of this NFT:

File: execute.rs# setbidtobuy () 675:

if token.sell.auto_approve { 676:

// update the approval list (remove any for the same spender before adding) 677:

let expires = Expiration::Never { }; 678: token.approvals.

retain (|apr| apr.spender != info.sender); 679:

let approval = Approval { 680: spender: info.sender.

clone (), 681: expires, 682: }; 683: token.approvals.

push (approval); 684:

685: } Using the same function setbidtobuy() any address that has an existing bid in the NFT can cancel its bid and receive back all the initial funds (no fees in this function).

On the other side, the owner or any approved address can invoke execute.rs#withdrawtolandlord() and specify the receiver of the withdrawal funds (this function gives the homeowners the ability to withdraw a part of the funds even before the rent end, this is only for longterm rentals).

File: execute.rs 1787:

pub fn withdrawtolandlord ( /**CODE**/ 1796: address:

String 1797: ) -> Result <Response<C>, ContractError> { /**CODE**/ 1848:.

add_message (BankMsg::

Send { 1849: to_address: address, 1850: amount:

vec!

[Coin { 1851: denom: token.longterm_rental.denom, 1852: amount: Uint128::

from (amount) - Uint128::

new (( u128::

from (amount) * u128::

from (fee_percentage)) / 10000 ), However, the Attacker can create a sophisticated attack using withdrawtolandlord() and setbidtobuy():

Choose an NFT that has a token.sell.auto_approve == true and an active long-term rental.

Call setbidtobuy() this will give him the necessary approval to finish the attack; he also needs to transfer the asked funds.

Trigger withdrawtolandlord() and transfer the maximum amount of tokens.

File: execute.rs# withdrawtolandlord () 1832:

if item.deposit_amount - Uint128::

from (token.longterm_rental.price_per_month) < Uint128::

from (amount) { 1833:

return Err(ContractError::UnavailableAmount { }); 1834: } Invoke setbidtobuy() to receive his original deposited funds.

## Impact

Steal the funds from long-term reservations using setbidtobuy().

## Recommended Mitigation Steps

File: execute.rs 1787: pub fn withdrawtolandlord( 1788: &self, 1789: deps: DepsMut, 1790: env: Env, 1791: info: MessageInfo, 1792: token_id: String, 1793: tenant: String, 1794: renting_period: Vec<String>, 1795: amount:u64, 1796: address:String 1797: ) -> Result<Response<C>, ContractError> { 1798: let mut token = self.tokens.load(deps.storage, &token_id)?; 1799:

-1800: self.check_can_send(deps.as_ref(), &env, &info, &token)?; +1800: self.check_can_approve(deps.as_ref(), &env, &info, &token)?;

## Assessed type

Invalid Validation blockchainstar12 (Coded Estate) acknowledged

# [H-02] setbidtobuy allows token purchase even when sale is no longer listed

- **Contest:** Coded Estate Invitational
- **Slug:** 2024-10-coded-estate-invitational
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-10-coded-estate-invitational
- **Source snapshot:** competitions/2024-10-coded-estate-invitational/final_report.html

setbidtobuy allows token purchase even when sale is no longer listed Submitted by nnez, also found by adeolu The bug allows buyers to purchase tokens that have been delisted by the seller, bypassing the seller’s intent to halt the sale. This can result in tokens being sold against the seller’s wishes.

Description The setbidtobuy function is responsible for allowing buyers to submit bids to purchase a token listed for sale. A seller can invoke setlistforsell to list a token, specifying the price, payment token (denom), and whether the sale is auto-approved. If auto-approve is set to true, any buyer who calls setbidtobuy can acquire the token without further input from the seller, while a manual approval is required when auto-approve is set to false.

However, there is a flaw in the logic of setbidtobuy —it does not check the sell.islisted flag, which is supposed to indicate whether a token is still available for sale. Even if the seller later decides to delist the token by setting sell.islisted to false, buyers can still invoke setbidtobuy and proceed with the purchase if auto-approve is enabled. This creates a scenario where sellers lose control over the sale, allowing unintended buyers to purchase delisted tokens.

Example Scenario:

A seller lists a token using setlistforsell, specifying the sale details including price, payment token, and setting auto-approve to true.

After some time, the seller receives no bids and decides to delist the token, changing sell.islisted to false while leaving other parameters unchanged.

A buyer invokes setbidtobuy, and because the function does not respect the islisted flag and auto-approve is true, the token is sold despite the seller’s intent to delist it. This results in an unintended sale, leading to potential loss or misuse of assets by the seller.

An action of delisting the token on sale in this manner is justified because there is no other functions serving this purpose as in short-term rental and long-term rental where there is a specific function to unlist the token from rental service.

Code Snippet The following snippet shows that the islisted flag is not verified in setbidtobuy, which allows unintended purchases:

pub fn setlistforsell ( & self, deps: DepsMut, env: Env, info: MessageInfo, islisted:

bool, token_id:

String, denom:

String, price:

u64, auto_approve:

bool, ) -> Result <Response<C>, ContractError> { let mut token = self.tokens.

load (deps.storage, &token_id)?; // ensure we have permissions self.

check_can_approve (deps.

as_ref (), &env, &info, &token)?; // @c4-contest islisted indicates whether token is available for sale or not token.sell.islisted = Some(islisted); token.sell.price = price; token.sell.auto_approve = auto_approve; token.sell.denom = denom; self.tokens.

save (deps.storage, &token_id, &token)?; Ok(Response::

new ().

add_attribute ( "action", "setlistforsell" ).

add_attribute ( "sender", info.sender).

add_attribute ( "token_id", token_id)) } pub fn setbidtobuy ( // function arguments ) -> Result <Response<C>, ContractError> { let mut token = self.tokens.

load (deps.storage, &token_id)?; // @c4-contest: no check for the value of sell.islisted flag let mut position:

i32 = - 1; let mut amount = Uint128::

from ( 0u64 ); for (i, item) in token.bids.

iter ().

enumerate () { if item.address == info.sender.

to_string () { position = i as i32; amount = item.offer.

into (); break; } if position == - 1 { if info.funds[ 0 ].denom != token.sell.denom { return Err(ContractError::InvalidDeposit {}); } if info.funds[ 0 ].amount < Uint128::

from (token.sell.price) { return Err(ContractError::InsufficientDeposit {}); } if token.sell.auto_approve { // update the approval list (remove any for the same spender before adding) let expires = Expiration::Never { }; token.approvals.

retain (|apr| apr.spender != info.sender); let approval = Approval { spender: info.sender.

clone (), expires, }; token.approvals.

push (approval); } let bid = Bid { address: info.sender.

to_string (), offer:info.funds[ 0 ].amount, }; token.bids.

push (bid); } else { // update the approval list (remove any for the same spender before adding) token.bids.

retain (|item| item.address != info.sender); } self.tokens.

save (deps.storage, &token_id, &token)?; if position != - 1 && (amount > Uint128::

from ( 0u64 )) { Ok(Response::

new ().

add_attribute ( "action", "setbidtobuy" ).

add_attribute ( "sender", info.sender.

clone ()).

add_attribute ( "token_id", token_id).

add_message (BankMsg::

Send { to_address: info.sender.

to_string (), amount:

vec!

[Coin { denom: token.sell.denom, amount: amount, }], })) } else { Ok(Response::

new ().

add_attribute ( "action", "setbidtobuy" ).

add_attribute ( "sender", info.sender).

add_attribute ( "token_id", token_id)) } This lack of validation enables buyers to acquire delisted tokens without the seller’s consent.

## Recommended Mitigation

Disallow buying token with sell.islisted flag set to false/none.

## Assessed type

Context blockchainstar12 (Coded Estate) acknowledged Lambda (judge) increased severity to High

# [H-03] Insufficient price validation in transfer_nft function enables theft of listed tokens

- **Contest:** Coded Estate Invitational
- **Slug:** 2024-10-coded-estate-invitational
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-10-coded-estate-invitational
- **Source snapshot:** competitions/2024-10-coded-estate-invitational/final_report.html

transfer_nft function enables theft of listed tokens Submitted by nnez, also found by Ch_301 This vulnerability allows malicious buyers to acquire listed NFTs without payment to sellers.

Description Users can list their tokens for sale by calling setlistosell and specifying a price and payment token (denom). Buyers can then purchase the token by calling setbidtobuy and transferring the payment into the contract.

The trade is finalized when transfer_nft is invoked and the recipient is the buyer. The caller can be the seller, or, if auto_approve is set to true, the caller can also be the buyer as they’re given approval upon calling setbidtobuy.

However, transfer_nft function lacks a proper validation during the transfer. This vulnerability stems from two key oversights:

The function doesn’t verify if the offer bid amount matches the listed price of the token.

It allows caller to freely specify recipient and transfer to recipients with no active bids, defaulting to a zero payment.

These oversights enable malicious buyers to acquire NFTs without paying the listed price, effectively stealing them from sellers.

transfer_nft implementation:

fn transfer_nft ( & self, deps: DepsMut, env: Env, info: MessageInfo, recipient:

String, // @c4-contest caller of this function can freely specify `recipient` address token_id:

String, ) -> Result <Response<C>, ContractError> { let mut token = self.tokens.

load (deps.storage, &token_id)?; // ensure we have permissions self.

check_can_send (deps.

as_ref (), &env, &info, &token)?; // set owner and remove existing approvals let prev_owner = token.owner; token.owner = deps.api.

addr_validate (&recipient)?; // @c4-contest ownership is transferred to recipient token.approvals = vec!

[]; let fee_percentage = self.

get_fee (deps.storage)?; let mut position:

i32 = - 1; let mut amount = Uint128::

from ( 0u64 ); // @c4-contest: amount is default to zero for (i, item) in token.bids.

iter ().

enumerate () { if item.address == recipient.

to_string () { position = i as i32; amount = item.offer.

into (); break; } // @c4-contest: if recipient doesn't have bid on this token, amount is default to zero if position != - 1 && amount > Uint128::

new ( 0 ) { self.

increase_balance (deps.storage, token.sell.denom.

clone (), Uint128::

new (( u128::

from (amount) * u128::

from (fee_percentage)) / 10000 ))?; } let amount_after_fee = amount.

checked_sub (Uint128::

new (( u128::

from (amount) * u128::

from (fee_percentage)) / 10000 )).

unwrap_or_default (); token.bids.

retain (|bid| bid.address != recipient); self.tokens.

save (deps.storage, &token_id, &token)?; // @c4-contest: no validation whether the bid amount matches with the listed price.

if amount > Uint128::

new ( 0 ) { Ok(Response::

new ().

add_attribute ( "action", "transfer_nft" ).

add_attribute ( "sender", info.sender.

clone ()).

add_attribute ( "token_id", token_id).

add_message (BankMsg::

Send { to_address: prev_owner.

to_string (), amount:

vec!

[Coin { denom: token.sell.denom, amount: amount_after_fee, }], })) } else { // @c4-contest: if amount is zero, the transfer go through with no payment to seller Ok(Response::

new ().

add_attribute ( "action", "transfer_nft" ).

add_attribute ( "sender", info.sender.

clone ()).

add_attribute ( "token_id", token_id)) } This vulnerability can be exploited in two scenarios:

Auto-approve enabled - When auto_approve is set to true, a buyer can exploit the system by:

Calling setbidtobuy to gain approval.

Invoking transfer_nft with a different recipient address that has no active bid.

Cancelling their original bid for a full refund.

Auto-approve disabled - Even when auto_approve is false, an attacker can:

Place a bid on the token.

Front-run the seller’s transfer_nft transaction, cancelling their bid.

The seller’s transaction is executed after, transferring the token without payment.

## Recommended Mitigation

If token is listed for sell, check that the offer bid amount is exactly matched with the listed price set by seller.

if token.sell.isListed { if amount < token.sell.price{ // throw error } else { // proceed to complete the trade } else { // do normal transfer }

## Assessed type

Invalid Validation blockchainstar12 (Coded Estate) confirmed

# [H-04] Lack of differentiation between rental types leads to loss of funds

- **Contest:** Coded Estate Invitational
- **Slug:** 2024-10-coded-estate-invitational
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-10-coded-estate-invitational
- **Source snapshot:** competitions/2024-10-coded-estate-invitational/final_report.html

Submitted by nnez, also found by Ch_301 ( 1, 2, 3 )

- https://github.com/code-423n4/2024-10-coded-estate/blob/main/contracts/codedestate/src/execute.rs#L1413
- https://github.com/code-423n4/2024-10-coded-estate/blob/main/contracts/codedestate/src/execute.rs#L870

## Impact

This vulnerability allows an attacker to exploit the lack of distinction between short-term and long-term rental types to withdraw funds in a different, more valuable token than the one initially used for payment, effectively steal other users’ funds deposited in the contract.

Description In the CodedEstate system, a property (token) can be listed for both short-term and long-term rentals, with each rental type having separate configurations; including the denomination ( denom ) of the token used for payments. The rental information for both types of rentals is stored in the same vector, rentals, and a rental_type flag is used within the Rental struct to differentiate between short-term ( false ) and long-term ( true ) rentals.

File: packages/cw721/src/query.rs pub struct Rental { pub denom:

String, pub deposit_amount: Uint128, pub rental_type:

bool, // @c4-contest: differentiates between short-term (false) and long-term (true) rentals pub cancelled:

bool, pub renting_period:

Vec < u64 >, pub address:

Option <Addr>, pub approved:

bool, pub approved_date:

Option < String >, pub guests:

usize, } File: contracts/codedestate/src/execute.rs pub struct TokenInfo <T> { pub owner: Addr, pub approvals:

Vec <Approval>, pub longterm_rental: LongTermRental, // long-term rental agreement pub shortterm_rental: ShortTermRental, // short-term rental agreement pub rentals:

Vec <Rental>, // @c4-contest: both types of rental are saved in this vector pub bids:

Vec <Bid>, pub sell: Sell, pub token_uri:

Option < String >, pub extension: T, } However, the contract does not make use of the rental_type flag in any function that handles rental operations. As a result, functions designated for short-term rentals can be used for long-term rentals, and vice versa, without proper validation of the rental type. This becomes problematic, especially since short-term and long-term rentals may use different denom tokens.

File: contracts/codedestate/src/execute.rs pub fn setlistforshorttermrental ( // function arguments ) -> Result <Response<C>, ContractError> { let mut token = self.tokens.

load (deps.storage, &token_id)?; // ensure we have permissions self.

check_can_approve (deps.

as_ref (), &env, &info, &token)?; self.

check_can_edit_short (&env, &token)?; token.shortterm_rental.islisted = Some( true ); token.shortterm_rental.price_per_day = price_per_day; token.shortterm_rental.available_period = available_period; token.shortterm_rental.auto_approve = auto_approve; token.shortterm_rental.denom = denom; // @c4-contest <-- can be a different denom from long-term rental token.shortterm_rental.minimum_stay = minimum_stay; token.shortterm_rental.cancellation = cancellation; self.tokens.

save (deps.storage, &token_id, &token)?; Ok(Response::

new ().

add_attribute ( "action", "setlistforshorttermrental" ).

add_attribute ( "sender", info.sender).

add_attribute ( "token_id", token_id)) } pub fn setlistforlongtermrental ( // function arguments ) -> Result <Response<C>, ContractError> { let mut token = self.tokens.

load (deps.storage, &token_id)?; // ensure we have permissions self.

check_can_approve (deps.

as_ref (), &env, &info, &token)?; self.

check_can_edit_long (&env, &token)?; token.longterm_rental.islisted = Some( true ); token.longterm_rental.price_per_month = price_per_month; token.longterm_rental.available_period = available_period; token.longterm_rental.auto_approve = auto_approve; token.longterm_rental.denom = denom; // @c4-contest <-- can be a different denom from short-term rental token.longterm_rental.minimum_stay = minimum_stay; token.longterm_rental.cancellation = cancellation; self.tokens.

save (deps.storage, &token_id, &token)?; Ok(Response::

new ().

add_attribute ( "action", "setlistforlongtermrental" ).

add_attribute ( "sender", info.sender).

add_attribute ( "token_id", token_id)) } An attacker can exploit this by performing the following steps:

Supposed there are two legitimate tokens in on Nibiru chain (deployment chain), TokenX ~ $0.01 and USDC ~ $1.

List a short-term rental using a low-value token (e.g., TokenX).

List a long-term rental using a high-value token (e.g., USDC).

Reserve a short-term rental by paying in TokenX using short-term function setreservationforshortterm.

Cancel the short-term rental using the long-term rental’s cancellation function cancelreservationbeforeapprovalforlongterm, which refunds in USDC.

This results in the attacker receiving a refund in the higher-value token, effectively stealing funds from other users who deposited USDC.

pub fn setreservationforshortterm ( // function arguments ) -> Result <Response<C>, ContractError> {...... snipped...

// @c4-contest: token with shortterm_rental denom if info.funds[ 0 ].denom != token.shortterm_rental.denom { return Err(ContractError::InvalidDeposit {}); } let sent_amount = info.funds[ 0 ].amount; let fee_percentage = self.

get_fee (deps.storage)?; let rent_amount = token.shortterm_rental.price_per_day * (new_checkout_timestamp - new_checkin_timestamp)/( 86400 ); if sent_amount < Uint128::

from (rent_amount) + Uint128::

new (( u128::

from (rent_amount) * u128::

from (fee_percentage)) / 10000 ) { return Err(ContractError::InsufficientDeposit {}); } self.

increase_balance (deps.storage, info.funds[ 0 ].denom.

clone (), sent_amount - Uint128::

from (rent_amount))?; let traveler = Rental { denom:token.shortterm_rental.denom.

clone (), rental_type:

false, approved_date:None, deposit_amount: Uint128::

from (rent_amount), renting_period:

vec!

[new_checkin_timestamp, new_checkout_timestamp], address: Some(info.sender.

clone ()), approved: token.shortterm_rental.auto_approve, cancelled:

false, guests:guests, }; token.rentals.

insert (placetoreserve as usize, traveler); // @c4-contest: rental is saved into rentals vector...... snipped...

} pub fn cancelreservationbeforeapprovalforlongterm ( // function arguments ) -> Result <Response<C>, ContractError> { let mut token = self.tokens.

load (deps.storage, &token_id)?; let mut position:

i32 = - 1; let mut amount = Uint128::

from ( 0u64 ); let tenant_address = info.sender.

to_string (); // @c4-contest: rental is loaded from rentals vector for (i, item) in token.rentals.

iter ().

enumerate () { if item.address == Some(info.sender.

clone ()) && item.renting_period[ 0 ].

to_string () == renting_period[ 0 ] && item.renting_period[ 1 ].

to_string () == renting_period[ 1 ] { if item.approved_date.

is_some () { return Err(ContractError::ApprovedAlready {}); } else { position = i as i32; amount = item.deposit_amount; } if position == - 1 { return Err(ContractError::NotReserved {}); } else { token.rentals.

remove (position as usize ); self.tokens.

save (deps.storage, &token_id, &token)?; } if amount > Uint128::

new ( 0 ) { Ok(Response::

new ().

add_attribute ( "action", "cancelreservationbeforeapprovalforlongterm" ).

add_attribute ( "sender", info.sender).

add_attribute ( "token_id", token_id).

add_message (BankMsg::

Send { to_address: tenant_address, amount:

vec!

[Coin { denom: token.longterm_rental.denom, // @c4-contest: Funds are sent back in long-term denom according to long-term rental agreement amount: amount, // @c4-contest: deposit_amount is loaded from saved short_term rental }], })) } else { Ok(Response::

new ().

add_attribute ( "action", "cancelreservationbeforeapprovalforlongterm" ).

add_attribute ( "sender", info.sender).

add_attribute ( "token_id", token_id)) }

## Recommended Mitigation

Utilize rental_type flag to differentiate between short-term and long-term rental and enforce usage of functions according to its type.

## Assessed type

Invalid Validation blockchainstar12 (Coded Estate) confirmed

# [H-05] Cancelling bid doesn’t clear token approval of bidder allows malicious bidder to steal any tokens listing for sale with auto-approve enabled

- **Contest:** Coded Estate Invitational
- **Slug:** 2024-10-coded-estate-invitational
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-10-coded-estate-invitational
- **Source snapshot:** competitions/2024-10-coded-estate-invitational/final_report.html

Submitted by nnez, also found by Ch_301 This vulnerability allows malicious actors to steal tokens on sell with auto-approve enabled without payment to sellers.

Description The bug arises from an oversight in the token approval management within the bidding and cancellation process. When a seller sets auto_approve to true for their token, a bidder is granted approval upon calling the setbidtobuy function. This approval is intended to allow the buyer to call the transfer_nft function themselves to complete the trade.

The transfer_nft function performs the following actions:

Clears all approvals.

Transfers ownership to the buyer.

Transfers funds to the seller.

However, a flaw exists in the bid cancellation process. When a buyer cancels their bid by calling setbidtobuy again, the function removes their bid and returns the deposited funds, but it fails to revoke the previously granted approval.

This oversight allows a malicious buyer to exploit the system through the following steps:

Bid on a token with auto_approve set to true, gaining approval.

Immediately cancel the bid, receiving a refund while retaining the approval.

Call transfer_nft to transfer the token to themselves without payment, as their bid has been deleted from cancelling process.

This bug effectively allows the attacker to steal the token from the seller without providing any payment to seller.

The severity is set as high because the token (property) listing for sell must have an intrinsic monetary value or else it would not make sense to list it for sale. For example, it could be a property that already has a long-term renter and is receiving a stable income from said renter.

Relevant code snippet pub fn setbidtobuy ( & self, deps: DepsMut, _env: Env, info: MessageInfo, token_id:

String, ) -> Result <Response<C>, ContractError> {...snipped...

// @c4-contest cancellation case else { // update the approval list (remove any for the same spender before adding) token.bids.

retain (|item| item.address != info.sender); // @c4-contest <-- remove bid but doesn't clear approvals } self.tokens.

save (deps.storage, &token_id, &token)?; // @c4-contest cancellation case refunds the bidder if position != - 1 && (amount > Uint128::

from ( 0u64 )) { Ok(Response::

new ().

add_attribute ( "action", "setbidtobuy" ).

add_attribute ( "sender", info.sender.

clone ()).

add_attribute ( "token_id", token_id).

add_message (BankMsg::

Send { to_address: info.sender.

to_string (), amount:

vec!

[Coin { denom: token.sell.denom, amount: amount, }], })) }...snipped...

}

## Recommended Mitigation

Revoke approval of bidder when they cancel the bid.

## Assessed type

Context blockchainstar12 (Coded Estate) disputed Note: For full discussion, see here.

# [H-06] Lack of validation in setlistforsell allows changing denom while there is active bid, leading to stealing of other users’ funds

- **Contest:** Coded Estate Invitational
- **Slug:** 2024-10-coded-estate-invitational
- **Finding ID:** H-06
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-10-coded-estate-invitational
- **Source snapshot:** competitions/2024-10-coded-estate-invitational/final_report.html

setlistforsell allows changing denom while there is active bid, leading to stealing of other users’ funds Submitted by nnez, also found by adeolu and Ch_301 ( 1, 2 ) This vulnerability allows attacker to manipulate the token denom during an active bid. By exploiting this bug, attackers can cancel their own bids and receive refunds in a more valuable token than originally used, effectively stealing funds from the contract’s pool of user deposits.

Description The bug stems from a lack of validation in the setlistforsell function, which allows sellers to change the payment token (denom) even when there are active bids on a token.

The setbidtobuy function, when used to cancel a bid, refunds the buyer using the current denom specified for the token:

pub fn setlistforsell ( & self, deps: DepsMut, env: Env, info: MessageInfo, islisted:

bool, token_id:

String, denom:

String, price:

u64, auto_approve:

bool, ) -> Result <Response<C>, ContractError> { let mut token = self.tokens.

load (deps.storage, &token_id)?; // ensure we have permissions self.

check_can_approve (deps.

as_ref (), &env, &info, &token)?; // @c4-contest: no validation whether there is active bid token.sell.islisted = Some(islisted); token.sell.price = price; token.sell.auto_approve = auto_approve; token.sell.denom = denom; self.tokens.

save (deps.storage, &token_id, &token)?; Ok(Response::

new ().

add_attribute ( "action", "setlistforsell" ).

add_attribute ( "sender", info.sender).

add_attribute ( "token_id", token_id)) } pub fn setbidtobuy ( & self, deps: DepsMut, _env: Env, info: MessageInfo, token_id:

String, ) -> Result <Response<C>, ContractError> { //... (snipped code) if position != - 1 && (amount > Uint128::

from ( 0u64 )) { // if the bid exists Ok(Response::

new ().

add_attribute ( "action", "setbidtobuy" ).

add_attribute ( "sender", info.sender.

clone ()).

add_attribute ( "token_id", token_id).

add_message (BankMsg::

Send { to_address: info.sender.

to_string (), amount:

vec!

[Coin { denom: token.sell.denom, // funds are sent back in the denom set in `setlistforsell` amount: amount, }], })) } //... (snipped code) } However, the setlistforsell function lacks checks for active bids, allowing a seller to change the denom at any time. This creates an exploit scenario where an attacker can:

Mint a new token.

List the token for sale, specifying a low-value token (e.g., TokenX worth $0.01 ) as the denom.

Bid on their own token, paying with the low-value TokenX.

Call setlistforsell again, changing the denom to a high-value token (e.g., USDC worth $1 ).

Cancel their bid by calling setbidtobuy, receiving a refund in the new, more valuable USDC.

This exploit allows the attacker to drain funds from the contract that were deposited by other users. For example, if the attacker initially bid 1,000 TokenX ( $10 ), they could receive 1,000 USDC ( $1,000 ) as a refund, effectively stealing USDC from the contract.

## Assessed type

Invalid Validation blockchainstar12 (Coded Estate) acknowledged

# [H-07] Logic flaw in check_can_edit_short allows editing short-term rental before finalization enabling theft of users’ deposited funds

- **Contest:** Coded Estate Invitational
- **Slug:** 2024-10-coded-estate-invitational
- **Finding ID:** H-07
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-10-coded-estate-invitational
- **Source snapshot:** competitions/2024-10-coded-estate-invitational/final_report.html

check_can_edit_short allows editing short-term rental before finalization enabling theft of users’ deposited funds Submitted by nnez, also found by nnez Malicious actor can exploit this vulnerability to steal other users’ deposited token from the contract.

Description The landlord (property owner) invokes finalizeshorttermrental on a specific rental to settle the payment. If the rental is canceled after approval or has concluded (reached check-out time), the contract sends the payment to the token owner’s address.

The bug stems from an oversight in the function that checks whether a property can be re-listed for short-term rental.

The finalizeshorttermrental function uses the denom (token type) stored in the shortterm_rental struct to determine which token to use for payment:

fn finalizeshorttermrental (...snipped...

if amount > Uint128::new(0) { Ok(Response::

new ().

add_attribute ( "action", "finalizeshorttermrental" ).

add_attribute ( "sender", info.sender).

add_attribute ( "token_id", token_id).

add_message (BankMsg::

Send { to_address: target.

clone (), amount:

vec!

[Coin { denom: token.shortterm_rental.denom, // @contest-info denom is loaded from short-term rental agreement amount: amount, }], })) }...snipped...

The setlistforshorttermrental function, which can change this denom, is supposed to be callable only when there are no active rentals. This is checked by the check_can_edit_short function:

pub fn setlistforshorttermrental ( // function arguments ) -> Result <Response<C>, ContractError> { let mut token = self.tokens.

load (deps.storage, &token_id)?; // ensure we have permissions self.

check_can_approve (deps.

as_ref (), &env, &info, &token)?; self.

check_can_edit_short (&env, &token)?; token.shortterm_rental.islisted = Some( true ); token.shortterm_rental.price_per_day = price_per_day; token.shortterm_rental.available_period = available_period; token.shortterm_rental.auto_approve = auto_approve; token.shortterm_rental.denom = denom; token.shortterm_rental.minimum_stay = minimum_stay; token.shortterm_rental.cancellation = cancellation; self.tokens.

save (deps.storage, &token_id, &token)?; Ok(Response::

new ().

add_attribute ( "action", "setlistforshorttermrental" ).

add_attribute ( "sender", info.sender).

add_attribute ( "token_id", token_id)) } pub fn check_can_edit_short ( & self, env:&Env, token:&TokenInfo<T>, ) -> Result <(), ContractError> { if token.rentals.

len () == 0 { return Ok(()); } else { let current_time = env.block.time.

seconds (); let last_check_out_time = token.rentals[token.rentals.

len ()- 1 ].renting_period[ 1 ]; if last_check_out_time < current_time { return Ok(()); } else { return Err(ContractError::RentalActive {}); } However, this function only checks if the current time exceeds the last rental’s check-out time. It doesn’t verify whether all rentals have been finalized or if there are any pending payments.

This oversight allows a malicious landlord to change the denom after a rental period has ended but before finalization, potentially getting payment in a more valuable token than originally configured.

The attack scenario could unfold as follows:

Attacker starts with two accounts, one as landlord and one as renter.

Attacker (as landlord) mints a new token and lists it for short-term rental, specifying a low-value token (e.g., TokenX worth $0.01 ) as the denom.

Attacker (as renter) reserves a short-term rental on their own token, paying with TokenX (e.g., 1,000 TokenX ≈ $10 ).

After the rental period ends ( current time > check_out_time ), the attacker (as landlord) calls setlistforshorttermrental to change the denom to a high-value token (e.g., USDC worth $1 ).

Attacker then calls finalizeshorttermrental to settle the payment.

Attacker receives 1,000 USDC ( $1,000 ) instead of TokenX, effectively stealing $990 from the contract’s pool of user deposits.

This exploit allows the attacker to artificially inflate the value of their rental payment, draining funds from the contract that were deposited by other users.

## Recommended Mitigation

Only allow editing when there is no rental.

pub fn check_can_edit_short ( & self, env:&Env, token:&TokenInfo<T>, ) -> Result <(), ContractError> { if token.rentals.

len () == 0 { return Ok(()); } return Err(ContractError::RentalActive {}); }

## Assessed type

Invalid Validation blockchainstar12 (Coded Estate) confirmed

# [H-08] Adversary can use send_nft to bypass the payment and steal seller’s token in auto-approve scenario

- **Contest:** Coded Estate Invitational
- **Slug:** 2024-10-coded-estate-invitational
- **Finding ID:** H-08
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-10-coded-estate-invitational
- **Source snapshot:** competitions/2024-10-coded-estate-invitational/final_report.html

send_nft to bypass the payment and steal seller’s token in auto-approve scenario Submitted by nnez, also found by Ch_301 This vulnerability allows malicious actor to steal tokens without payment when auto-approve is enabled.

Description The bug arises from an oversight in the token transfer mechanisms when auto_approve is set to true. While the transfer_nft function includes logic for settling payments, the send_nft function does not.

When a seller enables auto_approve, a bidder is granted approval of the token upon calling the setbidtobuy function. This approval is intended to allow the buyer to use transfer_nft to complete the trade, as this function handles both the token transfer and payment settlement.

However, the contract fails to account for the send_nft function, which can also be used to transfer tokens. Unlike transfer_nft, send_nft does not include any trade settlement logic:

File: contracts/codedestate/src/execute.rs fn send_nft ( & self, deps: DepsMut, env: Env, info: MessageInfo, contract:

String, token_id:

String, msg: Binary, ) -> Result <Response<C>, ContractError> { // Transfer token self.

_transfer_nft (deps, &env, &info, &contract, &token_id)?; // @c4-contest: just transfer token, no trade settlement logic let send = Cw721ReceiveMsg { sender: info.sender.

to_string (), token_id: token_id.

clone (), msg, }; // Send message Ok(Response::

new ().

add_message (send.

into_cosmos_msg (contract.

clone ())?).

add_attribute ( "action", "send_nft" ).

add_attribute ( "sender", info.sender).

add_attribute ( "recipient", contract).

add_attribute ( "token_id", token_id)) } pub fn _transfer_nft ( & self, deps: DepsMut, env: &Env, info: &MessageInfo, recipient: & str, token_id: & str, ) -> Result <Response<C>, ContractError> { let mut token = self.tokens.

load (deps.storage, token_id)?; // ensure we have permissions self.

check_can_send (deps.

as_ref (), env, info, &token)?; // set owner and remove existing approvals token.owner = deps.api.

addr_validate (recipient)?; token.approvals = vec!

[]; self.tokens.

save (deps.storage, token_id, &token)?; Ok(Response::

new ().

add_attribute ( "action", "_transfer_nft" ).

add_attribute ( "sender", info.sender.

clone ()).

add_attribute ( "token_id", token_id)) } This oversight allows a malicious buyer to exploit the system through the following steps:

Place a bid on a token with auto_approve set to true, gaining approval.

Use send_nft to transfer the token to their own custom contract that implements Cw721ReceiveMsg, bypassing payment.

Cancel their original bid to receive a full refund.

This exploit effectively allows the attacker to steal the token from the seller without providing any payment to the seller.

## Recommended Mitigation

Disallow the use of send_nft when token is on sale.

## Assessed type

Context blockchainstar12 (Coded Estate) acknowledged

# [H-09] Token owner can burn their token with active rental leading to renters’ funds being stuck

- **Contest:** Coded Estate Invitational
- **Slug:** 2024-10-coded-estate-invitational
- **Finding ID:** H-09
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-10-coded-estate-invitational
- **Source snapshot:** competitions/2024-10-coded-estate-invitational/final_report.html

Submitted by nnez, also found by Ch_301 If the property owner calls the burn function while active rentals exist, the rental information, including deposits, is deleted. This prevents renters from retrieving their funds through the cancellation process, leading to funds of renters being stuck in the contract.

Description The burn function in the contract deletes all data associated with a token, including any active rental information. In Coded Estate, renters must deposit funds in advance for short-term rentals, and this information is stored in a vector, rentals, linked to the token.

The issue arises because the burn function only checks whether the caller is the owner or has approval to burn the token. It does not validate whether there are any active rentals associated with the token. As a result, if the property owner calls the burn function while rentals are still active, all rental data, including the deposit amounts, is deleted from storage.

Without the rental information, renters can no longer use the cancellation function to retrieve their deposits, as the contract does not retain any record of the rental. This leads to irreversible loss of funds for the renters.

Relevant code snippets File: contracts/codedestate/src/state.rs pub struct TokenInfo <T> { /// The owner of the newly minted NFT pub owner: Addr, pub approvals:

Vec <Approval>, pub longterm_rental: LongTermRental, pub shortterm_rental: ShortTermRental, pub rentals:

Vec <Rental>, // <-- rental information is stored here pub bids:

Vec <Bid>, pub sell: Sell, pub token_uri:

Option < String >, pub extension: T, } File: contracts/codedestate/src/execute.rs pub fn setlistforshorttermrental ( //...

//... function arguments //...

) -> Result <Response<C>, ContractError> {...... snipped...

let traveler = Rental { denom:token.shortterm_rental.denom.

clone (), rental_type:

false, approved_date:None, deposit_amount: Uint128::

from (rent_amount), renting_period:

vec!

[new_checkin_timestamp, new_checkout_timestamp], address: Some(info.sender.

clone ()), approved: token.shortterm_rental.auto_approve, cancelled:

false, guests:guests, }; // token.shortterm_rental.deposit_amount += sent_amount; token.rentals.

insert (placetoreserve as usize, traveler); // deposited amount is stored in rentals vector...... snipped...

} fn burn ( & self, deps: DepsMut, env: Env, info: MessageInfo, token_id:

String, ) -> Result <Response<C>, ContractError> { let token = self.tokens.

load (deps.storage, &token_id)?; self.

check_can_send (deps.

as_ref (), &env, &info, &token)?; // <-- Only checks ownership or approval self.tokens.

remove (deps.storage, &token_id)?; // <-- Deletes all token data including saved rentals vector self.

decrement_tokens (deps.storage)?; Ok(Response::

new ().

add_attribute ( "action", "burn" ).

add_attribute ( "sender", info.sender).

add_attribute ( "token_id", token_id)) } Example Scenario A property owner lists a property for short-term rental, and several renters reserve it by depositing funds in advance.

The property owner calls the burn function to burn the token while rentals are still active.

All rental information, including the deposit amounts, is erased.

When renters attempt to cancel their reservations expecting a refund, the transaction will revert as the rental information is deleted with the token.

## Recommended Mitigation

Add a validation in burn function that there is no active rental.

## Assessed type

Invalid Validation blockchainstar12 (Coded Estate) confirmed Medium Risk Findings (9)

# [M-01] Malicious NFT owners can rug the reservation of the long-term

- **Contest:** Coded Estate Invitational
- **Slug:** 2024-10-coded-estate-invitational
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-10-coded-estate-invitational
- **Source snapshot:** competitions/2024-10-coded-estate-invitational/final_report.html

Submitted by Ch_301, also found by nnez

- https://github.com/code-423n4/2024-10-coded-estate/blob/main/contracts/codedestate/src/execute.rs#L1490-L1541
- https://github.com/code-423n4/2024-10-coded-estate/blob/main/contracts/codedestate/src/execute.rs#L1786-L1854
Description Due to the long period of the long-term rent, the Homeowner has an advantage in that type of reservation, which is the ability to withdraw a part from the deposited amount by the tenant. This applies only to reservations made more than one month in advance. This could be done by using execute.rs#withdrawtolandlord() function.

if item.deposit_amount - Uint128::

from (token.longterm_rental.price_per_month) < Uint128::

from (amount) { The withdrawn amount will be subtracted from the user’s deposit_amount state:

token.rentals[position as usize ].deposit_amount -= Uint128::

from (amount); On the other side, the NFT owner can trigger execute.rs#rejectreservationforlongterm() to reject any reservation at any time even if it currently running, it will send back.deposit_amount as a refundable amount to the user.

However, a malicious homeowner can the advantages of execute.rs#rejectreservationforlongterm() and execute.rs#withdrawtolandlord() to steal a user’s funds and reject them in two simple steps:

Wait for the reservation to start and call execute.rs#withdrawtolandlord(). this will transfer most of the funds out.

Now, invoke execute.rs#rejectreservationforlongterm() to kick the user out, this will transfer back to the user only a small presenting of his initial deposit.

Note: The homeowner has the power to reject any reservation even if it is currently active by triggering rejectreservationforlongterm() and refunding user money; however, using this function, the refundable amount is the same initial deposit.

## Recommended Mitigation Steps

Don’t allow to reject active reservations.

blockchainstar12 (Coded Estate) acknowledged and commented:

Actually, the platform will work as monthly deposit logic and this won’t be issue.

# [M-02] Users can’t cancel reservation due to out-of-gas

- **Contest:** Coded Estate Invitational
- **Slug:** 2024-10-coded-estate-invitational
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-10-coded-estate-invitational
- **Source snapshot:** competitions/2024-10-coded-estate-invitational/final_report.html

Submitted by Ch_301, also found by nnez In execute.rs#cancelreservationafterapprovalforshortterm and execute.rs#cancelreservationafterapprovalforlongterm(), multiple iterations occur over the cancellation vector, which may cause the transaction to fail due to an out-of-gas error.

Consequently, malicious NFT owners could exploit this by setting a big list inside the cancellation vector by invoking execute.rs#setlistforshorttermrental() or execute.rs#setlistforlongtermrental():

pub fn setlistforlongtermrental ( /***CODE***/ cancellation:

Vec <CancellationItem>, ) -> Result <Response<C>, ContractError> { /***CODE***/ token.longterm_rental.cancellation = cancellation; This will force the cancellation of the reservation to fail due to gas limits.

## Recommended Mitigation Steps

Set a cap for the length of the cancellation vector that owners can set it.

blockchainstar12 (Coded Estate) acknowledged and commented:

Nobody sets cancellation array, as such big list and such transaction cannot be confirmed.

# [M-03] Use of u64 for price_per_day and price_per_month limits handling tokens with 18 decimals

- **Contest:** Coded Estate Invitational
- **Slug:** 2024-10-coded-estate-invitational
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-10-coded-estate-invitational
- **Source snapshot:** competitions/2024-10-coded-estate-invitational/final_report.html

u64 for price_per_day and price_per_month limits handling tokens with 18 decimals Submitted by nnez

- https://github.com/code-423n4/2024-10-coded-estate/blob/main/contracts/codedestate/src/msg.rs#L168
- https://github.com/code-423n4/2024-10-coded-estate/blob/main/contracts/codedestate/src/msg.rs#L111

## Impact

The use of u64 for price_per_day and price_per_month prevents setting rental prices higher than approximately 18 tokens when using tokens with 18 decimals, potentially restricting landlords from setting appropriate rental prices in tokens with 18 decimals.

## Assessed type

Context Lambda (judge) commented:

This can indeed limit the functionality of the protocol under reasonable assumptions. 18 decimal stablecoins are very common and it can be expected that some bridged asset will have 18 decimals. In such scenarios, a maximum price of $18 per month or day will be too low for many properties, meaning that these tokens cannot be used.

blockchainstar12 (Coded Estate) acknowledged and commented:

We use tokens with 6 decimals in the platform.

# [M-04] Incorrect use of u64 for arg amount in withdrawtolandlord can cause withdrawal failure

- **Contest:** Coded Estate Invitational
- **Slug:** 2024-10-coded-estate-invitational
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-10-coded-estate-invitational
- **Source snapshot:** competitions/2024-10-coded-estate-invitational/final_report.html

u64 for arg amount in withdrawtolandlord can cause withdrawal failure Submitted by nnez

- https://github.com/code-423n4/2024-10-coded-estate/blob/main/contracts/codedestate/src/execute.rs#L1786-L1796
- https://github.com/code-423n4/2024-10-coded-estate/blob/main/contracts/codedestate/src/msg.rs#L156-L162

## Impact

The use of u64 for token amount in the withdrawtolandlord function can lead to failed withdrawals when handling tokens with 18 decimals, limiting the landlord’s ability to withdraw their entitled funds if the amount exceeds the maximum value of u64.

## Recommended Mitigation

Change type of amount to u128 for consistency with other parts in the system.

## Assessed type

Context Lambda (judge) commented:

Does not seem to be a significant problem to me at first sight, if such a scenario would ever happen, withdrawal should be possible with multiple calls.

blockchainstar12 (Coded Estate) acknowledged Lambda (judge) decreased severity to Low and commented:

Unlike #29, this does not impact the functionality of the protocol significantly. While a larger data type could still be a good idea here, the owner can still withdraw funds by splitting up the withdrawals into multiple calls.

nnez (warden) commented:

@Lambda - I might have overstated the impact in the report (unable to withdraw funds). However, I still believe that this issue should be classified as Medium severity. This bug does impact the protocol’s functionality.

Consider the scenario wherein the required deposit is 5_000e18 tokens. In this scenario, the token owner would have to split their transaction into 5_000e18 / (2^64-1) = 271.05 → 272 separate transactions in order to withdraw all the funds.

That’s a lot of transactions and this is just for one long-term rental. An individual token owner’s can have more than one property and they can have more than one active long-term rental with deposit to withdraw.

Instead of paying gas for one transaction, users unnecessarily have to pay 200x+ more of gas in order to withdraw the full amount.

Additionally, 5_000e18 is just an arbitrary reasonable number for a 18 decimals token worth $1; the problem could get worse with a larger amount of tokens. For example, 50_000e18 of $0.1 would take 2711 transactions to withdraw the full amount.

Lambda (judge) increased severity to Medium and commented:

That’s true, $5,000 is a reasonable amount for such a protocol to handle. Potentially even low, with business apartments in cities like Zurich that often cost $5,000 per month, so you could easily have $30,000 for a longer rental. 18 decimal stable coins are also very common.

So it is not that unlikely that a landlord would have to perform ~1,632 calls for one withdrawal. On the one hand, this would be of course very cumbersome (especially if the UI did not support this), but it can also become pretty expensive (if one call were roughly $1, this would be an almost 5% fee on top). So based on that, Medium is indeed more appropriate.

# [M-05] Incorrect refund amount is sent to the tenant if long term reservation is cancelled after approval

- **Contest:** Coded Estate Invitational
- **Slug:** 2024-10-coded-estate-invitational
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-10-coded-estate-invitational
- **Source snapshot:** competitions/2024-10-coded-estate-invitational/final_report.html

Submitted by adeolu, also found by adeolu token.longterm_rental.cancellation.percentage is not deducted from the token.longterm_rental.deposit_amount and refunded back to the user as expected after a cancelreservationafterapprovalforlongterm() call to cancel a reservation that has been approved.

## Recommended Mitigation

Use token.longterm_rental.cancellation.percentage to calculate amount to be returned to tenant instead of self.get_fee(deps.storage) if the deduction will be enforced in finalizelongtermrental().

OR Add extra logic like below into cancelreservationafterapprovalforlongterm() to check that refundable amount is calculated as directed by the landlord/token owner.

let mut cancellation = token.longterm_rental.cancellation.clone();.....

let diff_days = (check_in_time_timestamp - current_time)/86400; for (_i, item) in cancellation.iter().enumerate() { if item.deadline < diff_days { refundable_amount = Uint128::new((amount.u128() * u128::from(item.percentage)) / 100); break; }.....

if refundable_amount > Uint128::new(0) { Ok(Response::new().add_attribute("action", "cancelreservationafterapprovalforlongterm").add_attribute("sender", info.sender).add_attribute("token_id", token_id).add_message(BankMsg::Send { to_address: traveler_address, amount: vec![Coin { denom: token.longterm_rental.denom, amount: refundable_amount, }], })) }

## Assessed type

Context blockchainstar12 (Coded Estate) acknowledged and commented:

This is intended logic.

Lambda (judge) decreased severity to Medium and commented:

I agree that it seems weird that the cancellation vector for long term rentals is completely ignored. While this seems to be intended according to the sponsor, I have not found any documentation indicating this and an owner may therefore, have different expectations. Because of this, I am judging it as impact on the function of the protocol / value leak with external requirements (assumptions about the long-term cancellation process).

# [M-06] Lack of upfront cost for long-term reservations allows fake reservations, blocking real users

- **Contest:** Coded Estate Invitational
- **Slug:** 2024-10-coded-estate-invitational
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-10-coded-estate-invitational
- **Source snapshot:** competitions/2024-10-coded-estate-invitational/final_report.html

Submitted by nnez This issue allows a malicious actor to reserve long-term rentals without upfront payment, making large time slots unavailable for other potential renters. It creates an unfair scenario where legitimate users are blocked out from booking, as the property becomes unavailable for both short-term and long-term rentals during the reserved period. This could lead to decreased revenue for property owners.

Description The setreservationforlongterm function allows users to reserve long-term rentals without any upfront payment. Once a reservation is made, the reserved period is marked as unavailable, blocking other users from reserving the same property during that period for either long-term or short-term rentals.

pub fn setreservationforlongterm ( & self, deps: DepsMut, info: MessageInfo, token_id:

String, renting_period:

Vec < String >, guests:

usize, ) -> Result <Response<C>, ContractError> { let mut token = self.tokens.

load (deps.storage, &token_id)?; let new_checkin = renting_period[ 0 ].

parse::< u64 >(); let new_checkin_timestamp; match new_checkin { Ok(timestamp) => { new_checkin_timestamp = timestamp; } Err(_e) => { return Err(ContractError::NotReserved {}); } let new_checkout = renting_period[ 1 ].

parse::< u64 >(); let new_checkout_timestamp; match new_checkout { Ok(timestamp) => { new_checkout_timestamp = timestamp; } Err(_e) => { return Err(ContractError::NotReserved {}); } if ((new_checkout_timestamp - new_checkin_timestamp)/ 86400 ) < token.longterm_rental.minimum_stay { return Err(ContractError::LessThanMinimum {}); } let mut placetoreserve:

i32 = - 1; let lenofrentals = token.rentals.

len (); let mut flag = false; // @c4-contest: if the renting period overlap with an existing rental, the placetoreserve will be -1 for (i, tenant) in token.rentals.

iter ().

enumerate () { let checkin = tenant.renting_period[ 0 ]; let checkout = tenant.renting_period[ 1 ]; if new_checkout_timestamp < checkin { if i == 0 { placetoreserve = 0; break; } else if flag { placetoreserve = i as i32; break; } else if checkout < new_checkin_timestamp { flag = true; if i == lenofrentals - 1 { placetoreserve = lenofrentals as i32; break; } else { flag = false; } if placetoreserve == - 1 { if lenofrentals > 0 { return Err(ContractError::UnavailablePeriod {}); } else { placetoreserve = 0; } let tenant = Rental { denom:token.longterm_rental.denom.

clone (), rental_type:

true, approved:token.longterm_rental.auto_approve, deposit_amount: Uint128::

from ( 0u64 ), // @c4-contest: no upfront payment required renting_period:

vec!

[new_checkin_timestamp, new_checkout_timestamp], address: Some(info.sender.

clone ()), approved_date: None, cancelled:

false, guests, }; token.rentals.

insert (placetoreserve as usize, tenant); self.tokens.

save (deps.storage, &token_id, &token)?; Ok(Response::

new ().

add_attribute ( "action", "setreservationforlongterm" ).

add_attribute ( "sender", info.sender).

add_attribute ( "token_id", token_id)) } This lack of an upfront cost creates an opening for abuse. A malicious actor could spam the system by making multiple long-term reservations across various periods for a property, essentially making all time slots unavailable. By doing so, legitimate users are blocked from renting the property, potentially causing financial harm to the property owner.

Even though property owners can reject these reservations manually, they cannot easily distinguish between legitimate and fake reservations. The actor could use multiple addresses to make the fake reservations appear legitimate. This forces the owner to either wait for a deposit via depositforlongtermrental or communicate with the renter through other channels (like messaging) to verify if the booking is genuine.

The key issue here is that all of these actions involve a wait time, during which legitimate renters might lose interest and book other properties. This wait time represents an opportunity cost, reducing the property’s chances of being rented by honest users. The inability to distinguish between genuine and fake reservations, combined with the opportunity cost, makes this finding valid and harmful to the system’s integrity.

Example Scenario:

A malicious user reserves multiple periods for a popular property using different addresses, without any upfront payment.

Legitimate users attempt to reserve the property but are blocked because the periods are marked as unavailable.

The property owner is forced to wait for the malicious user to make a deposit or use external communication to verify the reservation, leading to lost rental opportunities as honest users may move on to other properties.

## Recommended Mitigation

Consider requiring some amount of upfront payment for long-term rental reservation with cancellation policy as already implemented in short-term rental flow.

## Assessed type

Context Lambda (judge) commented:

Definitely a good point to raise, on the fence about the severity here. One could argue that this is by design for such platforms, as there are many other web2 sites where you can make reservations for free and therefore block a valid user. On the other hand, because this is a smart contract where you can easily submit transactions from multiple addresses, doing this becomes very easy and hard to prevent after an initial deployment. A malicious user could easily perform a lot of reservations to block properties all the time, which would impact the intended function of the protocol and its availability. This matches the definition of a valid Medium.

blockchainstar12 (Coded Estate) acknowledged and commented:

We have manual reject logic at this contract, so request without deposit won’t be confirmed to owners.

# [M-07] Reservations can be made outside of rental property’s available_period

- **Contest:** Coded Estate Invitational
- **Slug:** 2024-10-coded-estate-invitational
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-10-coded-estate-invitational
- **Source snapshot:** competitions/2024-10-coded-estate-invitational/final_report.html

available_period Submitted by adeolu There is no check for if the renting_period is within the rentals available period ( token.shortterm_rental.available_period ). This means that reservations can be made to rent the property on dates outside its available period.

## Recommended Mitigation

Check that the renting_period specified by renting users is within the property’s available_period.

## Assessed type

Context blockchainstar12 (Coded Estate) acknowledged and commented:

It’s not necessary logic as owners can reject any request, available period is optional.

adeolu (warden) commented:

It’s not necessary logic as owners can reject any request, available period is optional.

But owners can set an available_period time, with the idea that they expect renters to make reservations for that period only. Just because it’s optional doesn’t mean that when the feature is to be used it should not work as expected.

# [M-08] Can impersonate another high value rental because token_uri is arbitrary and supplied by user

- **Contest:** Coded Estate Invitational
- **Slug:** 2024-10-coded-estate-invitational
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-10-coded-estate-invitational
- **Source snapshot:** competitions/2024-10-coded-estate-invitational/final_report.html

token_uri is arbitrary and supplied by user Submitted by adeolu Because token_uri value is not sanitized and is arbitrary/provided by a user, a malicious user can provide a token uri which may have " or, or simply a fake url which points to a different higher value rental property in order to phish unsuspecting users.

## Recommended Mitigation

Don’t make token uri arbitrary, generate it in code. Ensure your generation logic rejects strings that contain " and, as these can also be exploited by attackers to do json injection of false fields.

## Assessed type

Context blockchainstar12 (Coded Estate) acknowledged Lambda (judge) commented:

Requires some assumptions about the off-chain usage, but similar issues have historically been judged as Medium, as seen here and here and there is a valid attack pattern.

nnez (warden) commented:

@Lambda - I disagree with the Medium severity of this issue. It should be classified as a QA-level finding at most.

There are 2 claimed impacts here:

Impersonation of another high-value property (rental).

JSON injection.

Let’s explore the validity of these claims.

Impersonation To generalize the problem, NFTs are typically distinguished by several identifiers such as tokenId, tokenURI, and specific token attributes. For example, a token with tokenId=1 and tokenURI=A is a different asset from one with tokenId=2 and tokenURI=B.

In this particular protocol, the key identifiers of the tokens (representing properties) are:

TokenID: Each token’s tokenId is unique and user-specified, ensuring that no two tokens can have the same tokenId.

TokenURI: This field is arbitrary, meaning that its content (whether in JSON format, URL, or any other structure) does not follow a strict convention. However, the format or content of the tokenURI is irrelevant to the impersonation risk, as it merely serves as metadata.

In traditional NFT protocols, tokenURI plays a significant role in defining a token’s value, as it may contain important unique metadata. If an attacker could replicate the tokenURI, it might be possible to create a token that looks identical and that eliminates the value of the unique NFT. However, in this protocol, the value is instead linked to the real-world property and, therefore, to the ownership of that property.

Real-World Analogy:

Consider a rental platform like Airbnb. If two properties look identical, a user will check the legitimacy of the owner to verify the booking. Similarly, in this protocol, the core identifier is the property’s owner, not just the tokenURI. The protocol must ensure that the owner’s identity, along with other token identifiers, is clearly presented on the front-end to avoid confusion.

Thus, while the tokenURI is arbitrary, impersonation in this protocol relies primarily on the ownership of the property, making it a front-end issue, not a smart contract level concern.

To simply put it, one should distinguish each token using not just one of its identifiers but all of its identifiers.

JSON Injection Regarding JSON injection, the concern here appears to stem from the assumption that tokenURI might be used in a structured format such as JSON. However, this is speculative. The tokenURI field is arbitrary, and without explicit evidence, like in the cited findings, we can’t assume that it’s gonna be constructed at a smart contract level in JSON format.

Besides, the risk of impersonation related to the tokenURI, regardless of format, was already addressed in the previous section.

Conclusion In conclusion, while the claim regarding the arbitrary tokenURI is valid, the claimed impact is not. This issue should be regarded as a front-end concern rather than a smart contract vulnerability, as there is no effective mitigation at the smart contract level to address it directly.

That is, the front-end should:

Must ensure that the owner’s identity, along with other token identifiers, is clearly presented on the front-end to avoid confusion.

Must sanitize and validate the input from tokenURI (I don’t think you can do that effectively on the smart contract level, given the computational limit by nature of transaction execution on blockchain).

Although there is precedent for classifying similar issues as Medium severity, I believe it is more appropriate to tailor the severity to the specific context of this protocol, rather than generalizing the issue based on previous cases.

adeolu (warden) commented:

This issue should be regarded as a front-end concern rather than a smart contract vulnerability, as there is no effective mitigation at the smart contract level to address it directly.

@nnez - But there is a good mitigation for this, which is preventing arbitrary strings to be used as token URI. And I put a snippet of a better token Uri generation implementation in my original submission. high value protocols that use nft; i.e., uniswap never allows arbitrary uri generation.

Also, how is it a front end concern and not a contract vuln if the issue stems from a misuse of the smart contract? this protocol is very well dependent on the token Uri for their use case As token Uri contains all attributes and possibly images of the rental.

nnez (warden) commented:

Say the protocol were to implement the construct function as you suggested:

function constructTokenURI(TokenURIParams memory params) public pure returns (string memory) { string memory json = string( abi.encodePacked( '{"name":"', params.name, '", "description":"', params.description, '", "image": "', params.image, '", "animation_url": "', params.animation_url, '"}' ) ); return string(abi.encodePacked("data:application/json;base64,", Base64.encode(bytes(json)))); } Here, the name, the description, the image and other metadata on the token is still an arbitrary params. How would you effectively prevent a malicious actor from using the same name, same description, and the same image of the legitimate rental property?

Even if you hash all the inputs and prevent the same inputs from being used twice, a malicious actor can just change the url, add another character or words to the name and description.

My whole point here is that one cannot rely solely on tokenURI for uniqueness of the token. One will know for sure that they’re making a reservation on a legitimate token (property) if one knows all three information:

owner, tokenId and tokenURI. One can never know for sure if they only know one of the three.

owner and tokenId are both unique and cannot be forged.

You must have a private key of the owner to impersonate as owner The logic of the contract prevents the token with same tokenId from being created So, it does make sense to allow an arbitrary information in tokenURI so that token owner can put their property’s information there. How other protocols uses their tokenURI is irrelevant here as I have pointed out that the context for this protocol is different.

It is the front-end responsbility to display all required information ( owner, tokenId and tokenURI ) to users to enable them to distinguish between genuine and fake properties.

Lambda (judge) commented:

@nnez - I agree with the points raised about impersonation. You cannot solely rely on these attributes, which is a problem that all NFTs face to a certain point (there is for instance nothing stopping anyone from creating a fake BAYC contract that points to the same image) and is very hard to solve (especially without introducing some centralized instance that would e.g., verify the attributes).

For the second point:

Regarding JSON injection, the concern here appears to stem from the assumption that tokenURI might be used in a structured format such as JSON. However, this is speculative.

This is indeed somewhat speculative (it relies on external requirements, which is generally fine for a Medium), but seems like a reasonable assumption. The ERC721 standard even requires this with its metadata assumption. The implementation is based on CW721 with similar requirements/recommendations (see here ). Of course, it is also not clear what external systems are doing with this information. But a reasonable assumption here is that it is parsed and/or downloaded and displayed in a frontend.

These things are valid concerns and have happened in the past (see here or here, for e.g.). Of course, they should also be addressed in a frontend by taking respective measures. But I see the valid attack path with external requirements here (although I would have liked a few more details in the issue description).

nnez (warden) commented:

Of course, it is also not clear what external systems are doing with this information. But a reasonable assumption here is that it is parsed and or downloaded and displayed in a frontend.

Isn’t this an indication that the issue resides on the front-end side?

I believe the criteria for external requirement is the other way around where it requires a specific situation for the bug to occur on smart contract not that it would happen on the external system.

The issue would be valid if the tokenURI were intended to be immutable like traditional NFTs. However, in this case, it’s designed to allow arbitrary information because users need to input their rental information.

Would your perspective on the issue change if the field name were changed to description and allowed arbitrary string? Would it be the front-end’s responsibility to filter and sanitize the string data retrieved before using it?

Lambda (judge) commented:

It is definitely debatable whose responsibility it is and I’d recommend everyone writing a frontend to sanitize any tokenURI return value before using it in the frontend. Nevertheless, this is unfortunately not always done (see above, this was even a major NFT platform) and in such cases, users might actually blame you/your contract because your contracts ultimately caused the malicious payload.

Would your perspective on the issue change if the field name were changed to description and allowed arbitrary string?

Depends, if the string were sanitized, definitely. Otherwise I’d still see the valid attack path.

# [M-09] User supplied owner address which is meant to be token owner is never the token owner

- **Contest:** Coded Estate Invitational
- **Slug:** 2024-10-coded-estate-invitational
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-10-coded-estate-invitational
- **Source snapshot:** competitions/2024-10-coded-estate-invitational/final_report.html

Submitted by adeolu In the function mint(), owner is a parameter which is accepted by the function and is meant to be set into the TokenInfo struct’s owner field during the mint. But the issue is that the TokenInfo struct sets the owner to be the info.sender. This is wrong because info.sender, which is the function caller is not always the owner arg. This means the mint logic is defective, the user supplied value for owner will never be the owner of the newly minted token.

## Recommended Mitigation

Set user supplied owner value as owner in the token struct.

let token = TokenInfo { owner: owner, approvals:

vec!

[], rentals:

vec!

[], bids:

vec!

[], longterm_rental, shortterm_rental, sell, token_uri, extension, };

## Assessed type

Context blockchainstar12 (Coded Estate) acknowledged and commented:

It does not make any issues actually.

adeolu (warden) commented:

It does not make any issues actually.

The function accepts user specified Param address to be owner; the function doesn’t set the user specified param address as owner in token struct and then returns a response that it has set owner to be the user specified “owner” param value. The owner param value is not always same as info.sender.

## Rejected Primary Findings

# Rejected Primary Findings: Coded Estate Invitational

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** requires_authentication
