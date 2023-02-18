# C_SOURCES = $(wildcard src/kernel/*.c src/drivers/*.c)
# HEADERS = $(wildcard src/kernel/*.h src/drivers/*.h)
# OBJ = ${C_SOURCES:.c=.o}

arch := x86_64
kernel := bin/kernel-$(arch).bin
iso := bin/RobOS-$(arch).iso

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg

assembly_source_files := $(shell find . -type f -wholename './src/arch/$(arch)/*.asm')
assembly_object_files := $(patsubst ./src/arch/$(arch)/%.asm, ./bin/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all run iso clean

all: iso

run: $(iso)
	qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	mkdir -p bin/isofiles/boot/grub
	cp $(kernel) bin/isofiles/boot/kernel.bin
	cp $(grub_cfg) bin/isofiles/boot/grub
	grub-mkrescue -o $(iso) bin/isofiles 2> /dev/null
	rm -r bin/isofiles

$(kernel): $(assembly_object_files) $(linker_script)
	ld -n -T $(linker_script) -o $(kernel) $(assembly_object_files)

# compile assembly files
bin/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	mkdir -p $(shell dirname $@)
	nasm -felf64 $< -o $@

clean :
	rm -f $(assembly_object_files) $(iso) $(kernel)