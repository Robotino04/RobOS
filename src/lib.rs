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
pub extern fn rust_main(multiboot_address: usize) -> !{
    vga_buffer::WRITER.lock().set_background(vga_buffer::Color::Black);
    vga_buffer::WRITER.lock().set_foreground(vga_buffer::Color::Green);
    println!("Rust OK");

    loop{}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    vga_buffer::WRITER.lock().set_foreground(vga_buffer::Color::Red);
    print!("{}", info);
    vga_buffer::WRITER.lock().set_foreground(vga_buffer::Color::Green);
    loop {}
}