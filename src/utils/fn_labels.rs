pub fn get_visibility_label(visibility: &str) -> String {
    let label = match visibility {
        "external" => "[EXTERN]",
        "public" => "[PUBLIC]",
        "internal" => "[INTERNAL]",
        "private" => "[PRIVATE]",
        _ => "",
    };

    label.to_string()
}

pub fn get_modifiers_label(modifiers: &str) -> String {
    let owner = modifiers.split(',').any(|m| m == "onlyOwner");
    let mod_tag = if owner { "[OWNER]" } else { "" };

    mod_tag.to_string()
}
