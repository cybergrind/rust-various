# Greeting CLI Task

## Task Overview
Create a simple CLI greeting tool using the `clap` crate.

## Learning Goals
- Learn how to add and use external crates (dependencies)
- Understand command-line argument parsing with `clap`
- Practice error handling in Rust
- Explore pattern matching
- Write basic tests

## Requirements
1. Accept a name as an argument: `./greeting John`
2. Add optional flags for different greeting styles:
   - `--shout` or `-s` for uppercase output
   - `--formal` or `-f` for formal greeting
3. Handle errors gracefully (e.g., when no name is provided)

## Working Approach
- User prefers to figure out implementation details independently
- User will describe their plan before implementing
- Assistant should provide guidance when asked, not overly detailed steps
- Focus on answering specific questions rather than providing complete solutions

## Progress
- [x] Add clap dependency to Cargo.toml (added with derive feature)
- [x] Create binary target structure (src/bin/greeting.rs)
- [x] Configure Cargo.toml with [[bin]] section
- [ ] Implement basic argument parsing
- [ ] Add greeting logic with different styles
- [ ] Add error handling
- [ ] Write tests

## Current Status
- User is reading clap documentation to implement the CLI parser
- Basic project structure is set up and working
- Can run the stub with: `cargo run --bin greeting`

## Implementation Details
- Using clap with derive feature for easier CLI definition
- Binary target: `src/bin/greeting.rs` → produces `greeting` executable
- Using Rust edition 2024

## Notes
- This is the first practical Rust task after initial setup
- Building foundation for future CLI tools
- User prefers to read documentation and implement independently