# ChainShield Client Outreach Plan

Generated: 2026-05-13

## Objective

Find early buyers for the ChainShield Discovery Pass by running a focused outbound campaign to teams with clear security urgency.

The goal is not broad brand awareness. The goal is to get serious teams to take one of these actions:

- Book a 15-minute security discovery call.
- Share a public or private scoped repo/module.
- Approve a $1,000 Discovery Pass.
- Agree to an anonymized case study if ChainShield finds valid issues.
- Introduce another founder or CTO who has the same launch/security pain.

## Core Offer

### ChainShield Discovery Pass

48-hour AI-assisted smart contract security discovery with audit-level results, before the team commits to a full audit, contest, or mainnet launch.

### Buyer-Facing Positioning

> Find the worst smart contract bugs before your expensive audit does.

### What The Buyer Gets

- 48-hour turnaround.
- One scoped Solidity repo, module, or contract set.
- Audit-style report.
- Validated Medium/High/Critical candidates only.
- Affected code paths and exploit reasoning.
- Severity rationale.
- Fix guidance.
- Coverage summary and risk map if no serious finding is discovered.
- Optional remediation check after fixes.

### What This Is Not

This is not a formal audit certificate and must not be positioned as one.

Use this language:

> ChainShield is a pre-audit discovery pass. It gives audit-level findings, but it is not a full-coverage audit or a guarantee that the protocol is safe to launch.

## Pricing

### Recommended Pilot Pricing

- $1,000 fixed Discovery Pass.
- 48-hour turnaround.
- 1 repo or scoped module.
- Medium+ candidates only.
- Audit-style report.

### Optional Add-Ons

- $500 remediation check after fixes.
- Success fee for validated serious findings:
  - $1,000 per Medium.
  - $3,000 per High.
  - $10,000 per Critical.
  - Cap total success fee at $15,000-$25,000 for early pilots.

### Case Study Discount

For the first 3-5 customers, optimize for proof over revenue.

Offer a discounted or waived success fee if the team allows an anonymized case study with:

- Repo size.
- Runtime.
- Number of validated findings.
- Severity mix.
- Fix outcome.
- Customer quote.

Do not offer unlimited free audits. Free or discounted passes are only for teams that can become strong proof.

## Ideal Customer Profile

### Primary ICP: Pre-Launch DeFi Teams

The best first buyer is:

> A pre-launch DeFi founder or CTO with code nearly ready for mainnet, limited audit budget, and real fear of shipping exploitable contracts.

### Strong Buying Signals

- Mainnet launch planned in the next 30-90 days.
- Public testnet, beta, points season, or TGE announced.
- Solidity repo has recent commits.
- Protocol handles funds or economic state.
- Team mentions audit, security review, contest, launch, or bounty.
- They have not yet completed a full audit.
- They recently completed an audit but are shipping new contracts.
- They have budget pressure and cannot easily afford a $50k-$100k audit.

### Best Protocol Categories

Prioritize teams with high-value attack surfaces:

- Vaults.
- Lending.
- Borrowing.
- Liquidations.
- Staking.
- Rewards.
- Bridges.
- AMMs.
- DEXs.
- Perps.
- Oracles.
- Restaking.
- Tokenized real-world assets.

### Weak Fit

Avoid or deprioritize:

- NFT-only communities with no financial mechanics.
- Token-only marketing pages with no product.
- Teams with no technical founder.
- Teams with no repo, docs, or shipped product.
- Mature teams already locked into a top-tier audit firm unless they have a new urgent module.
- Anyone asking for a free unlimited review with no case-study value.

## Secondary ICP: Frontend Security Buyers

This is a parallel test, not the primary wedge.

Target senior engineering leaders at startups with risky React/Next.js surfaces:

- Auth.
- Payments.
- Admin panels.
- Multi-tenant dashboards.
- API permission boundaries.
- AI apps handling private user data.
- File upload.
- OAuth.
- Wallet connection flows.
- Web3 frontends with transaction signing.

Frontend positioning:

> 48-hour security review for high-risk React/Next.js flows: auth, roles, payments, API exposure, XSS, CSRF, and user-data leaks.

Use this track to test whether buyer trust is higher because the founder is a senior frontend engineer.

## Lead Sources

### Web3 Sources

Use these every week:

- Code4rena active and recent contests: https://code4rena.com/audits
- Cantina competitions: https://cantina.xyz/competitions
- Sherlock contests and protocol pages: https://audits.sherlock.xyz/
- Immunefi programs: https://immunefi.com/explore/
- DeFiLlama new and trending protocols: https://defillama.com/
- Crypto Twitter/X launch announcements.
- ETHGlobal project pages and finalists.
- GitHub searches for recent Solidity activity.
- VC portfolio pages for crypto funds.
- Accelerator demo days and web3 founder communities.

### Search Queries

Use these exact searches:

- `"mainnet soon" Solidity`
- `"testnet live" DeFi`
- `"audit soon" Solidity`
- `"security review" "mainnet"`
- `"points season" DeFi`
- `"TGE" "audit"`
- `"staking" "mainnet soon"`
- `"vault" "testnet"`
- `"perps" "audit"`
- `"bridge" "security review"`
- `"Code4rena" "mitigation review"`
- `"Cantina" "competition" "DeFi"`
- `"Sherlock" "audit" "launch"`
- `"Immunefi" "bug bounty" "DeFi"`

### Frontend Sources

- HackerOne program directory: https://hackerone.com/bug-bounty-programs
- Bugcrowd program directory: https://www.bugcrowd.com/bug-bounty-list/
- YC company directory.
- Vercel/Next.js showcase companies.
- Product Hunt launches with payments, AI, auth, or user data.
- LinkedIn posts from CTOs hiring security or platform engineers.
- Startups announcing SOC 2, enterprise readiness, or public beta.

## Lead Qualification Score

Score every lead from 0-10.

### Add Points

- +2 launch or audit timing in the next 90 days.
- +2 handles user funds or high-value private data.
- +2 recent code activity.
- +1 public founder/CTO contact available.
- +1 prior security spend, bounty, audit, or contest.
- +1 budget pressure or early-stage team.
- +1 narrow module can be reviewed in 48 hours.

### Outreach Threshold

- 8-10: contact immediately with a personalized message.
- 6-7: contact if the team has a strong technical surface.
- 4-5: save for later.
- 0-3: ignore.

## CRM Fields

Create a spreadsheet or CRM with these columns:

| Field | Description |
| --- | --- |
| Company / protocol | Name of the target |
| Website | Public URL |
| Founder / CTO | Actual human, not company account |
| Role | Founder, CTO, protocol engineer, security lead |
| Contact | Email, X, Telegram, Discord, LinkedIn, warm intro |
| Source | Code4rena, Cantina, Sherlock, Immunefi, GitHub, X, event, intro |
| Chain / stack | EVM, Solidity, React, Next.js, etc. |
| Launch signal | Why now? |
| Security signal | Audit soon, bounty, contest, unaudited, recent exploit, SOC 2 |
| Repo status | Public, private, unknown |
| Risk surface | Vault, lending, bridge, auth, payments, admin, etc. |
| Qualification score | 0-10 |
| Outreach status | Not contacted, contacted, replied, call booked, repo access, paid |
| Last touch | Date |
| Next action | Follow-up, call, pilot proposal, close lost |
| Notes | Specific context from research or call |

## Weekly Execution Plan

### Week 1: Build The List And Send First Messages

Target:

- 50 qualified leads.
- 50 personalized first-touch messages.
- 10 replies.
- 5 calls booked.
- 2 repo-access conversations.
- 1 paid or case-study pilot.

Daily workflow:

1. Research 10 leads.
2. Score each lead.
3. Find the actual founder, CTO, protocol engineer, or security lead.
4. Write one specific first line for each lead.
5. Send the message.
6. Log status and next action.
7. Follow up on prior messages.

### Week 2: Follow Up And Close Pilots

Target:

- 50 additional leads.
- 50 new first-touch messages.
- Follow up with all Week 1 non-responders.
- Convert 2-3 conversations into Discovery Pass pilots.

Daily workflow:

1. Send 10 new messages.
2. Send follow-ups to messages from 3-5 business days ago.
3. Run discovery calls.
4. Send scoped pilot proposals.
5. Ask for repo/module access.
6. Close the smallest possible paid pass.

## Outreach Rules

### Do

- Personalize the first line.
- Mention a specific launch, repo, contest, bounty, module, or risk surface.
- Ask for a small next step.
- Sell the 48-hour discovery pass, not a platform.
- Use the founder's SR Warden proof.
- Say "audit-level findings" but not "formal audit."
- Keep messages short.
- Follow up twice.

### Do Not

- Do not send generic "AI security platform" messages.
- Do not pitch replacing formal audits.
- Do not claim full coverage.
- Do not promise to find bugs.
- Do not offer unlimited free work.
- Do not ask for a 60-minute meeting.
- Do not talk about the whole future vision in the first message.

## First-Touch Messages

### Web3 Cold Email / DM

```text
Hey {name}, I saw {specific launch/repo/security signal}.

I’m building ChainShield. We run a $1,000 pre-audit Discovery Pass for Solidity teams: 48 hours, audit-level findings, no scanner spam.

The goal is to catch the worst Medium/High issues before you spend $50k+ on a formal audit, contest, or launch review.

I’m an SR Warden on Code4rena and have used this workflow to find accepted High/Medium issues.

Would it be useful if I looked at {specific repo/module} and showed you what comes back?
```

### Short X / Telegram Version

```text
Hey {name}, saw {specific launch/repo signal}. I run a $1k / 48-hour pre-audit Discovery Pass for Solidity teams: audit-level Medium+ findings, no scanner spam. Useful before a contest, audit, or mainnet launch. Want me to look at {specific module/repo}?
```

### Warm Intro Ask

```text
Hey {name}, quick ask. Do you know one DeFi founder or CTO shipping Solidity code in the next 60 days who is worried about audit cost or timing?

I’m offering a $1,000 / 48-hour ChainShield Discovery Pass: audit-level Medium+ findings before a full audit or contest.

Best fit is a serious pre-launch team with real code and budget pressure.
```

### Frontend Security Version

```text
Hey {name}, I saw {specific product/auth/payments/admin signal}.

I’m testing a 48-hour security Discovery Pass for React/Next.js apps focused on auth, permissions, payments, API exposure, XSS, CSRF, and user-data leaks.

I’m a senior frontend engineer, so this is not a generic scanner pass. The output is concrete exploit paths and fix guidance your team can act on.

Would it be useful if I reviewed one high-risk flow and showed you what comes back?
```

## Follow-Up Sequence

### Follow-Up 1: 3-5 Business Days Later

```text
Quick follow-up. The best fit is when a team is about to launch, schedule an audit, or ship a risky module.

If {protocol/company} has one area you are worried about, I can scope the Discovery Pass around just that part and keep the review tight.
```

### Follow-Up 2: 7-10 Business Days Later

```text
Last note from me. If timing is bad, no worries.

If you are planning a launch/audit later, I’m happy to run the Discovery Pass on one scoped module first so you can see whether the signal is useful before committing to anything bigger.
```

### Breakup Message

```text
Closing the loop here. If security timing becomes painful before launch, send me the highest-risk module and I’ll tell you whether it is a fit for a 48-hour Discovery Pass.
```

## Discovery Call Script

Keep calls to 15 minutes.

### Goal

Learn whether they have urgent security pain and whether they will share scope/repo access.

### Questions

1. When are you trying to launch or ship the next risky change?
2. What is your current security plan?
3. Have you budgeted for a formal audit, contest, or bounty?
4. What part of the protocol/app worries you most?
5. What would delay launch if security review takes too long?
6. Have you had a bad audit, contest, or bug bounty experience before?
7. If we found one valid serious issue in 48 hours, what would that be worth?
8. Would you be willing to share one scoped repo/module for a limited Discovery Pass?

### Listen For

- Budget pain.
- Launch pressure.
- Audit scheduling delay.
- Fear around a specific module.
- Prior security incidents.
- Willingness to share repo access.
- Willingness to pay.
- Urgency this week, not someday.

### Red Flags

- "Sounds interesting" but no next step.
- No launch date.
- No repo access.
- No willingness to pay anything.
- Wants a full free audit.
- Cannot name a risky module.
- No technical owner on the call.

## Pilot Proposal Template

```text
Based on our call, I recommend a scoped ChainShield Discovery Pass on {repo/module}.

Scope:
- {contract/module 1}
- {contract/module 2}
- Exclusions: {anything out of scope}

Deliverable:
- Audit-style report within 48 hours.
- Validated Medium/High/Critical candidates only.
- Exploit reasoning, affected code paths, and fix guidance.
- Coverage summary and risk map if no serious finding is discovered.

Price:
- $1,000 fixed.
- Optional $500 remediation check.
- Optional success fee only if we agree in writing before the pass.

Important:
This is a pre-audit discovery pass, not a formal audit certificate or full-coverage guarantee.

Next step:
Please send repo access, docs, branch/commit hash, scope notes, and any known exclusions.
```

## Scope Intake Checklist

Before starting a Discovery Pass, collect:

- Repo URL.
- Branch or commit hash.
- In-scope files/contracts/modules.
- Out-of-scope files/contracts/modules.
- Existing docs.
- Prior audits.
- Known issues.
- Build instructions.
- Deployment chain/network.
- Any unusual tokens, oracles, bridges, keepers, or privileged roles.
- Contact for technical questions.
- Permission to use anonymized metrics in a case study.

## Success Metrics

Track weekly:

- Leads researched.
- Messages sent.
- Reply rate.
- Positive reply rate.
- Calls booked.
- Calls completed.
- Repo-access requests.
- Repo access granted.
- Discovery Passes sold.
- Revenue.
- Valid findings produced.
- Case studies secured.
- Referrals received.

### Minimum Healthy Benchmarks

Early outbound is noisy. Use these as rough targets:

- 50 messages should produce at least 5 replies.
- 10 calls should produce at least 2 repo-access conversations.
- 3 repo-access conversations should produce at least 1 pilot.

If results are worse, revise the ICP or offer before sending hundreds more messages.

## Positioning Notes For Marketing

### Say This

- "Pre-audit discovery pass."
- "Audit-level findings."
- "48-hour turnaround."
- "Validated Medium+ candidates only."
- "No scanner spam."
- "Catch the worst bugs before the expensive audit."
- "Useful before a formal audit, contest, or launch review."

### Avoid This

- "AI replaces auditors."
- "Cheap audit."
- "Guaranteed bug finding."
- "Complete coverage."
- "Fully automated audit."
- "Safe to launch."
- "Security certification."

## Website / Landing Page Copy

### Hero

> Find the worst smart contract bugs before your expensive audit does.

### Subhead

> ChainShield runs a 48-hour pre-audit Discovery Pass on scoped Solidity repos and returns audit-level Medium+ findings with exploit reasoning and fix guidance.

### CTA

> Book a 15-minute scope call

### Proof Points

- Built by a Code4rena SR Warden.
- Used to produce accepted High/Medium findings.
- Combines static analysis, actor analysis, invariant analysis, and multi-agent validation.
- Designed for serious pre-launch teams that need signal before spending $50k+ on a formal audit.

### Disclaimer

> ChainShield Discovery Pass is not a formal audit certificate or full-coverage security guarantee. It is a focused discovery review intended to surface serious issues early.

## Marketing Team Daily Checklist

Every weekday:

1. Add 10 qualified leads.
2. Score each lead.
3. Find the actual decision-maker.
4. Write one specific first line.
5. Send 10 first-touch messages.
6. Send scheduled follow-ups.
7. Update CRM status.
8. Flag any warm replies for founder follow-up.
9. Book calls directly on the founder's calendar.
10. Record objections and buyer language.

## Founder Responsibilities

The marketing team can research, write, send, and schedule.

The founder should handle:

- Technical discovery calls.
- Final scope approval.
- Pricing exceptions.
- Repo access decisions.
- Delivery of the Discovery Pass.
- Case study approval.
- Learning from objections.

## Objection Handling

### "We already have an audit scheduled."

Response:

```text
That is actually the best timing. The Discovery Pass is meant to catch serious issues before the formal audit starts, so the expensive audit is not spent on bugs you could patch this week.
```

### "We do not have budget."

Response:

```text
That is why the pass is scoped at $1,000. It is not a replacement for a full audit. It is a way to find the highest-risk issues before committing to a much larger audit budget.
```

### "Can you do it for free?"

Response:

```text
For a strong case-study fit, we can discount the first pass in exchange for anonymized metrics and a quote. We still need a tight scope and a real technical owner on your side.
```

### "How is this different from a scanner?"

Response:

```text
Scanners produce alerts. ChainShield produces audit-style findings: exploit path, affected code, severity rationale, validation notes, and fix guidance. We report only serious validated candidates.
```

### "Is this a full audit?"

Response:

```text
No. It is a pre-audit discovery pass. The output is audit-level, but the scope is intentionally narrower and faster. It helps you find serious bugs earlier, but it does not certify the system as safe.
```

### "We are not ready yet."

Response:

```text
No problem. The best timing is usually when you have one module that is close enough to review but early enough that fixes are still cheap. When will that be true for you?
```

## Two-Week Experiment Decision

After 2 weeks, review the data.

Continue web3-first if:

- At least 100 messages sent.
- At least 10 positive replies.
- At least 5 calls completed.
- At least 2 repo-access conversations.
- At least 1 paid or case-study pilot.

Increase frontend-security testing if:

- Web3 reply quality is weak.
- Frontend buyers respond faster.
- Frontend buyers trust the founder faster.
- Frontend teams share repo/app access more easily.
- Frontend teams show clearer willingness to pay.

Do not pivot based on feelings. Pivot based on buyer behavior.

## Final Operating Principle

The marketing team is not selling "AI security."

They are finding teams with urgent launch/security pain and offering a small, concrete, low-risk next step:

> $1,000. 48 hours. Audit-level serious findings. No scanner spam. Not a formal audit. A way to find the worst bugs before the expensive audit does.
