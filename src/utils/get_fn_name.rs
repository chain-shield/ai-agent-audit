use regex::Regex;

pub fn get_function_name(function_interface: &str) -> String {
    let re = Regex::new(r"^([a-zA-Z_][a-zA-Z0-9_]*)\s*\(").unwrap();

    if let Some(caps) = re.captures(function_interface) {
        let function_name = &caps[1];
        return function_name.to_string();
    }
    "".to_string()
}
