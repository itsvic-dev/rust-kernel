TARGET := riscv64gc-unknown-none-elf
KERNEL := $(PWD)/target/$(TARGET)/debug/rust-kernel
KERNEL_RUSTFLAGS := -C link-arg=-Tkernel/src/link.ld

.PHONY: all
all: $(KERNEL)

$(KERNEL):
	@echo "   CARGO $(KERNEL)"
	@RUSTFLAGS="$(KERNEL_RUSTFLAGS)" cargo build --target $(TARGET) -p rust-kernel

.PHONY: clean
clean:
	@rm -r target

-include $(KERNEL).d
