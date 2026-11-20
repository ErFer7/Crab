.PHONY: qemu-riscv64-virt
qemu-riscv64-virt:
	cargo +nightly build --target ./platforms/qemu_riscv64_virt/qemu-riscv64-virt.json --features qemu-riscv64-virt

.PHONY: qemu-riscv32-virt
qemu-riscv32-virt:
	cargo +nightly build --target ./platforms/qemu_riscv32_virt/qemu-riscv32-virt.json --features qemu-riscv32-virt

.PHONY: longan-nano
longan-nano:
	cargo +nightly build --target ./platforms/longan_nano/longan-nano.json --release --features longan-nano
	rust-objcopy -O binary ./target/longan-nano/release/crab ./bin/crab.bin

.PHONY: flash-longan-nano
flash-longan-nano:
	dfu-util -a 0 -s 0x08000000:leave -D ./bin/crab.bin

.PHONY: run
run-riscv64-virt:
	qemu-system-riscv64 -machine virt -cpu rv64 -m 2G -nographic -bios none -kernel target/qemu-riscv64-virt/debug/crab

.PHONY: run
run-riscv32-virt:
	qemu-system-riscv32 -machine virt -cpu rv32 -m 2G -nographic -bios none -kernel target/qemu-riscv32-virt/debug/crab

.PHONY: debug
debug-riscv64-virt:
	qemu-system-riscv64 -machine virt -cpu rv64 -m 2G -nographic -bios none -kernel target/qemu-riscv64-virt/debug/crab -s -S & kitty -e gdb target/qemu-riscv64-virt/debug/crab -ex "target remote :1234" -ex "set architecture riscv:rv64" -ex "b *0x80000000" -ex "c"

.PHONY: debug
debug-riscv32-virt:
	qemu-system-riscv32 -machine virt -cpu rv32 -m 2G -nographic -bios none -kernel target/qemu-riscv32-virt/debug/crab -s -S & kitty -e gdb target/qemu-riscv32-virt/debug/crab -ex "target remote :1234" -ex "set architecture riscv:rv32" -ex "b *0x80000000" -ex "c"

.PHONY: clean
clean:
	cargo clean
