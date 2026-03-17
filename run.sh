#!/usr/bin/env bash
set -eu
cd kernel
cargo build
cd ..
echo "[+] Starting QEMU..."
exec , qemu-system-riscv64 \
  -M virt \
  -serial mon:stdio \
  -nographic \
  -bios none \
  -kernel target/riscv64gc-unknown-none-elf/debug/rust-kernel \
  "$@"
