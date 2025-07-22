# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust learning project consisting of various small units to explore different Rust features and popular crates.

### Learning Goals
- Start with practical, small command-line tools
- Learn popular crates: `clap`, `anyhow`, `serde`, `tokio`, and others
- Progress from simple to more complex concepts
- Build real-world applicable skills

### Working Approach
- **Living Documentation**: Track progress in `ai_task/` with living documents that enable seamless resumption
- **Lean Development**: Ask clarifying questions before charting course
- **Adaptive Pacing**: Match learning pace and adapt to coding style
- **Practical Focus**: Create small, achievable tasks for hands-on coding

### Important Instructions for Claude Code

1. **Verify Information**: Use WebSearch and WebFetch to check Rust documentation, The Rust Book, and Rustonomicon to ensure accuracy
2. **Sub-Agent Delegation**: When needed, use `npx @anthropic-ai/claude-code --permission-mode=acceptEdits -p <PROMPT>` to delegate tasks for verification
3. **Progress Tracking**: Maintain a living plan document in `ai_task/rust_learning/status.md` following the rules in `ai_context/aa_main_rules.md`
4. **Code Quality**: Suggest improvements and best practices while teaching
5. **Flexibility**: The learning plan should evolve based on progress and interests

### User's Preferred Working Style
- User prefers to figure out implementation details independently
- User will describe their plan before implementing
- Assistant should provide guidance when asked specific questions
- Avoid overly detailed step-by-step instructions unless requested
- Focus on answering questions and providing targeted help



## Development Commands

### Building
- `cargo build` - Build the project in debug mode
- `cargo build --release` - Build with optimizations
- `cargo build -p <package>` - Build specific package (if workspace grows)

### Running
- `cargo run` - Run the main binary
- `cargo run --release` - Run with optimizations
- `cargo run -- <args>` - Run with command line arguments

### Testing
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run specific test by name
- `cargo test --release` - Run tests with optimizations
- `cargo test -- --nocapture` - Show println! output during tests

### Other Common Commands
- `cargo check` - Quick compile check without producing binaries
- `cargo clippy` - Run the Rust linter
- `cargo fmt` - Format code according to Rust style guidelines

## AI Context Structure

This project follows a specific structure for AI-assisted development:

- `ai_context/` - Contains AI-related documentation and rules
  - `aa_main_rules.md` - Main rules for AI context management
  - `howto_code_overview.md` - Guidelines for code documentation
- `ai_project/` - General project-related AI documentation (to be created as needed)
- `ai_task/` - Task-specific documentation (to be created as needed)

When working on tasks, always follow the rules in `ai_context/aa_main_rules.md` which emphasize:
- Creating living documents for task tracking
- Maintaining proper file organization
- Adding backlinks in code comments to relevant documentation
- Documenting findings progressively

## Current Architecture

Currently, the project has a minimal structure:
- Single main.rs file with a basic "Hello, world!" program
- No dependencies declared in Cargo.toml
- No tests or benchmarks yet implemented

## Current Task
Working on: [Word Count CLI Task](ai_task/rust_learning/wordcount_cli_task.md) - File I/O and error handling with anyhow

## Completed Tasks
- ✓ [Greeting CLI Task](ai_task/rust_learning/greeting_cli_task.md) - Simple CLI tool with clap

## Planned Tasks
- [Stack Inspector Task](ai_task/rust_learning/stack_inspector_task.md) - Low-level programming with naked functions (after wordcount)

## Project Evolution Strategy

As we progress through learning exercises:
1. **Module Organization**: Create separate modules for different learning topics
2. **Example Structure**: Each learning unit can be a separate example in `examples/`
3. **Testing Practice**: Write tests alongside each feature to learn testing patterns
4. **Documentation**: Practice writing good documentation with doc comments

## Learning Resources

When working on Rust concepts, always reference:
- [The Rust Book](https://doc.rust-lang.org/book/) - Primary learning resource
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Practical examples
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - Advanced unsafe Rust
- [Crates.io](https://crates.io/) - For exploring and understanding crates
- [docs.rs](https://docs.rs/) - For crate documentation

## Browser Tools Usage

With browser permissions enabled, you can now use browser tools to:
- Navigate to Rust documentation sites and verify information
- Search for crate examples and best practices
- View real-world Rust code implementations on GitHub
- Check latest Rust RFC discussions and proposals
- Verify compiler error explanations and solutions

Browser tools should be used to ensure accuracy and provide up-to-date information when WebSearch and WebFetch are insufficient.

## Trusted Rust Information Sources

### Official Documentation
1. **doc.rust-lang.org** - Official Rust documentation hub
   - `/book/` - The Rust Programming Language book
   - `/std/` - Standard library documentation
   - `/reference/` - The Rust Reference (language specification)
   - `/nomicon/` - The Rustonomicon (advanced unsafe Rust)
   - `/rust-by-example/` - Learn Rust with examples
   - `/cargo/` - Cargo documentation
   - `/edition-guide/` - Edition migration guides
   - `/error-index/` - Compiler error index

2. **blog.rust-lang.org** - Official Rust blog for announcements and updates

3. **forge.rust-lang.org** - Rust Forge (contributor documentation)

### Package Resources
4. **crates.io** - Official Rust package registry
   - Search for crates and their popularity
   - Check download statistics and dependencies
   - Review crate documentation links

5. **docs.rs** - Automatic documentation hosting for crates
   - View crate documentation
   - Check different versions
   - See feature flags and examples

6. **lib.rs** - Alternative crate discovery platform
   - Categorized crate listings
   - Quality metrics and comparisons

### Community Resources
7. **users.rust-lang.org** - Official Rust users forum
   - Beginner questions and answers
   - Best practices discussions
   - Code review requests

8. **internals.rust-lang.org** - Rust internals forum
   - Language design discussions
   - RFC pre-discussions
   - Compiler implementation details

9. **reddit.com/r/rust** - Rust subreddit (verify information)
   - Community discussions
   - Project showcases
   - News and updates

### Learning Platforms
10. **rustlings** (github.com/rust-lang/rustlings) - Small exercises to learn Rust

11. **exercism.org/tracks/rust** - Rust track with mentored exercises

12. **rust-learning** (github.com/ctjhoa/rust-learning) - Curated learning resources

### Code Examples & Best Practices
13. **github.com/rust-unofficial/awesome-rust** - Curated list of Rust resources

14. **github.com/rust-unofficial/patterns** - Rust design patterns

15. **rosetta.alhur.es** - Rosetta Code for Rust examples

### Advanced Topics
16. **smallcultfollowing.com/babysteps** - Niko Matsakis' blog (Rust team lead)

17. **os.phil-opp.com** - Writing an OS in Rust tutorial

18. **fasterthanli.me** - Amos' detailed Rust articles

### Security & Safety
19. **rustsec.org** - Rust security advisory database

20. **github.com/rust-secure-code/safety-dance** - Guide to unsafe Rust

### Performance
21. **github.com/flamegraph-rs/flamegraph** - Profiling Rust applications

22. **bheisler.github.io/criterion.rs** - Rust benchmarking documentation

### Async Rust
23. **tokio.rs** - Tokio async runtime documentation

24. **async.rs** - Async programming in Rust book

25. **github.com/async-rs/async-std** - Alternative async runtime

### WebAssembly
26. **rustwasm.github.io/book** - Rust and WebAssembly book

27. **yew.rs** - Yew framework documentation

### Embedded Rust
28. **docs.rust-embedded.org** - Embedded Rust documentation

29. **github.com/rust-embedded/awesome-embedded-rust** - Embedded resources

### Tools & Ecosystem
30. **rust-analyzer.github.io** - Official LSP implementation

31. **github.com/rust-lang/rustfmt** - Code formatting tool

32. **github.com/rust-lang/rust-clippy** - Linting tool

### Verification Priority
When searching for information, prioritize sources in this order:
1. Official Rust documentation (doc.rust-lang.org)
2. Official crate documentation (docs.rs)
3. Official Rust forums and blogs
4. Well-known community members' blogs
5. GitHub repositories with high stars/activity
6. General programming forums (with verification)
