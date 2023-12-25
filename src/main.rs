#![no_main]
#![no_std]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(rk::test_runner)]

use core::panic::PanicInfo;
use rk::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello rk, version {}", 1.0);
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
    rk::test_panic_handler(info);
}
