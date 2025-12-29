# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an educational Rust project building a low-level backend system from scratch to understand how client-server systems, HTTP over TCP, and databases work internally. No frameworks, async runtimes, or ORMs.

**The goal is learning, not building.** Prioritize understanding over features. Explain concepts, show how things work internally, and help build mental models rather than just producing working code.

## Build Commands

```bash
cargo build          # Build all crates
cargo run            # Run main binary (currently single crate)
cargo test           # Run tests
cargo clippy         # Lint
cargo fmt            # Format code
```

## Architecture

The system is designed as three components (currently in early development):

```
Client  ──HTTP over TCP──>  Server  ──function calls──>  Mini Database
```

**Planned workspace structure:**
- `crates/minidb/` - File-backed key-value database library (no networking knowledge)
- `crates/server/` - TCP listener with minimal HTTP parsing, calls minidb
- `crates/client/` - TCP client for testing (like a tiny curl)

**Current state:** Single crate at root level, workspace structure not yet implemented.

## Design Principles

- Explicit over implicit
- Simple over clever
- Separation of concerns: database never knows HTTP exists, server never touches internal DB structures
- All persistence logic lives in minidb

## Key Constraints

- HTTP/1.1, GET only
- Manual serialization (no JSON initially)
- File-backed persistence with page-based or record-based storage
- Educational, not production-ready
