#![no_std] // freestanding binary
#![no_main] // gaada main, gantinya looping di fungsi start

use core::panic::PanicInfo;

static MESSAGE: &[u8] = b"Fiat justitia ruat caelum";

#[unsafe(no_mangle)] // don't mangle the name of this function (mencegah namanya ditambah tambahin random, biar namanya tetep _start)
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8; // memori VGA Buffer, write langsung ke 0xb8000
    let screen_width = 80;
    let screen_height = 25;
    let message_len = MESSAGE.len();

    // Center text
    let row = screen_height / 2;
    let col = (screen_width - message_len) / 2;
    let start_offset = (row * screen_width + col) * 2;

    let attribute = 0x0fu8;

    for (i, &byte) in MESSAGE.iter().enumerate() {
        unsafe {
            // unsafe : kasih tau rust kalo write disini (alamat vga_buffer) aman
            *vga_buffer.offset((start_offset + i * 2) as isize) = byte;
            *vga_buffer.offset((start_offset + i * 2 + 1) as isize) = attribute;
        }
    }

    loop {
    }
}

/// This function is called on panic. (handle error disini)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}
