
run:
	cargo bootimage
	qemu-system-x86_64 -drive format=raw,file=target/rcasey/debug/bootimage-rcaseyos.bin