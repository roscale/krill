all: boot.o utility_functions.o kernel
	i686-elf-gcc \
		-T linker.ld \
		-o krill.bin \
		-ffreestanding -O2 -nostdlib -lgcc \
		\
		src/boot.o \
		src/utility_functions.o \
		target/i686-krill/debug/libkrill.a

boot.o: src/boot.s
	i686-elf-as src/boot.s -o src/boot.o

utility_functions.o: src/utility_functions.s
	i686-elf-as src/utility_functions.s -o src/utility_functions.o

kernel:
	cargo build

.PHONY: clean
clean:
	rm -rf krill.bin src/boot.o src/utility_functions.o

run: all
	qemu-system-i386 -m 4G -kernel krill.bin -serial stdio

run_debug: all
	qemu-system-i386 -m 4G -kernel krill.bin -S -s -daemonize