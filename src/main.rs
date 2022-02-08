#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use uefi::data_types::char::{Handle, Status};
use uefi::tables::system_table::SystemTable;

#[no_mangle]
unsafe extern "efiapi" fn efi_main(_image_handle: Handle, mut system_table: SystemTable) -> Status {

    let vendor = system_table.firmware_vendor;
    system_table.stdout().clear_screen();
    system_table.stdout().output_string(&(*vendor));
    loop{}
    // Status::SUCCESS
}

#[panic_handler]
fn panic_impl(_info: &core::panic::PanicInfo) -> ! {
    Status::FAILURE;
    loop {}
}
