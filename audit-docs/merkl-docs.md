# Technical Overview

Merkl operates through an offchain engine that analyzes both onchain and offchain data to track user activity and allocate rewards based on campaign rules set by campaign creators. The Merkl engine processes reward data into a merkle tree, compresses it into a merkle root, and pushes it onchain, enabling users to claim rewards transparently and efficiently.

## 🗺️ Platform Overview

Merkl operates through campaigns created by campaign creators. A campaign is a time-bound incentive program where Merkl tracks onchain and offchain activity based on predefined rules. Rewards or points are either posted onchain or updated offchain via Merkl’s endpoints, allowing users to monitor their accumulated rewards.

### How it works

1. **Campaign Creation**: Campaign creators set up campaigns using a smart contract called the **Merkl Distribution Creator**. This includes defining eligibility criteria, reward structures, and distribution methods. Campaign details are pushed onchain, along with the incentive tokens that need to be distributed
2. **User Participation**: Users engage with the protocol (e.g., providing liquidity, lending, borrowing) in order to earn the rewards/points as specified in the campaign.
3. [**Reward Computation:**](#reward-computation) At fixed intervals, a system called the **Merkl Engine** fetches campaigns from the **Merkl Distribution Creator** contract and processes reward calculations based on available onchain and offchain data and on the rules set by the campaign creator. **The Merkl Engine tracking is exhaustive and does not rely on discretionary snapshots**.
4. [**Reward Update:**](#reward-updates) After each reward computation, users can see that they have earned **pending rewards** (awaiting onchain reward update). At regular intervals, (potentially different from the reward computation intervals), the Merkl Engine generates a merkle tree from processed campaigns. This tree is then compressed as a merkle root that is pushed onchain to a contract called the **Merkl Distributor** contract. A reward file is also made available [on the Merkl status page](https://app.merkl.xyz/status), in order to ensure transparency and enable anyone to manually audit past reward distributions.
5. [**Dispute Period:**](#dispute-process) After this step, a 1-2 hour dispute period begins, during which newly computed rewards cannot be claimed yet. However, rewards from the previous merkle root remain claimable. During this period, dispute bots verify the published reward file. If an inconsistency is found, a dispute can be raised to halt incorrect distributions before they become effective.
6. **Reward Availability:** Once the dispute period ends, users can see their rewards on any frontend integrated with the Merkl API. Users can claim rewards through a Merkl-powered frontend. Merkle proofs, required for claiming, are provided by the Merkl API or can be computed from reward files.

### Key Features

* **Single Merkl Root per Chain:** Merkl consolidates rewards from multiple campaigns into one merkle root per chain, enabling efficient batch claiming rewards from multiple different campaigns in a single transaction.
* **Aggregated Campaigns:** While campaigns operate independently, Merkl batches multiple campaign updates into a single onchain transaction
* **Automatic Catch-Up Mechanism:** If any rewards are not included in an update, they are automatically distributed in the next cycle. The Merkl engine ensures no missed rewards, processing only the data from the previous execution point.
* **Unclaimed Rewards Roll Over**: Users can claim their rewards at anytime they want: each merkle tree update takes the previous merkle tree state and simply adds the new rewards, which are then reflected in the published merkle root.

## 🤿 Deep-Dive

### Reward Computation

**Reward computation** (or "compute") is the process by which the Merkl Engine calculates reward allocations based on campaign rules.

**Computation mechanism**: The Merkl Engine relies on events to reconstruct the exact position of each participant. An integral is then computed over the selected time frame for every user, with no approximations—everything is exact. Users are rewarded proportionally to their individual area under the curve compared to the total area of all participants. In rare occurrences (e.g., debt accrual in lending markets), events alone are not enough to reconstruct the exact positions (e.g., debt accrual events are only emitted when a user interacts with the market), to remedy this, the Merkl Engine keeps track of **invariants** for these protocols, it compares at regular intervals the total event-based positions of users and the onchain value (e.g., calling `totalDebt()` on a lending market). If the two values differ by more than **1%**, the Merkl Engine will check each user's position onchain to update the computed value and remain exact.

**Computation frequency**: The Merkl engine processes rewards for each campaign approximately every 2 hours on average, analyzing all relevant onchain events affecting the incentivized asset.

**After computation**: Once a campaign completes its computation cycle, rewards are not immediately claimable onchain but appear as **pending rewards** in user dashboards.

**Parallel processing**: Campaigns on a chain are processed independently and in parallel. This means rewards from some campaigns may be available onchain while others are still computing or awaiting updates.

**Delayed computations**: Occasionally, campaign computations may be delayed due to processing constraints. Track delayed campaigns on the [Merkl status page](https://app.merkl.xyz/status).

**Continuous processing**: When a campaign is processed, the Merkl Engine resumes from its last checkpoint, ensuring complete reward coverage even when updates occur at different intervals.

{% hint style="info" %}
**Reward delays & retroactive distribution**

Occasional delays of a few hours may occur in reward distribution. If an invariant is triggered or a third-party data feed experiences a temporary issue, calculations are paused until all safety checks pass at the next scheduled computation.

When this happens, the Merkl system is designed so that **all rewards are distributed retroactively**, with no rewards undistributed.
{% endhint %}

### Reward Updates

**Reward updates** are the process of posting computed rewards onchain, making them claimable by users.

**Update frequency**: Reward updates occur approximately every 8 hours (ranging from 4 to 12 hours), depending on the chain.

**Delayed updates**: Like computations, updates may occasionally be delayed. Monitor update status on the [Merkl status page](https://app.merkl.xyz/status).

**Transparency**: Each reward update includes a publicly available reward file on the status page for transparency and auditability.

**Independent cycles**: Reward computation and reward updates operate on independent schedules. Updates push the results of completed computations onchain, but the two processes follow separate lifecycles.

**Example**: Between two reward updates (8 hours apart on average), a campaign's rewards may be recomputed up to four times. Each computation refines the pending rewards, but they only become claimable when the next update occurs.

**Rollover mechanism**: Unclaimed rewards automatically roll over into subsequent reward updates, allowing users to claim at their convenience.

**Tracking updates**: View the last reward update for each chain on the [Merkl status page](https://app.merkl.xyz/status).

### Dispute Period & Process

The **dispute period** is a security window of 1-2 hours following each reward update during which anyone can contest reward allocations made by the Merkl engine. This safeguard ensures the integrity and consistency of the reward infrastructure and allows campaign creators to oversee the reward computations before they become final and claimable.

**Raising a dispute**: If an issue is detected during the dispute period or if you want to contest reward allocations for a campaign, raise a dispute by sending a `disputeToken` to the Merkl Distributor contract.

**Dispute outcomes**:

* **Valid dispute**: The merkle root is revoked and the disputer is refunded
* **Invalid dispute**: The disputer forfeits their funds and the dispute period restarts

**Dispute parameters**: Retrieve dispute conditions (`disputeToken`, `disputeAmount`, `disputePeriod`) from the Distributor contract on the relevant chain.

#### Dispute Bots

**Security infrastructure**: The Merkl team operates several independent dispute bots on separate infrastructure to ensure redundancy and prevent malicious actors from compromising the reward update process.

**Open-source reference**: While the production bot code is closed source for security reasons, you can reference [this open-source repository](https://github.com/AngleProtocol/merkl-dispute) to understand how to build a dispute bot.

{% hint style="info" %}
**Strengthen the network**: More active dispute bots make the system more secure. Need help setting one up? Reach out to the Merkl team—we're happy to assist!
{% endhint %}

## 📌 Key Merkl Concepts

### Opportunity vs. Campaign

Understanding the distinction between **opportunities** and **campaigns** is fundamental to how Merkl operates.

* **Campaign**: An individual incentive program created by a campaign creator with specific parameters including [a distribution type](https://docs.merkl.xyz/merkl-mechanisms/distributions), [a scoring type](https://docs.merkl.xyz/merkl-mechanisms/scoring), [customization options](https://docs.merkl.xyz/merkl-mechanisms/customization-options), a budget amount, and a duration. Each campaign has [a specific type](https://docs.merkl.xyz/merkl-mechanisms/campaign-types) and targets a particular onchain behavior (e.g., providing liquidity in a pool, holding a token, lending/borrowing)—this targeted behavior represents an opportunity.
* **Opportunity**: A specific asset (e.g., pool, vault) and its associated action (e.g., depositing liquidity, borrowing assets) that can be incentivized. Multiple campaigns can run in parallel on a single opportunity, meaning users performing one onchain action can simultaneously earn rewards from several different campaigns.\
  Example: *Providing liquidity to a SushiSwap V3 pool is an opportunity that may have multiple active campaigns offering different rewards.*

<figure><img src="https://3295124503-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2FKRQdTiHhBGLRKeCCOqdc%2Fuploads%2Fgit-blob-8465bb5f03eba6831d981e5103eb0dd32d256392%2FGroup%2023.png?alt=media" alt=""><figcaption><p>An opportunity page with several incentive campaigns running on it</p></figcaption></figure>

### Main Metrics

The following metrics are fundamental to Merkl and appear throughout all Merkl user-facing components: the API, the app, and Merkl Studio.

#### Daily Rewards

Daily rewards for a campaign represent the total amount of tokens or points distributed each day, shared among all eligible users of the campaign. While this number is typically accurate, it may be estimated for certain [distribution types](https://docs.merkl.xyz/merkl-mechanisms/distributions) (such as fixed and capped reward rates) that distribute based on the eligible TVL in the campaign.

When multiple campaigns run on an opportunity, including subcampaigns, the daily rewards displayed for the opportunity represent the sum of all individual campaign daily rewards.

#### TVL

The Total Value Locked (TVL) of a campaign on Merkl represents the total value of **eligible** assets for the campaigns on the opportunity. This reflects only the assets that meet the campaign's eligibility criteria, for example:

* if a campaign includes a blacklist, the TVL excludes the value held by blacklisted addresses.
* for certain campaign types like net-lending campaigns, the TVL represents the net supplied TVL (i.e., total supplied minus borrowed), as this reflects only the liquidity that's eligible for rewards.

In these cases, Merkl may initially approximate the TVL and requires an engine computation to display the accurate TVL for the campaign. As such, the TVL may not be accurate at launch for newly created campaigns. In some situations, the TVL might also be over- or underestimated due to approximations made for computational efficiency or because exact computing logic has not yet been implemented.

When the Merkl system has a fallback mechanism, TVL values update every 10 minutes on the Merkl app and API. For complex campaigns without such fallbacks, TVLs (and consequently APRs) only update after each computation cycle. This means TVL updates may occur at most every 2 hours for the opportunities related to these campaigns.

Overall, the TVL serves as a key indicator of the market or pool's size and liquidity depth.

When multiple campaigns run on the same opportunity with different eligibility rules, the displayed TVL for the opportunity is **the maximum eligible TVL** across all campaigns.

#### APR

The Annual Percentage Rate (APR) represents the yearly return from participating in a campaign, expressed as a percentage. Depending on the [distribution type](https://docs.merkl.xyz/merkl-mechanisms/distributions), the APR can be fixed and remain constant throughout the campaign, or it can be variable and fluctuate based on factors such as eligible TVL and the number of participants.

For [distribution types](https://docs.merkl.xyz/merkl-mechanisms/distributions) where the APR is not fixed, the APR for a campaign is calculated as:

$$
\frac{\text{Daily Rewards} \times 365}{\text{Eligible TVL}}
$$

In these cases, the APR updates at the same frequency as the eligible TVL (every 10 minutes for simple campaigns, or every 2 hours for more complex campaigns where TVL cannot be simply approximated).

At the opportunity level, the displayed APR is the sum of the APRs from all campaigns ([including subcampaigns](https://docs.merkl.xyz/reward-forwarding#linked-opportunities)) running on that opportunity.

## 🧱 User-Facing Components

Beyond the Merkl engine and dispute bots, the Merkl ecosystem includes several user-facing components that work together to enable campaign creation, reward tracking, and claiming.

### Merkl Smart Contracts

Deployed on each blockchain integrated by Merkl, Merkl's smart contracts store campaign data, hold incentive tokens, and process reward claims.

These contract are [publicly available](https://github.com/AngleProtocol/merkl-contracts) and have been audited by Code4rena ([Audit Report](https://code4rena.com/reports/2023-06-angle)).

Key contracts:

* `DistributionCreator`: Stores campaign details and configurations.
* `Distributor`: Holds tokens and processes reward claims based on merkle proofs.
* `AccessControlManager`: A multisig-controlled contract that manages disputes, fees, and access control but cannot alter distributions.

Smart contract addresses, categorized by chain are listed [here](https://app.merkl.xyz/status).

### Merkl API

[Merkl API](https://docs.merkl.xyz/integrate-merkl/app) provides real-time access to Merkl data, including rewards, APRs, merkle proofs, and analytics. This API enables any frontend to integrate Merkl seamlessly.

### Merkl App

[The Merkl App](https://app.merkl.xyz/), built on the Merkl API, enables users to explore reward opportunities, track APRs, and seamlessly claim their rewards.

<figure><img src="https://3295124503-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2FKRQdTiHhBGLRKeCCOqdc%2Fuploads%2Fgit-blob-f0f88bee46f646fc81fe16684fd112d789d0266d%2FCapture%20d%E2%80%99%C3%A9cran%202025-09-17%20%C3%A0%2010.51.55%201.png?alt=media" alt="Homepage of the Merkl app"><figcaption><p>Home page of the Merkl App</p></figcaption></figure>

The interface is organized around several types of pages:

* **Home page (all opportunities)** – displays all available opportunities with filtering options
* **Opportunity page** – dedicated view for each opportunity with all its campains
* **Protocol / Chain / Liquidity program page** – groups all opportunities related to a specific protocol, chain, or program
* **Dashboard** – where users can claim their rewards

<figure><img src="https://3295124503-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2FKRQdTiHhBGLRKeCCOqdc%2Fuploads%2Fgit-blob-66c4725495129ad3292956be237448baf02a04f9%2FCapture%20d%E2%80%99%C3%A9cran%202025-11-12%20%C3%A0%2015.14.56%201.png?alt=media" alt=""><figcaption><p>Dashboard page where users can claim their rewards</p></figcaption></figure>

{% hint style="info" %}
Among all these pages, the **Opportunity page** is central, as this is where you’ll find the campaigns created.
{% endhint %}

Detailed info for each campaign running on an opportunity is available across several tabs:

* **Overview:** global details such as dates, APR, and eligibility rules,…
* **Advanced**: distribution progress, last snapshot, creator address, and campaign ID,…
* **Leaderboard**: list of addresses participating in the opportunity
* [**Linked opportunities**](https://docs.merkl.xyz/reward-forwarding#linked-opportunities) *(optional):* displays opportunities connected through shared l siquidity and rewards

<figure><img src="https://3295124503-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2FKRQdTiHhBGLRKeCCOqdc%2Fuploads%2Fgit-blob-76b81415c35d023dc338b3210960c879ac352dc3%2FGroup%2024.png?alt=media" alt=""><figcaption><p>Campaign-specific info on the opportunity page</p></figcaption></figure>

### Merkl Studio

[Merkl Studio](https://studio.merkl.xyz) is the command center for campaign creators. It enables anyone to launch and manage incentive campaigns independently within minutes, providing powerful tools for campaign configuration and oversight.

# Incentive Mechanisms

Merkl is designed to be highly versatile, supporting a wide range of incentive structures across both DeFi and broader Web3 ecosystems.

## 🔧 Campaign Design

When creating a campaign on Merkl, campaign creators can configure:

### 1️⃣ Campaign Type

[Campaign types](https://docs.merkl.xyz/merkl-mechanisms/campaign-types) define the action that needs to be done to be eligible to rewards.

Merkl supports multiple campaign types, rewarding users for both financial activities (e.g., liquidity provision, lending) and other onchain/offchain actions.

Some of the most commonly used campaign types include

* [**Concentrated Liquidity**](https://docs.merkl.xyz/merkl-mechanisms/campaign-types/concentrated-liquidity-mechanisms): Rewards liquidity providers (LPs) in Concentrated Liquidity AMMs (CLAMMs) such as Uniswap V3 or Uniswap V4.
* [**Lending & Borrowing**](https://docs.merkl.xyz/merkl-mechanisms/campaign-types/lending-borrowing): Encourages activity on lending protocols like Morpho, Euler, or Aave, or rewards specific behaviors within these protocols.
* [**Airdrop**](https://docs.merkl.xyz/merkl-mechanisms/campaign-types/airdrop): Distributes tokens to a potentially millions of users based on either a JSON file or a snapshotted token balance.
* [**Token Holding**](https://docs.merkl.xyz/merkl-mechanisms/campaign-types/erc20-mechanisms): Rewards users for holding an ERC20 token over time. This can be used for virtually any protocol that gives ERC20 receipt tokens to their stakeholders.

### 2️⃣ Distribution Type

[Distribution types](https://docs.merkl.xyz/merkl-mechanisms/distributions) define the **reward distribution model** and **how the total campaign budget is spent over time**. Common models include:

* [Variable reward rate campaigns](https://docs.merkl.xyz/distributions#variable-reward-rate-campaigns): rewards are distributed proportionally based on time-weighted liquidity within the eligibility pool.
* [Fixed reward rate campaigns](https://docs.merkl.xyz/distributions#fixed-reward-rate-campaigns): a predefined amount of rewards per unit of liquidity is distributed at a fixed rate.
* [Capped reward rate campaigns](https://docs.merkl.xyz/distributions#capped-reward-rate-campaigns): similar to variable rate campaigns, but with a maximum APR that cannot be exceeded.

### 3️⃣ Scoring Type

[Scoring types](https://docs.merkl.xyz/merkl-mechanisms/scoring) define **how individual user contributions are measured and converted into reward shares**. Common models include:

* **1:1 mapping (default)**: User reward share equals their share of total contributions
* **Max balance scoring**: Caps eligible balances by taking the minimum between a user's time-weighted balance and a predefined maximum

### 4️⃣ Customization Options

Merkl supports a wide range of [customization options](https://docs.merkl.xyz/merkl-mechanisms/customization-options) to further personalize campaigns beyond core settings. These include filters to dynamically restrict or boost the eligible users for a campaign based on whether they hold a token.

## 🔄 Feature Compatibility

Not all campaign types are compatible with all distribution types. Similarly, some customization options may only work with specific campaign or distribution types.

To check the status of Merkl’s features, the compatibility between campaign types, distribution or scoring types, or to view the list of supported chains and tokens, simply visit [**Merkl Studio**](https://studio.merkl.xyz/) and simulate the creation of a campaign.

Some [customization options](https://docs.merkl.xyz/merkl-mechanisms/customization-options), [campaign types](https://docs.merkl.xyz/merkl-mechanisms/campaign-types), [distribution types](https://docs.merkl.xyz/merkl-mechanisms/distributions) and [scoring types](https://docs.merkl.xyz/merkl-mechanisms/scoring) are not configurable directly via Merkl Studio. In such cases, we recommend reaching out to us — we can either configure your campaign for you or provide dedicated API endpoints to help you set it up.

Merkl’s capabilities continuously expand, adding support for new campaign types, distribution methods, and customization options. If you need a custom incentivization model, contact us by opening a [BD ticket on Discord](https://discord.gg/jnYfrGxDbe) or sending a message on Telegram.

# Reward Forwarding

Merkl Engine intelligently enables users to **earn rewards even when they don't hold the incentivized asset directly in their wallet**. For example, in a campaign rewarding token holders, many users may have their tokens staked in a contract. If Merkl has integrated that staking contract, those users will still receive rewards based on their staked amount.

This process of allocating rewards to the end users rather than to the contract itself is called forwarding, and the smart contract holding incentivized asset on behalf of users is referred to in this case as a **a Merkl forwarder**.

## How It Works

In most [campaign types](https://github.com/AngleProtocol/merkl-docs/blob/main/mechanisms/hooks/mechanisms/README.md), Merkl automatically detects and applies any integrated forwarding logic.

<figure><img src="https://3295124503-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2FKRQdTiHhBGLRKeCCOqdc%2Fuploads%2Fgit-blob-03a29ca2d4811ee2224171f0774b90f5288f5b9b%2FForwarderScan.png?alt=media" alt=""><figcaption><p>The forwarder scan in Merkl Studio</p></figcaption></figure>

{% hint style="info" %}
Check whether a contract is recognized as a forwarder by Merkl at <https://forwarders.merkl.xyz/>
{% endhint %}

**Key characteristics:**

* **Complexity Varies**: While some simple forwarders, such as staking contracts for ERC20 tokens, have straightforward integration and forwarding processes, others are equivalent to a complex protocol integration and involve forwarding rewards across multiple stakeholders on several smart contract layers.
* **Auto-Detection**: Once a forwarder is integrated, it's automatically applied—no manual configuration is needed. The Merkl frontend includes a scan tool to check if an address matches any known forwarder patterns.
* **Protocol Fidelity**: Merkl mirrors the logic of each protocol. For example, if a protocol charges a fee on accrued rewards, Merkl will automatically account for and replicate that fee in the reward forwarding process.

<figure><img src="https://3295124503-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2FKRQdTiHhBGLRKeCCOqdc%2Fuploads%2Fgit-blob-acc8614b8ed0d7d909e275388ccbd954354a2db1%2FDocs-merkl-forwarders.png?alt=media" alt=""><figcaption></figcaption></figure>

## Examples

**Example 1: Staked Token Rewards**:

* A campaign incentivizes USDA holders.
* Users who staked USDA and received stUSD would normally be ineligible for rewards because they don't hold USDA directly in their wallet.
* As forwarding is automatically enabled, Merkl recognizes stUSD holders as indirect USDA holders and distributes rewards accordingly.

**Example 2: Morpho Rewards**:

* A campaign targets USDA holders.
* USDA is used across multiple Morpho markets, either as collateral or loan token.
* Merkl detects the Morpho singleton as a forwarder and automatically distributes the USDA rewards among relevant stakeholders across all markets, proportionate to their contribution to the singleton's USDA holdings.

**Example 3: Pendle Rewards**:

* A campaign rewards sUSDe holders.
* Merkl rewards sUSDe "spot" holders and forwards rewards only to eligible Pendle stakeholders (YT and LP token holders), excluding PT holders, while applying the standard Pendle treasury fee.

## Linked Opportunities

When Merkl detects an address as a forwarder, it automatically creates a **subcampaign** that can be tracked within the associated opportunity. This creates what we call on the app **linked opportunities**—DeFi opportunities connected by liquidity flows, where one opportunity (the child) deposits funds into another (the parent) and receives rewards from it.

For example, a vault managing user funds (child opportunity) may deposit liquidity into a Uniswap pool (parent opportunity) that has active Merkl campaigns. When Merkl detects the vault as a forwarder, it creates a subcampaign visible on the vault's opportunity page. Thanks to this reward forwarding system, participants in the child opportunity—who are indirectly participating in the parent opportunity—receive rewards from the parent campaigns, in addition to any direct Merkl rewards from campaigns on the child opportunity itself.

Linked opportunities are displayed in the Merkl App to help users understand the full reward potential of their deposits and make informed decisions about where to allocate liquidity.

{% hint style="info" %}
**Important note about subcampaign/linked opportunity visibility:** Subcampaigns are only created after the Merkl engine has completed its first computation cycle on the main campaign. This means that immediately after a campaign is created, the API cannot report any subcampaigns or linked opportunities, even though the engine will process them once the first compute runs. For example, if you renew a campaign, the new campaign will initially show no APR on linked opportunities right after it starts, even though the previous campaign displayed this information. The linked opportunities and their APRs will appear once the first engine computation completes.
{% endhint %}

### Example of linked opportunities

In the below example, the main campaign incentivizes USDC supply across all whitelisted Morpho markets on Ethereum.

<figure><img src="https://3295124503-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2FKRQdTiHhBGLRKeCCOqdc%2Fuploads%2Fgit-blob-369ba7b7d61733a6ffe85972428f7a247bc826f8%2Flinked-opportunities-example.png?alt=media" alt=""><figcaption></figcaption></figure>

Each child campaign has its own APR, determined by the vault’s allocation to the whitelisted Morpho markets. You can click on any child opportunity to view the APR associated with this child campaign, and check whether other campaigns are stacked on top of this opportunity. In the screenshot below, you can see the Prime Vault opportunity page, which displays the APR from the USDC Morpho campaign.

<figure><img src="https://3295124503-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2FKRQdTiHhBGLRKeCCOqdc%2Fuploads%2Fgit-blob-4cc10877ca4f2051d8e77a5e0c359e10017b2efc%2Fchild-campaign-example.png?alt=media" alt=""><figcaption></figcaption></figure>

A child campaign can also have its own children. In this example, because the Prime Vault sources a part of its liquidity from Karpatkey Morpho Vault V2, the Merkl Engine creates an additional child campaign for that flow.

## Enabling Forwarding

Forwarding is enabled by default for most campaign types on Merkl. If your campaign requires integration with a new forwarder or support for a different protocol, please contact our team.

To ensure efficient distribution, Merkl enforces a minimum distribution threshold for each reward token. Campaigns can only be created if the token amount meets or exceeds this threshold. The same rule applies to forwarding: rewards are only forwarded if they pass the threshold.

Forwarding will not be enabled for an address if the total rewards over a given period fall below the minimum per-hour threshold for that token. For example, if an ERC20 vault receives just $0.01 of rewards in a day and the token's threshold is $0.10 per hour, those rewards will not be forwarded. Instead, they'll remain accrued at the vault address.

## Address Remapping

If you're earning rewards through a smart contract that cannot claim them (e.g., non-upgradeable contracts on Uniswap V4 that cannot call `toggleOperator` or transfer), you may need **address remapping** to redirect your rewards to a claimable address.

**Remapping vs. Forwarding:**

While both mechanisms redirect rewards, they serve different purposes:

* **Forwarding** (described above) automatically distributes rewards to users who hold the incentivized asset indirectly through integrated protocols (e.g., staking contracts or LP tokens). Forwarding works at the protocol level and is integrated directly into Merkl's reward distribution logic.
* **Address remapping** is a manual configuration set up by the Merkl team that redirects rewards from one specific address (your contract) to another specific address (your claimable wallet). It's used when a contract receives rewards but cannot claim them, and you want those rewards sent to a claimable address for the entire campaign duration.

**How to request address remapping:**

Contact the Merkl team via [Discord](https://discord.com/channels/1209830388726243369/1210212731047776357) with:

* Your campaign ID(s)
* The source address (contract) from which rewards should be redirected
* The destination address (claimable wallet) where rewards should be sent

Address remapping is particularly useful for long-running campaigns where you expect regular reward accumulation on addresses that cannot claim. Rather than performing frequent manual reallocations, remapping provides a seamless, automated solution.

# Customization Options

Merkl enables campaign creators to **customize their incentive programs with optional features** — from participant eligibility to reward boosts and referral mechanisms — providing greater flexibility beyond standard campaign settings.

The full list of customization options is available in [Merkl Studio](https://studio.merkl.xyz/create-campaign/erc20) under the **Personalize** step when creating a campaign (you can simulate the campaign creation to explore all options — no need to actually launch a campaign).

<figure><img src="https://3295124503-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2FKRQdTiHhBGLRKeCCOqdc%2Fuploads%2Fgit-blob-7969b373f063363754d1154f37f26b2c73629457%2FGroup%2015.png?alt=media" alt=""><figcaption></figcaption></figure>

Below are some of the most common customization options.

## Access Control

### 🚫 OFAC Compliance

Merkl allows you to **blacklist addresses flagged by the U.S. Office of Foreign Assets Control (OFAC)** to ensure compliance with U.S. economic and trade sanctions imposed on certain countries, organizations, and individuals.

To do this, you need to provide the address of the contract that contains the registry of OFAC-flagged addresses, such as the [Chainalysis Sanctions Oracle on Ethereum](https://etherscan.io/address/0x40c57923924b5c5c5455c48d93317139addac8fb).

### 🌍 Worldchain ID

Merkl enables filtering of campaign participants using Worldchain’s identity system, [World ID](https://docs.world.org/world-id), ensuring that only real, unique humans — not bots — are rewarded.

## Eligibility

### 🔒 Token Holding

Merkl allows campaign creators to add a customization option that **limits reward eligibility to users holding a minimum amount of a specific token in their wallet over a given period** — and this token can differ from the reward token.

Example of a campaign requiring 500 stUSD held for 30 days to earn rewards:

* A user holding 600 stUSD for 44 days is eligible
* A user holding 400 stUSD for 60 days is not eligible

Addresses that fail to meet either the token amount or duration threshold are excluded from the campaign’s rewards, ensuring incentives are focused on long-term participants rather than short-term holders.

### 📸 Token Snapshot

You can also customize campaigns to reward only users who hold a minimum amount of a specific token in their wallet at a given snapshot, instead of over a duration.

## Boosts

### 🚀 Boost

Merkl allows campaign creators to boost rewards for users holding a specific token or NFT.

* Similar to Curve’s vote-escrowed boost formula but with more flexibility.
* No 2.5x limit – You can customize boost multipliers as needed.

#### Boost Formula Computation:

$$
B = b \times \frac{R \times v}{V \times r} + 1
$$

Where:

* **B**: Boost multiplier
* **b**: Custom boost factor chosen by campaign creator
* **R**: Total rewards per epoch
* **r**: User’s reward per epoch
* **V**: Total supply of the boost token/NFT
* **v**: User’s holdings of the boost token/NFT

{% hint style="warning" %}
**Example:** To achieve a boost of 10%, you'll need to indicate a Boost Multiplicator of 1.1
{% endhint %}

#### NFT-Based Boosting

Boosts can be based on NFT holdings. Contact us for help setting up NFT-based reward boosts.

### 📡 API Boost

Merkl provides several methods to manage dynamic boosting through an API. Here are the different methods available:

#### Multiply

Multiply the current amount by the provided input:

$$
\text{amount} = \text{amount} \times \text{boost}
$$

#### Multiply with Offset

Apply a 1 + boost computation:

$$
\text{amount} = \text{amount} \times (1 + \text{boost})
$$

#### Add

Add the boost number to the current amount:

$$
\text{amount} = \text{amount} + \text{boost}
$$

#### Replace

Replace the current amount with the boosted amount:

$$
\text{amount} = \text{boost}
$$

#### Implementation

The customization option has the following parameters:

* **url**: The endpoint to which the API call will be made.
* **boostingFunction**: The function used to calculate the boost. Options include `REPLACE`, `ADD`, `MULTIPLY`, and `MULTIPLY_WITH_OFFSET`.
* **sendScores**: A boolean indicating whether to send ongoing reward computation scores along with the addresses.
* **defaultBoost**: The default boost value to use if no specific boost is provided. Options include `ZERO_ADDRESS` and `ERROR`.
  * `ZERO_ADDRESS`: If an eligible address as per our reward computation is not within your API response, we will use the `ZERO_ADDRESS` boost as a default boost value for this address. Since we always exclude the zero address in our computation, it is indeed safe to use the null address as a default. If you use this option, you must therefore absolutely input a boost value for the zero address.
  * `ERROR`: If there is an address eligible to rewards that we do not find in your API response, the campaign will not proceed.

Depending on whether `sendScores` is true or false, we will POST the following body along with the API call:

`sendScores=True`

```jsx
let body: { address: string, score: string }[]
```

`sendScores=False`

```jsx
let body: { addresses: string[] }
```

{% hint style="info" %}
The Merkl Engine will first make a POST request with all the addresses rewarded by the campaign. If this request fails, it will split the payload in 2 and make 2 requests with 50% of the addresses in each request. The script will continue dividing the payload size by 2 until your API responds correctly. This being said, we recommend supporting at least payloads of 250 addresses to prevent our engine from making too many requests to your backend.
{% endhint %}

We will expect the following response:

```jsx
const data : {
  address!: string;
  boost!: string; // should be a bigint in BASE 9 (e.g. 1 = "1000000000")
}[]
```

**Ensure your endpoint is non-null and in the correct format; otherwise reward computation will fail. Also, ensure there are no duplicate addresses: if duplicates exist, only the first boost value will be used!**

{% hint style="warning" %}
**Important:** **The boost values are to be given in base 9**. This means that for a boost of 1, the value given for boost should be: "1000000000".
{% endhint %}

#### Dynamic whitelist (example)

If you expect to **add whitelisted addresses over time**, use this method so you don't have to cancel and recreate the campaign with the updated list of whitelisted addresses. To achieve this:

* Use the `MULTIPLY` boosting function
* Set a boost of "1000000000" (i.e 1× in base 9) for whitelisted addresses, "0" for the `ZERO_ADDRESS` (that way all non-whitelisted addresses do not receive rewards)

```json
[
  {
    "address": "0x1234567890abcdef1234567890abcdef12345678",
    "boost": "1000000000"
  },
  {
    "address": "0xabcdef1234567890abcdef1234567890abcdef1234",
    "boost": "1000000000"
  },
  {
    "address": "0x0000000000000000000000000000000000000000",
    "boost": "0"
  }
]
```

{% hint style="info" %}
This is our current recommended method for private LP deals: whitelisted addresses aren’t shown in the Merkl app, and users only see “API boost” in the campaign rules. More features for such campaigns will be rolled out in the future!
{% endhint %}

#### Dynamic blacklist (example)

Similarly, **you can keep a mutable blacklist without having to cancel and then recreate a campaign**. To achieve this:

* Use the `MULTIPLY` boosting function
* Set a boost of "0" for blacklisted addresses, and "1000000000" (i.e 1× in base 9) for the `ZERO_ADDRESS` (that way all non-blacklisted addresses receive rewards)

```json
[
  {
    "address": "0x1234567890abcdef1234567890abcdef12345678",
    "boost": "0"
  },
  {
    "address": "0xabcdef1234567890abcdef1234567890abcdef1234",
    "boost": "0"
  },
  {
    "address": "0x0000000000000000000000000000000000000000",
    "boost": "1000000000"
  }
]
```

## Advanced Logic

### 🌉 Jumper Bridge

Merkl has partnered with Jumper to enable campaign creators to reward users who bridged liquidity from another chain before participating in a campaign.

This feature ensures that only cross-chain liquidity is incentivized, not movements within the same chain.

### 🤝 Referral Program

The onchain Referral Program allows campaign creators to reward users who refer others to a campaign they’ve participated in, as well as the invitees themselves. This helps acquire new users and accelerate liquidity participation.

The feature offers a wide range of customization options — from user rewards to whitelist gates — all secured by blockchain technology.

#### Key Features:

* **Create Unlimited Referral Programs**: Launch as many referral programs as you want.
* **Referral Code Generation**: Users can generate unique referral codes, share them with their friends, and earn rewards.
* **Whitelabel Integration**: Easily integrate the program into your front-end with whitelabel options (Contact Merkl for details).
* **Cross-Protocol Support**: Referral programs are compatible across the entire Merkl ecosystem, allowing creators to incentivize on any protocol/behaviour integrated with Merkl.
* **Customizable Rewards**: Tailor rewards to users, referrers, invited users, or even non-participating users.
* **Conditions to participate**: Add a whitelist restriction to the program, and optionally charge a fee to create a referral code. Or let anyone participate.
* **Blockchain Security**: Users need to sign a transaction to confirm their referral action, ensuring secure and verified participation.

{% hint style="info" %}
**Contact Merkl for details on how to implement referral programs**
{% endhint %}

### 🎟️ Raffle

Merkl allows you to set up raffles that randomly select lucky winners for your campaigns. You can customize these raffles in various ways to match your campaign needs.

#### Customization

Merkl provides several options for you to tailor your raffles:

* **Multiple raffles**: Choose how often you want raffles to run. Every day? Every week? The choice is yours
* **Number of winners**: Decide how many winners you want to select in each raffle. You can have one grand prize winner, or ay number of lucky winners, depending on your preference.
* **Selection method**: You can choose how winners are selected.
  * **Everyone is equal**: All participants have an equal chance of winning.
  * **Whales first**: Users with higher campaign scores have a better chance of winning (this can help reward top participants).
* **Multiple selection**: You can set up multiple raffles that run at the same time, each with its own rules on how rewards are distributed.

#### Reproducibility

**Seed**\
The seed is the **block hash** of the very next block after the timestamp set by the campaign creator.

**Generating Winning Numbers**\
The random number generation is done using the **XORShift128Plus** pseudo-random number generator (PRNG). This PRNG is seeded with the previously generated seed.\
The **step** is the minimum score of a user divided by 1000. (In the case where the score is not used to pick users, the step is, of course, 1.)

```typescript
function getSelectedNumbers(
  seed: number,
  numberOfWinners: number,
  totalScore: number,
  step: number,
): number[] {
  const random = new XORShift128Plus(seed)
  const selectedNumbers = []
  for (let i = 0; i < numberOfWinners; i++) {
    selectedNumbers.push(random.randBelow(totalScore / step) * step)
  }
  return selectedNumbers
}
```

**Winners**\
The selected numbers correspond to different **"ranges"** in the list of participants (based on their amount or score). The system picks winners by matching these random numbers with ranges of amounts that participants have.\
Winners are picked by sorting all the participants by their address.

In the case of a **snapshot** or **airdrop**, obtaining the list of addresses is straightforward because the participants are known ahead of time. This list typically includes all the addresses that are eligible for the snapshot or airdrop, and they are usually collected from a specific event or condition.

For **more complex campaigns**, where winners are determined through specific criteria or weighted selections, one approach is to run a campaign with the **exact same parameters** but **without the raffle option**.

The key idea is:

* You run this simplified version of the campaign **with a smaller amount** (i.e., a lower prize or allocation) and **without the raffle option**.
* This will **generate the list of users** that Merkl identifies as eligible or **potential winners**.
* You can use this list to generate the list of users.

In essence, the result of this campaign (without the raffle option) gives you the list of users who met the conditions set by the campaign. These users are then the ones Merkl considers for potential winning when the raffle customization option is applied.

#### Example with 5 users, 5 weights, and one number picked

We have **5 users** in the raffle, each with an associated **weight** (which represents their chance of winning). The system will pick **one winner** based on a randomly selected number.

Let’s assume the users and their weights are as follows:

| User | Weight (Amount) |
| ---- | --------------- |
| A    | 10              |
| B    | 20              |
| C    | 30              |
| D    | 40              |
| E    | 50              |

We want to pick one winner randomly based on these weights.

**1/ Total Weight Calculation:**

First, we calculate the **total weight** by summing the weights of all the users. This total represents the "pool" from which the random number will be drawn.

Total Weight = 10 + 20 + 30 + 40 + 50 = 150

**2/ Random Number Generation**:

The system will then pick a random number. This random number will be between **0 and the total weight (150)**.

Let’s assume the randomly picked number is **107**.

**3/ Weighted Selection**

The system uses this random number to determine which user will win. To do this, it calculates cumulative weights, which define "ranges" for each user.

* For **User A**: The range is from **0 to 10** (because A has a weight of 10).
* For **User B**: The range is from **10 to 30** (since A’s range is 0–10, and B has a weight of 20).
* For **User C**: The range is from **30 to 60**.
* For **User D**: The range is from **60 to 100**.
* For **User E**: The range is from **100 to 150**.

So, the cumulative ranges are as follows:

| User | Cumulative Weight Range | Range |
| ---- | ----------------------- | ----- |
| A    | 0–10                    | 10    |
| B    | 10–30                   | 20    |
| C    | 30–60                   | 30    |
| D    | 60–100                  | 40    |
| E    | 100–150                 | 50    |

**4/ Determine the Winner:**

Now, the system checks where the random number falls in the cumulative weight ranges:

* **Random number**: 107\
  The number **107** falls in **User E's** range (100–150), so **User E** is the winner.

# Additional Features

Beyond campaign types, distribution methods, and customization options, Merkl offers several advanced features to enhance campaign flexibility and reward management.

## ❌ Blacklisting

Blacklisting excludes specific addresses from receiving rewards.

<figure><img src="https://3295124503-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2FKRQdTiHhBGLRKeCCOqdc%2Fuploads%2Fgit-blob-d25f78bac99df7e4a235a432c6f040d3943df144%2FGroup%2027.png?alt=media" alt=""><figcaption></figcaption></figure>

{% hint style="danger" %}
If a [forwarder](https://docs.merkl.xyz/merkl-mechanisms/reward-forwarding) is blacklisted, all associated users are also ineligible.

Example of a staking contract blacklist:

* A user holds 10 USDA but has staked 6 USDA in a blacklisted staking contract (forwarder).
* Only the 4 USDA in the user’s wallet qualifies for rewards.
  {% endhint %}

## ✅ Whitelisting

Whitelisting restricts rewards to a specific address or set of addresses (e.g., forwarders, or individual users).

Example of Uniswap V3 whitelisting:

* A campaign incentivizes LPs in a Uniswap V3 pool with three forwarders — here Automated Liquidity Managers.
* The campaign creator whitelists only two forwarders and one user address.
* Result:
  * Only liquidity providers using the two approved forwarders or the whitelisted user will receive rewards
  * Rewards are distributed normally among whitelisted addresses based on liquidity share.

{% hint style="danger" %}
**Whitelisting overrides blacklisting. If an address is whitelisted, all other addresses are automatically blacklisted.**
{% endhint %}

{% hint style="warning" %}
If multiple campaigns run on the opportunity (e.g. Uniswap v4 ETH-USDC), some may have whitelists while others do not. This means that, for the same opportunity, the users receiving rewards can differ.

Users should check the campaign details on the opportunity page to confirm eligibility requirements.
{% endhint %}

## 🔥 Run Multiple Campaigns on the Same Opportunity

Merkl allows multiple campaigns to be created simultaneously for the same pool or set of actions.

**Why is this useful?**

* Co-incentives: Different parties can independently add incentives to the same opportunity.
* Flexible Stacking: Multiple campaigns can run alongside each other on the same pool or token, supporting different incentive structures.

## ⏳ Campaigns of Any Duration

Merkl campaigns can run for any length of time, from as short as one hour to as long as six months (or more).

This flexibility allows for:

* Short-term boosts (e.g., flash incentives for new launches).
* Long-term, sustained incentive programs for ongoing ecosystem growth.

## 🎭 Infinite Customizability with Token Wrappers

Merkl campaigns can distribute tokens with custom properties, known as token wrappers, to introduce advanced incentive mechanisms.

**Examples of Token Wrapper Use Cases:**

* Vesting & Slashing Conditions: Add vesting schedules or penalties for early withdrawals.
* Non-Prefunded Campaigns: Instead of preloading tokens, rewards are pulled from a multisig when users claim.
* Time-Locked Transfers: Issue non-transferable tokens that unlock after a set period.
* Redeemable Tokens: Distribute placeholder tokens that can be redeemed later.

Merkl provides a suite of template contracts for token wrappers in the Merkl GitHub repository so anyone can build [its own token wrapper](https://github.com/AngleProtocol/merkl-contracts/tree/main/contracts/partners/tokenWrappers). Some templates have already been audited by Merkl partners!

{% hint style="info" %}
Got a custom use case? Let us know—we’re happy to collaborate and help build your solution.
{% endhint %}

## 🌍 Cross-Chain Campaigns

Merkl allows you to incentivize activity on one chain while distributing rewards on another.

**How It Works:**

* Activity is tracked on Chain A (e.g., a protocol running on Arbitrum).
* Rewards remain claimable on Chain B (e.g., distributed token stays on Ethereum): the chain where the token is claimable is the chain where the campaign was created

**Why is this useful?**

* Efficient token management: Keeps governance tokens on a single chain, reducing the need for bridging.
* Cross-chain flexibility: Supports protocols that operate on multiple chains without fragmenting incentives.

<figure><img src="https://3295124503-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2FKRQdTiHhBGLRKeCCOqdc%2Fuploads%2Fgit-blob-9272a2c28b7eda47637a3b922ee53364c81f3259%2FGroup%2011.png?alt=media" alt=""><figcaption><p>Rewards sent on a different chain than the one where users perform the action to be eligible</p></figcaption></figure>

**Important considerations**:

Some smart contracts on the chain you are incentivizing activity may not exist on the chain where users can claim their reward (or may exist at a different address):

* Affected addresses will be unable to claim rewards in this case
* Solution: As a campaign manager, you should blacklist any ineligible addresses to prevent reward loss. Or you can reallocate rewards (more below) of addresses that cannot claim to an address controlled by the same provider that can claim its rewards

## ◀️ Retroactive Campaigns

You can create campaigns in the past to reward OG users. It can start and end in the past or it can end in the future.

## Campaign Management

Campaign managers can perform several actions on their live and past campaigns, including:

* [Overriding live campaigns](https://docs.merkl.xyz/distribute-with-merkl/campaign-management#️-campaign-overrides)
* [Cancelling existing campaigns](https://docs.merkl.xyz/distribute-with-merkl/campaign-management#-campaign-cancellation)
* [Reallocating unclaimed rewards from past campaigns](https://docs.merkl.xyz/distribute-with-merkl/campaign-management#-campaign-reallocation)

# Campaign Configuration

A **campaign configuration** is the collection of all parameters—such as [campaign type](https://docs.merkl.xyz/merkl-mechanisms/campaign-types), [distribution method](https://docs.merkl.xyz/merkl-mechanisms/distributions), and [customization options](https://docs.merkl.xyz/merkl-mechanisms/customization-options)—that define how a campaign operates. It serves as the source of truth for the Merkl engine.

When a campaign is launched, an encoded version of this configuration is stored onchain in the Merkl Distribution Creator contract. The Merkl engine continuously monitors this contract to detect new campaigns, then parses each configuration to understand among others:

* What type of user activity to track (e.g., providing liquidity, holding tokens, lending assets)
* How to calculate user scores based on their participation
* How rewards should be distributed over time
* Which addresses are eligible or excluded from rewards

This configuration-driven approach allows Merkl to support diverse incentive mechanisms while maintaining a unified reward computation and distribution infrastructure.

## Creating and Retrieving Configurations

You can generate or retrieve campaign configurations in two main ways:

* **Via Merkl Studio**: When you create a campaign using [Merkl Studio](https://docs.merkl.xyz/distribute-with-merkl/create-a-campaign), the configuration is generated automatically.
* **Via Merkl API**: You can programmatically generate or retrieve configurations using the [Merkl API](https://docs.merkl.xyz/distribute-with-merkl/create-multiple-campaigns).

You can also retrieve the configuration of any existing campaign using its database ID via this [Merkl API endpoint](https://api.merkl.xyz/docs#tag/config/get/v4/config/{id}).

## Configuration Structure

A campaign configuration is typically represented as a JSON object containing various parameters.

### Common Parameters

These parameters are standard across most campaigns:

* `creator`: The address managing the campaign
* `rewardToken`: The address of the token distributed as rewards
* `distributionChainId`: The chain ID where rewards are distributed
* `computeChainId`: The chain ID where user activity is tracked (can differ from `distributionChainId` for cross-chain campaigns)
* `startTimestamp`: The start date of the campaign (Unix timestamp)
* `endTimestamp`: The end date of the campaign (Unix timestamp)
* `amount`: The total amount of rewards to be distributed
* `blacklist`: A list of addresses excluded from receiving rewards
* `whitelist`: A list of addresses allowed to receive rewards (if set, all others are excluded)
* `campaignType`: The [campaign type](https://docs.merkl.xyz/merkl-mechanisms/campaign-types)
* `computeScoreParameters`: The [scoring method](https://docs.merkl.xyz/merkl-mechanisms/scoring) used by the campaign
* `distributionMethodParameters`: The [distribution method](https://docs.merkl.xyz/merkl-mechanisms/distributions) for reward distribution

{% hint style="info" %}
You can use [this converter](https://www.unixtimestamp.com/) to convert dates to Unix timestamps.
{% endhint %}

### Distribution Methods

The `distributionMethodParameters` field defines how rewards are distributed over time.

**Variable Reward Rate (Dutch Auction):**

```json
{
  "distributionMethod": "DUTCH_AUCTION"
}
```

**Fixed APR:**

```json
{
  "distributionMethod": "FIX_APR",
  "distributionSettings": {
      "apr": "0.08",
      "targetToken": "0x...",
      "rewardTokenPricing": true,
      "targetTokenPricing": true
  }
}
```

**Capped APR:**

```json
{
  "distributionMethod": "MAX_APR",
  "distributionSettings": {
      "apr": "1",
      "targetToken": "0x...",
      "rewardTokenPricing": true,
      "targetTokenPricing": true
  }
}
```

### Campaign-Specific Parameters

Each campaign type has its own set of specific parameters in addition to the common parameters listed above.

**Examples by Campaign Type:**

* **ERC20 / Token Holding (Type 18)** - [Example config](https://api.merkl.xyz/v4/config/8270489034958466914)
  * `targetToken`: Address of the token to incentivize (or LP token for V2 pools)
* **Uniswap V3 (Type 2)** - [Example config](https://api.merkl.xyz/v4/config/9427880006586247706)
  * `poolAddress`: The pool address
  * `weightToken0`, `weightToken1`, `weightFees`: Weights for scoring liquidity
* **Uniswap V4 (Type 13)** - [Example config](https://api.merkl.xyz/v4/config/14050222419773482936)
  * Specific hook and pool parameters
* **Morpho (Type 57)** - [Example config](https://api.merkl.xyz/v4/config/3011317640800818752)
  * `targetToken`: Address of the supplied token on a Morpho Market
* **Euler Supply (Type 12)** - [Example config](https://api.merkl.xyz/v4/config/16912425279432080078)
  * `evkAddress`: Address of the incentivized vault
* **Euler Borrow (Type 12)** - [Example config](https://api.merkl.xyz/v4/config/17331543524323336682)
  * `evkAddress`: Address of the incentivized vault

## Encoding and Decoding Configurations

The Merkl API provides endpoints to convert between campaign configurations and the encoded format used onchain.

**Encoding configurations for onchain deployment:**

* [Encode single campaign from config](https://api.merkl.xyz/docs#tag/config/post/v4configencode)
* [Encode multiple campaigns from their respective configs](https://api.merkl.xyz/docs#tag/config/post/v4configencodebatch)

**Decoding onchain campaign data into configurations:**

* [Decode from onchain campaign ID](https://api.merkl.xyz/docs#tag/config/get/v4configdecodeonchaindistributionchainidcampaignid)
* [Decode from onchain campaign data](https://api.merkl.xyz/docs#tag/config/post/v4configdecodedistributionchainid)
