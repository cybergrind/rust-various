# Stack Inspector Task

## Task Overview
Create a simple stack inspection tool using Rust's naked functions feature to demonstrate low-level stack manipulation and assembly programming. **This will be implemented as a separate binary with cross-platform support for Linux and macOS (ARM64).**

## Learning Goals
- Understand and use naked functions (`#[unsafe(naked)]`)
- Work with inline assembly via `naked_asm!`
- Learn about calling conventions and ABIs (x86_64 and ARM64)
- Practice unsafe Rust in a controlled environment
- Understand stack frames and function call mechanics
- Work with raw pointers safely
- **Master cross-platform assembly (Linux/macOS on ARM64)**

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
- **Target both x86_64 and ARM64 architectures**
- **Support Linux and macOS (focus on ARM64 for macOS)**
- Handle platform differences gracefully with `cfg` attributes
- Document safety requirements clearly
- Implement as a separate binary in `src/bin/stackinspector.rs`

## Example Usage
```bash
# Run stack inspector
./stackinspector

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
./stackinspector --test-crash
```

## Safety Considerations
- All naked functions must be marked `unsafe`
- Ensure ABI consistency with function signatures
- Validate all pointer dereferences
- Use proper compiler barriers where needed
- Test on single platform first before generalizing

## Progress (Fresh Implementation)
- [ ] Create new binary `src/bin/stackinspector.rs`
- [ ] Implement naked function for stack pointer capture (x86_64)
- [ ] Implement naked function for stack pointer capture (ARM64)
- [ ] Create cross-platform frame walker logic
- [ ] Add cycle counter reading (x86_64 with RDTSC)
- [ ] Add cycle counter reading (ARM64 with system registers)
- [ ] Create profiling checkpoint system
- [ ] Implement signal handler setup (if feasible)
- [ ] Add exception handling logic
- [ ] Write comprehensive safety documentation
- [ ] Add tests (where possible with unsafe code)

## Current Status
**Resuming April 2026 after ~9 month pause.** Dev machine is x86_64 Linux (Rust nightly 1.96.0, edition 2024).

### Done so far
- x86_64 `get_stack_pointer()` (reads `rbp`) and aarch64 version (reads `sp`) — both naked functions
- Frame walker using `rbp` chain: `[rbp]` = prev frame pointer, `[rbp+8]` = return address
- Heuristic pointer validation (null, alignment, direction, stack size bounds)
- Recursive `loop_before_walk()` to test deep stacks
- Discovered: `rbp` walking requires `-C force-frame-pointers=yes` (without it, `rbp` is used as a general-purpose register)
- Discovered: `#[inline(never)]` prevents inlining but not tail call optimization in release mode; `std::hint::black_box` after recursive call prevents TCO

### Next: Universal stack walking via DWARF `.eh_frame`
The `rbp`-based walker only works with frame pointers enabled. A universal approach uses `.eh_frame` (DWARF unwind info), which is always present in the binary regardless of compiler flags. Plan:
- Use `gimli` crate for `.eh_frame` parsing (CIE/FDE records, CFA opcodes)
- Only need `rsp` and `rip` from naked functions — no `rbp` dependency
- Keep existing `rbp`-based walker available via a clap flag (e.g., `--mode rbp` vs `--mode dwarf`)
- This also serves as a learning exercise for the `gimli` crate and DWARF format

### Reference implementations
Available in `reference_implementations/`:
1. **naked_function_demo.rs** - Shows naked function basics (x86_64 only)
2. **stack_inspector.rs** - Stack inspection without naked functions

### Implementation approach
- `src/bin/stackinspector.rs` with clap CLI
- Support both x86_64 and ARM64 architectures via `cfg` attributes
- Use proper naked functions with `#[unsafe(naked)]`

## Key Learnings
- Naked functions stabilized in Rust 1.88.0
- Must use `#[unsafe(naked)]` attribute syntax
- Function body must be single `naked_asm!` invocation
- No compiler-generated prologue/epilogue
- Manual ABI compliance required
- Stack grows downward on x86_64
- Can measure function overhead with RDTSC
- Without `-C force-frame-pointers=yes`, compiler uses `rbp` as a general-purpose register — `rbp` chain walking breaks
- `.eh_frame` (DWARF unwind info) is always present in the binary, regardless of frame pointer settings
- `#[inline(never)]` prevents inlining but NOT tail call optimization; use `std::hint::black_box` after recursive calls to prevent TCO
- Debug mode doesn't inline/TCO so `#[inline(never)]` is redundant there — only matters for release builds

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
- Use `#[cfg(target_arch = "x86_64")]` and `#[cfg(target_arch = "aarch64")]` for architecture-specific code
- Use `#[cfg(target_os = "linux")]` and `#[cfg(target_os = "macos")]` for OS-specific code
- ARM64 calling convention: x29 (FP), x30 (LR), SP for stack
- x86_64 calling convention: RBP (FP), RSP (stack), return address on stack
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
