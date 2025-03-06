pub fn validate_script(script: &str) -> bool {
    !script.is_empty() && script.len() < 500
}

pub fn format_script(script: &str) -> String {
    script.trim().to_string()
}