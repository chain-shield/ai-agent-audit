//  check if content contains at least 1 contract
//  that does not have 'mock' in its name
pub fn has_non_mock_contract(content: &str) -> bool {
    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("contract ") {
            if let Some(name) = trimmed
                .split_whitespace()
                .nth(1)
                .map(|s| s.trim_end_matches('{').to_ascii_lowercase())
            {
                if !name.contains("mock") {
                    return true;
                }
            }
        }
    }
    false
}
