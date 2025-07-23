# Word Count CLI Task

## Task Overview
Create a line/word/character counter tool using `clap` for CLI parsing and `anyhow` for error handling.

## Learning Goals
- Practice file I/O operations in Rust
- Learn error handling with `anyhow` crate
- Work with iterators and string processing
- Build more complex CLI with multiple flags
- Create multiple functions with clear separation of concerns
- Introduction to writing unit tests

## Requirements
1. Count lines, words, and/or characters in files
2. Support multiple files: `wordcount file1.txt file2.txt`
3. Flags to control what to count:
   - `--lines` or `-l` to show line count
   - `--words` or `-w` to show word count
   - `--chars` or `-c` to show character count
   - Default behavior (no flags) shows all three
4. Handle errors gracefully using `anyhow`:
   - File not found
   - Permission denied
   - Invalid UTF-8
   - Directories instead of files
5. Show totals when processing multiple files

## Example Usage
```bash
# Count everything in one file
./wordcount README.md

# Count only lines in multiple files
./wordcount --lines src/*.rs

# Count words and lines
./wordcount -w -l file1.txt file2.txt
```

## Technical Requirements
- Use `anyhow` for error handling and context
- Separate counting logic into its own function(s)
- Use `std::fs` for file operations
- Consider using `anyhow::Context` trait for adding error context

## Progress
- [x] Add clap and anyhow dependencies
- [x] Create basic CLI structure with file arguments
- [x] Set up binary target in Cargo.toml
- [x] Define Args struct with all flags (lines, words, chars)
- [x] Support multiple files via Vec<String>
- [x] Implement file reading with error handling
- [x] Add counting logic (lines, words, chars)
- [x] Handle case where no flags provided (show all)
- [x] Handle multiple files with totals
- [x] Add proper error context with anyhow
- [x] Write unit tests for counting functions
- [x] Write integration tests for CLI
- [x] Improve output formatting with aligned columns

## Current Status
- Task completed successfully!
- All requirements implemented and tested
- Output formatting improved with {:>7} and {:>8} alignment
- Tests passing for both unit and integration tests

## Implementation Details
- Binary target: `src/bin/wordcount.rs`
- Using clap derive for CLI parsing
- Files stored as Vec<String> for multiple file support
- Three boolean flags for count types

## Final Implementation Notes
- Used BufReader for efficient file processing
- Single-pass algorithm for counting all metrics
- Error handling with partition_map from itertools
- fold_options for elegant Option<usize> summing
- Output alignment: {:>7} for lines, {:>8} for words/chars
- Only shows totals when processing multiple files

## Lessons Learned
- `Result<T, E>` and `?` operator for error propagation
- Iterator methods: partition_map, fold_options
- anyhow's Context trait for better error messages
- Integration testing with assert_cmd and predicates
- Importance of letting learner implement first, then iterate

## Task Completed ✓
Moving to next task: [Stack Inspector Task](stack_inspector_task.md) - Low-level programming with naked functions
