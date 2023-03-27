# RobOS
A homemade OS.
It is written in rust just so I can learn the language (and maybe because I'm following [this series](https://os.phil-opp.com/edition-1/)).


# Error codes
| Error code    | Description |
|---------------|-------------|
| 0             | The kernel wasn't loaded by a multiboot2 compilant bootloader. |
| 1             | CPUID isn't supported. |
| 2             | Long mode isn't supported. |
| 3             | rust_main returned. |

# Building
1. install rustup from [rustup.rs](https://rustup.rs)
2. install rust nightly using `$ rustup toolchain install nightly --component rust-src clippy`
3. switch to rust nightly using `$ rustup default nightly`
4. install xargo using `$ cargo install xargo`
5. install gcc things using your package manager (make, gcc, ld, etc.)
6. install grub-mkrescue and xorriso  using your package manager
6. compile the kernel with `$ make all`

# Running
You can either burn the generated ISO to a disk/USB and run it on real hardware (may harm the hardware)
or you can install qemu-system-x86_64 and use `$ make run` ru run RobOS in a vm.

# Troubleshooting
If the generated ISO won't boot and is really small, you may need to install something like ubuntus `grub-pc-bin`.