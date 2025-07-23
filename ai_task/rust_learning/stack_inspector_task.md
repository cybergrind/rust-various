# Stack Inspector Task

## Task Overview
Create a simple stack inspection tool using Rust's naked functions feature to demonstrate low-level stack manipulation and assembly programming.

## Learning Goals
- Understand and use naked functions (`#[unsafe(naked)]`)
- Work with inline assembly via `naked_asm!`
- Learn about calling conventions and ABIs
- Practice unsafe Rust in a controlled environment
- Understand stack frames and function call mechanics
- Work with raw pointers safely

## Requirements
1. **Stack Frame Walker**
   - Capture current stack pointer using naked function
   - Walk up the stack frames
   - Display return addresses in hexadecimal
   - Show approximate stack depth

2. **Simple Profiling Helper**
   - Implement cycle counter reading via naked function
   - Create checkpoints with minimal overhead
   - Measure cycles between checkpoints
   - Display timing report

3. **Basic Exception Handler** (x86_64 Linux)
   - Set up signal handler using naked functions
   - Catch SIGSEGV (segmentation fault)
   - Print stack trace when fault occurs
   - Safely exit after handling

## Technical Requirements
- Use `#[unsafe(naked)]` attribute (requires Rust 1.88+)
- Use `core::arch::naked_asm!` for assembly
- Target x86_64 initially (can add ARM64 later)
- Handle platform differences gracefully
- Document safety requirements clearly

## Example Usage
```bash
# Run stack inspector
./stack_inspector

# Output might show:
Stack Frame Walker:
  Frame 0: 0x7fff12345678
  Frame 1: 0x7fff12345690
  Frame 2: 0x7fff123456a8
  Stack depth: ~3 frames

Profiler:
  Checkpoint A to B: 1234 cycles
  Checkpoint B to C: 5678 cycles

# Test exception handler
./stack_inspector --test-crash
```

## Safety Considerations
- All naked functions must be marked `unsafe`
- Ensure ABI consistency with function signatures
- Validate all pointer dereferences
- Use proper compiler barriers where needed
- Test on single platform first before generalizing

## Progress
- [x] Set up project with edition 2024
- [x] Implement basic naked function example
- [x] Create stack pointer capture function
- [x] Implement frame walker logic
- [x] Add cycle counter reading
- [x] Create profiling checkpoint system
- [ ] Implement signal handler setup
- [ ] Add exception handling logic
- [x] Write comprehensive safety documentation
- [ ] Add tests (where possible with unsafe code)

## Current Status
Task partially completed! Created two demonstration programs:

1. **naked_function_demo.rs** - Shows naked function basics:
   - Simple constant return
   - Function with arguments following x86_64 ABI
   - Stack pointer and return address access
   
2. **stack_inspector.rs** - Stack inspection without naked functions:
   - Stack/frame pointer access via inline assembly
   - Recursive stack growth demonstration
   - Local variable layout analysis
   - CPU cycle counting with RDTSC

## Key Learnings
- Naked functions stabilized in Rust 1.88.0
- Must use `#[unsafe(naked)]` attribute syntax
- Function body must be single `naked_asm!` invocation
- No compiler-generated prologue/epilogue
- Manual ABI compliance required
- Stack grows downward on x86_64
- Can measure function overhead with RDTSC

## Implementation Challenges
- Initial syntax confusion (attribute changed in 1.88)
- Naked functions require exact ABI compliance
- Limited debugging capability in naked functions
- Signal handling more complex than anticipated

## Files Created (Reference Implementations)
- `ai_task/rust_learning/reference_implementations/naked_function_demo.rs` - Naked function examples
- `ai_task/rust_learning/reference_implementations/stack_inspector.rs` - Stack inspection demo
- Note: Moved to reference directory so learner can implement their own version

## Implementation Notes
- Start with simple assembly (just reading registers)
- Use `#[cfg(target_arch = "x86_64")]` for platform-specific code
- Consider using `libc` crate for signal handling
- May need to disable optimizations for some functions
- Reference compiler-builtins for naked function examples

## Learning Resources
- [Rust Blog: Stabilizing Naked Functions](https://blog.rust-lang.org/2025/07/03/stabilizing-naked-functions/)
- [RFC 2972: Constrained Naked Functions](https://rust-lang.github.io/rfcs/2972-constrained-naked.html)
- x86_64 calling convention documentation
- Linux signal handling documentation

## Notes
- This is an advanced task exploring low-level systems programming
- Requires understanding of assembly and CPU architecture
- Good preparation for OS development or embedded programming
- Demonstrates Rust's capability for systems programming