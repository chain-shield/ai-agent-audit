# Code4rena Bounty Criteria

Source URLs:
- https://docs.code4rena.com/bounties
- https://docs.code4rena.com/bounties/bounty-criteria

Use this file as mandatory context for `validation_profile: code4rena-bounty`.

## Submission Bar

- Only Critical and High risk issues are acceptable unless the bounty README says otherwise.
- Each invalid or spam submission loses its 25 USDC deposit, so weak findings should be marked `Do Not Submit` or `Needs Human Review`.

## Threat Model

Keep only currently exploitable issues in in-scope code where the attacker is unprivileged.

Exclude issues requiring leaked keys, compromised credentials, privileged address access, malicious or mistaken trusted roles, basic economic/governance attacks, stablecoin depegs not directly caused by in-scope code, Sybil attacks, lack of liquidity, centralization risks, test/config files unless explicitly included, feature requests, best practices, and automated tool output without a proven exploit.

## Critical

Critical requires high impact with high likelihood and one of:
- governance vote result manipulation that changes the intended voted effect
- direct theft of user funds, except unclaimed yield
- direct theft of user NFTs, except unclaimed royalties
- permanent freezing of funds or NFTs
- unauthorized minting of NFTs
- manipulable RNG that abuses principal or NFTs
- unintended alteration of NFT meaning, such as token URI, payload, or art
- protocol insolvency

## High

High requires high impact with any likelihood and one of:
- theft of unclaimed yield or royalties
- permanent freezing of unclaimed yield or royalties
- temporary freezing of funds or NFTs

