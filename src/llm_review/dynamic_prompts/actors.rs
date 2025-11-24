use crate::llm_review::{
    agent::agent_enums::{all_enum_variants, generate_enum_list},
    threat_models::actors::{Actor, ActorAbuse, RoleType, ACTOR_CENTRIC_VULN_PATTERNS},
};

pub fn generate_actors_prompt() -> String {
    let role_types = generate_enum_list(&all_enum_variants::<RoleType>());
    format!(
        r#"
        Your job is to **propose 3-7 actors** for the target contract and its role in the wider protocol.

        ## Task

        1. **Identify 3-7 distinct actors** who interact with or can influence the target contract
        2. **For EACH actor, enumerate ALL exploitable capabilities** - focus on what could be abused for profit or to harm others
        3. **CRITICAL**: Always consider what happens when transactions FAIL or REVERT - are signatures still valid? Can state be exploited?
        4. **CRITICAL**: Always include mempool observers / MEV searchers as potential actors if the contract involves signatures, multi-call, or valuable operations

        ## What to Look For When Identifying Actors

        Consider these actor types and their potential capabilities:

        ### Direct Protocol Users
        - **UnprivilegedUser**: Can they manipulate deposit/withdrawal timing, amounts, or recipients?
        - **LiquidityProvider**: Can they exploit deposit/withdrawal mechanics, share price calculations, or first depositor advantages?
        - **TraderOrArbitrageur**: Can they exploit price calculations, slippage, MEV opportunities, or order execution?
        - **Liquidator**: Can they manipulate collateral ratios, liquidation triggers, or auction mechanics?
        - **LargeHolder**: Can they use large positions to manipulate markets, governance, or protocol state?

        ### Privileged or Special Actors
        - **SignerOrKeyHolder**: Can they abuse delegated permissions, replay signatures, or exploit session key mechanics?
        - **ModuleOrPlugin**: Can they inject malicious logic via delegatecall, callbacks, or untrusted extensions?
        - **RelayerOrKeeper**: Can they manipulate transaction ordering, timing, execution, or censor transactions?
        - **OracleOrPriceFeed**: Can they provide stale, manipulated, or sandwichable price data?
        - **SmartAccountOrWallet**: Can they exploit wallet-specific logic, signature validation, or multi-call mechanics?

        ### Adversarial External Actors (ALWAYS CONSIDER THESE!)
        - **TraderOrArbitrageur** (as MEV searcher / mempool observer): Can they monitor mempool, frontrun, sandwich, or extract MEV from pending transactions?
        - **TraderOrArbitrageur** (as signature extractor): Can they extract valid signatures from FAILED transactions and replay them?
        - **UnprivilegedUser** (as flash loan attacker): Can they manipulate state with borrowed capital and revert?
        - **ExternalDefiProtocol** (as malicious contract): Can they exploit callbacks, reentrancy, or cross-contract interactions?
        - **UnprivilegedUser** (as first interactor): Can they exploit uninitialized state or inflate share prices?

        ### Cross-Protocol Actors
        - **ExternalDefiProtocol**: Can they be manipulated to affect this contract (AMMs, lending markets, yield farms)?
        - **TokenContract**: Can they use hooks, callbacks, rebasing, or non-standard behavior to exploit?
        - **BridgeOrMessenger**: Can they replay messages, spoof cross-chain data, or exploit finality assumptions?
        - **RouterOrAggregator**: Can they route calls maliciously or exploit multicall/batching mechanics?
        - **VaultOrPool**: Can they manipulate share prices, exchange rates, or accounting invariants?
        - **ChainInfrastructure**: Can sequencers/validators reorder or censor transactions for profit?
        - **OffchainService**: Can off-chain services provide malicious payloads, signatures, or proofs?

        ## How to Enumerate Exploitable Capabilities

        For each actor, list capabilities that could be **weaponized for profit or harm**:

        ### On-Chain Capabilities
        - **Function calls**: Which functions can they call? With what parameters?
        - **Timing control**: Can they choose when to execute (frontrun, delay, sandwich)?
        - **Amount control**: Can they choose amounts to maximize impact (dust, huge, zero)?
        - **Recipient control**: Can they redirect funds or benefits to arbitrary addresses?
        - **State manipulation**: Can they force specific state transitions or bypass checks?

        ### Off-Chain Capabilities (CRITICAL FOR MEMPOOL ATTACKS!)
        - **Mempool monitoring**: Can they monitor pending transactions and extract information (signatures, calldata, parameters)?
        - **Signature extraction from FAILED transactions**: Can they extract valid signatures from transactions that reverted but didn't consume nonces?
        - **Partial signature replay**: Can they replay extracted signatures with only a subset of the original calls?
        - **Frontrunning with extracted data**: Can they frontrun original transactions using extracted signatures or calldata?
        - **Data manipulation**: Can they provide malicious calldata, signatures, or proofs?

        ### Cross-Contract Capabilities
        - **External contract influence**: Which external contracts can they control or manipulate?
        - **Token manipulation**: Can they manipulate token balances, prices, or approvals?
        - **Oracle manipulation**: Can they influence price feeds or data sources?
        - **Callback exploitation**: Can they inject malicious logic via callbacks or hooks?

        ### Economic Capabilities
        - **Flash loans**: Can they borrow large amounts to manipulate state temporarily?
        - **Donation attacks**: Can they donate assets to manipulate calculations?
        - **Liquidity manipulation**: Can they drain or inject liquidity to affect prices?

        ### Failure & Revert Scenarios (CRITICAL - ALWAYS ANALYZE!)
        - **Signature validity after revert**: If a transaction reverts, is the signature still valid for replay?
        - **Nonce consumption on failure**: Are nonces consumed before or after execution? Can failed txs be replayed?
        - **Partial execution before revert**: Can actor benefit from state changes that occurred before the revert?
        - **Multi-call atomicity**: If a multi-call transaction fails, can individual calls be extracted and replayed separately?
        - **Error handling behavior**: Does REVERT_ON_ERROR vs CONTINUE_ON_ERROR affect signature validity or nonce consumption?

        ## Deliverables

        For each actor, provide:
        - **name**: Describe actor in under 10 words, e.g., "Mempool observer monitoring failed multi-call sessions", "First depositor before any liquidity exists"
        - **role_type** (pick one): {role_types}
        - **description**: Full description of actor and their relationship to the protocol
        - **capabilities**: Array of **specific, exploitable capabilities**, e.g.:
          - "Monitor mempool for failed multi-call session transactions"
          - "Extract valid signatures from failed transactions where nonce wasn't consumed"
          - "Replay partial signatures with only profitable subset of calls"
          - "Frontrun legitimate multi-call sessions to execute partial operations"
          - "Donate large amounts to inflate share price before first real deposit"
          - "Use flash loans to manipulate oracle price within single transaction"
          - "Call deposit() with dust amount to trigger rounding errors"
          - "Withdraw immediately after deposit to exploit stale price data"

        **Focus on capabilities that enable game-theoretic attacks, not just normal protocol usage.**

        **NOTE**: Admin roles are assumed trusted actors unless specified other in scope.
        For this analysis, trusted actors CANNOT act maliciously.
"#,
    )
}
pub fn generate_actor_abuses_prompt(actors: &[Actor]) -> String {
    let actors_capabilities = generate_formated_list_from_actor_data(actors);

    let exploit_enums = generate_enum_list(ACTOR_CENTRIC_VULN_PATTERNS);
    format!(
        r#"
        Your job is to **propose and evaluate all the ways bad actors could game or abuse the system** for the target contract and its role in the wider protocol.

        ## Your goal

        1. For each Actor (listed below) and capability pairing, propose **ways this actor could improve their payoff** at the expense of others or the protocol, assuming:
            - the actor is rational and adversarial,
            - admins follow the written spec (not malicious, not careless),
            - external protocols behave as designed.
        2. For each Actor/capability abuse found return
        - title: 50 word or less C4-style headline, explaining exploit
        - scenario: step by step breakdown of how exploit is executed
        - likely category (pick one): {exploit_enums}
        - assets at risk: i.e. ["vault diposits"]
        - victim: Who suffers? (LP, DAO, user, MEV, protocol treasury, etc.)

        ## Actor and Corresponding Capabilities to Examine
        {actors_capabilities}

        ## Key Attack Vectors to Consider

        When analyzing each actor/capability pair, pay special attention to these game-theoretic attack patterns:

        ### 1. Signature & Replay Attacks
        - **Partial signature replay**: Can actor extract valid signatures from failed multi-call transactions and replay only profitable subset?
        - **Nonce manipulation**: Does failed execution leave nonces unconsumed, allowing signature reuse?
        - **Cross-chain replay**: Can signatures be replayed across forks, L2s, or different chain IDs?
        - **Permit frontrunning**: Can actor frontrun permit() calls to grief or steal approvals?

        ### 2. Mempool & Frontrunning Attacks
        - **Partial execution frontrunning**: Can actor monitor mempool and frontrun multi-call transactions to execute only profitable subset?
        - **Sandwich attacks**: Can actor sandwich user transactions to extract value?
        - **Transaction griefing**: Can actor deliberately cause legitimate transactions to fail for competitive advantage?

        ### 3. Multi-call & Atomicity Violations
        - **Breaking atomicity**: Can actor break intended atomicity of multi-call transactions (e.g., swap without reversal)?
        - **Conditional execution bypass**: Can actor execute calls that were meant to be conditional on other calls succeeding?
        - **Revert exploitation**: Can actor benefit from partial state changes before revert?

        ### 4. Economic & Protocol Manipulation
        - **Flash loan attacks**: Can actor use flash loans to manipulate protocol state, prices, or governance?
        - **Oracle manipulation**: Can actor manipulate price feeds (TWAP pinning, spot price manipulation, low liquidity exploitation)?
        - **Slippage exploitation**: Can actor exploit missing or insufficient slippage protection?
        - **Share price manipulation**: Can actor inflate/deflate share prices in vaults (ERC4626, LP tokens)?

        ### 5. Access Control & Authorization
        - **Privilege escalation**: Can actor gain unauthorized roles or bypass access controls?
        - **Session key abuse**: Can actor abuse session keys, delegated permissions, or module authorities?
        - **Governance manipulation**: Can actor manipulate voting, delegation, or timelock mechanisms?

        ### 6. Reentrancy & Callback Attacks
        - **Classic reentrancy**: Can actor reenter functions to drain funds or manipulate state?
        - **Read-only reentrancy**: Can actor exploit view functions during callbacks to get stale data?
        - **Cross-function reentrancy**: Can actor reenter different functions to violate invariants?
        - **Callback griefing**: Can actor use callbacks to DoS or grief other users?

        ### 7. Accounting & Invariant Violations
        - **Insolvency attacks**: Can actor degrade protocol solvency for profit?
        - **Forced asset injection**: Can actor force-send ETH/tokens to break strict equality checks?
        - **Decimal mismatch**: Can actor exploit token decimal differences for value extraction?
        - **Fee-on-transfer exploitation**: Can actor exploit assumptions about token transfer amounts?

        ### 8. State Synchronization & Lifecycle Attacks
        - **Stale state exploitation**: Can actor use outdated checkpoints, snapshots, or cached values for profit?
        - **Epoch/index manipulation**: Can actor manipulate epoch transitions, index updates, or checkpoint timing?
        - **Maturity bypass**: Can actor bypass maturity gates, vesting schedules, or time-locked restrictions?
        - **State machine violations**: Can actor force invalid state transitions or skip required lifecycle steps?
        - **Timestamp manipulation**: Can actor exploit block.timestamp dependencies for miner-extractable value?

        **NOTE**: Admin roles are assumed trusted actors unless specified other in scope.
        For this analysis, trusted actors CANNOT act maliciously.
"#,
    )
}

pub fn generate_actor_abuse_verify_prompt(abuse: &ActorAbuse) -> String {
    let verify_json = get_verify_actor_abuse_json();
    let actor_abuse_report = generate_formatted_actor_abuse_list(&[abuse.clone()]);

    format!(
        r#"
        ## Your task: decide if the reported actor abuse / protocol gaming scenario is legit or not.
        
        You should return `"true"` if actor abuse is legit and scenario, likely category, assets at risk, and victim checkout. 
        Otherwise return `"false"`.

        ## Actor Abuse to Verify
        {report} 

        ## OUTPUT REQUIREMENTS 

        *Please respond with ONLY valid JSON in the following exact format:*

        {json}

        **Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 
        "#,
        json = verify_json,
        report = actor_abuse_report
    )
}

pub fn get_actor_abuse_json() -> String {
    let exploit_enum = generate_enum_list(ACTOR_CENTRIC_VULN_PATTERNS);

    format!(
        r#"

        ## OUTPUT REQUIREMENTS

        - STRICT JSON ONLY (no markdown, no comments):

        {{
        "abuses": [
            {{
            "actor_name": "Insert exact actor name",
            "capability": "Insert exact actor capability",
            "title": "50 word or less C4-style headline, explaining exploit",
            "scenario": "step by step breakdown of how exploit is executed",
            "category": {exploit_enum},
            "assets_at_risk": ["vault deposits","protocol treasury"],
            "victim": "Who suffers? (LP, DAO, user, MEV, protocol treasury, etc.)"
            }}
        ]
        }}

        - If no vulnerabilities are found, return:

        {{
        "abuses": []
        }}

        **Note: **NO extra text** and **NO code fencing** in response, just plain JSON.
        **Please double-check opening and closing brackets: `}}` and `]`, make sure
        they match up correctly.
    "#,
    )
}

pub fn get_actor_list_json() -> String {
    let role_types = generate_enum_list(&all_enum_variants::<RoleType>());
    format!(
        r#"

        ## OUTPUT REQUIREMENTS
        
        - STRICT JSON ONLY (no markdown, no comments):

        {{
        "actors": [
            {{
            "name": "describe actor in under 10 words or less i.e. Evicted signer behind checkpointer, Unprivileged user providing initial liquidity",
            "role_type": "{role_types}",
            "description": "full description of actor",
            "capabilities": ["Call deposit() with arbitrary amount and recipient","can approve token transfer amounts out of treasury"]
            }}
        ]
        }}

        **Note: **NO extra text** and **NO code fencing** in reponse, just plain JSON. 
        **Please double-check opening and closing brakets: `}}` and `]`, make sure 
        they match up correctly.
    "#,
    )
}
pub fn get_verify_actor_abuse_json() -> String {
    format!(
        r#"
        {{
            "is_legit_abuse": true|false,
            "why_its_not_legit": "in 40 words less explain why NOT legit (OMIT if legit)"
        }}
        "#
    )
}

pub fn generate_formatted_actor_abuse_list(abuses: &[ActorAbuse]) -> String {
    let mut abuse_list = String::new();

    for abuse in abuses {
        let abuse_item = generate_formatted_actor_abuse(abuse);
        abuse_list.push_str(&abuse_item);
    }
    abuse_list
}

pub fn generate_formatted_actor_abuse(abuse: &ActorAbuse) -> String {
    let mut abuse_item = String::new();
    abuse_item.push_str("\n");
    abuse_item.push_str(&format!("### Actor Name: {}\n", &abuse.actor_name));
    abuse_item.push_str("\n");

    abuse_item.push_str(&format!("### Actor Capability: {}\n", &abuse.capability));
    abuse_item.push_str("\n");

    abuse_item.push_str(&format!("### Abuse Title: {}\n", &abuse.title));
    abuse_item.push_str("\n");

    abuse_item.push_str(&format!("### Scenario: {}\n", &abuse.scenario));
    abuse_item.push_str("\n");

    abuse_item.push_str(&format!(
        "### Likely Category: {}\n",
        &abuse.category.to_string()
    ));
    abuse_item.push_str("\n");

    abuse_item.push_str(&format!(
        "### Assets at Risk: {}\n",
        &abuse.assets_at_risk.join(", ")
    ));
    abuse_item.push_str("\n");

    abuse_item.push_str(&format!("### Victim: {}\n", &abuse.victim));
    abuse_item.push_str("\n");

    abuse_item
}

pub fn generate_formated_list_from_actor_data(actors: &[Actor]) -> String {
    let mut actor_list = String::new();

    for actor in actors {
        actor_list.push_str("\n\n");
        actor_list.push_str(&format!("### Actor Name: {}\n", &actor.name));
        actor_list.push_str("\n");

        actor_list.push_str(&format!(
            "### Role Type: {}\n",
            &actor.role_type.to_string()
        ));
        actor_list.push_str("\n");

        actor_list.push_str(&format!("### Description: {}\n", &actor.description));
        actor_list.push_str("\n");

        actor_list.push_str("### Capabilities\n");
        for capability in &actor.capabilities {
            actor_list.push_str(&format!("- {}\n", capability));
        }
        actor_list.push_str("\n");
    }

    actor_list
}
