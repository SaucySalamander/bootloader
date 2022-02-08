#!/bin/bash

#cargo build
cargo clean
cargo +nightly build

mkdir -p ./target/x86_64-unknown-uefi/debug/esp/EFI/Boot/
cp ./target/x86_64-unknown-uefi/debug/bootloader.efi ./target/x86_64-unknown-uefi/debug/esp/EFI/Boot/BootX64.efi

qemu-system-x86_64 -enable-kvm -drive if=pflash,format=raw,unit=0,readonly=on,file=/usr/share/ovmf/x64/OVMF.fd -drive if=pflash,format=raw,unit=1,readonly=on,file=/usr/share/ovmf/x64/OVMF_VARS.fd -drive format=raw,file=fat:rw:target/x86_64-unknown-uefi/debug/esp/