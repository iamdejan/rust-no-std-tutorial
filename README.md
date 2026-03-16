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
| `cargp run` | Build and run the program |
| `cargo fmt` | Format the code using rustfmt |
| `cargo clippy` | Run clippy linter |
| `cargo clippy --fix --allow-dirty` | Auto-fix linting issues |
| `cargo clean` | Clean build artifacts |
