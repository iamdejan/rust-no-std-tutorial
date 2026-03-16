# Rust `no_std` Tutorial

A minimum example of Rust code using `no_std`. This repository is inspired by [app-template](https://github.com/knurling-rs/app-template) and [The Rust Embedded Book](https://doc.rust-lang.org/beta/embedded-book/start/qemu.html).

## Prerequisites

Before running this program, ensure you have the following installed:

1. **Rust** and **Cargo**: install via [rustup](https://rustup.rs/).
   Because this is using `no_std` feature, we are unable to use Rust inside Pixi.

2. **rust-docs-mcp** - An MCP server that provides comprehensive access to Rust crate documentation, source code analysis, dependency trees, and module structure visualization. Install it using one of the following methods:

   **Quick Install (recommended):**
   ```bash
   curl -sSL https://raw.githubusercontent.com/snowmead/rust-docs-mcp/main/install.sh | bash
   ```

   **Or via Cargo:**
   ```bash
   cargo install rust-docs-mcp
   ```

   For more installation options, see the [official installation guide](https://github.com/snowmead/rust-docs-mcp#installation).

3. **qemu-system-arm** - QEMU is an emulator for Cortex M3 and other microcontrollers. Install it using:

   **apt (Ubuntu):**
   ```bash
   sudo apt install qemu-system-arm
   ```

4. **qemu-run** - Runs `qemu-system-arm` but decodes `defmt` data sent to [semihosting](https://doc.rust-lang.org/nightly/embedded-book/start/semihosting.html). Install it using:
   **Cargo:**
   ```bash
   cargo install qemu-run
   ```

## Run the Program

Use the following command to build and run the project:
   ```bash
   cargo run
   ```
This will automatically build the code into `./target/thumbv7m-none-eabi/debug/main` file, which will be used for `qemu-run`

### Available Commands

| Command | Description |
|---------|-------------|
| `cargo build` | Build the Rust project |
| `cargo run` | Build and run the program |
| `cargo fmt` | Format the code using rustfmt |
| `cargo clippy` | Run clippy linter |
| `cargo clippy --fix --allow-dirty` | Auto-fix linting issues |
| `cargo clean` | Clean build artifacts |

## Project Tree Structure

```
Root/
├── Cargo.toml              # Rust package manifest
├── Cargo.lock              # Dependency lockfile
├── memory.x                # Linker script for memory layout
├── LICENSE.txt             # MIT License
├── README.md               # Project documentation
├── AGENTS.md               # AI agent guidelines
├── .editorconfig           # Editor configuration
├── .gitignore              # Git ignore rules
├── .cargo/                 # Cargo configuration
│   └── config.toml         # Target and runner configuration
├── .vscode/                # VSCode settings
│   └── settings.json       # rust-analyzer configuration
├── .kilocode/              # Kilo Code assistant rules
│   └── mcp.json            # MCP server configuration
├── src/
│   ├── lib.rs              # Library entry point (no_std, no_main)
│   └── bin/
│       └── main.rs         # Binary entry point
└── tests/
    └── integration.rs      # Integration tests
```

This section explains the purpose of each file in the repository:

### Root Directory Files

| File | Description |
|------|-------------|
| [`Cargo.toml`](Cargo.toml) | Rust package manifest that defines the project metadata, dependencies, and build configuration. It specifies the package name, version, edition (2024), and linter rules for dead code and clippy. This project targets `no_std` embedded systems. |
| [`Cargo.lock`](Cargo.lock) | Automatically generated file that locks dependency versions to ensure reproducible builds. Commit this to Git for consistent builds across machines. |
| [`memory.x`](memory.x) | Linker script that defines the memory layout for the target microcontroller (LM3S6965). It specifies the FLASH and RAM memory regions with their origin addresses and lengths. |
| [`LICENSE.txt`](LICENSE.txt) | MIT License file granting permission to use, copy, modify, and distribute the software. |
| [`README.md`](README.md) | Project documentation containing setup instructions, prerequisites, and usage commands. |
| [`AGENTS.md`](AGENTS.md) | Guidelines for AI coding agents, including code generation rules, validation steps (fmt, lint-ci, test), and documentation requirements. |
| [`.editorconfig`](.editorconfig) | Editor configuration that ensures consistent coding style across different editors (4-space indentation for Rust and TOML files, 2-space for Markdown). |
| [`.gitignore`](.gitignore) | Specifies files and directories that Git should ignore (e.g., build artifacts in `target/`, IDE files, pixi environments in `.pixi/`). |

### Cargo Configuration

| Path | Description |
|------|-------------|
| [`.cargo/`](.cargo/) | Directory containing Cargo build configuration for the embedded target. |
| [`.cargo/config.toml`](.cargo/config.toml) | Configures the target platform as `thumbv7m-none-eabi` (Cortex-M3), sets up the QEMU runner, and configures linker flags including `flip-link`, custom linker scripts (`link.x`, `defmt.x`), and `nmagic` for proper memory alignment. |

### VSCode Configuration

| Path | Description |
|------|-------------|
| [`.vscode/`](.vscode/) | Directory containing VSCode workspace settings. |
| [`.vscode/settings.json`](.vscode/settings.json) | Overrides rust-analyzer defaults to only check binary targets (not test targets) since this is a `no_std` project that cannot compile tests by default. |

### Kilo Code Configuration

| Path | Description |
|------|-------------|
| [`.kilocode/`](.kilocode/) | Directory containing MCP server configuration for the Kilo Code assistant. |
| [`.kilocode/mcp.json`](.kilocode/mcp.json) | Configures the rust-docs MCP server with stdio transport, allowing various documentation and source code analysis tools. |

### Source Files

| File | Description |
|------|-------------|
| [`src/lib.rs`](src/lib.rs) | The library crate entry point with `no_std` and `no_main` attributes. Provides global logger via `defmt_semihosting`, panic handler via `panic-probe`, and defines the `exit()` function for semihosting. Includes a unit tests module using `defmt-test`. |
| [`src/bin/main.rs`](src/bin/main.rs) | The binary entry point that runs on the embedded target. Uses `#[cortex_m_rt::entry]` attribute. Demonstrates `defmt::Format` derive macro with generic structs and prints "Hello, world!" to the semihosting output. |

### Test Files

| File | Description |
|------|-------------|
| [`tests/integration.rs`](tests/integration.rs) | Integration tests for the `no_std` crate. Uses `defmt_test::tests` attribute and includes a simple test that asserts `true`. Requires harness = false in Cargo.toml. |
