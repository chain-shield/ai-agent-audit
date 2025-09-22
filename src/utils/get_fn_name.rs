use regex::Regex;

pub fn get_function_name_from_func_id(function_id: &str) -> String {
    let re = Regex::new(r"^[0-9]+_([a-zA-Z_][a-zA-Z0-9_#]*)").unwrap();

    if let Some(caps) = re.captures(function_id) {
        let function_name = &caps[1];
        return function_name.to_string();
    }
    "".to_string()
}

pub fn get_function_name_from_interface(function_interface: &str) -> String {
    let re = Regex::new(r"^([a-zA-Z_][a-zA-Z0-9_]*)\s*\(").unwrap();

    if let Some(caps) = re.captures(function_interface) {
        let function_name = &caps[1];
        return function_name.to_string();
    }
    "".to_string()
}

pub fn string_starts_with_char(fn_name: &str, c: char) -> bool {
    if let Some(first_char) = fn_name.chars().next() {
        if first_char == c {
            true
        } else {
            false
        }
    } else {
        false
    }
}
