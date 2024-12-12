use std::collections::HashMap;

pub fn listify(request: String) -> HashMap<String, String> {
    // Split into lines, and handle possible \r\n endings
    let lines: Vec<&str> = request.split('\n').map(|line| line.strip_suffix("\r").unwrap_or(line)).collect();
    
    // Create a HashMap for headers
    let mut headers = HashMap::new();

    for line in lines {
        // Split into key and value based on ':', ignoring invalid lines
        if let Some((key, value)) = line.split_once(':') {
            headers.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    headers
}