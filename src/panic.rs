use crate::println;

#[allow(unused)]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn handle_panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
