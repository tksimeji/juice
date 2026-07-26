use crate::{memory, uart};
use juice_api::boot::BootContext;
use juice_api::memory::MemoryRegionKind;

type CommandHandler = fn(&BootContext<'_>, &str);

struct Command {
    name: &'static str,
    description: &'static str,
    handler: CommandHandler,
}

const COMMANDS: &[Command] = &[
    Command {
        name: "help",
        description: "Show available commands",
        handler: command_help,
    },
    Command {
        name: "echo",
        description: "Print the given text",
        handler: command_echo,
    },
    Command {
        name: "el",
        description: "Show the current exception level",
        handler: command_el,
    },
    Command {
        name: "clear",
        description: "Clear the terminal",
        handler: command_clear,
    },
    Command {
        name: "crash",
        description: "Trigger a test exception",
        handler: command_crash,
    },
    Command {
        name: "mem",
        description: "Show the kernel memory layout",
        handler: command_mem,
    },
];

pub fn execute(boot_context: &BootContext<'_>, command_line: &str) {
    let command_line = command_line.trim();

    if command_line.is_empty() {
        return;
    }

    let (name, arguments) = match command_line.split_once(' ') {
        Some((name, arguments)) => (name, arguments.trim()),
        None => (command_line, ""),
    };

    let command = COMMANDS.iter().find(|command| command.name == name);

    match command {
        Some(command) => {
            (command.handler)(boot_context, arguments);
        }

        None => {
            uart::write_str("Unknown command: ");
            uart::write_str(name);
            uart::write_str("\nType 'help' to see available commands.\n");
        }
    }
}

fn command_help(_boot_context: &BootContext<'_>, _arguments: &str) {
    uart::write_str("Available commands:\n");

    for command in COMMANDS {
        uart::write_str("   ");
        uart::write_str(command.name);
        uart::write_str(" - ");
        uart::write_str(command.description);
        uart::write_str("\n");
    }
}

fn command_echo(_boot_context: &BootContext<'_>, arguments: &str) {
    uart::write_str(arguments);
    uart::write_str("\n");
}

fn command_el(_boot_context: &BootContext<'_>, _arguments: &str) {
    let current_el: u64;

    unsafe {
        core::arch::asm!(
            "mrs {value}, CurrentEL",
            value = out(reg) current_el,
            options(nomem, nostack)
        );
    }

    let exception_level = ((current_el >> 2) & 0b11) as u8;

    uart::write_str("Current exception level: EL");
    uart::write_byte(b'0' + exception_level);
    uart::write_byte(b'\n');
}

fn command_clear(_boot_context: &BootContext<'_>, _arguments: &str) {
    uart::write_str("\x1b[2J\x1b[H");
}

fn command_crash(_boot_context: &BootContext<'_>, _arguments: &str) {
    uart::write_str("Triggering breakpoint exception...\n");

    unsafe {
        core::arch::asm!("brk #0");
    }
}

fn command_mem(boot_context: &BootContext<'_>, _arguments: &str) {
    uart::write_str("Memory regions:\n");

    for region in boot_context.memory_regions {
        uart::write_str("  ");

        match region.kind {
            MemoryRegionKind::Usable => {
                uart::write_str("Usable");
            }
            MemoryRegionKind::Reserved => {
                uart::write_str("Reserved");
            }
            MemoryRegionKind::Kernel => {
                uart::write_str("Kernel");
            }
            MemoryRegionKind::DeviceTree => {
                uart::write_str("DeviceTree");
            }
        }

        uart::write_str("  ");
        uart::write_hex_u64(region.start.as_usize() as u64);
        uart::write_str(" - ");
        uart::write_hex_u64(region.end.as_usize() as u64);
        uart::write_str(" size=");
        uart::write_hex_u64(region.size() as u64);
        uart::write_str("\n");
    }

    uart::write_str("Total usable: ");
    uart::write_hex_u64(memory::usable_size(boot_context) as u64);
    uart::write_str(" bytes\n");

    uart::write_str("Page size: ");
    uart::write_hex_u64(memory::PAGE_SIZE as u64);
    uart::write_str("bytes \n");
}
