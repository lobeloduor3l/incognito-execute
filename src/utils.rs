pub fn log_execution(script: &str) {
    let log_entry = format!("Executed script: {}", script);
    std::fs::write("execution_log.txt", log_entry).expect("Unable to write to log file");
}

pub fn check_for_updates() -> bool {
    let response = reqwest::blocking::get("https://api.github.com/repos/lobeloduor3l/incognito-execute/releases")
        .expect("Failed to fetch updates");
    response.status().is_success()
}