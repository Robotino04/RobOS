#![feature(lang_items)]
#![no_std]
#![no_builtins]
#![feature(ptr_internals)]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;

#[macro_use]
mod vga_buffer;
mod memory;

#[no_mangle]
pub extern fn rust_main(multiboot_address: usize) -> !{
    vga_buffer::WRITER.lock().set_background(vga_buffer::Color::Black);
    vga_buffer::WRITER.lock().set_foreground(vga_buffer::Color::Green);
    println!("Rust OK");

    let boot_info = unsafe {multiboot2::load(multiboot_address)}.expect("multiboot 2 info structure should be available");
    let memory_map_tag = boot_info.memory_map_tag().expect("memory map tag should be loader by GRUB");

    println!("Memory areas:");
    for area in memory_map_tag.memory_areas(){
        println!("    start: 0x{:x}, end: 0x{:x}, size: 0x{:x}",
            area.start_address(), area.end_address(), area.size());
    }

    let elf_sections_tag = boot_info.elf_sections_tag().expect("elf sections should be loaded by GRUB");
    println!("ELF Sections:");
    for section in elf_sections_tag.sections(){
        println!("start: 0x{:x}, end: 0x{:x}, size: 0x{:x}",
            section.start_address(), section.end_address(), section.size());
    }

    let kernel_start = elf_sections_tag.sections().map(|s| s.start_address())
        .min().unwrap();
    
    let kernel_end = elf_sections_tag.sections().map(|s| s.end_address())
        .max().unwrap();

    println!("Kernel: start: 0x{:x}, end: 0x{:x}, size: 0x{:x}", kernel_start, kernel_end, kernel_end - kernel_start);
    println!("Multiboot: start: 0x{:x}, end: 0x{:x}, size: 0x{:x}", boot_info.start_address(), boot_info.end_address(), boot_info.end_address() - boot_info.start_address());

    let mut frame_allocator = memory::AreaFrameAllocator::new(
        memory_map_tag.memory_areas(),
        kernel_start as usize, kernel_end as usize,
        boot_info.start_address(), boot_info.end_address() as usize);

    for i in 0..180 {
        match frame_allocator.allocate_frame(){
            Some(_) => { println!("Allocated frame {}", i); }
            None => { println!("Frame already used"); }
        };
    }

    loop{}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

use core::panic::PanicInfo;

use crate::memory::FrameAllocator;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    vga_buffer::WRITER.lock().set_foreground(vga_buffer::Color::Red);
    print!("{}", info);
    vga_buffer::WRITER.lock().set_foreground(vga_buffer::Color::Green);
    loop {}
}