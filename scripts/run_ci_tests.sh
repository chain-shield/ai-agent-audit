#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "Running hermetic CI test suite"

cargo test --lib

tests=(
  all_round_unit_test
  codeblock_db_unit_test
  deployment_scripts_detection_test
  detect_source_code_dependencies_test
  enum_serde_test
  gemini_json_parsing_test
  generate_prompts_test
  interface_implementation_detection_test
  invariants_json_parsing_test
  json_extraction_test
  lib_package_json_detection_test
  library_file_detection_test
  nested_lib_test
  node_modules_direct_dependency_test
  path_canonicalization_test
  remapping_integration_test
  slither_project_detection_test
  summarize_db_unit_test
  test_actual_gemini_error
  test_broken_gemini_json
  test_escaped_newline_pattern
  test_gemini_failure_actual
  test_user_actual_failure
  validation_round_unit_test
  verify_nested_lib_behavior
)

for test_name in "${tests[@]}"; do
  echo
  echo "==> cargo test --test ${test_name}"
  cargo test --test "${test_name}"
done
