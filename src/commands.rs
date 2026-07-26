use crate::{uart_write_byte, uart_write_str};

type CommandHandler = fn(&str);

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
];

pub fn execute(command_line: &str) {
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
            (command.handler)(arguments);
        }

        None => {
            uart_write_str("Unknown command:");
            uart_write_str(name);
            uart_write_str("\nType 'help' to see available commands.\n");
        }
    }
}

fn command_help(_arguments: &str) {
    uart_write_str("Available commands:\n");

    for command in COMMANDS {
        uart_write_str("   ");
        uart_write_str(command.name);
        uart_write_str(" - ");
        uart_write_str(command.description);
        uart_write_str("\n");
    }
}

fn command_echo(arguments: &str) {
    uart_write_str(arguments);
    uart_write_str("\n");
}

fn command_el(_arguments: &str) {
    let current_el: u64;

    unsafe {
        core::arch::asm!(
            "mrs {value}, CurrentEL",
            value = out(reg) current_el,
            options(nomem, nostack)
        );
    }

    let exception_level = ((current_el >> 2) & 0b11) as u8;

    uart_write_str("Current exception level: EL");
    uart_write_byte(b'0' + exception_level);
    uart_write_byte(b'\n');
}

fn command_clear(_arguments: &str) {
    uart_write_str("\x1b[2J\x1b[H");
}
