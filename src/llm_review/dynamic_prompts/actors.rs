use crate::{
    config::JSON_REQUIREMENT_SECTION,
    llm_review::{
        agent::agent_enums::{all_enum_variants, generate_enum_list},
        dynamic_prompts::prompt_index,
        threat_models::actors::{Actor, RoleType},
    },
};
use rand::seq::SliceRandom;

pub fn generate_actors_prompt() -> String {
    let role_types = generate_enum_list(&all_enum_variants::<RoleType>());
    let toc = prompt_index::generate_actor_discovery_table_of_contents();
    let section_1_header = prompt_index::generated_section_header("CORE INSTRUCTIONS", 1);
    let section_1_1_header =
        prompt_index::generated_sub_header("SYSTEMATIC ACTOR DISCOVERY PROCESS", 1, 1);
    let section_1_2_header =
        prompt_index::generated_sub_header("STEP 1: MAP ALL ENTRY POINTS", 1, 2);
    let section_1_3_header =
        prompt_index::generated_sub_header("STEP 2: IDENTIFY ROLE-BASED ACTORS", 1, 3);
    let section_1_4_header =
        prompt_index::generated_sub_header("STEP 3: IDENTIFY INTERACTION-BASED ACTORS", 1, 4);
    let section_1_5_header =
        prompt_index::generated_sub_header("STEP 4: IDENTIFY TEMPORAL/MEV ACTORS", 1, 5);
    let section_1_6_header =
        prompt_index::generated_sub_header("STEP 5: IDENTIFY MISSING ACTORS CHECKLIST", 1, 6);
    let section_1_7_header = prompt_index::generated_sub_header("DELIVERABLES", 1, 7);

    format!(
        r#"
{toc}

{section_1_header}

Your task: Identify ALL actors who can interact with or influence the target contract,
including adversarial actors who may exploit edge cases, race conditions, or unintended
interactions.

{section_1_1_header}

Follow this 5-step process to ensure comprehensive coverage:

{section_1_2_header}

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

{section_1_3_header}

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

{section_1_4_header}

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

{section_1_5_header}

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

{section_1_6_header}

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

{section_1_7_header}

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
pub fn get_actor_list_json() -> String {
    let role_types = generate_enum_list(&all_enum_variants::<RoleType>());
    let section_11_header = prompt_index::generated_section_header(
        "JSON OUTPUT REQUIREMENTS",
        JSON_REQUIREMENT_SECTION,
    );

    format!(
        r#"

{section_11_header}

**Output must be strictly valid JSON** with this structure (no extra text or code fencing):
        
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

pub fn generate_formated_list_from_actor_data(
    actors_slice: &[Actor],
    section_num: u8,
) -> (String, String) {
    let actor_title =
        "Potential Bad Actors to Consider When Searching for Security Vulnerabilities";

    let mut actor_list = format!(
        r#"

{}

**NOTE**: The actors below are pertinent to the target contract, please incorporate them in your analysis.

                "#,
        prompt_index::generated_section_header(&actor_title.to_uppercase(), section_num)
    );

    let mut actor_index = prompt_index::generated_table_of_context_header(actor_title, section_num);

    let mut actors = actors_slice.to_vec();

    // Randomize the order of patterns
    let mut rng = rand::rng();
    actors.shuffle(&mut rng);

    for (section, actor) in actors.iter().enumerate() {
        // update table of contents with new entry
        actor_index.push_str(&format!(
            "- {}.{} {}\n",
            section_num,
            section + 1,
            actor.name
        ));

        // add new actor to list
        actor_list.push_str(&prompt_index::generated_sub_header(
            &actor.name.to_uppercase(),
            section_num,
            section + 1,
        ));

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

    (actor_list, actor_index)
}
