use core::arch::naked_asm;

#[cfg(target_arch = "x86_64")]
const STACK_SIZE: u64 = 8388608;
const MAX_STACK: u64 = 200;

#[cfg(target_arch = "aarch64")]
#[unsafe(naked)]
unsafe extern "C" fn get_stack_pointer() -> *const u64 {
    naked_asm!("mov x0, sp", "ret");
}

#[cfg(target_arch = "x86_64")]
#[unsafe(naked)]
unsafe extern "C" fn get_stack_pointer() -> *const u64 {
    naked_asm!("mov rax, rbp", "ret");
}


fn is_valid_candidate(candidate: *const u64, prev_frame: *const u64) -> bool {
    if prev_frame >= candidate {
        println!("Invalid candidate: candidate ({:?}) is not above previous frame ({:?})", candidate, prev_frame);
        return false;
    }

    if candidate as u64 > (prev_frame as u64 + STACK_SIZE) {
        println!("Invalid candidate: candidate ({:?}) is too far from previous frame ({:?})", candidate, prev_frame);
        return false;
    }

    if candidate as u64 % 8 != 0 {
        println!("Invalid candidate: candidate ({:?}) is not 8-byte aligned", candidate);
        return false;
    }

    true
}
// returns previous frame address and previous return address
#[cfg(target_arch = "x86_64")]
unsafe fn get_previous_frame_info(current_frame: *const u64, previous_frame: *const u64) ->  Option<(*const u64, *const u64)> {
    if current_frame.is_null() {
        return None;
    }
    if previous_frame != std::ptr::null() && !is_valid_candidate(current_frame, previous_frame) {
        return None;
    }
    let prev_frame: *const u64;
    let return_address: *const u64;
    unsafe {
        prev_frame = *(current_frame as *const *const u64);
        return_address = *((current_frame as *const *const u64).add(1));
    }
    Some((prev_frame, return_address))
}

#[inline(never)]
fn walk_stack_frames() {
    let stack_pointer: *const u64;
    unsafe {
        stack_pointer = get_stack_pointer();
    }
    println!("Current stack pointer: {:?}", stack_pointer);

    let mut current_frame = stack_pointer;
    let mut previous_frame: *const u64 = std::ptr::null();

    println!("Stack trace:");
    for _ in 0..MAX_STACK {
        match unsafe { get_previous_frame_info(current_frame, previous_frame) } {
            Some((prev_frame, return_address)) => {
                println!("  Frame: {:?}, Return Address: {:?}", current_frame, return_address);
                previous_frame = current_frame;
                current_frame = prev_frame;
            }
            None => {
                println!("  Reached end of stack frames.");
                break;
            }
        }
    }
}

#[inline(never)]
fn loop_before_walk(iterations: u8) {
    if iterations == 0 {
        walk_stack_frames();
    } else {
        loop_before_walk(iterations - 1);
        std::hint::spin_loop(); // prevent tail call optimization
    }
}

fn main() {
    let stack_pointer: *const u64;
    unsafe {
        stack_pointer = get_stack_pointer();
    }
    println!("Current stack pointer: {:?}", stack_pointer);

    let mut current_frame = stack_pointer;
    let mut previous_frame: *const u64 = std::ptr::null();

    println!("Stack trace:");
    for _ in 0..MAX_STACK {
        match unsafe { get_previous_frame_info(current_frame, previous_frame) } {
            Some((prev_frame, return_address)) => {
                println!("  Frame: {:?}, Return Address: {:?}", current_frame, return_address);
                previous_frame = current_frame;
                current_frame = prev_frame;
            }
            None => {
                println!("  Reached end of stack frames.");
                break;
            }
        }
    }
    println!("Finished walk in main function\n\n");

    loop_before_walk(15);

    println!("exit 0");
}
