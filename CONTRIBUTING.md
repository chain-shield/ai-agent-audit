# Contributing

Thanks for helping improve AI Agent Audit.

## Before You Open A PR

- Open an issue first for large changes so we can align on scope.
- Keep pull requests focused. Smaller PRs are much easier to review.
- If your change affects prompts, analysis flow, or findings quality, include a short note on how you validated it.

## Local Setup

1. Install Rust, Git, Slither, and the local Solidity build tools you need (`forge`, Node.js/npm, Yarn, or pnpm).
2. Copy `.env.example` to `.env`.
3. Start Qdrant on `localhost:6334` if you are using vector search features.
4. Run `cargo check`.
5. Run `bash scripts/run_ci_tests.sh` for the hermetic CI suite.

## Development Expectations

- Format Rust code with `cargo fmt`.
- Run `cargo check` before opening a PR.
- Prefer tests that do not require live API keys.
- If a test requires external credentials or a private/local repo, gate it so CI can skip it cleanly.

## Test Tiers

- `bash scripts/run_ci_tests.sh` runs the public, hermetic CI suite.
- `cargo test -- --ignored` is reserved for manual diagnostics and live-provider integration checks.

## Pull Request Notes

- Describe the problem and the change.
- Call out any behavior changes, new env vars, or migrations.
- Include reproduction or verification steps for bug fixes.

## Security

Please do not file public issues for exploitable vulnerabilities. Use the process in `SECURITY.md`.
