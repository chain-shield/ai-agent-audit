use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::prompts::{
    access_control::ACCESS_CONTROL, array_limits::ACCESS_OUTSIDE_ARRAY_LIMITS,
    confidential_data::SAVING_CONFIDENTIAL_DATA, default_visibility::DEFAULT_VISIBILITIES,
    dos::DOS, float_precision::INTEGER_OVERFLOW, inheritance::WRONG_INHERITANCE,
    oracle::ORACLE_MANIPULATION, pragma::FLOATING_PRAGMA, randomness::RANDOMNESS,
    reentrancy::REENTRANCY, replay_attack::REPLAY_SIGNATURES_ATTACK, self_destruct::SELF_DESTRUCT,
    short_address_attack::SHORT_ADDRESS_ATTACK, storage_variables::STORAGE_VARIABLE,
    tx_origin::TX_ORIGIN, unchecked_return_value::UNCHECK_RETURN_VALUES,
    unexpected_eth::UNEXPECTED_ETH, zero_code::CONTRACTS_WITH_ZERO_CODE,
};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum Severity {
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Finding {
    // [Severity-issue number] - List Issue (Reentrancy, Denial of Service, etc) and
    // <Contract>::<Function> its localed in
    pub title: Option<String>,
    pub description: Option<String>, // description of issue, include code snippet if relevant
    pub impact: Option<String>,      // Impact of Issue
    pub proof_of_concept: Option<String>, // Demonstrate how issue can be exploited by hacker
    pub proof_of_code: Option<String>, // Write Foundry Unit test to prove issue exists
    #[schemars(rename = "severity")]
    #[schemars(description = "Severity level: High, Medium, Low, Info")]
    pub severity: Option<String>, //severity of issue
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Findings {
    pub findings: Vec<Finding>,
}

pub const SECURITY_PROMPTS: [&str; 19] = [
    ACCESS_CONTROL,
    ACCESS_OUTSIDE_ARRAY_LIMITS,
    DEFAULT_VISIBILITIES,
    DOS,
    INTEGER_OVERFLOW,
    SAVING_CONFIDENTIAL_DATA,
    WRONG_INHERITANCE,
    ORACLE_MANIPULATION,
    FLOATING_PRAGMA,
    RANDOMNESS,
    REENTRANCY,
    REPLAY_SIGNATURES_ATTACK,
    SELF_DESTRUCT,
    SHORT_ADDRESS_ATTACK,
    STORAGE_VARIABLE,
    TX_ORIGIN,
    UNCHECK_RETURN_VALUES,
    UNEXPECTED_ETH,
    CONTRACTS_WITH_ZERO_CODE,
];
//
// pub const REENTRANCY: &str = r#"Analyze the Solidity code for reentrancy vulnerabilities.
// Look for any function where an external call (e.g., call, send, transferring Ether or calling another contract)
// is made before the function updates its own state. Specifically, check if the contract sends Ether or calls an
// external contract prior to updating critical state variables like balances. If such a pattern exists, explain
// how an attacker’s contract could repeatedly call back into the function (via fallback) and exploit the
// unchanged state to perform multiple operations (e.g., multiple withdrawals) in a single transaction."#;
//
// pub const ACCESS_CONTROL: &str = r#"Review the contract for access control issues. Identify any functions
// that perform sensitive actions (e.g., transferring Ether, minting or burning tokens, changing ownership or
// critical settings) and ensure they are restricted to authorized addresses. Flag any function that lacks an
// onlyOwner modifier or equivalent access check. Pay special attention to functions like mint, burn, initialize,
// or withdrawAll – if these can be called by arbitrary users, explain why that is a serious vulnerability."#;
//
// pub const INTEGER_OVERFLOW: &str = r#"Check the contract for integer overflow or underflow vulnerabilities.
// Focus on arithmetic operations (+, -, *, etc.) on state variables, especially in Solidity versions before 0.8
// where overflows do not revert. Look for any math that could exceed the variable’s range (e.g., incrementing
// counters without bounds or decrementing below zero). If the code uses unchecked in Solidity 0.8+, scrutinize
// those sections for potential overflow. Report any arithmetic operations that lack proper bounds checking or
// SafeMath protection in Solidity <0.8."#;
//
// pub const UNCHECK_RETURN_VALUES: &str = r#"Scan the code for places where it calls external contracts or sends
// Ether without properly handling the return status. In particular, look at any usage of low-level call, delegatecall,
// or external function calls on interfaces. If the code doesn’t check for success (e.g., missing a require(success)
// after a call), identify that as a vulnerability. Explain how a failed call (due to the callee reverting or running
// out of gas) could go unnoticed and what impact that would have on the contract’s state (e.g., state updated as if
// the call succeeded, leading to loss of funds or inconsistent state)."#;
//
// pub const DOS_UNEXPECTED_REVERT: &str = r#"Check if the contract aggregates multiple external calls in a single
// function (for example, sending Ether to a list of addresses or calling multiple contracts). If so, determine
// whether a malicious or failing callee can cause a revert of the entire function. Highlight any code where one
// recipient’s failure (revert or out-of-gas) would block the function (e.g., require(ok) in a loop over external calls).
// Suggest using the withdrawal pattern or otherwise handling each call in isolation to avoid a DoS via unexpected revert."#;
//
// pub const DOS_GAS_LIMIT: &str = r#"Identify any function that might exceed the block gas limit due to unbounded computation.
// Look for loops that iterate over dynamic arrays or mappings with no fixed upper bound, or functions that perform
// heavy computation proportional to user-controlled input size. If a data structure can grow indefinitely (e.g., an
// array of users) and the contract has a function iterating over it, point out that as a gas limit DoS risk.
// Explain that once the array becomes large enough, the function will run out of gas and revert on every call,
// causing a Denial of Service."#;
//
// pub const TX_ORIGIN: &str = r#"Check if the contract uses tx.origin anywhere, especially in require statements
// for access control. If so, explain that this is a security vulnerability: an attacker can create a contract that
// calls the function, and trick the privileged user into initiating a transaction to the attacker’s contract. Since
// tx.origin will still be the user’s address, the check passes and the attacker’s contract gains access. Recommend
// using msg.sender for access control and never relying on tx.origin."#;
//
// pub const RANDOMNESS: &str = r#"Identify any use of block variables for randomness (e.g., block.timestamp,
// block.difficulty/prevrandao, blockhash, etc.). If found, point out that these are not truly random: miners or
// validators can influence them within certain bounds or foresee them. Explain how an attacker or miner could
// exploit this (for example, by timing a call or manipulating a timestamp to win a game or lottery). Recommend
// using a secure randomness source or a commit-reveal pattern instead of relying on block attributes for critical
// decisions."#;
//
// pub const STORAGE_VARIABLE: &str = r#"Check for uninitialized storage variables or pointers. Look at any local
// variables of reference type (arrays, structs, mappings) declared in functions without initialization. These can
// unintentionally reference storage slot 0 or other slots, causing corruption of state variables. If found, explain
// which state variables could be overwritten by writes to this uninitialized variable. Also, check for state variables
// that should be set but aren’t (e.g., an owner not initialized in a proxy contract’s context). Warn that uninitialized
// storage references can lead to unpredictable and exploitable behavior."#;
//
// pub const DELEGATE_CALL: &str = r#"Examine the contract for any usage of delegatecall. If found, determine whether
// the target address of the delegatecall can be influenced by an external party. If the delegatecall target is not
// strictly a trusted contract (for example, if it’s directly using a function parameter or a storage variable that
// users can modify), flag this as a critical vulnerability. Explain that an attacker could supply a malicious contract
// to execute within this contract’s context, allowing them to modify storage or selfdestruct the contract. Also mention
// any signs of proxy patterns – ensure that upgradeable proxy contracts properly restrict upgrades and maintain storage
// compatibility. Highlight any instance where delegatecall could lead to arbitrary code execution in the contract’s context."#;
//
// pub const SELF_DESTRUCT: &str = r#"Check if the contract can be self-destructed. Look for any usage of selfdestruct.
// If present, verify that it is properly restricted to an authorized account (e.g., only the contract owner can trigger it).
// If it’s unrestricted or can be indirectly triggered by anyone, explain that an attacker could call this function to
// destroy the contract and seize its Ether. Emphasize the severity: an unprotected selfdestruct allows a complete
// takeover/denial-of-service of the contract."#;
//
// pub const UNEXPECTED_ETH: &str = r#"Check if the contract’s logic makes assumptions about its Ether balance. If you
// see code comparing address(this).balance to some expected value or using it in a conditional or calculation along
// with internal accounting, consider the force-feeding attack. Explain that an attacker can send Ether to the contract
// without calling its functions (via selfdestruct), causing address(this).balance to change without the contract updating
// its internal state. This could break invariants or enable unforeseen outcomes. Flag any instance where logic could be
// subverted by an unexpected increase in contract balance."#;
//
// pub const FLOATING_PRAGMA: &str = r#"Check the Solidity pragma at the top of the contract. If it is a floating pragma
// (using ^ or open-ended ranges), note that this can lead to the code being compiled with a newer compiler that might
// have undiscovered bugs or differences. Explain that using a fixed compiler version (or locking the range to a safe,
// tested version) is recommended for security. If the pragma is, for example, ^0.8.0, point out the importance of
// compiling with a known safe version (like 0.8.17) rather than an arbitrary latest 0.8.x."#;
//
// pub const DEFAULT_VISIBILITIES: &str = r#"Check if any functions in the contract have default visibility (i.e., no
// visibility modifier is specified). If so, ensure that their visibility is appropriate for the function’s purpose.
// Functions that should be restricted (e.g., onlyOwner) must have proper access control modifiers. Flag any function
// without an explicit visibility modifier (public, private, internal, external) as a potential vulnerability, as it
// defaults to public and may allow unauthorized access to sensitive operations. Explain the risks of unintended public
// access and recommend explicitly defining visibility for all functions."#;
//
// pub const SHORT_ADDRESS_ATTACK: &str = r#"Identify functions that take parameters of fixed-size types (like addresses)
// and check if they properly validate the input length or handle cases where the input might be shorter than expected.
// Look for functions where a shorter input could be misinterpreted due to the EVM padding it with zeros, potentially
// leading to incorrect data interpretation. If such cases are found, highlight how this could result in unexpected
// behavior, such as incorrect address resolution or data corruption. Recommend adding explicit input validation to
// ensure parameters meet the expected length."#;
//
// pub const FLOATING_POINTS_PRECISION: &str = r#"Check for any arithmetic operations involving division or multiplication
// where precision might be lost. Since Solidity lacks native floating-point support, ensure that the contract uses a
// consistent precision level (e.g., 18 decimals for ERC20 tokens) and that operations are performed in a way that
// minimizes precision loss, such as multiplying before dividing. Flag any division operations that might truncate
// results or calculations that could lead to inaccuracies in financial logic. Suggest using fixed-point arithmetic
// libraries or ordering operations to preserve precision."#;
//
// pub const OPERATION_MANAGEMENT_DOS: &str = r#"Check for scenarios where the loss of access to critical roles (e.g.,
// the contract owner) could lead to a denial of service. Identify functions or logic that rely on a single address
// (e.g., owner) for critical operations like configuration changes, fund withdrawals, or contract upgrades. If no
// fallback or recovery mechanism exists, flag this as a vulnerability. Explain that loss of the private key for such
// an address could render the contract inoperable. Recommend implementing multi-signature wallets or emergency recovery
// mechanisms to mitigate this risk."#;
//
// pub const SAVING_CONFIDENTIAL_DATA: &str = r#"Check if the contract stores any sensitive or confidential data, such as
// private keys, user personal information, or unencrypted secrets. If found, verify whether this data is properly
// encrypted or hashed before storage, as all data on the blockchain is publicly accessible. Flag any unencrypted or
// unhashed sensitive data as a vulnerability. Explain that public exposure of such data could lead to exploitation or
// privacy breaches. Recommend storing sensitive data off-chain or using cryptographic techniques like hashing or
// encryption."#;
//
// pub const REPLAY_SIGNATURES_ATTACK: &str = r#"If the contract uses signatures for authentication, check if there is a
// mechanism to prevent replay attacks, such as using a nonce that increments with each use. Look for functions that
// verify signatures without checking a unique identifier (e.g., nonce or timestamp). If no such mechanism exists,
// flag this as a vulnerability. Explain that an attacker could reuse a valid signature to execute unauthorized
// transactions. Recommend implementing a nonce system or other replay protection mechanisms to ensure each signature
// is used only once."#;
//
// pub const WRONG_INHERITANCE: &str = r#"Review the inheritance structure of the contract. Ensure that the order of
// inheritance is correct and that there are no conflicts in function or variable names across parent contracts.
// Pay attention to the use of the `virtual` and `override` keywords to ensure proper overriding of functions. If
// conflicts or incorrect inheritance patterns are detected, flag them as vulnerabilities. Explain how such issues
// could lead to unexpected behavior or security flaws, such as incorrect function execution. Recommend verifying
// the inheritance hierarchy and using explicit overrides to resolve ambiguities."#;
//
// pub const ACCESS_OUTSIDE_ARRAY_LIMITS: &str = r#"Check for any operations on arrays where the index might be out of
// bounds. Look for array accesses (e.g., array[i]) where the index `i` is not validated against the array’s length.
// If such cases are found, flag them as vulnerabilities. Explain that accessing non-existent elements could cause
// runtime errors or be exploited to manipulate contract logic. Recommend adding bounds checking (e.g.,
// `require(i < array.length)`) before any array access to ensure safe operation."#;
//
// pub const CONTRACTS_WITH_ZERO_CODE: &str = r#"Check if the contract has any logic that depends on the code of the
// caller (e.g., checking if `msg.sender` is a contract using `extcodesize`). Ensure that such checks are properly
// implemented, considering that contracts can be self-destructed, resulting in zero code size. If the contract
// assumes a non-zero code size for access control, flag this as a vulnerability. Explain that a self-destructed
// contract or an empty contract could bypass these checks, gaining unauthorized access. Recommend using alternative
// access control mechanisms that do not rely on caller code size."#;
//
// pub const ORACLE_MANIPULATION: &str = r#"Check if the contract relies on external oracles for critical data, such
// as price feeds, random numbers, or other off-chain information. Identify functions that use oracle data for
// sensitive operations (e.g., financial calculations, liquidations, or access control). If the contract uses a
// single oracle source or a source that could be manipulated (e.g., a single DEX price feed), flag this as a
// vulnerability. Examine whether the contract implements safeguards like multiple oracle aggregation, medianization,
// or time-weighted average prices (TWAP). Explain that an attacker could manipulate the oracle (e.g., via flash loans
// or market manipulation) to provide false data, leading to incorrect contract behavior or financial losses.
// Recommend using decentralized, manipulation-resistant oracles (e.g., Chainlink) or aggregating multiple sources
// to mitigate this risk."#;
