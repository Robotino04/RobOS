global long_mode_start

section .text
bits 64

%include "boot/vga_buffer.asm"

long_mode_start:
    ; load 0 into all data segment registers
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    
    ; print "ASM OK"
    mov rax, 0x0220026d02730261
    mov qword [VGA_POSITION(0, VGA_HEIGHT-2)], rax
    mov rax, 0x00000000024b024f
    mov qword [VGA_POSITION(4, VGA_HEIGHT-2)], rax

    extern rust_main
    call rust_main ; should not return

    ; print error
    mov dword [VGA_POSITION(0, 0)], 0x4f524f45
    mov dword [VGA_POSITION(2, 0)], 0x4f3a4f52
    mov dword [VGA_POSITION(4, 0)], 0x4f204f20
    mov byte  [VGA_POSITION(5, 0)], "3"
    hlt
