use regex::Regex;

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

/// Check if contract name exists in content using word boundary
pub fn contains_contract_reference(contract_name: &str, content: &str) -> bool {
    // \b = word boundary
    // This ensures we match the exact word, not substrings
    let pattern = format!(r"\b{}\b", regex::escape(contract_name));

    match Regex::new(&pattern) {
        Ok(regex) => regex.is_match(content),
        Err(_) => false,
    }
}
