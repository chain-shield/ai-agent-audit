use regex::Regex;

pub fn get_function_name_from_func_id(function_id: &str) -> String {
    let re = Regex::new(r"^[0-9]+_([a-zA-Z_][a-zA-Z0-9_#]*)").unwrap();

    if let Some(caps) = re.captures(function_id) {
        caps[1].to_string()
    } else {
        String::new()
    }
}

pub fn get_function_name_from_interface(function_interface: &str) -> String {
    let re = Regex::new(r"^([a-zA-Z_][a-zA-Z0-9_]*)\s*\(").unwrap();

    if let Some(caps) = re.captures(function_interface) {
        caps[1].to_string()
    } else {
        String::new()
    }
}

pub fn string_starts_with_char(fn_name: &str, c: char) -> bool {
    if let Some(first_char) = fn_name.chars().next() {
        first_char == c
    } else {
        false
    }
}
