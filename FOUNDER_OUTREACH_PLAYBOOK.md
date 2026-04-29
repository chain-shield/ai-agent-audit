# ChainShield Founder Outreach Playbook

Generated from office hours on 2026-04-28.

## The Current Truth

ChainShield has strong technical proof, but not customer demand proof yet.

The app has already produced real Code4rena signal, including SR Warden status and validated H/M findings. That proves the audit engine can find serious bugs. It does not yet prove that protocol teams will pay, change their workflow, or depend on the product.

The next milestone is simple:

> Get a handful of pre-launch DeFi teams to give ChainShield repo access, run fast discovery audits, and convert at least one into a paid or case-study pilot.

## The Wedge

Do not pitch "AI audits will replace manual audits" first. That sounds too big and triggers skepticism.

Pitch this instead:

> Before you spend $80k-$150k and wait weeks for a traditional audit, ChainShield runs a 48-hour pre-launch discovery pass on your Solidity repo. You get validated Medium/High findings with PoCs, and you only pay meaningful money if we find valid issues.

The wedge is:

- Pre-launch DeFi teams
- Solidity/EVM first
- Serious enough to need security
- Too budget-constrained for a six-figure audit
- Close enough to launch that speed matters now

## ICP: The Actual Buyer

Avoid "web3 startups" as a category. It is too broad.

The first buyer is likely:

> A pre-launch DeFi founder or CTO with code nearly ready for mainnet, limited audit budget, and real fear of shipping exploitable contracts.

Strong signals:

- Public testnet or beta is live
- Mainnet launch planned in the next 30-90 days
- Repo has recent Solidity commits
- The protocol handles user funds, vaults, staking, lending, swaps, bridges, perps, or rewards
- Team mentions audit, security review, launch, points season, TGE, or TVL
- They are too early for Trail of Bits/OpenZeppelin-level pricing

Weak signals:

- NFT-only communities
- Token-only marketing pages with no product
- Teams with no public code and no technical founder
- Generic web3 networking rooms with no builders

## Where To Find Them

### 1. Code4rena, Cantina, Sherlock, Immunefi Adjacency

Start with teams already paying for security or visibly preparing for it.

Look at:

- Active and recent Code4rena contests
- Small or mid-sized prize pools
- Mitigation reviews
- Protocols that completed a contest but will ship follow-up contracts
- Similar protocols that have not yet run a contest

Why this works:

These teams already believe security matters. You do not need to educate them on audit pain. You only need to prove ChainShield is faster, cheaper, and useful before or between formal audits.

### 2. NYC Founder-Dense Events

Yes, go to NYC. But do not "network." Run a customer-discovery sprint.

Good rooms:

- Web3 founder pitch nights
- DeFi/fintech founder events
- ETHGlobal side events
- VC/founder mixers
- Crypto accelerator demo days
- Builder-heavy meetups with technical founders

Bad rooms:

- NFT/social events
- Token promotion rooms
- General crypto panels
- Investor-only panels where no founders are building

Your goal after each NYC trip:

- 10 founder conversations
- 3 follow-up calls booked
- 2 repo-access offers or intros

### 3. VC And Accelerator Warm Intros

Ask crypto seed funds, accelerators, devrel teams, and infra companies this exact question:

> Which portfolio company is shipping Solidity code in the next 60 days and is worried about audit cost or timing?

Do not ask:

> Do you know anyone who needs security?

That is too vague.

### 4. GitHub, X, Discord, And Launch Signals

Search for teams using phrases like:

- "mainnet soon"
- "testnet live"
- "audit soon"
- "security review"
- "points season"
- "TGE"
- "vault"
- "lending"
- "staking"
- "bridge"
- "perps"
- "AMM"
- "yield"
- "Solidity"

Then check whether they have:

- Public GitHub activity
- Recent Solidity commits
- Founder/CTO contact info
- Docs describing fund flows
- A launch timeline

## This Week's Assignment

Build a list of 30 pre-launch DeFi teams.

For each team, capture:

| Field | Notes |
| --- | --- |
| Protocol | Name of the project |
| Website | Public URL |
| Founder/CTO | Actual person, not company account |
| Contact | X, Telegram, Discord, email, or warm intro path |
| Chain | EVM, Solidity, or other |
| Launch signal | Why now? |
| Security status | Unaudited, audit soon, contest planned, recently audited |
| Repo status | Public, private, unknown |
| Risk surface | Vault, lending, oracle, bridge, staking, perps, etc. |
| Outreach status | Not contacted, contacted, replied, call booked, repo access |
| Notes | Specific pain or context |

Minimum target:

- 30 teams researched
- 30 personalized messages sent
- 10 actual founder conversations
- 5 repo-access conversations
- 1 discounted or unpaid pilot that can become a public case study

## Outreach Message

Use this as the base, but personalize the first line.

```text
Hey {name}, I saw {specific launch/repo/product signal}.

I am building ChainShield, a fast security discovery pass for pre-launch Solidity teams.

Traditional audits can take weeks and cost $80k+. We run a 48-hour pass that produces validated Medium/High findings with PoCs, so teams can patch the serious issues before a full audit, contest, or mainnet launch.

We have already used the system to produce valid Code4rena findings, including high-severity results.

Would it be useful if I ran a small discovery pass on {protocol/repo/module} and showed you what comes back?
```

## Call Script

Keep the call about their pain, not your product.

Ask:

1. When are you trying to launch?
2. What is your current security plan?
3. Have you budgeted for an audit yet?
4. What would delay launch if security review takes too long?
5. What part of the protocol worries you most?
6. If we found one valid Medium/High issue in 48 hours, what would that be worth to you?
7. Would you be willing to give repo access for a limited discovery pass?

Listen for:

- Budget pain
- Launch pressure
- Audit scheduling delays
- Prior bad audit experience
- Fear around specific contract flows
- Willingness to share repo access
- Willingness to pay for validated findings

## Pilot Offer

For the first few teams, optimize for proof and testimonials, not margin.

Recommended pilot:

> $1,500 onboarding fee, then pay only for valid Medium+ findings, capped at $25,000.

Alternative case-study pilot:

> Free or discounted 48-hour run in exchange for permission to publish anonymized metrics: repo size, runtime, validated findings, and customer quote.

Do not give away endless free audits. The free/discounted offer is only for case-study-quality teams.

## What Counts As Demand

Real demand:

- They give repo access
- They pay or agree to pay
- They ask for a second run
- They introduce another founder
- They delay launch to wait for your results
- They want ChainShield in CI before merge
- They get nervous if you cannot run it this week

Not demand:

- "Interesting"
- "Cool idea"
- "AI security is important"
- "Let's stay in touch"
- Newsletter signups
- Generic positive feedback

## Investor Proof To Build Before Raising

Before raising $3M-$4M, aim to show:

- 3-5 real protocol pilots
- At least 1 paid customer
- Validated H/M findings from customer repos
- One public or anonymized case study
- Code4rena performance trend toward top-5 consistency
- Runtime and cost metrics proving the automation advantage
- Evidence that findings survive professional review
- Evidence that the process is not dependent on one founder manually steering every result

## The Fundraising Narrative

The concise version:

> Software is going to be built by AI agents at massive scale. Manual security auditing cannot keep up. It is slow, expensive, and still misses critical bugs. ChainShield automates professional-grade security discovery and validation, starting with Solidity audits, then expanding to every high-stakes codebase AI agents create.

The proof order:

1. We can find real bugs.
2. We can validate them automatically.
3. We can do it in 1-2 days.
4. It costs us almost nothing compared with human audits.
5. Protocol teams will pay because the current process is too slow and expensive.
6. Once trusted, this becomes continuous auditing on every commit.

## Do Next

1. Build the 30-team target list.
2. Book one NYC founder-heavy event.
3. Ask 5 crypto investors/devrel people for pre-launch Solidity team intros.
4. Send 30 personalized messages.
5. Run one pilot as soon as a team gives repo access.
6. Turn the result into a case study, even if anonymized.

