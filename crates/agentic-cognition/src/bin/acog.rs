//! AgenticCognition CLI binary (core crate entry point)
//!
//! This binary delegates to the agentic-cognition-cli crate.
//! For the full CLI, use: cargo install agentic-cognition-cli

fn main() {
    eprintln!("acog: For the full CLI, install agentic-cognition-cli");
    eprintln!("  cargo install agentic-cognition-cli");
    eprintln!("  acog --help");
    std::process::exit(1);
}
