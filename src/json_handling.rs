use std::collections::HashMap;
///this file will deserialize the json file
use std::fs::File;
use std::io::{self, Read};

pub fn deserialize_json(path: &str) -> Result<HashMap<String, String>, io::Error> {
    let mut file = File::open(path)?;
    let mut json_content = String::new();
    file.read_to_string(&mut json_content)?;

    let mut json_values = HashMap::new();

    let content = if json_content.trim().starts_with("{") && json_content.trim().ends_with("}") {
        let trimmed = json_content.trim();
        &trimmed[1..trimmed.len() - 1] // Remove { and }
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Not valid JSON object",
        ));
    };

    for line in content.lines() {
        let line = line.trim();
        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim().trim_matches('"');
            let value = line[colon_pos + 1..]
                .trim()
                .trim_matches(',')
                .trim_matches('"');
            json_values.insert(key.to_string(), value.to_string());
        }
    }

    Ok(json_values)
}
