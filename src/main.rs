#![no_std] // freestanding binary
#![no_main] // gaada main, gantinya looping di fungsi _start

mod vga_buffer;

use core::panic::PanicInfo;

#[unsafe(no_mangle)] // don't mangle the name of this function (mencegah namanya ditambah tambahin random, biar namanya tetep _start)
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    loop {
    }
}

/// This function is called on panic. (handle error disini)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {
    }
}
