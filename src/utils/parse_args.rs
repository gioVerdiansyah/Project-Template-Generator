use std::collections::HashMap;
use std::env;
use serde_json::Value;

pub fn parse_args() -> Option<HashMap<String, String>> {
    let args: Vec<String> = env::args().collect();

    for arg in args {
        if arg.starts_with("--pattern=") {
            let json_str = arg.trim_start_matches("--pattern=");

            // Debug output
            println!("Trying to parse JSON: {}", json_str);

            let processed_json = if !json_str.starts_with('{') {
                json_str.to_string()
            } else {
                json_str.to_string()
            };

            // Parse JSON string to HashMap
            match serde_json::from_str::<HashMap<String, Value>>(&processed_json) {
                Ok(json_map) => {
                    // Convert Value to String for easier handling
                    let mut pattern_map = HashMap::new();
                    for (key, value) in json_map {
                        // Convert Value to String representation
                        let value_str = match value {
                            Value::String(s) => s,
                            _ => value.to_string(),
                        };
                        pattern_map.insert(key, value_str);
                    }
                    return Some(pattern_map);
                },
                Err(e) => {
                    eprintln!("Error parsing JSON pattern: {}", e);
                    eprintln!("Make sure your JSON format is correct, example: --pattern='{{\"<package_name>\": \"test_app\"}}'");

                    // Alternative parsing for simple key-value formats
                    if json_str.contains(':') {
                        println!("Trying alternative parsing method...");
                        let mut pattern_map = HashMap::new();

                        // Remove outer braces if present
                        let clean_str = json_str
                            .trim_start_matches('{')
                            .trim_end_matches('}')
                            .trim();

                        // Split by comma for multiple key-value pairs
                        for pair in clean_str.split(',') {
                            if let Some((key, value)) = pair.split_once(':') {
                                let clean_key = key
                                    .trim()
                                    .trim_matches('"')
                                    .trim_matches('\'');

                                let clean_value = value
                                    .trim()
                                    .trim_matches('"')
                                    .trim_matches('\'');

                                pattern_map.insert(clean_key.to_string(), clean_value.to_string());
                            }
                        }

                        if !pattern_map.is_empty() {
                            println!("Alternative parsing successful!");
                            return Some(pattern_map);
                        }
                    }

                    return None;
                }
            }
        }
    }

    None
}