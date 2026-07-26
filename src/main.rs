#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};

global_asm!(include_str!("boot.s"));

const UART_BASE: usize = 0x0900_0000;
const UART_DR: usize = UART_BASE;
const UART_FR: usize = UART_BASE + 0x18;

const UART_FR_TX_FULL: u32 = 1 << 5;

fn uart_write_byte(byte: u8) {
    unsafe {
        while read_volatile(UART_FR as *const u32) & UART_FR_TX_FULL != 0 {
            core::hint::spin_loop();
        }
        write_volatile(UART_DR as *mut u32, byte as u32);
    }
}

fn uart_write_str(text: &str) {
    for byte in text.bytes() {
        if byte == b'\n' {
            uart_write_byte(b'\r');
        }

        uart_write_byte(byte);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    uart_write_str("Hello, world!\n");

    loop {
        unsafe {
            core::arch::asm!("wfe");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfe");
        }
    }
}
