

all: run


gas: 
	as -W -o ./build/boot.o ./src/boot.S  
	#	-mlittle-endian

linker: gas 
	ld -o ./build/boot.out -Ttext 0x7c00 ./build/boot.o 

objcopy: linker 
	objcopy -O binary --only-section=.text ./build/boot.out ./build/boot.bin

qemu: objcopy
	qemu-system-x86_64 -drive format=raw,file=./build/boot.bin

run: qemu
