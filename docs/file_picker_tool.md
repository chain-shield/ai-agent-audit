# File Picker Tool

The File Picker Tool provides LLM agents with direct, reliable access to specific files in the codebase without relying on semantic search or vector embeddings.

## Overview

Unlike the semantic file retrieval tool that uses vector search, the File Picker Tool:

1. **Scans the repository** for relevant files (Solidity, documentation, etc.)
2. **Presents a complete list** of available files to the LLM
3. **Allows direct selection** of up to 3 files by exact path
4. **Returns full file contents** without any search complexity

## Key Advantages

- **🎯 Reliability**: No dependency on vector search quality or embeddings
- **👁️ Transparency**: LLM can see exactly what files are available
- **🔍 Precision**: Direct file access by exact path matching
- **⚡ Efficiency**: No complex semantic matching or scoring required
- **🧩 Simplicity**: Straightforward file listing and retrieval

## Usage

### In Agent Configuration

```rust
use ai_agent_audit::llm_review::agent_factory::{AgentConfig, AgentFactory};

let config = AgentConfig::for_security_audit(repo_paths)
    .with_file_picker(true);  // Enable file picker tool

let agent = AgentFactory::create_openai_agent(&config)?;
```

### Tool Parameters

The LLM can call the tool with:

```json
{
  "files": [
    "contracts/Token.sol",
    "test/TokenTest.sol", 
    "README.md"
  ]
}
```

**Constraints:**
- Maximum 3 files per request
- Files must exist in the repository
- Paths are relative to repository root

### Tool Response

Returns combined content with clear file separators:

```
=== contracts/Token.sol ===
pragma solidity ^0.8.0;

contract Token {
    // ... file content
}

=== README.md ===
# Project Documentation
...
```

## Supported File Types

The tool automatically includes:
- **Solidity files** (`.sol`) from `repo.sol_files`
- **Documentation** (`.md`, `.txt`) from `repo.docs`

## Integration with Other Tools

The File Picker Tool complements the existing File Retrieval Tool:

- **File Picker**: When you know exactly which files you need
- **File Retrieval**: When you need semantic search across file contents

Both tools can be enabled simultaneously for maximum flexibility.

## Error Handling

The tool provides clear error messages for:
- Too many files requested (>3)
- File not found in repository
- IO errors reading files

## Example LLM Usage

```
LLM: "I need to examine the main token contract and its test file."

Tool Call: pick_files({"files": ["contracts/Token.sol", "test/TokenTest.sol"]})

Response: Combined content of both files with clear separators
```

This makes file access much more predictable and reliable for LLM agents performing security audits.
