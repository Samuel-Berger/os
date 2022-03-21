#![no_std]  // Don't use the standard library.
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

/*  We are currently operating before main is used: crt0 -> start -> main
fn main() {
//    println!("Hello, world!");
}
*/

static HELLO: &[u8] = b"Hello World!";

/*
// Instead use our own entry point.
#[no_mangle]    // Use _start and not a random name.
pub extern "C" fn _start() -> !{

    //Location of the buffer cast as a raw pointer.
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {    // Since the raw buffer might not be valid.
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;   //0xb is light cyan
        }
    }

    loop {}
}
*/

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {  // PanicInfo contains file and line for the panic.
    loop{}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::test_print();

    loop {}
}
