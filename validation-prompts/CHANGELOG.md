# Validation Prompt Changelog

## v2

Derived from `v1` using the first scored Olas benchmark run.

Main changes:

- added an explicit evidence hierarchy so validators rely on actual Solidity code and benchmark docs rather than finding text alone;
- tightened the burden of proof around explicit support, unsupported token behavior, deployment / configuration issues, and trusted-component assumptions;
- strengthened invariant checking so claimed protocol properties must be evidenced, not assumed;
- narrowed parameter-omission reasoning so missing `deadline`, `minOut`, or freshness controls are not automatically treated as valid;
- clarified that missing context should usually lead to `Invalid`, with `Needs Review` reserved for genuinely unavailable code;
- emphasized approving only the narrowest proven root cause and supported severity.

Structural note:

- `v2` keeps a stable prompt layout and uses one `Core Validation Principles` section instead of adding a version-named principles block that would keep growing every iteration.
