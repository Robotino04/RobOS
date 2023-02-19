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