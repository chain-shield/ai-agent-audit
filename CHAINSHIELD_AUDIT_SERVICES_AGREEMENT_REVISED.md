# Smart Contract Security Audit Services Agreement

## Introduction

This Smart Contract Security Audit Services Agreement (this "Agreement") is entered into by and between [Auditor] (the "Auditor") and [Client] (the "Client"). The Agreement sets forth the terms under which the Auditor will perform security audit services on the Client's smart contract code. The parties agree as follows:

## 1. Definitions

For the purposes of this Agreement:

"Audit Intake" means any questionnaire, scope document, client-provided assumptions, known-issues list, supported-assets list, trusted-roles list, or other intake material attached to or referenced in an SOW.

"Audit Services" means a formal review of the Protocol's source code to identify potential vulnerabilities, security flaws, and design issues in the on-chain portion of the Protocol. Audit Services do not include off-chain systems, third-party systems, oracle providers, relayers, bridges, external dependencies, or new modules unless expressly agreed in an SOW.

"Authorized Tools" means automated analysis tools, static analyzers, AI-assisted review systems, large language model providers, local or cloud compute, vector databases, storage systems, collaboration tools, and other third-party service providers used by Auditor to perform the Audit Services.

"Client Materials" means code, documentation, credentials, test data, repositories, branches, commits, protocol information, Audit Intake responses, and other materials provided or made available by Client.

"Critical Vulnerability" means a vulnerability that presents a credible risk of imminent or material harm to user funds, Protocol funds, Protocol control, or critical Protocol operations.

"Deliverables" means any audit reports, findings, recommendations, or other work product provided by the Auditor to the Client under this Agreement or an SOW.

"Findings" refers to security issues discovered during the Audit Services. Findings are classified by severity as described in Schedule A unless the SOW states otherwise.

"Pre-existing Auditor Materials" means Auditor's pre-existing tools, scripts, templates, prompts, methodologies, checklists, software, know-how, processes, and generalized security knowledge.

"Protocol" means the Client's smart contract(s) or decentralized application, including only the specified repositories, commits, branches, folders, files, and modules identified in an applicable SOW.

"Second Audit Round" means a limited follow-up review, at the Client's request, covering the Client's code changes made in response to High- or Medium-severity Findings.

## 2. Scope of Services

2.1 Statement of Work. Audit Services will be delivered pursuant to a written SOW describing the Protocol repositories, commits, branches, folders, files, timelines, payment terms, deliverables, and any special instructions. Only those repositories, commits, branches, folders, and files listed in the SOW or attached Audit Intake are within scope. Any additional repositories, new modules, new deployments, or material changes to the Protocol will require a new or amended SOW.

2.2 Out-of-Scope Items. Unless expressly stated otherwise, Audit Services exclude off-chain systems (e.g., governance front ends, dashboards, backend services), external third-party systems, third-party protocol code, oracle providers, relayers, bridges, infrastructure providers, generated files, mocks, scripts, tests, and code or documentation not provided in the SOW. The Client's in-scope integration code and stated trust assumptions for third-party systems may be reviewed if listed in the SOW.

2.3 Audit Intake. The parties may attach a completed audit questionnaire or intake document to an SOW. The Audit Intake may identify scope, supported assets, trusted roles, known issues, accepted risks, invariants, emergency controls, external dependencies, and reporting preferences. The Auditor may rely on the Audit Intake when performing the Audit Services.

2.4 Second Audit Round. At the Client's request, the Auditor may perform a Second Audit Round limited to verifying fixes for previously reported High- and Medium-severity Findings. Unless the SOW states that a Second Audit Round is included, it will be described in a separate SOW and is subject to an additional fee. A Second Audit Round is not a full re-audit unless expressly stated in the SOW.

## 3. Client Responsibilities

3.1 Complete Information. The Client must provide the Auditor with complete and accurate source code, documentation, Audit Intake responses, scope files, known issues, accepted risks, trust assumptions, and access credentials (if any) necessary to perform the Audit Services. The Client must freeze the target branch at the start of the audit; subsequent changes may require a new or amended SOW.

3.2 Testing and Deployment. The Client is solely responsible for implementing fixes, testing the Protocol, reviewing any remediations, and deploying the Protocol on a blockchain network. The Auditor does not assume any responsibility for integration, operation, deployment, monitoring, incident response, or maintaining the Client's security program.

3.3 No Secrets Unless Requested. Client will not provide private keys, seed phrases, production admin keys, unrestricted production credentials, or other sensitive secrets unless expressly requested by Auditor in writing. If access is needed, Client should provide test credentials or least-privilege access whenever possible.

3.4 Client Decisions. Client is solely responsible for deciding whether, when, and how to remediate Findings, ship code, deploy contracts, migrate funds, pause systems, communicate with users, or make governance or operational changes.

## 4. Fees and Payment

4.1 Fee Structure. Fees will be set forth in the SOW. Unless otherwise stated, the Client will pay (i) a non-refundable deposit upon execution of the SOW; and (ii) the remaining fees upon delivery of the audit report. If payment is tied to Findings (e.g., a bounty-style or per-finding arrangement), the SOW must describe how High- or Medium-severity Findings are counted, grouped by root cause, validated, disputed, and paid.

4.2 Invoices; Late Payment; Taxes. Unless the SOW states otherwise, invoices are due upon receipt. Client is responsible for all taxes, duties, bank fees, wire fees, payment processor fees, blockchain gas fees, and similar charges, other than taxes on Auditor's net income. Overdue amounts may accrue interest at the lesser of 1.5% per month or the maximum rate permitted by law.

4.3 Crypto Payments. If the SOW permits payment in digital assets, the SOW must specify the asset, network, wallet address, exchange-rate source, conversion time, and responsibility for gas or network fees. Payment is not deemed received until final settlement on the applicable network or confirmation by Auditor.

4.4 Suspension for Nonpayment. Auditor may suspend work, withhold Deliverables, or pause any Second Audit Round if Client fails to pay undisputed amounts when due.

4.5 Expenses. The Client will reimburse the Auditor for any reasonable, pre-approved expenses incurred in connection with the Audit Services.

## 5. Confidentiality; Data Handling

5.1 Confidentiality. Both parties agree to protect each other's non-public information disclosed in connection with this Agreement ("Confidential Information"). A party will use Confidential Information only for the purposes of performing or receiving services under this Agreement and will not disclose it to any third party without the other party's prior written consent, except as required by law or as permitted in this Agreement.

5.2 Authorized Tools and Service Providers. Client acknowledges that Auditor may use automated analysis tools, static analyzers, AI-assisted review systems, large language model APIs, local or cloud compute, vector databases, storage systems, and collaboration tools to perform the Audit Services. Auditor will not knowingly submit Client Confidential Information to consumer-grade AI services or to services that use such information to train public models, unless Client gives prior written consent.

Where commercially available, Auditor will configure AI and third-party tools so Client Confidential Information is not used to train or improve third-party models. Auditor will use commercially reasonable measures to limit tool access to the minimum information necessary to perform the Audit Services and to protect Client Confidential Information.

Client must not provide private keys, seed phrases, production admin keys, unrestricted production credentials, or other high-risk secrets unless expressly requested by Auditor in writing and subject to additional safeguards agreed by the parties. Any credentials provided should be test credentials or least-privilege credentials whenever possible.

Upon request, Auditor will identify the material categories of third-party tools used for the Audit Services. Any heightened restrictions, including named-provider restrictions, approved-provider-only workflows, no-cloud review, local-only analysis, or air-gapped review, must be stated in the applicable SOW and may require additional fees or timeline changes.

5.3 Survival. Confidentiality obligations survive termination of this Agreement for three (3) years, except that any materials constituting trade secrets under applicable law will be protected indefinitely or until such information ceases to qualify as a trade secret through no fault of the receiving party.

## 6. Intellectual Property; Report Usage

6.1 Client IP. The Client retains all right, title, and interest in and to the Protocol, including all intellectual property rights. Nothing in this Agreement transfers ownership of the Client's code, documentation, trademarks, or other materials to the Auditor.

6.2 License to Auditor. The Client grants the Auditor a non-exclusive, royalty-free license to use, reproduce, analyze, execute, test, and create temporary copies of the Protocol and Client Materials solely for the purpose of performing the Audit Services and exercising Auditor's rights under this Agreement. This license ends upon completion or termination of the Services, except as needed for archival, legal, compliance, security, and dispute-resolution purposes.

6.3 Auditor Materials. Auditor retains all right, title, and interest in its Pre-existing Auditor Materials, including improvements that are not specific to Client Confidential Information. No rights are granted to Client except as expressly stated in this Agreement or an SOW.

6.4 Audit Report Usage. Subject to full payment, Auditor grants Client a limited, non-exclusive, non-transferable license to use the final audit report internally for Client's own security purposes. Client may publicly release the final audit report only with Auditor's prior written consent and only in full, unmodified form, except for redactions approved by Auditor or required by law. Client may not state or imply that Auditor certifies, guarantees, endorses, approves, or insures the Protocol, token, business model, or security of the Protocol.

## 7. Representations and Regulatory Compliance

The Client makes the following representations and acknowledgments:

Ownership and Rights. The Client warrants that it owns, or has lawful permission to use, all code, documentation, credentials, test data, and other materials provided for audit, and that providing those materials to Auditor will not infringe or misappropriate the rights of any third party.

No Malicious Code or Unlawful Purpose. The Client warrants that, to its knowledge, the materials provided for audit do not contain backdoors, malware, intentionally hidden unauthorized access mechanisms, or illicit code designed to evade detection or harm users, and that Client is not engaging Auditor for any unlawful purpose.

Client Compliance Responsibility. The Client is solely responsible for determining and complying with all laws and regulations applicable to the Protocol, tokens, business model, users, jurisdictions, sanctions, anti-money-laundering obligations, tax, securities, commodities, money transmission, consumer protection, data protection, and related matters. The Auditor does not provide legal, regulatory, tax, financial, investment, or compliance advice.

Sanctions and Restricted Parties. The Client represents that neither Client nor, to Client's knowledge, its controlling persons are subject to applicable sanctions restrictions, located in a comprehensively sanctioned jurisdiction, or prohibited from receiving the Audit Services under applicable law.

Export Controls. The Client will not export, re-export, transfer, or use the Deliverables in violation of applicable export control or sanctions laws.

## 8. Vulnerability Disclosure

If the Auditor identifies a Critical Vulnerability presenting a credible risk of imminent harm to Protocol users, user funds, or critical infrastructure, Auditor will promptly notify Client using the security contacts identified in the SOW and will cooperate in good faith on remediation. Client will use commercially reasonable efforts to acknowledge the notice promptly and pursue remediation on a timeline appropriate to the severity and exploitability of the vulnerability.

Auditor will not publicly disclose non-public vulnerability details without Client's prior written consent, except where disclosure is legally required or reasonably necessary to prevent imminent harm. Any such disclosure will be limited to the minimum information necessary and, where practical, directed to appropriate affected parties, maintainers, infrastructure providers, or responsible disclosure programs.

## 9. No Warranty; Limitation of Liability; Indemnification

9.1 No Warranty. The Auditor provides the Audit Services and Deliverables "as is" and disclaims all warranties, express or implied, including warranties of merchantability, fitness for a particular purpose, title, and non-infringement. The Auditor does not warrant that the Audit Services will find all vulnerabilities, that all Findings are exhaustive or error-free, or that the Protocol will be secure after remediation.

9.2 No Certification or Third-Party Reliance. The Deliverables are not a certification, guarantee, endorsement, insurance policy, or legal, financial, investment, regulatory, tax, or compliance opinion. No third party is entitled to rely on the Deliverables unless Auditor expressly agrees in a signed writing.

9.3 Limitation of Liability. To the maximum extent permitted by law, the Auditor's total liability under this Agreement for any claim is limited to the amounts actually paid by the Client to the Auditor under the applicable SOW in the twelve (12) months preceding the claim. In no event will either party be liable for any indirect, special, consequential, exemplary, incidental, or punitive damages, or for lost profits, lost revenue, loss of token value, loss of goodwill, loss of data, protocol downtime, trading losses, market losses, or blockchain/network losses, even if advised of the possibility of such damages.

9.4 Client Indemnification. The Client will defend, indemnify, and hold the Auditor harmless against any third-party claims arising out of or relating to (i) the Client's Protocol, tokens, products, business operations, or deployment decisions; (ii) Client's breach of representations, warranties, or obligations under this Agreement; (iii) Client's violation of applicable law; (iv) Client's public statements, marketing, or publication of the Deliverables; or (v) Client's failure to remediate, test, or safely deploy the Protocol. Nothing in this Agreement requires the Client to indemnify the Auditor for the Auditor's own gross negligence, willful misconduct, or fraud.

## 10. Term and Termination

10.1 Term. This Agreement commences on the Effective Date and continues until terminated by either party upon written notice or completion of the Audit Services.

10.2 Termination for Breach. Either party may terminate this Agreement if the other party materially breaches its obligations and fails to cure within fourteen (14) days after receiving notice. If the breach involves nonpayment, misuse of the Deliverables, disclosure of Confidential Information, unlawful conduct, or security risk, the non-breaching party may suspend performance immediately while the breach remains uncured.

10.3 Effect of Termination. Termination does not affect rights and obligations accrued prior to termination, including payment obligations, protection of Confidential Information, intellectual property rights, disclaimers, limitations of liability, indemnification, and dispute resolution. Client will pay Auditor for all Services performed and non-cancelable expenses incurred through the effective termination date.

## 11. Dispute Resolution and Governing Law

Any dispute arising out of or relating to this Agreement will be finally settled by binding arbitration administered by the American Arbitration Association (AAA) under its Commercial Arbitration Rules. The seat of arbitration and governing law is the State of Delaware, USA, and the proceedings shall take place in Delaware unless the parties agree to remote proceedings. The arbitrator may award relief only consistent with the terms of this Agreement, and the decision will be final and binding. Judgment on the award may be entered in any court of competent jurisdiction. The parties waive any right to a class action or jury trial. Nothing in this Section prevents either party from seeking temporary or permanent injunctive or equitable relief to protect Confidential Information, intellectual property, or security-sensitive information.

## 12. Miscellaneous

12.1 Entire Agreement. This Agreement, together with the Schedules, any SOWs, and any attached Audit Intake, constitutes the entire agreement between the parties and supersedes all prior or contemporaneous oral or written agreements related to the subject matter.

12.2 Order of Precedence. If there is a conflict among documents, the following order controls: (i) the applicable SOW; (ii) this Agreement; (iii) the attached Audit Intake, questionnaire, scope file, or client-provided assumptions; and (iv) other documentation or communications.

12.3 Amendments. Any amendment to this Agreement must be in writing and signed by both parties. Email approvals and electronic signatures are acceptable.

12.4 Assignment. Neither party may assign this Agreement or any rights or obligations hereunder without the other party's prior written consent, except in connection with a merger, acquisition, corporate reorganization, or sale of substantially all of its assets.

12.5 Severability. If any provision of this Agreement is held invalid or unenforceable, the remaining provisions will remain in full force and effect.

12.6 Force Majeure. Neither party will be liable for failure to perform due to circumstances beyond its reasonable control, including natural disasters, war, terrorism, civil unrest, governmental action, network outages, blockchain outages, RPC failures, cloud-provider failures, or other infrastructure failures outside the affected party's reasonable control.

12.7 Notices. Notices under this Agreement must be sent to the contacts identified in the applicable SOW or otherwise designated in writing. Security-critical notices may be sent by email, encrypted messaging, or other mutually used urgent communication channels, followed by written confirmation where practical.

12.8 Survival. Sections concerning payment, confidentiality, intellectual property, report usage, disclaimers, limitation of liability, indemnification, dispute resolution, and any provisions that by their nature should survive will survive termination or expiration of this Agreement.

## Schedule A - Private Client Severity Classification

This rubric defines how vulnerabilities are classified during a private audit engagement. The SOW and Audit Intake control trust assumptions, excluded behavior, supported assets, and client-specific rules.

### Core Concepts

Assets = user or protocol funds, NFTs, protocol value/ownership, positions, authorization rights, confidential data, and governance power.

### High (3)

A finding is High if:

- Assets, user positions, or protocol control/governance can be stolen, frozen, seized, destroyed, or arbitrarily modified through a valid attack path.
- A foreseeable user path created by unsafe contract design can cause catastrophic, irreversible loss.
- A control-flow or authorization flaw can cause systemic loss of funds or protocol control.

High = direct or systemic loss of funds or control.

### Medium (2)

A finding is Medium if:

- It cannot directly steal all assets but can significantly disrupt protocol operation, block core features, cause partial asset loss, inflict serious griefing or DoS, cause protocol-wide configuration failure, or create meaningful economic risk.

Includes:

- Control or governance logic bugs that can cause protocol-wide malfunction.
- User-facing issues where a typical user may lose funds due to unclear or ambiguous parameterization enabled by contract design.
- Loss of unmatured yield or long-tail exploit conditions.

Medium = meaningful but not existential risk.

### Low / QA

A finding is Low / QA if:

- It has no meaningful asset risk.
- It reflects minor state issues, UI/readability, spec mismatches, or stylistic problems.
- It is a generic centralization or social-risk observation with no code-level issue.
- It requires clearly unreasonable user behavior.
- It stems from external non-standard token behavior unless Client explicitly supports such tokens.
- It concerns view-only functions or event cosmetics without functional impact.

Low = no material security or economic impact.

### Informational

A finding is Informational if it is an observation, hardening suggestion, documentation improvement, code quality issue, or operational note that does not rise to Low / QA.

### Detailed Rules

Loss of Assets:

- Real asset loss -> High or Medium based on exploit conditions.
- Dust-level discrepancies or rounding errors -> Low.

Yield Loss:

- Matured yield already earned by users = High or Medium depending on conditions.
- Dust yield loss = Low.
- Unmatured or in-motion yield = capped at Medium.

User Mistake vs. User-Induced Design Failure:

- Valid High / Medium design failures include user loss through expected UI flow, typical transaction parameters, reasonable usage assumptions, or lack of guardrails where common mistakes lead to catastrophic outcomes.
- Low / QA user-fault cases include pure phishing, signing arbitrary data from unknown websites, sending tokens to the wrong address, or ignoring explicit warnings.

Non-Standard / Fee-on-Transfer Tokens:

- Out of scope unless Client declares explicit support.
- USDT and other widely used non-standard tokens may be treated as in scope if the Protocol interacts with them.

View Functions:

- Unused or non-critical view functions = Low.
- View functions feeding critical off-chain automation = severity matches the impact of the dependent process.

Out-of-Scope Libraries:

- Root cause in a third-party dependency = out of scope unless expressly included in the SOW.
- Incorrect usage of a dependency inside in-scope code = valid.

Speculation on Future Code:

- Findings must be tied to a real root cause in current in-scope code.
- Severity depends on realism of the scenario and the impact if triggered.

Event-Related Impacts:

- Events used in bridging, proofs, or automation: severity = impact of the affected process.
- Non-compliance with standard event formats = severity based on functional impact.
- Cosmetic or readability issues = Low.

Approve Race Condition:

- The classic approve front-run issue is not a valid vulnerability on its own.
- Using approve, increaseAllowance, or safeApprove is not, by itself, a bug.

Counting and Payment Rules:

Unless the SOW states otherwise, each distinct root-cause issue counts once regardless of how many functions, files, or chains are affected. Low / QA and Informational findings are reported for client value but do not count toward bounty-style or per-finding payment calculations unless expressly stated in the SOW.

## Schedule B - Default Statement of Work (SOW)

Scope of Audit: The Auditor will review only the smart contract repositories, commits, branches, folders, files, and modules listed below and in any attached Audit Intake.

Repository: [e.g., github.com/ProtocolRepo/contracts]

Commit Hash: [e.g., abcdef1234]

Branch: [e.g., main]

In-Scope Files / Folders: [insert exact paths]

Out-of-Scope Files / Folders: [insert exclusions]

Audit Intake / Questionnaire: [attach or link completed intake]

Known Issues / Accepted Risks: [attach or list]

Trusted Roles and Assumptions: [attach or list]

Supported Tokens / Assets: [attach or list]

External Dependencies / Oracles / Bridges: [attach or list]

Audit Window: The audit will start on [Start Date] and end on [End Date]. The Auditor will deliver the initial report within [X] days after the end date.

Fee & Payment Schedule: The Client will pay a fixed fee of US$[Amount], with [X]% due upon signing this SOW and the remainder due [upon delivery / within X days of invoice]. Any crypto payment terms, conversion rate, network fees, taxes, and payment wallet addresses must be stated in this SOW.

Deliverables: The Auditor will deliver a written report summarizing Findings and recommendations. The Client may ask reasonable clarifying questions about the report within [X] days after delivery.

Second Audit Round / Fix Review: [included / not included / priced separately]. Unless expressly included, fix verification is limited to previously reported High and Medium Findings and does not include a full re-audit.

Publication Rights: [private only / may publish final report with Auditor consent / other].

Special Instructions: [Insert any additional requirements, focus areas, CI integration, or expected updates.]

Contacts: Each party will designate a primary contact for scheduling, technical communications, security notices, and invoices.
