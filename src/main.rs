#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use crate::data_types::{Handle, Status};
use crate::table::system_table::SystemTable;

mod table;
mod protocols;
mod data_types;

#[no_mangle]
unsafe extern "efiapi" fn efi_main(_image_handle: Handle, mut system_table: SystemTable) -> Status {

    let mut vendor = system_table.firmware_vendor;
    system_table.stdout().clear_screen();
    system_table.stdout().output_string(&*vendor);
    loop{}
    // Status::SUCCESS
}

#[panic_handler]
fn panic_impl(_info: &core::panic::PanicInfo) -> ! {
    Status::FAILURE;
    loop {}
}
