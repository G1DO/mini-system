use std::net::TcpListener;
use std::io::{Read, Write};
use minidb::Db;

fn main() {
    // Open the database
    let db = Db::open("server_data.db");
    println!("Database opened!");

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        // Read the request
        let mut buffer = [0; 512];
        let bytes_read = stream.read(&mut buffer).unwrap();
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);

        // Parse the request line
        let request_line = request.lines().next().unwrap_or("");
        let parts: Vec<&str> = request_line.split(' ').collect();
        let full_path = parts.get(1).unwrap_or(&"/");

        // Split path and query string at ?
        let path_parts: Vec<&str> = full_path.splitn(2, '?').collect();
        let path = path_parts.get(0).unwrap_or(&"/");
        let query = path_parts.get(1).unwrap_or(&"");

        println!("Path: {}", path);
        println!("Query: {}", query);

        // Helper: find a parameter value
        let get_param = |name: &str| -> Option<&str> {
            for param in query.split('&') {
                let kv: Vec<&str> = param.splitn(2, '=').collect();
                if kv.len() == 2 && kv[0] == name {
                    return Some(kv[1]);
                }
            }
            None
        };

        // Handle routes
        let body = match *path {
            "/" => "Welcome! Use /get?key=1 or /set?key=1&value=hello".to_string(),

            "/get" => {

                
        
                if let Some(key_str) = get_param("key") {
                    if let Ok(key) = key_str.parse::<u32>() {
                        match db.get(key) {
                            Some(value) => format!("Value: {}", value),
                            None => "Key not found".to_string(),
                        }
                    } else {
                        "Invalid key (must be a number)".to_string()
                    }
                } else {
                    "Missing 'key' parameter".to_string()
                }
            }

            "/set" => {
                let key_str = get_param("key");
                let value = get_param("value");

                match (key_str, value) {
                    (Some(k), Some(v)) => {
                        if let Ok(key) = k.parse::<u32>() {
                            db.insert(key, v);
                            format!("Saved: {} = {}", key, v)
                        } else {
                            "Invalid key (must be a number)".to_string()
                        }
                    }
                    _ => "Missing 'key' or 'value' parameter".to_string(),
                }
            }
            "/update" => {
                let key_str = get_param("key");
                let value = get_param("value");

                match (key_str, value) {
                    (Some(k), Some(v)) => {
                        if let Ok(key) = k.parse::<u32>() {
                            db.update(key, v);
                            format!("Updated: {} = {}", key, v)
                        } else {
                            "Invalid key (must be a number)".to_string()
                        }
                    }
                    _ => "Missing 'key' or 'value' parameter".to_string(),
                }
            }
            "/delete" => {
                if let Some(key_str) = get_param("key") {
                    if let Ok(key) = key_str.parse::<u32>() {
                        db.delete(key);
                        format!("Deleted key: {}", key)
                    } else {
                        "Invalid key (must be a number)".to_string()
                    }
                } else {
                    "Missing 'key' parameter".to_string()
                }
            }

            "/all" => {
                let all_data = db.get_all();
                if all_data.is_empty() {
                    "Database is empty".to_string()
                } else {
                    let mut result = String::from("All data:\n");
                    for (key, value) in all_data {
                        result.push_str(&format!("{}: {}\n", key, value));
                    }
                    result
                }
            }

            _ => "404 Not Found".to_string(),
        };


        // Build HTTP response
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );

        stream.write_all(response.as_bytes()).unwrap();
        println!("Response: {}\n", body);
    }
}
