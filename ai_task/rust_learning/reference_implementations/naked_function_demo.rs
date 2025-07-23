use std::arch::naked_asm;

// Example 1: Simple naked function that returns a constant
#[unsafe(naked)]
unsafe extern "C" fn return_42() -> i32 {
    naked_asm!(
        "mov eax, 42",
        "ret",
    )
}

// Example 2: Naked function that adds two numbers
#[unsafe(naked)]
unsafe extern "C" fn add_two_numbers(a: i32, b: i32) -> i32 {
    // On x86_64 System V ABI:
    // First argument (a) is in EDI
    // Second argument (b) is in ESI
    // Return value goes in EAX
    naked_asm!(
        "mov eax, edi",    // Move first argument to eax
        "add eax, esi",    // Add second argument
        "ret",             // Return
    )
}

// Example 3: Get current stack pointer
#[unsafe(naked)]
unsafe extern "C" fn get_stack_pointer() -> *const u8 {
    // On x86_64, RSP holds the stack pointer
    naked_asm!(
        "mov rax, rsp",    // Move stack pointer to return register
        "ret",
    )
}

// Example 4: Simple stack frame inspection
#[unsafe(naked)]
unsafe extern "C" fn get_return_address() -> *const u8 {
    // The return address is at [rsp] when we enter the function
    naked_asm!(
        "mov rax, [rsp]",  // Load return address from stack
        "ret",
    )
}

// Helper function to print a pointer value
fn print_ptr(name: &str, ptr: *const u8) {
    println!("{}: {:p}", name, ptr);
}

fn main() {
    println!("=== Naked Functions Demo ===\n");
    
    // Example 1: Call the simple return function
    unsafe {
        let result = return_42();
        println!("return_42() = {}", result);
    }
    
    // Example 2: Call the add function
    unsafe {
        let sum = add_two_numbers(10, 32);
        println!("add_two_numbers(10, 32) = {}", sum);
    }
    
    // Example 3: Get stack pointer
    unsafe {
        let sp = get_stack_pointer();
        print_ptr("Current stack pointer", sp);
    }
    
    // Example 4: Get return address
    unsafe {
        let ret_addr = get_return_address();
        print_ptr("Return address", ret_addr);
    }
    
    // Demonstrate stack growth direction
    println!("\n=== Stack Growth Direction ===");
    let var1 = 42;
    let var2 = 84;
    println!("Address of var1: {:p}", &var1);
    println!("Address of var2: {:p}", &var2);
    
    unsafe {
        let sp_before = get_stack_pointer();
        demo_function_call();
        let sp_after = get_stack_pointer();
        
        print_ptr("Stack pointer before call", sp_before);
        print_ptr("Stack pointer after call", sp_after);
        
        if sp_before > sp_after {
            println!("Stack grows downward (typical for x86_64)");
        } else {
            println!("Stack grows upward");
        }
    }
}

fn demo_function_call() {
    // Just a function to demonstrate stack pointer changes
    let _local = 123;
}