.PHONY: rv64
rv64:
	cargo +nightly build --target ./platforms/rv64.json

.PHONY: run
run:
	qemu-system-riscv64 -machine virt -cpu rv64 -m 128M -nographic -bios none -kernel target/rv64/debug/Crab

.PHONY: debug
debug:
	qemu-system-riscv64 -machine virt -cpu rv64 -m 128M -nographic -bios none -kernel target/rv64/debug/Crab -s -S & kitty -e gdb target/rv64/debug/Crab -ex "target remote :1234" -ex "set architecture riscv:rv64" -ex "b *0x80000000" -ex "c"
