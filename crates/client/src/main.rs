use std::net::TcpStream;
use std::io::{Read, Write};
use std::env;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // args[0] = program name
    // args[1] = command (all, get, set, update, delete)
    // args[2] = key (for get, set, update, delete)
    // args[3] = value (for set, update)

    if args.len() < 2 {
        println!("Usage:");
        println!("  client all");
        println!("  client get <key>");
        println!("  client set <key> <value>");
        println!("  client update <key> <value>");
        println!("  client delete <key>");
        return;
    }

    let command = &args[1];

    // Build the path based on command
    let path = match command.as_str() {
        "all" => "/all".to_string(),

        "get" => {
            if args.len() < 3 {
                println!("Usage: client get <key>");
                return;
            }
            format!("/get?key={}", args[2])
        }

        "set" => {
            if args.len() < 4 {
                println!("Usage: client set <key> <value>");
                return;
            }
            format!("/set?key={}&value={}", args[2], args[3])
        }

        "update" => {
            if args.len() < 4 {
                println!("Usage: client update <key> <value>");
                return;
            }
            format!("/update?key={}&value={}", args[2], args[3])
        }

        "delete" => {
            if args.len() < 3 {
                println!("Usage: client delete <key>");
                return;
            }
            format!("/delete?key={}", args[2])
        }

        _ => {
            println!("Unknown command: {}", command);
            return;
        }
    };

    // Connect to server
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    // Send HTTP request
    let request = format!("GET {} HTTP/1.1\r\n\r\n", path);
    stream.write_all(request.as_bytes()).unwrap();

    // Read response
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);

    // Print just the body (skip headers)
    if let Some(body_start) = response.find("\r\n\r\n") {
        println!("{}", &response[body_start + 4..]);
    } else {
        println!("{}", response);
    }
}
