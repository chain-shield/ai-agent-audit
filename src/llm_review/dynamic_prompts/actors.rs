use crate::llm_review::{
    agent::agent_enums::{all_enum_variants, generate_enum_list},
    threat_models::actors::{Actor, ActorAbuse, RoleType, ACTOR_CENTRIC_VULN_PATTERNS},
};

pub fn generate_actors_prompt() -> String {
    let role_types = generate_enum_list(&all_enum_variants::<RoleType>());
    format!(
        r#"
            Your task: Identify ALL actors who can interact with or influence the target contract, 
            including adversarial actors who may exploit edge cases, race conditions, or unintended 
            interactions.

            ## SYSTEMATIC ACTOR DISCOVERY PROCESS

            Follow this 5-step process to ensure comprehensive coverage:

            ### STEP 1: Map All Entry Points

            Identify every way the contract can be called or influenced:

            1. **Direct Function Calls**:
            - List all external and public functions
            - Who can call each function? (anyone, specific roles, only self)
            - What parameters do they accept?

            2. **Callback/Hook Mechanisms**:
            - Does the contract call external contracts?
            - Are there before/after hooks or callbacks?
            - Can external contracts influence execution flow?

            3. **Delegatecall Targets**:
            - Does the contract delegatecall to other contracts?
            - What storage can delegatecall targets access?
            - Can delegatecall targets bypass access controls?

            4. **Event Listeners**:
            - What events does the contract emit?
            - Who might listen to these events off-chain?
            - Can event data be used to front-run or exploit?

            5. **Storage Dependencies**:
            - Does the contract read from other contracts' storage?
            - Does it depend on oracles, price feeds, or external state?
            - Who controls those dependencies?

            ---

            ### STEP 2: Identify Role-Based Actors

            For each role or privilege level in the contract:

            1. **Primary Roles**:
            - Owner, admin, governance
            - Authorized signers or operators
            - Token holders or stakers

            2. **Delegated Roles**:
            - Session keys or restricted signers
            - Guardians or recovery agents
            - Relayers or keepers

            3. **Implicit Roles**:
            - Any caller (permissionless functions)
            - Contract itself (self-calls)
            - Zero address or special addresses

            For each role, ask:
            - What can they do directly?
            - What can they do by combining multiple actions?
            - What happens if they're malicious or compromised?
            - What happens if they're removed but retain some access?

            ---

            ### STEP 3: Identify Interaction-Based Actors

            For each external interaction:

            1. **External Contracts Called**:
            - DeFi protocols (AMMs, lending, bridges)
            - Token contracts (ERC20, ERC721, ERC1155)
            - Oracles or price feeds
            - Other protocol components

            2. **External Contracts Calling In**:
            - Who can trigger callbacks?
            - Who can call public functions?
            - Who can send ETH or tokens?

            3. **Cross-Contract Interactions**:
            - Can external contracts reenter?
            - Can they manipulate state between calls?
            - Can they grief or DoS operations?

            For each external actor, ask:
            - Can they revert to influence execution flow?
            - Can they return malicious data?
            - Can they consume gas to DoS?
            - Can they donate assets to manipulate logic?

            ---

            ### STEP 4: Identify Temporal/MEV Actors

            For each transaction or operation:

            1. **Mempool Observers**:
            - Who can see pending transactions?
            - What information is leaked before execution?
            - Can they front-run, back-run, or sandwich?

            2. **Failed Transaction Exploiters**:
            - What happens when transactions revert?
            - Is state consumed before or after execution?
            - Can signatures/nonces be reused after revert?

            3. **Cross-Chain Actors**:
            - Can operations be replayed on other chains?
            - Are there chain-specific protections?
            - Can cross-chain state be manipulated?

            4. **Time-Based Actors**:
            - Who benefits from delays or timelocks?
            - Can timelocks be bypassed or extended?
            - Can operations be front-run before expiry?

            ---

            ### STEP 5: Identify Missing Actors Checklist

            Review this checklist to catch commonly missed actors:

            **Storage Manipulation Actors**:
            - [ ] Delegatecall targets that can write arbitrary storage
            - [ ] Contracts that can manipulate shared storage slots
            - [ ] Actors who can corrupt storage via reentrancy

            **Persistence Actors**:
            - [ ] Actors whose permissions persist after revocation
            - [ ] Actors who can use old/stale authorizations
            - [ ] Actors who benefit from missing cleanup logic

            **Race Condition Actors**:
            - [ ] Actors who can submit concurrent transactions
            - [ ] Actors who can exploit check-then-act patterns
            - [ ] Actors who can manipulate state between check and use

            **Callback/Hook Actors**:
            - [ ] Contracts receiving before/after execution hooks
            - [ ] Contracts called during critical operations
            - [ ] Contracts that can revert to grief or manipulate flow

            **Partial Execution Actors**:
            - [ ] Actors who can extract partial operations from batches
            - [ ] Actors who benefit from partial success/failure
            - [ ] Actors who can manipulate error handling flags

            **Off-Chain Actors**:
            - [ ] Services providing proofs or attestations
            - [ ] Relayers or bundlers submitting transactions
            - [ ] Indexers or watchers monitoring events

            **Griefing Actors**:
            - [ ] Actors who can DoS at low cost
            - [ ] Actors who can block others' operations
            - [ ] Actors who profit from causing failures

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
