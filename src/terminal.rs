use super::structures::MenuOption;
use std::io::{self, Write};
use std::process::Command;

pub fn cls() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Error al limpiar la consola");
    } else {
        Command::new("clear")
            .status()
            .expect("Error al limpiar la consola");
    }
}

pub fn br(mut lines: u8) {
    while lines != 0 {
        println!("\n");
        lines -= 1;
    }
}

fn terminal_input(message: &str) -> String {
    let mut input: String = String::new();

    print!("{}", message);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("An error occour reading the input");

    return input;
}

pub fn input(prompt: &str) -> String {
    return terminal_input(prompt);
}

pub fn number_input(prompt: &str, looping: bool) -> i64 {
    loop {
        let input: i64 = match terminal_input(prompt).trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if looping {
                    continue;
                } else {
                    0i64
                }
            }
        };

        return input;
    }
}

pub fn show_menu(title: &str, options: &mut Vec<MenuOption>, is_main: bool) {
    loop {
        println!("{}", title);

        for option in options.into_iter() {
            println!("{} - {}", option.code, option.title);
        }
        println!("0 - Salir");
        let input: u8 = number_input(">_", false) as u8;

        match input {
            0 => {
                cls();
                if is_main {
                    println!("Saliendo...");
                }
                break;
            }
            _ => match options.iter().find(|&option| option.code == input) {
                Some(option) => {
                    option.execute();
                }
                None => {
                    cls();
                    continue;
                }
            },
        }
    }
}
