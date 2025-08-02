pub const CODE4RENA_SEVERITY_RUBRIC: &str = r#"

| Severity  | Typical impact                                                                                          |
|-----------|----------------------------------------------------------------------------------------------------------|
| High      | Direct theft of user or protocol funds, permanent total loss/control, arbitrary code execution, permanent freezing or bricking of funds, governance takeover, significant unauthorized mint/burn/drain
| Medium    | Temporary loss until admin action, convincing DoS, reward/fee distortion, oracle/math skew with $ impact |
| Low       | Minor economic grief, rare‑case DoS, best‑practice deviations                                             |
| Gas/Info  | Gas optimizations, style, comments, docs                                                                  |

"#;

pub const SHERLOCK_SEVERITY_RUBRIC: &str = r#"

| Severity  | Typical impact                                                                                          |
|-----------|----------------------------------------------------------------------------------------------------------|
| High      | Direct loss of funds exceeding 1% and $10 of principal, yield, or protocol fees; permanent governance takeover; straightforward exploitable attack path |
| Medium    | Loss of funds exceeding 0.01% and $10 under specific conditions or constraints (e.g., requiring particular market conditions, user interactions, or protocol states; limited by time windows, admin actions, or partial exploitability); DoS locking funds over a week or disrupting time-sensitive functions; replayable attacks with escalating impact |
| Low       | Negligible impact on funds or functionality; gas optimizations, style issues, documentation gaps, or invalid categories (e.g., zero-address checks, front-running without irreversible damage) |

"#;

pub const DEFAULT_SEVERITY_RUBRIC: &str = r#"

| Severity     | Typical impact examples                                                                                                                                                                                                            | What it signals to the team               |
| ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------- |
| **Critical** | - Direct theft of any funds - Permanent, **total** loss or control of all user or protocol funds - Arbitrary code execution                                                                                                  | “Drop everything—patch immediately.”      |
| **High**     | - **Permanent freezing** or bricking of user or protocol funds (can’t be reversed without privileged migration) - Loss of governance control - Logic that lets an attacker mint/ burn / drain but under specific constraints | “Must fix before next release / upgrade.” |
| **Medium**   | - Temporary loss (funds stuck until admin action) - Convincing grief / DoS that makes the protocol unusable - Oracle or math bugs that skew accounting but don’t directly drain value                                        | “Important, schedule a patch.”            |
| **Low**      | - Minor economic grief (extra gas, incorrect event data) - Edge-case DoS that requires unusual conditions - Best-practice deviations with limited real-world impact                                                          | “Fix in regular development cycle.”       |
| **Insight**  | Code cleanliness, documentation issues, minor style or test suggestions                                                                                                                                                            | “Nice-to-have, no security impact.”       |


"#;
