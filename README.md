# Rust UEFI Bootloader

This is an uefi bootloader that is being worked on as a pet project to develop a better understanding of OS development.

#### Build Process

The script emulation.sh compiles the bootloader. Then sets up a mock directory structure with the compiled bootloader.
Finally, it starts QEMU with OVMF to launch and test the bootloader. In order to properly run this script you need to
have OVMF installed on your system. Currently, the script is set up to work while running on arch linux distributions.
