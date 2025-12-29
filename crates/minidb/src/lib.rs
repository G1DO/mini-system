// Mini database - file-backed key-value store
use std::fs::{File, OpenOptions};
use std::io::{Write, BufRead, BufReader};

pub struct Db {
    path: String,  // remember where our file is
}

impl Db {
    // Create or open database
    pub fn open(path: &str) -> Db {
        // Create file if it doesn't exist
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)
            .expect("Failed to create database file");

        Db {
            path: path.to_string(),
        }
    }

    // Insert a key-value pair
    pub fn insert(&self, key: u32, value: &str) {
        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.path)
            .expect("Failed to open database file");

        let line = format!("{}:{}\n", key, value);
        file.write_all(line.as_bytes())
            .expect("Failed to write to database");
    }

    // Get value by key
    pub fn get(&self, key: u32) -> Option<String> {
        let file = File::open(&self.path).ok()?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.ok()?;
            // Split "1:Alice" into ["1", "Alice"]
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                let k: u32 = parts[0].parse().ok()?;
                if k == key {
                    return Some(parts[1].to_string());
                }
            }
        }
        None
    }

    pub fn update(&self, key: u32, value: &str) {
        // Step 1: Read entire file
        let content = std::fs::read_to_string(&self.path)
            .expect("Failed to read database");

        // Step 2: Build new content
        let mut new_lines: Vec<String> = Vec::new();

        for line in content.lines() {
            if line.starts_with(&format!("{}:", key)) {
                // This is the one we want to change
                new_lines.push(format!("{}:{}", key, value));
            } else {
                // Keep it as is
                new_lines.push(line.to_string());
            }
        }

        // Step 3: Write back to file
        let new_content = new_lines.join("\n");
        std::fs::write(&self.path, new_content)
            .expect("Failed to write updated database");
    }

    // Get all key-value pairs
    pub fn get_all(&self) -> Vec<(u32, String)> {
        let mut results = Vec::new();

        let content = std::fs::read_to_string(&self.path).unwrap_or_default();

        for line in content.lines() {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                if let Ok(key) = parts[0].parse::<u32>() {
                    results.push((key, parts[1].to_string()));
                }
            }
        }

        results
    }

    pub fn delete(&self, key: u32) {
        // Step 1: Read entire file
        let content = std::fs::read_to_string(&self.path)
            .expect("Failed to read database");

        // Step 2: Build new content excluding the key to delete
        let mut new_lines: Vec<String> = Vec::new();

        for line in content.lines() {
            if !line.starts_with(&format!("{}:", key)) {
                // Keep it as is
                new_lines.push(line.to_string());
            }
        }

        // Step 3: Write back to file
        let new_content = new_lines.join("\n");
        std::fs::write(&self.path, new_content)
            .expect("Failed to write updated database");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // Helper: create a fresh test database
    fn setup_test_db(name: &str) -> Db {
        let path = format!("test_{}.db", name);
        let _ = fs::remove_file(&path); // delete if exists
        Db::open(&path)
    }

    #[test]
    fn test_insert_and_get() {
        let db = setup_test_db("insert_get");

        db.insert(1, "Alice");
        db.insert(2, "Bob");

        assert_eq!(db.get(1), Some("Alice".to_string()));
        assert_eq!(db.get(2), Some("Bob".to_string()));
    }

    #[test]
    fn test_get_missing_key() {
        let db = setup_test_db("missing");

        assert_eq!(db.get(99), None);
    }

    #[test]
    fn test_update() {
        let db = setup_test_db("update");

        db.insert(1, "Alice");
        db.update(1, "Alicia");  // change Alice to Alicia

        assert_eq!(db.get(1), Some("Alicia".to_string()));
    }

    #[test]
    fn test_delete() {
        let db = setup_test_db("delete");

        db.insert(1, "Alice");
        db.insert(2, "Bob");
        db.delete(1);  // remove Alice

        assert_eq!(db.get(1), None);
        assert_eq!(db.get(2), Some("Bob".to_string()));
    }
}
