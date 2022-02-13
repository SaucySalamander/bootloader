#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use alloc::string::ToString;
use uefi::data_types::chars::{Char16, Handle, Status};
use uefi::data_types::strings::CString16;
use uefi::tables::system_table::SystemTable;

#[no_mangle]
unsafe extern "efiapi" fn efi_main(_image_handle: Handle, mut system_table: SystemTable) -> Status {

    let vendor = system_table.firmware_vendor;
    system_table.stdout().clear_screen();
    system_table.stdout().output_char(&(*vendor));

    let test_string = "Hello World!".to_string();

    system_table.stdout().output_string(&CString16::try_from(test_string).unwrap());

    loop{}
    // Status::SUCCESS
}

#[panic_handler]
fn panic_impl(_info: &core::panic::PanicInfo) -> ! {
    Status::FAILURE;
    loop {}
}
