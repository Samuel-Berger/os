#![no_std]  // Don't use the standard library.
#![no_main]

use core::panic::PanicInfo;

/*  We are currently operating before main is used: crt0 -> start -> main
fn main() {
//    println!("Hello, world!");
}
*/

// Instead use our own entry point.
#[no_mangle]    // Use _start and not a random name.
pub extern "C" fn _start() -> !{
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {  // PanicInfo contains file and line for the panic.
    loop{}
}

