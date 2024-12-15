#![no_std]
#![no_main]

mod panic;
mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    for i in 0..100 {
        println!("Hello, World! {}", i);
    }

    panic!("Some panic message");
}
