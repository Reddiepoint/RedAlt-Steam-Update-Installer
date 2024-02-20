use std::collections::BTreeMap;
use std::io::{stdin, stdout, Write};
use crate::modules::settings::Settings;

mod modules;


const SOURCE: &str = "https://github.com/Reddiepoint/RedAlt-Steam-Update-Installer";

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        if args[1] == "--version" {
            println!("{}", env!("CARGO_PKG_VERSION"));
        } else if args[1] == "--help" {
            println!("Run the application to get started. This program requires \
            read, write and execute permissions.");
        }
        return;
    }

    println!("This is the companion installer for RedAlt-Steam-Update-Creator.\n\
    Version: v{}. See {} for the source code.\n\
    Enter \"help\" to get a list of commands. Enter \"update\" to update the game files.",
             env!("CARGO_PKG_VERSION"), SOURCE);
    let mut settings = Settings::default();

    println!("\n\nCurrent settings:\n{}", settings);

    loop {
        let input = get_input(">>");

        match input.as_str().split(' ').next().unwrap() {
            "changes" => settings.show_changes(),
            "exit" => break,
            "help" => get_help(input),
            "set" => settings.modify_fields(input),
            "settings" => println!("{}", settings),
            "update" => settings.update_game(),
            _ => println!("Command not recognised. Type \"help\" for a list of commands."),
        }
    }
}

pub fn get_input(prompt: &str) -> String {
    let mut line = String::new();
    print!("{} ", prompt);
    stdout().flush().expect("Error: Could not flush stdout");
    stdin().read_line(&mut line).expect("Error: Could not read a line");

    return line.trim().to_string()
}

fn get_help(input: String) {
    /*let input = input.split(' ').collect::<Vec<&str>>();
    match input.get(1) {
        None => {}
        Some(_) => {}
    };*/
    let mut help = BTreeMap::new();
    help.insert("changes", "Show the changelog.");
    help.insert("exit", "Exit the program.");
    help.insert("help", "Show help for the given command.");
    help.insert("set <field> <value>", "Set the given field to the given value.\
    To see available fields, type \"settings\".");
    help.insert("settings", "Get the current settings.");
    help.insert("update", "Update the game files.");

    for (key, value) in help {
        println!("{}: {}", key, value);
    }
}