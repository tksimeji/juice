#![no_std]
#![no_main]

mod commands;

use core::arch::global_asm;
use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};

global_asm!(include_str!("boot.s"));

const UART_BASE: usize = 0x0900_0000;
const UART_DR: usize = UART_BASE;
const UART_FR: usize = UART_BASE + 0x18;

const UART_FR_TX_FULL: u32 = 1 << 5;
const UART_FR_TX_EMPTY: u32 = 1 << 4;

pub(crate) fn uart_write_byte(byte: u8) {
    unsafe {
        while read_volatile(UART_FR as *const u32) & UART_FR_TX_FULL != 0 {
            core::hint::spin_loop();
        }
        write_volatile(UART_DR as *mut u32, byte as u32);
    }
}

pub(crate) fn uart_write_str(text: &str) {
    for byte in text.bytes() {
        if byte == b'\n' {
            uart_write_byte(b'\r');
        }

        uart_write_byte(byte);
    }
}

fn uart_read_byte() -> u8 {
    unsafe {
        while read_volatile(UART_FR as *const u32) & UART_FR_TX_EMPTY != 0 {
            core::hint::spin_loop()
        }

        (read_volatile(UART_DR as *const u32) & 0xff) as u8
    }
}

fn print_prompt() {
    uart_write_str("> ");
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    uart_write_str("Juice OS\n");
    uart_write_str("Type 'help' to see available commands.\n");

    let mut input = [0u8; 64];
    let mut input_length = 0;

    print_prompt();

    loop {
        let byte = uart_read_byte();

        match byte {
            b'\r' | b'\n' => {
                uart_write_str("\n");

                let input_bytes = &input[..input_length];

                match core::str::from_utf8(input_bytes) {
                    Ok(command_line) => {
                        commands::execute(command_line);
                    }

                    Err(_) => {
                        uart_write_str("Invalid UTF-8 input.\n");
                    }
                }

                input_length = 0;
                print_prompt();
            }

            8 | 127 => {
                if input_length > 0 {
                    input_length -= 1;

                    uart_write_byte(8);
                    uart_write_byte(b' ');
                    uart_write_byte(8);
                }
            }

            byte if byte.is_ascii_graphic() || byte == b' ' => {
                if input_length < input.len() {
                    input[input_length] = byte;
                    input_length += 1;

                    uart_write_byte(byte);
                } else {
                    uart_write_byte(7);
                }
            }

            _ => {}
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    uart_write_str("\nKernel panic!\n");

    loop {
        unsafe {
            core::arch::asm!("wfe");
        }
    }
}
