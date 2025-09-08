# zbg-rust

A Rust-based command-line tool git commands. A remix of [zbg](https://github.com/chshersh/zbg)

## Project goals

> This project is created in pursue of the following goals.

1. **Learning Rust.**
    + I learn better when I build.
2. **Git's internal working**
    + "So you think you know `git`?"

## Installation

```bash
# Clone repository
git clone https://github.com/Toyin5/zbg-rust
cd zbg-rust

# Build and install
cargo install --path .
⚠️ On Linux, make sure ~/.cargo/bin is in your PATH:

export PATH="$HOME/.cargo/bin:$PATH"
# Run status command with latest commit
zbg-rust status

# Or provide a commit/branch
zbg-rust status <commit>
```

## Project Structure
```
src/
│── main.rs        # Entry point
│── lib.rs         # Common exports
|── util.rs        # helper functions
│── models/        # Structs, enums, traits
│    └── mod.rs   
│── commands/      # all git commands
│    └── mod.rs    
```