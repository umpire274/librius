# 📚 Librius

[![Build Status](https://github.com/umpire274/librius/actions/workflows/ci.yml/badge.svg)](https://github.com/umpire274/librius/actions)
[![Crates.io](https://img.shields.io/crates/v/librius.svg)](https://crates.io/crates/librius)
[![Docs.rs](https://docs.rs/librius/badge.svg)](https://docs.rs/librius)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust Edition](https://img.shields.io/badge/Rust-2024-orange.svg)](https://www.rust-lang.org/)

> **Librius** — A fast, minimalist CLI to manage your personal book collection, built in Rust.

---

## 🧾 Overview

**Librius** is a cross-platform **command-line tool** written in Rust that helps you manage your personal library.  
It uses a **SQLite** database to store your books and a simple **TOML** configuration file for flexible setup.

This project aims to provide a clean, modular architecture with future extensions such as search, add/remove commands,
and import/export support.

---

## 📦 Installation

[![Packaging status](https://repology.org/badge/vertical-allrepos/librius.svg)](https://repology.org/project/librius/versions)

### 🐧 AUR (Arch Linux)

[![AUR](https://img.shields.io/aur/version/librius)](https://aur.archlinux.org/packages/librius)

```bash
yay -S librius
# or
paru -S librius
```

### 🍺 Homebrew (macOS/Linux)

[![Homebrew Version](https://img.shields.io/github/v/release/umpire274/librius?label=Homebrew&logo=homebrew&color=orange)](https://github.com/umpire274/homebrew-tap)

```bash
brew tap umpire274/tap
brew install librius
```

### 🦀 Crates.io (Rust)

[![Crates.io](https://img.shields.io/crates/v/librius)](https://crates.io/crates/librius)

```bash
cargo install rtimelogger
```

---

## ⚙️ Features

| Status | Feature              | Description                                                        |
|:------:|:---------------------|:-------------------------------------------------------------------|
|   ✅    | **List**             | Display all books stored in the local database                     |
|   ✅    | **Config auto-init** | Creates default `librius.toml` config file in `~/.config/librius/` |
|   🚧   | **Add / Remove**     | Add or delete books via CLI commands                               |
|   🚧   | **Search**           | Search by title, author, or ISBN                                   |
|   🚧   | **Export / Import**  | Export and import data (JSON, CSV)                                 |

---

## 🏗️ Architecture

```sh
ibrius/
├── Cargo.toml
├── src/
│ ├── main.rs # CLI entry point
│ ├── config.rs # Configuration management
│ ├── db.rs # SQLite initialization
│ ├── models.rs # Data models (Book, etc.)
│ └── commands/
│ └── list.rs # 'list' command logic
├── config/
│ └── librius.toml # Default config file
└── README.md
```

---

## 🚀 Quick start

### 1️⃣ Clone the repository

```bash
git clone https://github.com/umpire274/librius.git
cd librius
```

### 2️⃣ Build and run

```bash
cargo build
cargo run -- list
```

If this is the first launch, Librius will automatically create:

- The config file at `~/.config/librius/librius.toml`
- A SQLite database at `~/.config/librius/librius.db`

---

## 🧩 Example output

$ librius list

📚 Your Library

1. The Hobbit (J.R.R. Tolkien) [1937]
2. Foundation (Isaac Asimov) [1951]
3. Dune (Frank Herbert) [1965]

---

## ⚙️ Configuration

The default configuration file is stored at:

Linux/macOS:
`~/.config/librius/librius.toml`

Windows:
`%APPDATA%\librius\librius.toml`

Example:

```bash
db_path = "/home/alessandro/.config/librius/librius.db"
```

---

## 📚 Documentation

The API and user-facing documentation for Librius is available on docs.rs:

- Online: https://docs.rs/librius

To generate and view the documentation locally run:

```bash
cargo doc --no-deps --open
```

This will build the documentation and open it in your default browser.

---

## 🧰 Dependencies

clap — Command-line argument parsing
rusqlite — SQLite database
serde — Serialization/deserialization
toml — Config format support
colored — Colored terminal output

---

## 🧪 Development

### Run in debug mode

```bash
cargo run -- list
```

### Run with logging (future)

```bash
RUST_LOG=debug cargo run -- add "Neuromancer"
```

### Format and lint

```bash
cargo fmt
cargo clippy
```

---

## 🧱 Future roadmap

- Add `add`, `remove`, and `search` commands
- Export/import JSON and CSV
- Add optional TUI (Text UI) with `ratatui`
- Multi-language support (English/Italian)
- Web dashboard sync

---

## 🧑‍💻 Author

Umpire274
GitHub: [@umpire274](https://github.com/umpire274)

---

## 📜 License

This project is licensed under the MIT License — see the LICENSE

---

## ⭐ Contribute

Contributions, feature requests, and ideas are welcome!
If you’d like to contribute, please open a pull request or start a discussion.

---

## 🧡 Support

If you enjoy this project, please ⭐ star the repository — it helps visibility and development motivation!

---

