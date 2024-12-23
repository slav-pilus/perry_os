#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(perry_os::test_runner)]

mod vga_buffer;
mod serial;

use crate::vga_buffer::Color::*;
use crate::vga_buffer::WRITER;
use core::panic::PanicInfo;
use vga_buffer::ColorCode;


#[allow(undefined_naked_function_abi)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    WRITER.lock().set_color(ColorCode::new(Red, Black));
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
    WRITER.lock().set_color(ColorCode::new(Green, Black));
    println!("All systems up and running!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    perry_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}