.PHONY: dev release clean

dev:
	cargo build
	rust-objcopy -O binary target/aarch64-unknown-none/debug/pidp-1145 kernel8.img
	@ls -la kernel8.img

release:
	cargo build --release
	rust-objcopy -O binary target/aarch64-unknown-none/release/pidp-1145 kernel8.img
	@ls -la kernel8.img

clean:
	cargo clean
	rm -f kernel8.img
