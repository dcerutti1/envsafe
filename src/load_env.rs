//this file finds and parses the local .env file.
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn load_env(path: &str) -> Result<HashMap<String, String>, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut env_vars = HashMap::new();
    for line in contents.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty || trimmed_line.starts_with("#") {
            continue;
        }
        if let Some(separator_index) = trimmed_line.find('=') {
            let key = trimmed_line[..separator_index].trim().to_string();
            let value = trimmed_line[separator_index + 1..].trim().to_string();
            env_vars.insert(key, value);
        }
    }
    Ok(env_vars);
}
