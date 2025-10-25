use colored::Colorize;
use std::io::{self, Write};
use std::env;

enum ParseResult {
    Complete(Vec<String>),
    Incomplete,
    Error(String),
}

pub fn run_shell() {
    let logo = r#"
_______                   .__           .__  .__   
\   _  \             _____|  |__   ____ |  | |  |  
/  /_\  \   ______  /  ___/  |  \_/ __ \|  | |  |  
\  \_/   \ /_____/  \___ \|   Y  \  ___/|  |_|  |__
 \_____  /         /____  >___|  /\___  >____/____/
       \/               \/     \/     \/             
"#;
    println!("{}", logo.yellow());
    
    let mut accumulated_input = String::new();
    
    loop {
   //     let x  = env::current_dir() ;
        let current_dir = env::current_dir() 
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "?".to_string());
        
        let prompt: String = if accumulated_input.is_empty() {
            
            format!("{} {} ", current_dir.blue(), "$".green())
        } else {
            format!("{} ", ">".yellow())
        };
        
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
        
                if !accumulated_input.is_empty() {
                    println!();
                    accumulated_input.clear();
                } else {
                    println!();
                    break;
                }
            }
            Ok(_) => {
        
                input = filter_escape_sequences(&input);
                
                accumulated_input.push_str(&input);
                
                match parse_command(&accumulated_input) {
                    ParseResult::Complete(parts) => {
                        if !parts.is_empty() {
                            let command = &parts[0];
                            let args: Vec<&str> = parts[1..].iter().map(|s| s.as_str()).collect();
                            if let Err(e) = execute_command(command, &args) {
                                eprintln!("{}", e);
                            }
                        }
                        accumulated_input.clear();
                    }
                    ParseResult::Incomplete => {
                        continue;
                    }
                    ParseResult::Error(e) => {
                        if !e.is_empty() {
                            eprintln!("{}", e);
                        }
                        accumulated_input.clear();
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                accumulated_input.clear();
            }
        }
    }
}
fn filter_escape_sequences(input: &str) -> String {
    let mut filtered = String::new();
    let mut chars = input.chars();
    
    while let Some(ch) = chars.next() {

        if ch == '\x1b' {
            while let Some(next_ch) = chars.next() {
                if next_ch.is_alphabetic() {
                    break;
                }
            }
        } else if ch.is_control() && ch != '\n' && ch != '\t' && ch != '\r' {
    
            continue;
        } else {
            filtered.push(ch);
        }
    }
    
    filtered
}
fn parse_command(input: &str) -> ParseResult {
    let input = input.trim();
    if input.is_empty() {
        return ParseResult::Error("".to_string());
    }

    let mut parts = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();
    let mut in_double_quote = false;
    let mut in_single_quote = false;

    while let Some(ch) = chars.next() {
        match ch {
    
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
            }
    
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
            }
    
            '\\' if !in_single_quote => {
                if let Some(next_ch) = chars.next() {
                    match next_ch {
                        'n' => current.push('\n'),
                        't' => current.push('\t'),
                        'r' => current.push('\r'),
                        'a' => current.push('\x07'),    
                        'b' => current.push('\x08'),    
                        'e' => current.push('\x1B'),    
                        'f' => current.push('\x0C'),    
                        'v' => current.push('\x0B'),    
                        '0' => current.push('\0'),      
                        '\\' => current.push('\\'),
                        '"' => current.push('"'),
                        '\'' => current.push('\''),
                        ' ' => current.push(' '),
                        '$' => current.push('$'),
                        '`' => current.push('`'),
                        _ => {
                    
                            current.push(next_ch);
                        }
                    }
                } else {
                    return ParseResult::Incomplete;
                }
            }
    
            ' ' | '\t' if !in_double_quote && !in_single_quote => {
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
            }
    
            _ => {
                current.push(ch);
            }
        }
    }

    if in_double_quote || in_single_quote {
        return ParseResult::Incomplete;
    }

    if !current.is_empty() {
        parts.push(current);
    }

    if parts.is_empty() {
        ParseResult::Error("".to_string())
    } else {
        ParseResult::Complete(parts)
    }
}

fn execute_command(cmd: &str, args: &[&str]) -> Result<(), String> {
    match cmd {
        "echo" => crate::commands::echo::echo(args),
        "pwd" => crate::commands::pwd::pwd(args),
        "mkdir" => crate::commands::mkdir::mkdir(args),
        "rm" => crate::commands::rm::rm(args),
        "mv" => crate::commands::mv::mv(args),
        "cd" => crate::commands::cd::cd(args),
        "ls" => crate::commands::ls::ls(args),
        "cat" => crate::commands::cat::cat(args),
        "cp" => crate::commands::cp::cp(args),
        "exit" => {
            std::process::exit(0);
        }
        _ => Err(format!("Command '{}' not found", cmd)),
    }
}