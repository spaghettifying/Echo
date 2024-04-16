#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(echo::test_runner)]
#![reexport_test_harness_main = "test_main"]

use echo::print;
use echo::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> !{
    echo::init();

    println!("Hello World!");
    println!("It didn't crash!!");

    // #[cfg(test)]
    // test_main();

    loop {
        for _ in 0..10000 {};

        use echo::print;
        print!("-");
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    echo::test_panic_handler(info);
}