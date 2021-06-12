all: boot.o kernel
	i686-elf-gcc \
		-T linker.ld \
		-o krill.bin \
		-ffreestanding -O2 -nostdlib -lgcc \
		\
		src/boot.o \
		target/i686-krill/debug/libkrill.a

boot.o: src/boot.s
	i686-elf-as src/boot.s -o src/boot.o

kernel:
	cargo build

.PHONY: clean
clean:
	rm -rf src/boot.o krill.bin

run: all
	qemu-system-i386 -kernel krill.bin -serial stdio

run_debug: all
	qemu-system-i386 -kernel krill.bin -S -s -daemonize