use core::ptr::{read_volatile, write_volatile};

const UART_BASE: usize = 0x0900_0000;
const UART_DR: usize = UART_BASE;
const UART_FR: usize = UART_BASE + 0x18;

const UART_FR_TX_FULL: u32 = 1 << 5;
const UART_FR_TX_EMPTY: u32 = 1 << 4;

pub(crate) fn write_byte(byte: u8) {
    unsafe {
        while read_volatile(UART_FR as *const u32) & UART_FR_TX_FULL != 0 {
            core::hint::spin_loop();
        }
        write_volatile(UART_DR as *mut u32, byte as u32);
    }
}

pub(crate) fn write_hex_u64(value: u64) {
    const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";

    write_str("0x");

    for index in (0..16).rev() {
        let shift = index * 4;
        let digit = ((value >> shift) & 0x0f) as usize;

        write_byte(HEX_DIGITS[digit]);
    }
}

pub(crate) fn write_str(text: &str) {
    for byte in text.bytes() {
        if byte == b'\n' {
            write_byte(b'\r');
        }

        write_byte(byte);
    }
}

pub(crate) fn read_byte() -> u8 {
    unsafe {
        while read_volatile(UART_FR as *const u32) & UART_FR_TX_EMPTY != 0 {
            core::hint::spin_loop()
        }

        (read_volatile(UART_DR as *const u32) & 0xff) as u8
    }
}
