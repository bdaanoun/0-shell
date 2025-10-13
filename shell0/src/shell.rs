use colored::Colorize;
use std::io::{self, Write};
use std::env;
pub fn run_shell() {
    banner_draw();
    loop {
         let current_dir = match env::current_dir() {
            Ok(path) => path.display().to_string(),
            Err(_) => "?".to_string(),
        };
        print!("{} {} {} ", "$".green() ,  current_dir.blue(), "$".green());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input);

        match bytes_read {
            Ok(0) => {
                // Ctrl+D (EOF) pressed, exit shell
                println!();
                break;
            }
            Ok(_) => {
                let input = input.trim();
                if input.is_empty() {
                    continue;
                }

                let mut parts = input.split_whitespace();
                let command = parts.next().unwrap();
                let args: Vec<&str> = parts.collect();

                if let Err(e) = execute_command(command, &args) {
                    eprintln!("{}", e);
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
            }
        }
    }
}

fn execute_command(cmd: &str, args: &[&str]) -> Result<(), String> {
    match cmd {
        "echo" => crate::commands::echo::echo(args),
        "pwd" => crate::commands::pwd::pwd(args),
        "mkdir" =>  crate::commands::mkdir::mkdir(args),
        "rm" =>  crate::commands::rm::rm(args) ,
        "mv" => crate::commands::mv::mv(args) ,
        "cd" => crate::commands::cd::cd(args),
        "ls" => crate::commands::ls::ls(args),
        "cat" => crate::commands::cat::cat(args),
        
        "exit" => {
            std::process::exit(0);
        }
        _ => Err(format!("Command '{}' not found", cmd)),
    }
}

fn  banner_draw(){
      let banner = vec![
        "  ___         _____ _          _ _ ",
        " / _ \\       / ____| |        | | |",
        "| | | |_____| (___ | |__   ___| | |",
        "| | | |______\\___ \\| '_ \\ / _ \\ | |",
        "| |_| |      ____) | | | |  __/ | |",
        " \\___/      |_____/|_| |_|\\___|_|_|           Powred By 0xab",
    ];

    for line in banner {
        println!("{}", line.red());
    }
}