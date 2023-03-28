# C_SOURCES = $(wildcard src/kernel/*.c src/drivers/*.c)
# HEADERS = $(wildcard src/kernel/*.h src/drivers/*.h)
# OBJ = ${C_SOURCES:.c=.o}

arch ?= x86_64
kernel := bin/kernel-$(arch).bin
iso := bin/RobOS-$(arch).iso

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg

assembly_source_files := $(shell find . -type f -wholename './src/arch/$(arch)/*.asm')
assembly_object_files := $(patsubst ./src/arch/$(arch)/%.asm, ./bin/arch/$(arch)/%.o, $(assembly_source_files))
assembly_include_dir := ./src/arch/$(arch)/

rust_source_files := $(shell find . -type f -wholename './src/*.rs')

target ?= $(arch)-rob_os
rust_os := target/$(target)/debug/librob_os.a

.PHONY: all run iso clean kernel debug

all: iso

run: $(iso)
	qemu-system-x86_64 -cdrom $(iso)
debug: $(iso)
	qemu-system-x86_64 -s -S -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	mkdir -p bin/isofiles/boot/grub
	cp $(kernel) bin/isofiles/boot/kernel.bin
	cp $(grub_cfg) bin/isofiles/boot/grub
	grub-mkrescue -o $(iso) bin/isofiles 2> /dev/null
	rm -r bin/isofiles

$(kernel): kernel $(rust_os) $(assembly_object_files) $(linker_script)
	ld -g -n -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_os)

kernel: $(rust_source_files)
	RUST_TARGET_PATH=$(shell pwd) xargo build --target $(target)

# compile assembly files
bin/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	mkdir -p $(shell dirname $@)
	nasm -g -felf64 -I $(assembly_include_dir) $< -o $@

clean:
	rm -f $(assembly_object_files) $(iso) $(kernel)
	xargo clean