#![no_std]
#![no_main]
mod vga_buffer;
use core::{panic::PanicInfo};
// static HELLO:&[u8] = b"Hello World";
#[panic_handler]
fn panic(info:&PanicInfo)->!{
    println!("{}",info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start()-> !{
    // vga_buffer::print_something();
    // vga_buffer::WRITER.lock().write_str("hello again").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    // println!("Hello World{}","!");
    panic!("Some panic message");
    // loop {}
}




