// Stack Inspector - Learning about stack frames and assembly
// Task 3 from the Rust learning plan

use std::arch::asm;
use std::mem;

// Get the current stack pointer value
fn get_stack_pointer() -> usize {
    let sp: usize;
    unsafe {
        asm!(
            "mov {}, rsp",
            out(reg) sp,
            options(nomem, nostack, preserves_flags)
        );
    }
    sp
}

// Get the current base pointer (frame pointer) value
fn get_frame_pointer() -> usize {
    let bp: usize;
    unsafe {
        asm!(
            "mov {}, rbp",
            out(reg) bp,
            options(nomem, nostack, preserves_flags)
        );
    }
    bp
}

// A simple function to demonstrate stack frame creation
#[inline(never)] // Prevent inlining to ensure a stack frame is created
fn demo_function(x: i32, y: i32) -> i32 {
    let local1 = x + y;
    let local2 = x * y;
    println!("  In demo_function:");
    println!("    Parameters: x={}, y={}", x, y);
    println!("    Locals: local1={}, local2={}", local1, local2);
    println!("    Stack pointer: 0x{:016x}", get_stack_pointer());
    println!("    Frame pointer: 0x{:016x}", get_frame_pointer());
    
    // Inspect stack contents near the frame pointer
    unsafe {
        let bp = get_frame_pointer() as *const usize;
        println!("    Previous frame pointer: 0x{:016x}", *bp);
        println!("    Return address: 0x{:016x}", *bp.offset(1));
    }
    
    local1 + local2
}

// Recursive function to demonstrate stack growth
#[inline(never)]
fn recursive_function(depth: u32, max_depth: u32) {
    let sp = get_stack_pointer();
    println!("  Depth {}: SP = 0x{:016x}", depth, sp);
    
    if depth < max_depth {
        recursive_function(depth + 1, max_depth);
    }
}

// Function to inspect local variable layout
fn inspect_locals() {
    let var1: i32 = 0x11111111;
    let var2: i32 = 0x22222222;
    let var3: i64 = 0x3333333333333333;
    let array: [u8; 8] = [0xAA; 8];
    
    println!("\n=== Local Variable Layout ===");
    println!("  var1 (i32):  addr = {:p}, value = 0x{:08x}", &var1, var1);
    println!("  var2 (i32):  addr = {:p}, value = 0x{:08x}", &var2, var2);
    println!("  var3 (i64):  addr = {:p}, value = 0x{:016x}", &var3, var3);
    println!("  array [u8;8]: addr = {:p}", &array);
    
    // Calculate distances between variables
    let addr1 = &var1 as *const _ as usize;
    let addr2 = &var2 as *const _ as usize;
    let addr3 = &var3 as *const _ as usize;
    let addr_array = &array as *const _ as usize;
    
    println!("\n  Variable spacing:");
    println!("    var2 - var1 = {} bytes", addr2.abs_diff(addr1));
    println!("    var3 - var2 = {} bytes", addr3.abs_diff(addr2));
    println!("    array - var3 = {} bytes", addr_array.abs_diff(addr3));
}

// Simple cycle counter using inline assembly
fn rdtsc() -> u64 {
    let lo: u32;
    let hi: u32;
    unsafe {
        asm!(
            "rdtsc",
            out("eax") lo,
            out("edx") hi,
            options(nomem, nostack, preserves_flags)
        );
    }
    ((hi as u64) << 32) | (lo as u64)
}

fn measure_function_overhead() {
    println!("\n=== Measuring Function Call Overhead ===");
    
    // Warm up (reduced to avoid excessive output)
    for _ in 0..5 {
        let _ = demo_function(1, 2);
    }
    
    // Measure
    let start = rdtsc();
    let result = demo_function(42, 17);
    let end = rdtsc();
    
    println!("  Function returned: {}", result);
    println!("  Cycles elapsed: {}", end - start);
}

fn main() {
    println!("=== Stack Inspector Demo ===");
    println!("  Main function:");
    println!("    Stack pointer: 0x{:016x}", get_stack_pointer());
    println!("    Frame pointer: 0x{:016x}", get_frame_pointer());
    
    println!("\n=== Calling demo_function ===");
    let sp_before = get_stack_pointer();
    let result = demo_function(10, 20);
    let sp_after = get_stack_pointer();
    
    println!("\n  Back in main:");
    println!("    Result: {}", result);
    println!("    Stack pointer before: 0x{:016x}", sp_before);
    println!("    Stack pointer after:  0x{:016x}", sp_after);
    println!("    Stack pointer difference: {} bytes", sp_before - sp_after);
    
    println!("\n=== Stack Growth Direction ===");
    println!("  Recursive calls to depth 5:");
    recursive_function(0, 5);
    
    inspect_locals();
    
    // Only run cycle counter on x86_64
    #[cfg(target_arch = "x86_64")]
    measure_function_overhead();
    
    println!("\n=== Stack Memory Layout ===");
    println!("  Size of usize: {} bytes", mem::size_of::<usize>());
    println!("  Size of pointer: {} bytes", mem::size_of::<*const u8>());
    println!("  Stack grows: {}", if cfg!(target_arch = "x86_64") { 
        "downward (toward lower addresses)" 
    } else { 
        "architecture dependent" 
    });
}