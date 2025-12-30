# Rust Mini Backend System

**Client + Server + Mini Database (From Scratch)**

## Overview

This project is a **low-level backend system built from scratch in Rust**, designed to deeply understand:

- How client–server systems work
- How HTTP is implemented over TCP
- How databases store and retrieve data
- How Rust models memory, ownership, and safety
- How real backend abstractions are built (not used)

The system consists of **three components**:

1. **Client** — a simple TCP/HTTP client (like a tiny `curl`)
2. **Server** — a minimal HTTP server implemented using `std::net`
3. **Mini Database** — a file-backed key–value database implemented as a Rust library

No frameworks.

No async runtimes.

No ORMs.

No magic.

---

## System Architecture

```
Client  ──HTTP over TCP──>  Server  ──function calls──>  Mini Database

```

### Design Principles

- Explicit over implicit
- Simple over clever
- Separation of concerns
- Learn how things work internally before abstracting them

---

## Repository Structure

```
rust-mini-system/
├── Cargo.toml              # Workspace definition
├── README.md
├── crates/
│   ├── server/             # HTTP server binary
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   ├── client/             # Test client binary
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   └── minidb/             # Mini database library
│       ├── Cargo.toml
│       └── src/lib.rs
├── data/
│   └── database.db         # Persistent storage file

```

### Component Roles

### `minidb` (Library Crate)

- File-backed storage
- Owns all data layout and persistence logic
- Exposes a **small, stable API**
- Has no knowledge of networking or HTTP

### `server` (Binary Crate)

- TCP listener
- Minimal HTTP parsing
- Request routing
- Calls `minidb` functions
- Builds HTTP responses

### `client` (Binary Crate)

- Opens TCP connection
- Sends HTTP requests
- Prints raw HTTP responses
- Used for testing and learning

---

## Mini Database Design

The database is intentionally simple.

### Characteristics

- Key–value store
- File-backed (persistent)
- Page-based or record-based storage
- Manual serialization / deserialization
- No SQL
- No JSON (initially)

### Public API

```rust
pub struct Db {
    path: String,
}

impl Db {
    pub fn open(path: &str) -> Db;
    pub fn insert(&self, key: u32, value: &str);
    pub fn get(&self, key: u32) -> Option<String>;
    pub fn get_all(&self) -> Vec<(u32, String)>;
    pub fn update(&self, key: u32, value: &str);
    pub fn delete(&self, key: u32);
}
```

### Storage Format

Simple text file with one record per line:
```
1:Alice
2:Bob
3:Carol
```

### Rules

- The database never knows HTTP exists
- The server never touches internal DB structures
- All persistence logic lives in `minidb`

---

## Server Design

### Responsibilities

- Accept TCP connections
- Read raw bytes from clients
- Parse minimal HTTP requests
- Route requests
- Call database functions
- Send valid HTTP responses

### Supported Endpoints

```
GET /              → Welcome message
GET /all           → List all data
GET /get?key=1     → Get value by key
GET /set?key=1&value=hello    → Insert new record
GET /update?key=1&value=world → Update existing record
GET /delete?key=1  → Delete record
```

### HTTP Scope

- HTTP/1.1
- Only `GET` supported
- Minimal request parsing
- Correct status line and headers
- Proper `Content-Length`

This server is **educational**, not production-grade.

---

## Client Design

The client is a **testing tool**, not an application.

### Responsibilities

- Open TCP connection
- Send HTTP request
- Print server response

### Usage

```bash
cargo run -p client -- all              # List all data
cargo run -p client -- get 1            # Get value for key 1
cargo run -p client -- set 1 Alice      # Insert key=1, value=Alice
cargo run -p client -- update 1 Alicia  # Update key 1
cargo run -p client -- delete 1         # Delete key 1
```

This helps understand:

- How HTTP requests are formed
- How servers respond
- How tools like `curl` work internally

---

## Build Order (Important)

Follow this order strictly:

1. **Mini Database**
    - File I/O
    - Ownership
    - Persistence
2. **TCP Server (no HTTP)**
    - Accept connections
    - Read/write bytes
3. **HTTP Parsing**
    - Parse request line only
4. **Server ↔ Database Integration**
    - Real backend flow
5. **Client**
    - End-to-end testing

---

## Building the Project

### Requirements

- Rust (stable)
- Cargo
- Linux or macOS

### Build All Crates

```bash
cargo build

```

### Run Server

```bash
cargo run -p server

```

### Run Client

```bash
cargo run -p client

```

---

## Learning Goals

After completing this project, you should be able to:

- Explain how HTTP works over TCP
- Design a client–server architecture
- Build a file-backed database
- Understand Rust ownership in real systems
- Know why frameworks exist and what they abstract
- Confidently move to higher-level Rust/Go backends

---

