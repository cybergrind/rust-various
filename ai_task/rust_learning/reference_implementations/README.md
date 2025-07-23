# Reference Implementations

This directory contains Claude's reference implementations for learning tasks. These are moved here so the learner can implement their own versions while having these as reference.

## Stack Inspector Task (Task 3)

### Files
- `naked_function_demo.rs` - Basic naked functions examples
- `stack_inspector.rs` - More comprehensive stack inspection using inline assembly

### Key Implementation Details

#### Naked Functions (naked_function_demo.rs)
- Uses `#[unsafe(naked)]` attribute (Rust 1.88.0+)
- Shows x86_64 calling convention handling
- Examples:
  - Simple constant return
  - Addition following ABI (args in EDI/ESI, return in EAX)
  - Stack pointer access (RSP register)
  - Return address access ([RSP])

#### Stack Inspector (stack_inspector.rs)
- Uses regular `asm!` macro instead of naked functions
- Demonstrates:
  - Reading RSP/RBP registers
  - Stack frame chain walking
  - Stack growth direction analysis
  - Local variable memory layout
  - CPU cycle counting with RDTSC

### Technical Notes
- x86_64 System V ABI: RDI, RSI, RDX, RCX, R8, R9 for first 6 args
- Stack grows downward on x86_64
- Frame pointer chain: [RBP] = previous RBP, [RBP+8] = return address
- RDTSC instruction reads timestamp counter into EDX:EAX

### Safety Considerations
- Naked functions bypass all Rust safety guarantees
- Must manually ensure ABI compliance
- Stack pointer manipulation requires extreme care
- All pointer dereferences must be validated

These implementations serve as working examples of low-level assembly integration in Rust.