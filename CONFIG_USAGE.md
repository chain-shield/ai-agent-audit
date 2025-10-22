# YAML Configuration File Usage

AI Agent Audit supports loading configuration from YAML files, making it easier to manage complex audit configurations and share them across teams.

## Quick Start

Instead of passing all arguments on the command line:

```bash
cargo run https://github.com/sherlock-audit/2025-10-index-fun-order-book-contest-chainshieldai.git \
  --custom-doc indexfun-docs.md \
  --scoped-files indexfun-scope.txt \
  --audit-scope indexfun-scope.md \
  --subfolder orderbook-solidity \
  --builder custom \
  --build-cmd "make install && make build"
```

You can create a YAML config file and use it:

```bash
cargo run -- --config indexfun.yaml
```

## YAML Configuration Format

### Example: IndexFun Order Book Contest

See `indexfun.yaml` for a complete example:

```yaml
# Git repository URL (required)
repo: "https://github.com/sherlock-audit/2025-10-index-fun-order-book-contest-chainshieldai.git"

# Project subfolder (optional)
subfolder: "orderbook-solidity"

# Custom documentation file (optional)
custom_doc: "indexfun-docs.md"

# Scoped files list (optional)
scoped_files: "indexfun-scope.txt"

# Audit scope documentation (optional)
audit_scope: "indexfun-scope.md"

# Source code folders (required, defaults to ["src"])
code_folders:
  - "src"

# Builder configuration (required, use PascalCase)
builder: "Custom"
build_cmd: "make install && make build"

# Boolean flags (required, defaults to false)
via_ir: false
force_rebuild: false
```

## Field Reference

### Required Fields

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `repo` | String | Git repository URL (HTTPS only) | `"https://github.com/user/repo.git"` |
| `code_folders` | Array | Source code folders | `["src", "contracts"]` |
| `builder` | Enum | Build system type | `"Foundry"`, `"Hardhat"`, `"Custom"`, etc. |
| `via_ir` | Boolean | Use Foundry --via-ir flag | `true` or `false` |
| `force_rebuild` | Boolean | Force rebuild even if cached | `true` or `false` |

### Optional Fields

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `subfolder` | String | Project root subfolder | `"orderbook-solidity"` |
| `audit_scope` | String | Audit scope markdown file | `"scope.md"` |
| `doc_folder` | String | Documentation folder | `"docs"` |
| `monorepo_folders` | String | Monorepo packages list file | `"monorepo.txt"` |
| `custom_doc` | String | Custom documentation file | `"custom-docs.md"` |
| `exclude_folders` | Array | Folders to exclude | `["test", "script"]` |
| `scoped_files` | String | In-scope files list | `"scope.txt"` |
| `poc_instructions` | String | PoC instructions file | `"poc-instructions.md"` |
| `poc_template` | String | PoC template file | `"poc-template.sol"` |
| `test_folder` | String | Test folder path | `"test"` |
| `build_cmd` | String | Custom build command | `"make install && make build"` |

## Builder Types

Use **PascalCase** for builder enum values in YAML:

- `Foundry` - Foundry/Forge projects
- `Hardhat` - Hardhat projects (npm)
- `HardhatYarn` - Hardhat projects (yarn)
- `Custom` - Custom build command (requires `build_cmd`)
- `Auto` - Auto-detect build system

## Examples

### Foundry Project

```yaml
repo: "https://github.com/example/foundry-project.git"
code_folders:
  - "src"
builder: "Foundry"
via_ir: true
force_rebuild: false
```

### Hardhat Project with Yarn

```yaml
repo: "https://github.com/example/hardhat-project.git"
subfolder: "contracts"
code_folders:
  - "contracts"
builder: "HardhatYarn"
via_ir: false
force_rebuild: false
```

### Custom Build System

```yaml
repo: "https://github.com/example/custom-project.git"
subfolder: "orderbook-solidity"
code_folders:
  - "src"
builder: "Custom"
build_cmd: "make install && make build"
via_ir: false
force_rebuild: false
```

### With PoC Configuration

```yaml
repo: "https://github.com/example/project.git"
code_folders:
  - "src"
builder: "Foundry"
poc_instructions: "poc-instructions.md"
poc_template: "poc-template.sol"
test_folder: "test"
via_ir: false
force_rebuild: false
```

### With Scoped Files (Code4rena Style)

```yaml
repo: "https://github.com/code4rena/contest.git"
code_folders:
  - "src"
scoped_files: "scope.txt"
audit_scope: "README.md"
custom_doc: "docs.md"
builder: "Foundry"
via_ir: false
force_rebuild: false
```

### Monorepo Configuration

```yaml
repo: "https://github.com/example/monorepo.git"
code_folders:
  - "packages/core/src"
  - "packages/utils/src"
monorepo_folders: "monorepo-packages.txt"
exclude_folders:
  - "test"
  - "script"
  - "mock"
builder: "Foundry"
via_ir: false
force_rebuild: false
```

## Usage Tips

1. **File Paths**: All file paths in the YAML config (like `custom_doc`, `scoped_files`, etc.) should be relative to the **current directory** where you run the command.

2. **PascalCase Enums**: Always use PascalCase for enum values like `builder`:
   - ✅ `builder: "Custom"`
   - ❌ `builder: "custom"`

3. **Arrays**: Use YAML array syntax for multi-value fields:
   ```yaml
   code_folders:
     - "src"
     - "contracts"
   ```

4. **Boolean Values**: Use lowercase `true`/`false`:
   ```yaml
   via_ir: true
   force_rebuild: false
   ```

5. **Comments**: Use `#` for comments in YAML files

6. **Validation**: The tool will validate the repository URL and other fields when loading the config

## Testing Your Configuration

Run the integration tests to verify YAML parsing works correctly:

```bash
cargo test --test yaml_config_test
```

## Benefits of YAML Configuration

1. **Reusability**: Save and reuse configurations for different audits
2. **Version Control**: Commit config files to git for team collaboration
3. **Readability**: Easier to read and modify than long command lines
4. **Documentation**: Config files serve as documentation for audit setup
5. **Consistency**: Ensure consistent audit configurations across runs

## Troubleshooting

### "missing field" Error

Make sure all required fields are present:
- `repo`
- `code_folders`
- `builder`
- `via_ir`
- `force_rebuild`

### "unknown variant" Error

Check that enum values use PascalCase:
- Use `"Foundry"` not `"foundry"`
- Use `"Custom"` not `"custom"`

### File Not Found

Ensure file paths in the YAML are relative to the directory where you run the command, not relative to the YAML file itself.

