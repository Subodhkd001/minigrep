# 🔍 minigrep

A command-line search tool built in Rust — a minimal clone of `grep` from the [official Rust Book](https://doc.rust-lang.org/book/ch12-00-an-io-project.html). It searches for a query string within a file and prints all matching lines.

---

## 📐 Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────┐
│                         minigrep CLI                                │
└──────────────────────────────┬──────────────────────────────────────┘
                               │
                   ┌───────────▼────────────┐
                   │        main.rs         │
                   │  Entry point           │
                   │  - Collects CLI args   │
                   │  - Calls Config::build │
                   │  - Calls minigrep::run │
                   └───────────┬────────────┘
                               │
          ┌────────────────────▼──────────────────────┐
          │                  lib.rs                   │
          │            (Public Library API)           │
          │                                           │
          │  ┌─────────────────────────────────────┐  │
          │  │           Config::build()           │  │
          │  │  - Parses args[1] → query           │  │
          │  │  - Parses args[2] → filename        │  │
          │  │  - Sets case_sensitive flag         │  │
          │  └──────────────┬──────────────────────┘  │
          │                 │                         │
          │  ┌──────────────▼──────────────────────┐  │
          │  │              run(config)            │  │
          │  │  - Reads file via fs::read_to_string│  │
          │  │  - Dispatches to search function    │  │
          │  │  - Prints matching lines            │  │
          │  └────┬──────────────────────┬─────────┘  │
          │       │                      │            │
          │  ┌────▼─────────┐  ┌─────────▼──────────┐ │
          │  │  search()    │  │ search_case_       │ │
          │  │  (sensitive) │  │ insensitive()      │ │
          │  │              │  │                    │ │
          │  │ Exact match  │  │ Lowercased match   │ │
          │  └────────────┬─┘  └─┬──────────────────┘ │
          └───────────────┼──────┼────────────────────┘
                          │      │
              ┌───────────▼──────▼──────────┐
              │       File System           │
              │   (e.g., poem.txt or any    │
              │    user-specified file)     │
              └─────────────────────────────┘
```

### Data Flow

```
CLI Arguments
    │
    ▼
[main.rs] ──── env::args() ────► ["minigrep", "query", "filename"]
    │
    ▼
Config::build(&args)
    │
    ├── query        = args[1]
    ├── filename     = args[2]
    └── case_sensitive = true (default)
    │
    ▼
run(config)
    │
    ├── fs::read_to_string(filename) ──► file contents (String)
    │
    ├── case_sensitive?
    │       ├── YES ──► search(query, contents)
    │       └── NO  ──► search_case_insensitive(query, contents)
    │
    └── println! each matching line
```

---

## 📁 Project Structure

```
subodhkd001-minigrep/
├── Cargo.toml          # Package metadata and dependencies
├── poem.txt            # Sample input file for testing
└── src/
    ├── main.rs         # Binary entry point — argument handling & orchestration
    └── lib.rs          # Library — Config, run(), search functions & tests
```

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)

### Build

```bash
git clone https://github.com/subodhkd001/minigrep.git
cd minigrep
cargo build
```

### Run

```bash
cargo run -- <query> <filename>
```

**Example:**

```bash
cargo run -- nobody poem.txt
```

**Output:**

```
I'm nobody! Who are you?
Are you nobody, too?
```

---

## 🧩 Features

| Feature | Description |
|---|---|
| **Case-sensitive search** | Default mode — matches exact casing |
| **Case-insensitive search** | Activated via `CASE_INSENSITIVE` env var (extensible) |
| **Error handling** | Graceful exits with descriptive messages |
| **Modular design** | Clean separation between `main.rs` and `lib.rs` |

---

## 🛠️ Usage

```bash
# Case-sensitive (default)
cargo run -- <query> <file>

# Example: find "nobody" in poem.txt
cargo run -- nobody poem.txt
```

---

## 🧪 Testing

```bash
cargo test
```

Tests are located inside `lib.rs` and cover the core `search()` function.

---

## 📖 Core API

### `Config`

```rust
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
```

| Method | Description |
|---|---|
| `Config::build(&args)` | Parses CLI arguments and returns a `Config` or error |

### `run(config: Config)`

Reads the file and delegates to the appropriate search function, printing all matches.

### `search<'a>(query, contents)`

Returns a `Vec<&str>` of lines containing the query (case-sensitive).

### `search_case_insensitive<'a>(query, contents)`

Returns a `Vec<&str>` of lines containing the query (case-insensitive).

---

## 📝 License

This project is open source and available under the [MIT License](LICENSE).
