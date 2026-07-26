#![no_std]

mod commands;
mod memory;
mod uart;

use core::panic::PanicInfo;
use juice_api::boot::BootContext;

core::arch::global_asm!(include_str!("exceptions.s"));

fn print_prompt() {
    uart::write_str("> ");
}

fn exception_class_name(exception_class: u8) -> &'static str {
    match exception_class {
        0x00 => "Unknown",
        0x15 => "SVC from AArch64",
        0x20 => "Instruction abort from lower EL",
        0x21 => "Instruction abort from current EL",
        0x24 => "Data abort from lower EL",
        0x25 => "Data abort from current EL",
        0x3c => "Breakpoint from AArch64",
        _ => "Unhandled exception class",
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn exception_dispatch() -> ! {
    let esr_el1: u64;
    let elr_el1: u64;
    let far_el1: u64;
    let spsr_el1: u64;

    unsafe {
        core::arch::asm!(
            "mrs {esr}, ESR_EL1",
            "mrs {elr}, ELR_EL1",
            "mrs {far}, FAR_EL1",
            "mrs {spsr}, SPSR_EL1",
            esr = out(reg) esr_el1,
            elr = out(reg) elr_el1,
            far = out(reg) far_el1,
            spsr = out(reg) spsr_el1,
            options(nomem, nostack)
        );
    }

    let exception_class = ((esr_el1 >> 26) & 0x3f) as u8;

    uart::write_str("\n");
    uart::write_str("=== KERNEL EXCEPTION ===\n");

    uart::write_str("Type: ");
    uart::write_str(exception_class_name(exception_class));
    uart::write_str("\n");

    uart::write_str("ESR_EL1:   ");
    uart::write_hex_u64(esr_el1);
    uart::write_str("\n");

    uart::write_str("ELR_EL1:   ");
    uart::write_hex_u64(elr_el1);
    uart::write_str("\n");

    uart::write_str("FAR_EL1:   ");
    uart::write_hex_u64(far_el1);
    uart::write_str("\n");

    uart::write_str("SPSR_EL1:   ");
    uart::write_hex_u64(spsr_el1);
    uart::write_str("\n");

    uart::write_str("System halted.\n");

    halt()
}

pub fn start(boot_context: &BootContext<'_>) -> ! {
    uart::write_str("Juice OS\n");
    uart::write_str("Type 'help' to see available commands.\n");

    let mut input = [0u8; 64];
    let mut input_length = 0;

    print_prompt();

    loop {
        let byte = uart::read_byte();

        match byte {
            b'\r' | b'\n' => {
                uart::write_str("\n");

                let input_bytes = &input[..input_length];

                match core::str::from_utf8(input_bytes) {
                    Ok(command_line) => {
                        commands::execute(boot_context, command_line);
                    }

                    Err(_) => {
                        uart::write_str("Invalid UTF-8 input.\n");
                    }
                }

                input_length = 0;
                print_prompt();
            }

            8 | 127 => {
                if input_length > 0 {
                    input_length -= 1;

                    uart::write_byte(8);
                    uart::write_byte(b' ');
                    uart::write_byte(8);
                }
            }

            byte if byte.is_ascii_graphic() || byte == b' ' => {
                if input_length < input.len() {
                    input[input_length] = byte;
                    input_length += 1;

                    uart::write_byte(byte);
                } else {
                    uart::write_byte(7);
                }
            }

            _ => {}
        }
    }
}

fn halt() -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfe");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    uart::write_str("\nKernel panic!\n");
    halt()
}
