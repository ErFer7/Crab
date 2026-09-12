.PHONY: qemu-riscv64-virt
qemu-riscv64-virt:
	cargo +nightly build --target ./platforms/qemu_riscv64_virt/qemu-riscv64-virt.json --features qemu-riscv64-virt

.PHONY: longan-nano
longan-nano:
	cargo +nightly build --target ./platforms/longan-nano.json --features board-longan-nano

.PHONY: run
run:
	qemu-system-riscv64 -machine virt -cpu rv64 -m 2G -nographic -bios none -kernel target/qemu-riscv64-virt/debug/crab

.PHONY: debug
debug:
	qemu-system-riscv64 -machine virt -cpu rv64 -m 2G -nographic -bios none -kernel target/qemu-riscv64-virt/debug/crab -s -S & kitty -e gdb target/qemu-riscv64-virt/debug/crab -ex "target remote :1234" -ex "set architecture riscv:rv64" -ex "b *0x80000000" -ex "c"

.PHONY: clean
clean:
	cargo clean
