use core::arch::naked_asm;

#[cfg(target_arch = "aarch64")]
#[unsafe(naked)]
unsafe extern "C" fn get_stack_pointer() -> *const u8 {
    naked_asm!("mov x0, sp", "ret");
}

#[cfg(target_arch = "x86_64")]
#[unsafe(naked)]
unsafe extern "C" fn get_stack_pointer() -> *const u8 {
    naked_asm!("mov rax, rsp", "ret");
}

fn main() {
    let stack_pointer: *const u8;
    unsafe {
        stack_pointer = get_stack_pointer();
    }
    println!("Current stack pointer: {:?}", stack_pointer);

    println!("exit 0");
}
