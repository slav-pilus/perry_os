#![no_std]
#![no_main]
mod vga_buffer;

use crate::vga_buffer::Color::*;
use crate::vga_buffer::WRITER;
use core::ops::Deref;
use core::panic::PanicInfo;
use vga_buffer::ColorCode;

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

    loop {}
}
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
