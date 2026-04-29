# ChainShield Strategy

Living strategy doc. Updated during founder office-hours discussions.

Last updated: 2026-04-28

## One-Line Thesis

Software will be built by AI agents at massive scale. Manual security auditing cannot keep up. ChainShield starts by automating professional-grade smart contract security discovery and validation, then expands into continuous security review for every high-stakes codebase AI agents create.

## Current Stage

ChainShield has technical proof, not yet full customer demand proof.

Evidence so far:

- AI Agent Audit can run large Solidity protocol audits in roughly 1-2 days.
- The workflow produces validated High/Medium findings with PoCs.
- Operating cost is low, using only a small fraction of the monthly Codex plan limit.
- The founder has achieved SR Warden status on Code4rena using this app.
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
