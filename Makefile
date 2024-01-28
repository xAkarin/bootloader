

all: run


gas: 
	as -W -o ./build/boot.o ./src/boot.S  
	#	-mlittle-endian

linker: gas 
	ld -o ./build/boot.out -T linker.ld ./build/boot.o 

objcopy: linker 
	objcopy -O binary --only-section=.text ./build/boot.out ./build/boot.bin

qemu: objcopy
	qemu-system-x86_64 -drive format=raw,file=./build/boot.bin

qemu_debug: objcopy 
	qemu-system-x86_64 -s -S -drive format=raw,file=./build/boot.bin

run: qemu

debug: qemu_debug
