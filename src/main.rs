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
    perry_os::init();
    WRITER.lock().set_color(ColorCode::new(Green, Black));
    println!("All systems up and running!");

    #[cfg(test)]
    test_main();

    perry_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    perry_os::hlt_loop();
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