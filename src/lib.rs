#![feature(lang_items)]
#![no_std]
#![no_builtins]
#![feature(ptr_internals)]

extern crate rlibc;
extern crate volatile;
extern crate spin;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn rust_main() -> !{
    *vga_buffer::WRITER.lock().color_mut() = vga_buffer::ColorCode::new(
        vga_buffer::Color::Green, vga_buffer::Color::Black
    );
    println!("Rust OK");

    loop{}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}