#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#[allow(dead_code)]

mod vga_buffer;
mod serial;

use crate::vga_buffer::Color::*;
use crate::vga_buffer::WRITER;
use core::ops::Deref;
use core::panic::PanicInfo;
use vga_buffer::ColorCode;


#[allow(undefined_naked_function_abi)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    WRITER.deref().lock().set_color(ColorCode::new(Red, Black));
    println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
    println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
    println!("::'########::'########:'########::'########::'##:::'##::::::'#######:::'######::");
    println!("::##.... ##: ##.....:: ##.... ##: ##.... ##:. ##:'##::::::'##.... ##:'##... ##::");
    println!("::##:::: ##: ##::::::: ##:::: ##: ##:::: ##::. ####::::::: ##:::: ##: ##:::..:::");
    println!("::########:: ######::: ########:: ########::::. ##:::::::: ##:::: ##:. ######:::");
    println!("::##.....::: ##...:::: ##.. ##::: ##.. ##:::::: ##:::::::: ##:::: ##::..... ##::");
    println!("::##:::::::: ##::::::: ##::. ##:: ##::. ##::::: ##:::::::: ##:::: ##:'##::: ##::");
    println!("::##:::::::: ########: ##:::. ##: ##:::. ##:::: ##::::::::. #######::. ######:::");
    println!("::..:::::::::........::..:::::..::..:::::..:::::..::::::::::.......::::......:::");
    println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
    println!("\n\n\n\n\n");
    WRITER.deref().lock().set_color(ColorCode::new(Green, Black));
    println!("All systems up and running!");

    #[cfg(test)]
    test_main();

    loop {}
}




const ISA_DEBUG_EXIT_PORT: u16 = 0xf4;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(ISA_DEBUG_EXIT_PORT);
        port.write(exit_code as u32);
    }
}
