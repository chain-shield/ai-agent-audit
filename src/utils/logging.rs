use log::info;
use schemars::schema_for;

use crate::llm_review::findings::findings::Findings;

pub fn print_first_four_lines(text: &str) {
    let lines: Vec<&str> = text.lines().collect();

    for (index, line) in lines.iter().enumerate().take(4) {
        info!("Line {} (at line {}): {}", index + 1, line!(), line);
    }

    if lines.len() > 4 {
        info!("... ({} more lines)", lines.len() - 4);
    }
}

pub fn print_first_n_lines(number_of_lines: usize, text: &str) {
    let lines: Vec<&str> = text.lines().collect();
    let n = if number_of_lines < lines.len() {
        number_of_lines
    } else {
        lines.len()
    };

    for (index, line) in lines.iter().enumerate().take(n) {
        info!("Line {} (at line {}): {}", index + 1, line!(), line);
    }

    if lines.len() > n {
        info!("... ({} more lines)", lines.len() - n);
    }
}

pub fn print_schema() {
    let schema = schema_for!(Findings);
    println!(
        "Generated schema: {}",
        serde_json::to_string_pretty(&schema).unwrap()
    );
}
