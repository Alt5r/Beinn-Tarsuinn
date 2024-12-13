use std::collections::HashMap;


/// used to convert to list, now to hash map to help with the firewall 
/// functionality
pub fn listify(request: String) -> HashMap<String, String> {
    // Split into lines, and handle possible \r\n endings
    let lines: Vec<&str> = request.split('\n').map(|line| line.strip_suffix("\r").unwrap_or(line)).collect();
    
    // Create a HashMap for headers
    let mut headers = HashMap::new();

    // Handle the first line separately (e.g., "GET / HTTP/1.1")
    if let Some(first_line) = lines.first() {
        headers.insert("Received".to_string(), first_line.trim().to_string());
    }

    // Parse the remaining lines
    for line in &lines[1..] {
        // Split into key and value based on ':', ignoring invalid lines
        if let Some((key, value)) = line.split_once(':') {
            headers.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    println!("This is a hashmap of headers: {:?}", headers);
    headers
}