use crate::llm_review::{
    agent::agent_enums::{all_enum_variants, generate_enum_list},
    threat_models::actors::{Actor, ActorAbuse, RoleType, ACTOR_CENTRIC_VULN_PATTERNS},
};

pub fn generate_actors_prompt() -> String {
    let role_types = generate_enum_list(&all_enum_variants::<RoleType>());
    format!(
        r#"
        Your job is to **propose 3-7 actors** for the target contract and its role in the wider protocol.
        Task:
        1. Propose 3-7 actor, with description, and role type
        2. Enumerate all **capabilities** for EACH actor has against the target contract:
            - which functions can they call
            - which parameters they can choose
            - which external contracts they can influence (tokens, AMMs, oracles, routers)

        ## Deliverables
            - name: describe actor in under 10 words or less, i.e. "Evicted signer behind checkpointer", "Unprivileged user providing initial liquidity"
            - role type (pick one): {role_types}
            - description: full description of actor
            - capabilities: all capabilities of the actor as array of string, i.e. ["Call deposit() with arbitrary amount and recipient"]

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
