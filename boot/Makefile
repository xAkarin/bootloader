all: run 

run: ./build/boot.bin qemu 

./build/boot.bin: ./src/boot.asm
	nasm -f bin -o ./build/boot.bin -I /boot/src/ ./src/boot.asm

qemu: ./build/boot.bin
	qemu-system-i386 ./build/boot.bin 

clean:
	rm -rf ./build