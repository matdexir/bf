
A simple, fast Brainfuck interpreter written in Rust. ✨

## Features
- Interprets all 8 BF commands (> < + - . , [ ])
- Standard 30,000 cell memory tape
    - Currently only supports 10
- Handles stdin input (,) & stdout output (.)
- Correctly processes nested loops ([ ])

## Quick Start 🚀

(Assumes you have the Rust toolchain installed: rustup.rs)

### Clone & Enter:

```bash
git clone https://github.com/matdexir/bf.git
cd bf
```
### Build & Run:
Pass the path to your Brainfuck file (.bf) as an argument.

```bash
# Compiles (optimised) and runs in one step
cargo run --release -- <path/to/your_code.bf>
```
