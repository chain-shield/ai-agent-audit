# ChainShield Strategy

Living strategy doc. Updated during founder office-hours discussions.

Last updated: 2026-04-30

## One-Line Thesis

Software will be built by AI agents at massive scale. Manual security auditing cannot keep up. ChainShield starts by automating professional-grade smart contract security discovery and validation, then expands into continuous security review for every high-stakes codebase AI agents create.

## Current Stage

ChainShield has technical proof, not yet full customer demand proof.

Evidence so far:

- AI Agent Audit can run large Solidity protocol audits in roughly 1-2 days.
- The workflow produces validated High/Medium findings with PoCs.
- Marginal compute/tooling cost per audit is currently near $0.
- Earlier versions cost roughly $50-$600 per audit depending on codebase size.
- Current runs use a $200/month Codex Pro subscription that is also used for other work; a large audit barely dents the weekly limit.
- Human time per audit is currently about 1 hour, mostly final sanity check.
- The founder has achieved SR Warden status on Code4rena using this app.
- Current public Code4rena profile proof: SR Warden, 8 High findings, 7 Medium findings, 52 signal, and 8th-place finishes in SukukFi and Brix Money.
- The app is finding valid security issues in real competitive audit settings.
- Recent product work added automated validation for security findings.

The next milestone:

> Get a handful of real protocol teams to grant repo access, run paid or case-study discovery audits, and prove buyers will change behavior because of ChainShield.

## Product-Market Fit Definition

Current PMF target:

> ChainShield can consistently rank top 5 in Code4rena competitions.

Why this matters:

- Code4rena is a public, adversarial benchmark.
- Valid H/M findings are judged by external parties.
- Ranking top 5 repeatedly would prove the system can perform at or above professional auditor level.

Important caveat:

Top-5 Code4rena performance proves audit quality and signal. It does not, by itself, prove sales demand. Customer demand requires buyers to pay, give repo access, ask for repeat runs, or integrate ChainShield into their workflow.

## Fundraising Goal

Raise:

> $3M-$4M

Use of funds:

- Expand beyond Solidity/EVM into other high-stakes codebases.
- Turn the current audit workflow into a repeatable product.
- Build continuous commit-level security scanning.
- Hire security, AI, infra, and GTM talent.
- Create benchmark evidence that ChainShield can outperform slow manual audit workflows.

Investor proof needed before raise:

- 3-5 protocol pilots.
- At least 1 paid customer.
- Validated H/M findings from customer repos.
- One public or anonymized case study.
- Code4rena trend toward top-5 consistency.
- Runtime and cost metrics proving the automation advantage.
- Evidence that findings survive professional review.
- Evidence that the system is not dependent on one founder manually steering every result.

## YC Acceptance Plan

Current timing:

- As of 2026-04-30, YC is accepting applications for the Summer 2026 batch.
- The on-time deadline is 2026-05-04 at 8pm PT.
- On-time applicants get a decision by 2026-06-05.
- Interviews are expected in May and June.
- The Summer 2026 batch runs July to September in San Francisco.

### Goal

Maximize odds of YC acceptance by turning ChainShield from "impressive technical automation" into "a scary-fast company with third-party proof and emerging customer pull."

### What YC Should Believe

YC should leave the application thinking:

1. This founder can build unusually hard technical systems.
2. The market pain is real: smart contract audits are slow, expensive, and still miss serious bugs.
3. ChainShield has rare proof: live Code4rena results, not self-reported benchmarks.
4. The product is already producing professional-grade findings with PoCs in 1-2 days.
5. The next step is obvious: sell 48-hour pre-launch discovery runs to budget-constrained DeFi teams.
6. If this works in Solidity, it can expand into the security layer for AI-generated software.

### Application Story

Do not lead with the $3M-$4M raise. Lead with proof and speed.

Best one-liner:

> ChainShield automates professional-grade smart contract security audits, producing validated High/Medium findings with PoCs in 48 hours instead of weeks.

Sharper YC version:

> We use AI agents to find and validate serious smart contract vulnerabilities. The proof is not a private benchmark: our system has helped produce live Code4rena results under a public SR Warden profile.

### Application Proof Points

Use concrete facts:

- Public Code4rena SR Warden status.
- 8 High and 7 Medium valid findings on the public profile as of 2026-04-30.
- 8th-place finishes in SukukFi and Brix Money.
- Large protocol audit runtime: roughly 1-2 days.
- Findings include PoCs.
- Cost per audit run is low relative to human audit cost.
- Automated validation was recently added to reduce false positives.
- Website is live at chainshield.ai.

Avoid vague claims:

- "AI will change security."
- "The market is huge."
- "Everyone needs this."
- "We can replace all audits."

Translate those into specifics:

- "Traditional audits can cost tens or hundreds of thousands and take weeks."
- "Our initial wedge is pre-launch DeFi teams that cannot afford a six-figure audit but cannot safely skip review."
- "We are selling a 48-hour discovery pass before a full audit, contest, or mainnet launch."

### Before Applying

The application should be submitted by the YC deadline if applying to Summer 2026. Do not wait for perfect customer traction if the deadline is days away.

Before submission:

1. Rewrite the application in short, specific sentences.
2. Include the Code4rena profile link.
3. Include the ChainShield website link.
4. Include one short demo video or walkthrough if available.
5. Explain exactly how the audit pipeline works at a high level: repo ingest, static context, agent discovery, validation, PoC, report.
6. State what is automated versus human-reviewed today.
7. State the current wedge pricing.
8. State the next 30-day traction plan.

### 30-Day Traction Sprint Before Interviews

If YC interviews happen in May/June, the highest-leverage work before interview is customer proof.

Targets:

- 30 pre-launch DeFi teams researched.
- 30 personalized outbound messages sent.
- 10 founder conversations.
- 5 repo-access conversations.
- 2 pilots started.
- 1 paid pilot or signed design-partner commitment.
- 1 customer quote, even if anonymized.

The best interview update would be:

> Since applying, we ran ChainShield on two customer repos, found X validated H/M issues, got one paid pilot, and one team wants us in their pre-launch process.

### What To Say If Asked About Competitors

Do not say competitors are fake or bad. Say:

> Most AI audit tools show self-reported historical benchmarks. Our wedge is live third-party validation: Code4rena rankings, accepted findings, PoCs, and customer acceptance. We are building the feedback loop from raw candidate finding to judged validity to customer patch.

### What To Say If Asked About Defensibility

Answer:

> The moat is the validation corpus and workflow. Every run captures candidate findings, rejected findings, duplicate decisions, severity decisions, PoC outcomes, customer acceptance, and patch results. That data improves future validation and becomes hard to recreate without doing real audits at scale.

Then add:

> Long term, ChainShield becomes continuous security infrastructure: every PR is audited before merge.

### What To Say If Asked "Why Now?"

Answer:

> AI agents are making software creation much faster. Security review is still manual, expensive, and slow. If code volume increases 10x, human audit capacity cannot scale linearly. ChainShield is the security layer for that new software supply chain, starting with smart contracts because failures are immediate and expensive.

### What To Say If Asked "Why You?"

Answer with specific proof:

> We built the system, used it in live Code4rena competitions, and reached SR Warden status with accepted High/Medium findings. We are not theorizing about AI audits. We are proving it in adversarial public markets.

### Biggest Application Risk

The biggest risk is sounding like a technical demo instead of a company.

Fix:

- Make the buyer specific.
- Make the wedge specific.
- Make the price specific.
- Show customer conversations in progress.
- Show that the founders are learning from sales, not hiding in the product.

### YC Interview Prep

Prepare crisp answers to:

1. What do you do?
2. Who uses it?
3. What have users done so far?
4. How do you find customers?
5. Why is this 10x better?
6. Why is this not just a services business?
7. What is automated today?
8. How accurate is it?
9. How do you know findings are valid?
10. Why won't OpenAI or an audit firm build this?
11. How big can this get beyond Solidity?
12. What did you learn from your last 5 customer conversations?

The answer to "what did you learn from users?" must be real. Get those conversations now.

## YC Differentiation

The likely applicant field will include other AI security, AI code review, and AI audit startups.

ChainShield's differentiation should be framed as:

> We are not another AI audit tool with private benchmarks. We have live third-party validation, paying customers, happy pilots, 24-hour delivery, near-zero marginal run cost, and a mostly automated pipeline with human sanity check at the end.

### Differentiation Stack

#### 1. Third-Party Validated

Most competitors can claim they reproduced historical bugs. ChainShield can point to public Code4rena proof:

- SR Warden status.
- Public accepted High/Medium findings.
- Public contest placements.
- Independent judging.
- Duplicate and invalid-finding pressure.
- A leaderboard that cannot be edited by ChainShield.

YC-safe phrasing:

> Our benchmark is not a private spreadsheet. It is live Code4rena competition performance under third-party judging.

#### 2. Paying Clients And Happy Pilots

By Winter application time, this should be the biggest upgrade from the current story.

Target proof:

- 3-5 paying clients or signed paid pilots.
- Customer quotes.
- Repeat-run requests.
- Repo-access willingness.
- One team asking for continuous audit before merge.

YC-safe phrasing:

> We have moved from contest proof to buyer proof: teams give us repo access, pay for runs, and ask us to check future changes.

#### 3. Speed

If ChainShield can reliably deliver in 24 hours, make that the headline. If 24 hours is only true for some scopes, phrase carefully.

Good:

> We deliver validated H/M findings with PoCs in as little as 24 hours.

Better if consistently true:

> Our standard pre-launch discovery run delivers in 24 hours.

Avoid:

> We fully replace a manual audit in 24 hours.

That invites unnecessary skepticism.

#### 4. Near-Zero Marginal Cost

Do not say only "it costs almost nothing." Convert it into business metrics.

Current operating economics:

- Marginal compute/tooling cost per audit: near $0.
- Historical variable LLM/tooling cost before current Codex workflow: roughly $50-$600 per audit depending on codebase size.
- Current cost basis: $200/month Codex Pro subscription used across all founder work.
- Current large-audit usage: barely dents the weekly Codex Pro limit.
- Human time per audit: about 1 hour.
- Human role: final sanity check and customer-facing judgment.

Track:

- Model/API/tooling cost per run.
- Human review minutes per run.
- Gross margin at design-partner pricing.
- Cost per accepted H/M finding.
- Cost per 24 hours of delivery.

YC-safe phrasing:

> A run that would cost a team tens of thousands in manual review currently costs us near $0 in marginal tooling and about one hour of human sanity-check time.

More precise investor phrasing:

> Earlier versions cost us about $50-$600 per audit in model spend. The current Codex-based workflow runs under a $200/month subscription I already use for all work; even a large audit barely affects the weekly limit, so the marginal cost of an additional audit is effectively near zero.

#### 5. 98% Automated

This is powerful only if measured.

Define automation as:

- Repo ingestion.
- Build/static context extraction.
- Candidate finding generation.
- Validation.
- Deduplication.
- PoC generation.
- Report drafting.
- Patch/retest workflow.

Then define the human role:

- Final sanity check.
- Customer-facing judgment.
- Edge-case escalation.
- Legal/scope-sensitive review.

YC-safe phrasing:

> The pipeline is about 98% automated by elapsed workflow steps. A human performs the final sanity check before customer delivery.

Even better:

> We average X human minutes per completed audit run and Y human minutes per accepted H/M finding.

Current metric:

> We currently spend about 1 hour of human time per audit, mostly final sanity check.

### The Interview Answer

If YC asks, "How are you different from other AI audit startups?"

Answer:

> Three things. First, third-party proof: we compete in Code4rena and have public accepted H/M findings, not just private historical benchmarks. Second, customer pull: teams are paying us and giving repo access because audits are too slow and expensive. Third, economics: our run is about 98% automated, takes about 24 hours, and has near-zero marginal cost, with a human only doing final sanity check before delivery.

Then stop. Let them ask follow-ups.

### The Metrics To Bring

By application/interview time, have one table with:

| Metric | Target |
| --- | --- |
| Completed runs | 10+ |
| Paid pilots/customers | 3-5 |
| Delivery time | 24 hours standard, or range by scope |
| Human review time per run | About 1 hour today |
| Human review time per accepted H/M | Track and report actual |
| Model/tooling cost per run | Near $0 today |
| Gross margin | Track and report actual |
| Validated H/M findings | Track and report actual |
| Customer acceptance rate | Track and report actual |
| Repeat-run requests | 1-2+ |

The application should use actual numbers, not adjectives.

## Solo Founder Risk And Mitigation

Current team reality:

- Founder is solo after a previous cofounder dropped out last year.
- Two interns are helping.
- Founder is willing to quit day job and relocate to San Francisco for YC if accepted.
- Founder background is atypical for a first-time YC applicant: 20-year serial entrepreneur, prior software company built to 8-figure scale, and Inc. 5000 placement at #646.

YC's public FAQ says they regularly accept solo founders, but one-person startups are harder and founders are more likely to succeed with a cofounder. This means solo founder status is a risk, not a blocker.

### Founder Background Advantage

The founder should not present as a generic solo technical founder.

Positioning:

> I am a repeat entrepreneur who has built and scaled software businesses before. ChainShield combines that operating experience with a new technical wedge: AI-native security auditing with live third-party proof.

This background helps offset solo-founder risk because it suggests:

- The founder knows how to sell.
- The founder knows how to operate.
- The founder has survived company-building cycles before.
- The founder is less likely to confuse product-building with company-building.
- The founder can recruit from a broader business network than a first-time builder.

Use specifics:

- 20 years as a serial entrepreneur.
- Built a software company to 8-figure scale.
- Inc. 5000 #646.
- Now building ChainShield from direct technical work, not just managing others.

Do not let this become a backward-looking resume pitch. The story should be:

> I have built real businesses before. This is the most technically leveraged opportunity I have seen, and I am applying that operating experience to a market where speed, trust, and distribution matter.

### Do Not Panic-Add A Cofounder

Do not add a weak cofounder just to satisfy a perceived YC preference.

A bad cofounder is worse than no cofounder. YC partners have publicly warned that tacking on a cofounder who is not deeply trusted, aligned, or proven can be destructive.

Only add a cofounder if they are:

- Exceptional.
- Deeply trusted.
- Already creating founder-level value.
- Able to commit full-time.
- Strong in a missing area: security research, GTM/sales, infra/product, or enterprise trust.
- Someone the founder would want beside them for 10 years, not just for YC optics.

### Application Framing

Do not lead with:

> I am worried because I am a solo founder.

Lead with:

> I kept going after my cofounder dropped out, built the system, reached SR Warden status, automated validation, and started customer development.

The dropout can become a strength if framed correctly:

> My cofounder left last year. I continued alone, built the product forward, used it in live Code4rena competitions, and now have a repeatable audit workflow with about one hour of human review per run.

This shows persistence.

### What YC Needs To Believe

YC needs to believe:

1. The founder can build.
2. The founder can sell.
3. The founder can recruit.
4. The founder can handle pressure.
5. The company is not blocked by the absence of a cofounder.

### Mitigation Plan Before Applying

#### 1. Prove Founder Execution

Metrics that reduce solo-founder concern:

- Code4rena public proof.
- Paid pilots.
- Customer conversations.
- 24-hour delivery.
- About 1 hour human time per audit.
- Near-zero marginal tooling cost.
- Repeatable runbook.

#### 2. Prove Recruiting Ability

Turn the interns into evidence only if they are producing real work.

Track:

- What each intern owns.
- What they shipped.
- Whether they can run parts of the audit workflow.
- Whether one could become a founding engineer later.

If one intern becomes exceptional and wants to commit full-time, that may become a cofounder path. Do not force it.

#### 3. Build A Bench Of Possible Cofounders

Start meeting potential cofounders now, but do not rush.

Best profiles:

- Senior smart contract security researcher.
- Strong protocol sales/GTM founder.
- Infrastructure/product engineer who can turn the workflow into SaaS.
- Former audit firm/operator with buyer trust.

Use:

- YC Co-Founder Matching.
- Code4rena/Sherlock/Cantina researcher network.
- NYC crypto founder events.
- Security engineering communities.
- Warm intros from investors and protocol teams.

YC Co-Founder Matching is worth using because it is free, private to approved users, designed for founders at multiple stages, and has strong NYC/SF density. Treat it as a high-quality sourcing channel, not a commitment to add someone.

Profile stance:

> I am already building ChainShield and looking for a founder-level partner only if there is exceptional fit. The company has public Code4rena proof, a working audit engine, and a clear customer wedge.

Do not present as:

> I need a cofounder so YC will like me.

Present as:

> I am looking for a rare partner who can help turn proven audit automation into the default security layer for AI-generated software.

### Cofounder Scorecard

Only consider candidates who score highly on at least one core gap and are strong enough to be founder-level.

The bar should be binary. ChainShield does not need a generic cofounder. It needs one of two exceptional archetypes:

#### Archetype A: 10x Web3 Technical Cofounder

Profile:

- 10x web3 engineer.
- Deep Solidity/EVM or multi-chain security expertise.
- Can build production-grade infra, CI integrations, and developer workflows.
- Can reason from exploitability to product architecture.
- Can improve the audit engine, not just maintain it.
- Has enough security credibility to earn trust from protocol teams.

Jaw-dropping signals:

- Top C4/Sherlock/Cantina/Immunefi track record.
- Built high-TVL protocol infrastructure.
- Built serious developer tooling used by real teams.
- Found critical bugs in production protocols.
- Can ship complex infra alone in days.
- Can challenge the founder technically and be right.

What they unlock:

- Faster productization.
- Stronger technical defensibility.
- More credible customer delivery.
- Lower founder bottleneck.
- Expansion beyond EVM.

#### Archetype B: Web3 Executive / Rainmaker Cofounder

Profile:

- Former executive/operator in web3, security, protocol growth, BD, partnerships, or ecosystem development.
- Amazing network across protocol founders, funds, L2s, ecosystems, audit buyers, and exchanges.
- Wizard at biz dev and sales.
- Super connector with a jaw-dropping track record.
- Can get meetings and close design partners that would otherwise take months.
- Understands trust-heavy security sales.

Jaw-dropping signals:

- Former VP/Head/GM/founder role at a serious web3 company.
- Direct relationships with protocol founders and crypto investors.
- Has closed six/seven-figure partnerships or enterprise deals.
- Can get 10 qualified customer intros in a week.
- Can make ChainShield credible in rooms where AI security would otherwise be dismissed.
- Has receipts, not vibes.

What they unlock:

- Rapid customer acquisition.
- Paid pilots.
- Fundraising credibility.
- Ecosystem partnerships.
- Enterprise/security buyer trust.
- Distribution moat.

### Who Not To Add

Do not add:

- A merely good engineer.
- A generic startup generalist.
- A junior web3 enthusiast.
- A BD person without real relationships.
- A "strategic advisor" who wants founder equity.
- Anyone whose value is mostly "helps with YC optics."
- Anyone who cannot create measurable momentum in 2-4 weeks.

The default should remain solo unless the candidate is obviously exceptional.

Hard no:

- Wants founder title but not full-time commitment.
- Weak technical or commercial judgment.
- No security-market intuition.
- Wants to debate the vision before doing work.
- Joins mainly because YC might happen.
- Does not increase execution speed within 2-4 weeks.

Trial project before equity:

- One customer pilot.
- One bounty target.
- One C4 contest.
- One CI/continuous audit prototype.
- One outbound/customer-development sprint.

If the candidate cannot create obvious momentum during a trial, do not add them.

### Cofounder Equity Range

Equity depends on whether the person is truly a cofounder or a very strong early hire/advisor.

Current company context:

- Working MVP exists.
- Founder built the original system.
- Founder has public Code4rena proof.
- Founder has operating background and prior business wins.
- Customer proof is still early.
- Company is pre-YC and likely pre-institutional funding.

This means the right person is joining late relative to ideation/MVP, but early relative to venture scale.

Guideline:

| Role | Equity Range | Notes |
| --- | --- | --- |
| Advisor / connector | 0.25%-2% | No cofounder title. Specific intro/help expectations. |
| Fractional BD or technical helper | 1%-4% | Milestone-based or option-style, not founder equity. |
| Founding engineer / early exec | 2%-7% | Employee path, usually with salary or future salary. |
| Borderline late cofounder | 7%-10% | Be careful: may look like employee equity with founder title. |
| True late cofounder | 10%-15% | Full-time, founder-level ownership, major function owner. |
| Exceptional company-changing cofounder | 15%-20% | Rare. Must be obviously transformative within weeks. |
| Extreme outlier | 20%+ | Only for a once-in-a-generation operator/technologist. Assume no unless proven otherwise. |

Recommendation:

- Do not offer 5% to someone you want to be a real cofounder. It will likely attract the wrong psychology or signal that they are actually an employee.
- For a strong but unproven cofounder candidate, think 7%-10% with room to grow through milestones.
- For a true late cofounder, think 10%-15%.
- For a true 10x technical cofounder or true web3 rainmaker, think 12%-18%.
- Reserve 20%+ only for an extreme outlier who is obviously company-changing and irreplaceable. The default assumption is no.

Founder instinct:

> If someone asks for 25%, they better be a once-in-a-generation cofounder, not merely strong.

All founder/cofounder equity should have:

- 4-year vesting.
- 1-year cliff.
- IP assignment.
- Confidentiality.
- Clear role ownership.
- Founder departure terms.
- Board/control clarity.
- Lawyer review before signing anything.

Trial before equity:

- Do a 2-4 week trial project before promising cofounder equity.
- For a technical candidate: one audit-engine improvement, CI prototype, C4 run, or customer pilot.
- For a rainmaker: 10 qualified intros, 3 serious calls, 1 repo-access conversation, or 1 paid pilot.

If they cannot create obvious momentum before equity, do not make them a cofounder.

YC consideration:

YC often encourages close-to-equal founder splits for teams starting from zero, because most of the work is ahead. ChainShield is not starting from zero: the founder has built the MVP, produced C4 proof, and created the current workflow. A late cofounder can still earn meaningful ownership, but the split should reflect the existing proof while still being large enough to create true founder commitment.

#### 4. Build Advisor Trust

If no cofounder is ready, build a credible advisor/operator bench:

- Web3 security advisor.
- Crypto GTM advisor.
- Legal/security disclosure advisor.
- Protocol founder design partner.

Advisors do not replace a cofounder, but they reduce perceived blind spots.

### Interview Answer

If YC asks, "Why are you a solo founder?"

Answer:

> I had a cofounder, but he dropped out last year. I kept building. Since then I automated the audit workflow, reached SR Warden status on Code4rena, added validation, and got the process to about 24-hour delivery with roughly one hour of human review. I am open to adding an exceptional cofounder, but I am not going to add someone weak for optics. Right now the right proof is execution, customers, and recruiting strong people around the company.

If YC asks, "Can you do this alone?"

Answer:

> Not forever, and I do not intend to. But I can get to the next proof points: paid pilots, repeatable audit runs, and customer pull. I already have two interns helping, and I am using this phase to identify whether one of them or someone from the security/GTM network can become a true founder-level partner.

### Best Outcome By Application Time

Best case:

- One exceptional person has become a real cofounder through actual work together.

Good case:

- Founder remains solo, but has paid pilots, repeat customers, interns executing real workflows, and a strong candidate pipeline.

Bad case:

- Founder adds a weak cofounder for YC optics and creates future company risk.

## Bug Bounty Proof

Bug bounty wins can materially strengthen the YC application if they are connected directly to ChainShield's product capability.

Good proof:

- ChainShield finds a valid bounty issue.
- The issue is accepted by the program.
- The payout is meaningful, especially $25k-$100k+.
- The finding includes a clear root cause and PoC.
- The workflow is documented: time to find, time to validate, human minutes, cost, payout.
- The same pipeline that serves customers produced the bounty result.

Weak proof:

- Founder manually found bugs using personal skill with little product involvement.
- Bounty findings are unrelated to the audit pipeline.
- Results cannot be shared, even in anonymized form.
- The story sounds like consulting or solo researcher work rather than product leverage.

YC-safe phrasing:

> We also started running ChainShield against bug bounty targets. If a bounty is accepted, it gives another form of third-party validation: a real protocol accepted the vulnerability and paid for it.

If ChainShield wins a $25k+ bounty:

> ChainShield found a bounty accepted for $25k. The run took X hours, cost effectively near $0, required Y human review minutes, and produced a PoC accepted by the program.

If ChainShield wins a $100k+ bounty:

> ChainShield found a critical bug bounty accepted for $100k+. That is direct economic proof that the system finds vulnerabilities valuable enough for real teams to pay for, outside of contests and customer pilots.

### Bug Bounty Strategy

Use bounty hunting as a proof lane, not as the company strategy.

Targets:

- High-quality DeFi bounty programs.
- Protocols with public code and meaningful payout tables.
- Programs with clear scope and acceptance criteria.
- Targets similar to ChainShield's initial ICP: vaults, lending, staking, bridges, AMMs, perps, rewards, and oracle-heavy protocols.

Track:

- Target protocol.
- Scope.
- Run time.
- Human review minutes.
- Model/tooling cost.
- Candidate findings.
- Submitted findings.
- Accepted findings.
- Rejected findings and reasons.
- Payout.
- Whether disclosure can be anonymized or public.

Do not let bounty hunting replace customer development. The YC story is strongest when bounty wins sit beside paying customers:

> Code4rena proves public contest performance. Bug bounties prove accepted economic value. Customers prove buyer demand.

## YC Winter Batch Prep Plan

The founder is targeting a future Winter batch, not the immediate Summer 2026 batch.

Important current-date note:

- As of 2026-04-30, YC's public apply page is focused on Summer 2026.
- YC says founders can apply to future batches including Fall, Winter, and Spring.
- The exact next Winter deadline should be checked on the official YC application page when applications are open.
- Historically, Winter batch applications tend to be due in the fall, but use the official YC date, not memory.

Founder commitment:

- Willing to relocate to San Francisco for the batch.
- Willing to quit day job if accepted.

This is good, but it is not the main proof point. YC will care more about velocity, user pull, and founder execution. Mention availability when asked, but do not make it the center of the application.

### Objective

By the time the Winter application is submitted, ChainShield should no longer look like "great technical potential." It should look like:

> A fast-growing security company with public third-party validation, paying design partners, and a repeatable AI audit engine that gets better with every run.

### Target Application State

Best-case targets before applying:

- 10+ completed audit runs across contests and customer repos.
- 3-5 paying protocol customers or signed paid pilots.
- $25k-$100k in paid or committed audit revenue.
- 2+ customer quotes or anonymized case studies.
- 1-2 teams asking for repeat runs or continuous monitoring.
- Continued Code4rena ranking improvement, with at least one top-5 finish if possible.
- Clear run metrics: time, cost, human review minutes, raw findings, validated findings, PoC success rate.
- A short demo showing repo ingest to validated report.

Minimum credible targets:

- 3 paid pilots.
- 1 strong case study.
- 1 repeat customer or continuous-audit request.
- Public Code4rena proof remains strong and current.
- Founder can clearly explain what was learned from at least 20 customer conversations.

### Monthly Plan

#### May 2026: Customer Discovery And First Pilots

Goal:

- Prove who urgently wants this.

Actions:

- Build a 100-team pre-launch DeFi target list.
- Send 100 personalized outbound messages.
- Get 20 founder/CTO conversations.
- Get 5 repo-access conversations.
- Run 1-2 pilots, even if discounted.
- Track every objection.

Deliverable:

- A spreadsheet/CRM of targets, conversations, objections, and outcomes.
- One updated offer based on what buyers actually say.

#### June 2026: Paid Design Partners

Goal:

- Convert interest into payment.

Actions:

- Close 2-3 design partners.
- Use the design-partner pricing model.
- Get permission for anonymized metrics.
- Produce polished reports and post-fix retests.
- Ask every customer for one intro.

Deliverable:

- First paid revenue.
- First case study draft.
- Customer quote, even if anonymized.

#### July 2026: Repeatability

Goal:

- Prove the system is not founder magic.

Actions:

- Instrument every audit run.
- Track human minutes per accepted H/M finding.
- Track cost per audit and cost per valid H/M.
- Document runbooks.
- Have another team member run the process with minimal founder intervention.

Deliverable:

- Repeatability dashboard.
- "What is automated today" document.
- "What still requires human review" document.

#### August 2026: Continuous Audit Prototype

Goal:

- Show the path from services to product.

Actions:

- Build a lightweight GitHub/CI or commit-diff workflow.
- Run it on one customer or internal repo.
- Produce weekly security summaries.
- Validate patches after fixes.

Deliverable:

- Demo: PR or commit enters, ChainShield produces validated security output.
- One customer says they want this before merge.

#### September 2026: Defensibility Package

Goal:

- Make the moat legible.

Actions:

- Assemble validation corpus stats.
- Publish or prepare benchmark methodology.
- Summarize Code4rena live proof.
- Build a private benchmark dashboard.
- Collect rejected-finding categories and false-positive reductions.

Deliverable:

- Investor-ready defensibility memo.
- One slide or page: "No trust-me benchmarks. Live third-party proof."

#### October 2026: YC Application Drafting

Goal:

- Make the application impossible to misunderstand.

Actions:

- Draft YC application answers.
- Record a direct founder video.
- Record a short product demo.
- Ask 3-5 trusted founders/investors to review the application for clarity.
- Remove all hype and replace it with metrics.

Deliverable:

- Final application draft.
- Interview prep answers.
- Updated metrics snapshot.

#### November/Deadline Month: Submit And Keep Shipping

Goal:

- Apply with momentum, then create updates before interview.

Actions:

- Submit before the official deadline.
- Continue customer outreach.
- Continue audits.
- Send meaningful updates if YC allows.

Best update:

> Since applying, we added two paid pilots, found X validated H/M issues, and one customer asked for continuous monitoring.

### The Application Should Emphasize

1. Public proof: Code4rena SR Warden profile, accepted H/M findings, contest placements.
2. Customer proof: paid pilots, repo access, repeat requests, testimonials.
3. Speed: 48-hour audit discovery versus weeks/months.
4. Economics: low run cost versus high audit spend.
5. Repeatability: human minutes per accepted finding trending down.
6. Market expansion: Solidity first, then continuous security for AI-generated software.

### The Application Should Avoid

- Overclaiming that human audits are obsolete today.
- Sounding like a consultancy.
- Hiding the current human-in-the-loop pieces.
- Talking about raising $3M-$4M before showing customer pull.
- Claiming defensibility from prompts or model access.

### Interview Strengthener

By interview time, the founder should be able to say:

> I am ready to quit my job and relocate to SF for the batch. More importantly, customers are already pulling us in: we have X paid pilots, Y repo-access conversations, Z validated H/M findings, and one team wants ChainShield in their release process.

Commitment matters, but pull matters more.

## Positioning

Do not lead with:

> AI audits replace manual auditors.

That is the long-term thesis, but it invites skepticism too early.

Lead with:

> Before you spend $80k-$150k and wait weeks for a traditional audit, ChainShield runs a 48-hour pre-launch discovery pass on your Solidity repo. You get validated Medium/High findings with PoCs, so your team can patch the serious issues before a full audit, contest, or mainnet launch.

Core positioning:

- Faster serious-risk discovery.
- Validated H/M findings, not noisy scanner output.
- PoC-backed results.
- Affordable for pre-launch teams.
- Useful before a formal audit, contest, or launch.
- Eventually continuous on every commit.

## Beachhead Customer

Avoid broad language like:

> Web3 startups.

The actual first buyer:

> A pre-launch DeFi founder or CTO with code nearly ready for mainnet, limited audit budget, and real fear of shipping exploitable contracts.

Strong ICP signals:

- Public testnet or beta is live.
- Mainnet launch planned in the next 30-90 days.
- Recent Solidity commits.
- The protocol handles user funds, vaults, staking, lending, swaps, bridges, perps, or rewards.
- Team mentions audit, security review, launch, points season, TGE, or TVL.
- Too early or too budget-constrained for a six-figure audit.

Weak ICP signals:

- NFT-only communities.
- Token-only marketing pages with no working product.
- Teams with no public code and no technical founder.
- Generic web3 networking rooms with no builders.

## Demand Evidence: What Counts

Real demand:

- Customer gives repo access.
- Customer pays or agrees to pay.
- Customer asks for a second run.
- Customer introduces another founder.
- Customer delays launch to wait for ChainShield results.
- Customer wants ChainShield in CI before merge.
- Customer gets nervous if ChainShield cannot run this week.

Not demand:

- "Interesting."
- "Cool idea."
- "AI security is important."
- "Let's stay in touch."
- Newsletter signups.
- Generic positive feedback.

## Go-To-Market Motion

### 1. Founder-Led Outbound

Build a target list of 30 pre-launch DeFi teams.

Track:

- Protocol
- Website
- Founder/CTO
- Contact path
- Chain
- Launch signal
- Security status
- Repo status
- Risk surface
- Outreach status
- Notes

Weekly target:

- 30 teams researched.
- 30 personalized messages sent.
- 10 actual founder conversations.
- 5 repo-access conversations.
- 1 discounted or unpaid pilot that can become a public or anonymized case study.

### 2. Code4rena/Cantina/Sherlock Adjacency

Start near teams already paying for security:

- Active and recent contests.
- Small and mid-sized prize pools.
- Mitigation reviews.
- Protocols that completed a contest and will ship new contracts.
- Similar protocols that have not yet run a contest.

Why:

These teams already believe security matters. The sale is not "care about security." The sale is "get serious issues faster and cheaper before the next formal review."

### 3. NYC Founder-Dense Events

Go to NYC, but do not "network" vaguely.

Good rooms:

- Web3 founder pitch nights.
- DeFi/fintech founder events.
- ETHGlobal side events.
- VC/founder mixers.
- Crypto accelerator demo days.
- Builder-heavy meetups with technical founders.

Bad rooms:

- NFT/social events.
- Token promotion rooms.
- General crypto panels.
- Investor-only panels where no founders are building.

Goal after each NYC trip:

- 10 founder conversations.
- 3 follow-up calls booked.
- 2 repo-access offers or intros.

### 4. Warm Intros

Ask crypto investors, accelerators, devrel teams, and infra companies:

> Which portfolio company is shipping Solidity code in the next 60 days and is worried about audit cost or timing?

Do not ask:

> Do you know anyone who needs security?

### 5. Public Repo Teaser Audits

This can be an effective outbound motion if handled as responsible disclosure, not as "pay us or you do not get the bug."

Safe version:

1. Only analyze public source code passively and offline.
2. Do not interact with production systems, mainnet contracts, user funds, private APIs, or anything outside published scope.
3. Check for a security policy, bug bounty program, security.txt, or preferred disclosure channel first.
4. If a finding is serious and exploitable, disclose through the official security channel with enough detail for the team to understand and mitigate.
5. Use the finding to open a relationship and offer a paid full discovery run, retest, or continuous monitoring.

Unsafe version:

- Send exploit details over cold email.
- Withhold a critical live vulnerability unless they pay.
- Run active tests against production without authorization.
- Publicly name the team before coordinated disclosure.
- Make the message sound like a threat or ransom.

Recommended teaser:

> I ran a passive review of your public Solidity repo and found a candidate issue in the [module/category] area that looks like it could affect [high-level impact]. I do not want to send exploit details over email. Is there a security contact we should use for responsible disclosure? Separately, ChainShield can run a full 24-hour discovery pass with validated findings, PoCs, and patch retest.

What to share in the first email:

- Repo/module reviewed.
- High-level bug class.
- High-level impact category.
- Evidence of seriousness without copy-paste exploit steps.
- Code4rena proof link.
- Offer to disclose privately.
- Offer for full paid report or 24-hour discovery run.

What not to share in the first email:

- Complete exploit path.
- Mainnet exploit recipe.
- PoC that can be pasted and run.
- Public accusations.
- Pricing framed as the cost to receive an already-discovered critical vulnerability.

The full-report offer should be framed as:

> We can run the complete ChainShield discovery workflow across the repo and deliver a full validated report.

Not:

> Pay us to learn about this bug.

If a critical live issue is found, do the right thing and coordinate disclosure. Trust is the business.

## Pricing Strategy

Do not price from cost. ChainShield's low operating cost is margin and speed advantage, not the buyer's anchor.

Buyer anchor:

- Standard DeFi audits commonly cost tens of thousands of dollars.
- Complex/cross-chain work can exceed $100k.
- Competitive audit prize pools often range from low five figures to six figures.
- Rush timelines command a premium.

### Design Partner Pricing: First 3-5 Customers

Use this to reduce friction and create proof.

Recommended:

- $1,500-$3,000 onboarding fee.
- $2,000 per valid Medium.
- $6,000-$8,000 per valid High.
- $20,000-$30,000 total cap.
- Free post-fix retest.
- Discount requires case study, testimonial, or permission to publish anonymized metrics.

Current website pricing:

- $1,500 onboarding fee.
- $1,000 / $3,000 per valid Medium / Critical finding.
- $25,000 cap.

Recommendation:

Raise the High payout. A valid High that saves a protocol from a launch disaster is worth much more than $3k.

### Standard Launch Discovery Pricing

Use after the first proof pilots.

Small scope:

- $5,000 flat minimum.

Normal DeFi scope:

- $7,500-$12,500 minimum.

Complex scope:

- $15,000-$25,000 minimum.

Success fees:

- $2,500 per valid Medium.
- $7,500 per valid High.

Cap:

- $35,000-$60,000 depending on scope.

### Continuous Audit Pricing: Later

Future recurring product.

Startup:

- $2,000/month.

Growth:

- $5,000/month.

Serious protocol:

- $10,000-$20,000/month.

Include:

- Per-PR scanning.
- Weekly security summary.
- Validated H/M escalation.
- Discounted deep discovery runs.
- Post-fix verification.

## Offer

Primary wedge offer:

> 48-hour pre-launch discovery run. Validated Medium/High findings with PoCs. Small upfront fee, pay only for valid serious findings, capped below a traditional audit.

Alternative case-study offer:

> Free or discounted 48-hour run in exchange for permission to publish anonymized metrics: repo size, runtime, validated findings, and customer quote.

Rule:

Do not give away endless free audits. Free or discounted work is only for case-study-quality teams.

## Outreach Message

```text
Hey {name}, I saw {specific launch/repo/product signal}.

I am building ChainShield, a fast security discovery pass for pre-launch Solidity teams.

Traditional audits can take weeks and cost $80k+. We run a 48-hour pass that produces validated Medium/High findings with PoCs, so teams can patch the serious issues before a full audit, contest, or mainnet launch.

We have already used the system to produce valid Code4rena findings, including high-severity results.

Would it be useful if I ran a small discovery pass on {protocol/repo/module} and showed you what comes back?
```

## Discovery Call Script

Ask:

1. When are you trying to launch?
2. What is your current security plan?
3. Have you budgeted for an audit yet?
4. What would delay launch if security review takes too long?
5. What part of the protocol worries you most?
6. If we found one valid Medium/High issue in 48 hours, what would that be worth to you?
7. Would you be willing to give repo access for a limited discovery pass?

Listen for:

- Budget pain.
- Launch pressure.
- Audit scheduling delays.
- Prior bad audit experience.
- Fear around specific contract flows.
- Willingness to share repo access.
- Willingness to pay for validated findings.

## Key Risks

### 1. Trust

Teams may not trust AI-generated findings. Mitigation: emphasize validated findings, PoCs, founder Code4rena proof, and post-fix retest.

### 2. False Positives

Noisy scanner perception is dangerous. Mitigation: do not show raw output. Only sell validated H/M candidates.

### 3. Liability

Security buyers may assume "audit" means guarantee. Mitigation: define scope, disclaim guarantees, and position early runs as discovery passes unless full audit terms exist.

### 4. Founder Dependency

If the founder manually steers every run, scalability is weak. Mitigation: track automation level and produce repeatable runbooks.

### 5. Category Skepticism

Some buyers will assume AI audit tools hallucinate. Mitigation: lead with external validation and specific examples, not AI hype.

## Defensibility

The moat is not model access. Strong models will keep getting cheaper and more available.

The moat must become:

> A trusted security operating system that continuously learns from real audits, real invalidations, real PoCs, and real customer codebases.

### Investor Question To Answer

Investors will ask:

> Why will ChainShield win if OpenAI, Anthropic, a big audit firm, or another AI audit startup builds the same thing?

The answer cannot be:

> We have better prompts.

The defensibility story should be:

1. ChainShield has a proprietary corpus of validated and rejected findings.
2. ChainShield has a repeatable verification pipeline that reduces false positives.
3. ChainShield is benchmarked in public adversarial markets like Code4rena.
4. ChainShield learns from customer outcomes: what was valid, invalid, exploitable, patched, and paid for.
5. ChainShield becomes embedded in the customer's release workflow.
6. ChainShield builds trust as a security authority, not just as a scanner.

### The Real Moats

#### 1. Validation Data Moat

Every run should produce structured data:

- Raw candidate finding.
- Validation decision.
- Reason for rejection.
- Scope decision.
- Duplicate decision.
- Severity decision.
- PoC status.
- Customer acceptance/rejection.
- Patch outcome.
- Whether the finding would have won in Code4rena.

This becomes the training/evaluation corpus competitors cannot easily buy.

#### 2. Benchmark Moat

ChainShield should maintain a private and public benchmark suite:

- Historical Code4rena contests.
- Known H/M findings.
- Rejected/invalid findings.
- V12 duplicate examples.
- Unsupported-token false positives.
- Governance-risk false positives.
- Customer-anonymized cases.
- Non-EVM benchmark sets as expansion begins.

The goal:

> ChainShield can prove recall, precision, PoC success rate, runtime, and cost across real audit scenarios.

Important distinction:

Self-reported benchmark claims are useful but not enough. Competitors can say they reproduced a set of Code4rena findings after the fact, but that is weaker than competing live under a public warden profile where submissions are judged by third parties.

ChainShield should make public adversarial validation a central proof point:

- Public Code4rena profile.
- Public SR Warden status.
- Contest placements.
- Valid H/M finding counts.
- 90-day leaderboard movement.
- Repeat top-10 and eventually top-5 finishes.
- Links to accepted findings where possible.

This is hard to fudge because Code4rena contests use independent judging, sponsor review, duplicate handling, and public ranking. A private benchmark can be cherry-picked; a live C4 ranking is third-party market proof.

Strategic message:

> Lots of AI audit tools claim they find historical C4 bugs. ChainShield proves it in live contests, under third-party judging, where invalid findings and duplicates do not count.

Messaging principle:

> No "trust me bro" benchmarks. Real proof.

Investor/customer-safe version:

> ChainShield does not rely on self-reported benchmark claims. We prove performance in live adversarial competitions where independent judges decide what is valid.

#### 3. Workflow Moat

The product should own the workflow before, during, and after audit:

- Pre-launch discovery run.
- Finding verification.
- PoC generation.
- Report creation.
- Patch review.
- Continuous commit scanning.
- CI/CD integration.
- Security history per repo.

If ChainShield becomes where teams manage security findings over time, it is harder to replace than a one-off scanner.

#### 4. Trust Moat

Security is a trust market.

Trust assets to build:

- Public case studies.
- Founder Code4rena track record.
- Published benchmark methodology.
- Signed customer testimonials.
- Anonymized before/after patch stories.
- Reports that look professional enough for investors, exchanges, and protocol communities.
- Clear legal scope and disclaimers.

#### 5. Distribution Moat

The best distribution wedge is not generic SEO. It is proximity to teams about to launch.

Distribution channels:

- Code4rena/Cantina/Sherlock adjacency.
- Crypto seed funds and accelerators.
- Web3 devrel teams.
- Security-conscious L2 ecosystems.
- Audit firms that need leverage.
- CI integrations for protocol teams.

#### 6. Expansion Moat

Start with Solidity/EVM because the benchmark is strongest there.

Then expand by language/ecosystem:

- Solana/Rust.
- Move/Sui/Aptos.
- Cosmos/Go.
- ZK circuits.
- AI-agent-generated application code.
- High-stakes fintech/backend systems.

The expansion moat is not "we support many languages." It is that the same validation engine and feedback loop can learn each new codebase class.

## Next Hurdle

If ChainShield gets clients and keeps ranking well on Code4rena, the next hurdle is:

> Prove repeatability without founder magic.

Investors will want to know whether the system works because the product works, or because the founder is personally steering every audit.

The next proof package should include:

- 10 completed audit runs.
- Runtime per run.
- Cost per run.
- Number of raw candidates.
- Number of validated H/M findings.
- False positive rate after validation.
- PoC success rate.
- Customer acceptance rate.
- Time from repo access to report.
- Human minutes required per audit.
- Which steps are fully automated versus founder-reviewed.

The strongest metric:

> Validated H/M findings per $1,000 of cost and per 24 hours of elapsed time.

The second strongest metric:

> Human review minutes per accepted H/M finding.

The business becomes venture-scale when ChainShield can show that every new audit improves the system and reduces marginal human work.

## Defensibility Roadmap

### Phase 1: Services With Automation

Goal:

- Get paid.
- Produce valid findings.
- Build case studies.
- Capture structured data from every run.

Risk:

- Looks like a tech-enabled audit firm.

### Phase 2: Repeatable Audit Engine

Goal:

- Standardize inputs, outputs, validation gates, PoC workflow, and report generation.
- Show low human review time per finding.
- Build private benchmark dashboards.

Risk:

- Competitors can still claim similar automation.

### Phase 3: Continuous Security Workflow

Goal:

- Integrate into GitHub/CI.
- Scan every PR or commit.
- Track unresolved risks over time.
- Verify patches.
- Become part of release gating.

Risk:

- Must avoid noisy alerts. Security teams will churn if signal quality drops.

### Phase 4: Cross-Codebase Security Layer

Goal:

- Expand beyond EVM.
- Use the same validation and learning system across different languages and threat models.
- Become the security layer for AI-generated software.

Risk:

- Expansion before dominance could weaken focus.

## Near-Term Operating Plan

1. Finalize the streamlined audit runbook.
2. Build the 30-team pre-launch DeFi target list.
3. Send personalized outreach.
4. Book NYC founder-heavy events only if they contain real builders.
5. Run 1-3 pilots.
6. Convert the best pilot into a case study.
7. Continue Code4rena benchmarking.
8. Update pricing after the first 3 customer conversations.

## Open Questions

- What is the minimum customer scope where ChainShield reliably produces value?
- Should first customers get "audit" language or "discovery run" language?
- What legal terms are needed before touching private customer repos?
- How much human review is required before a finding is customer-facing?
- What is the right cap for complex protocols?
- How quickly can the system support non-EVM codebases?
- What evidence will convince investors the automation is repeatable?
