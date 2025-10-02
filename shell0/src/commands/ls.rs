use colored::Colorize;
use std::fs;
// use std::path::Path;
use std::env::current_dir;

pub fn ls(args: &[&str]) -> Result<(), String> {
    if args.is_empty() {
        let path = current_dir().map_err(|e| e.to_string())?;
        let mut files = Vec::new();

        for entry in fs::read_dir(&path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            files.push(entry.file_name());
        }

        for (i, f) in files.iter().enumerate() {
            let is_dir = fs::metadata(f).map(|m| m.is_dir()).unwrap_or(false);

            let display_text = if is_dir {
                format!("{}", f.display()).bold().blue()
            } else {
                format!("{}", f.display()).normal()
            };

            if i < files.len() - 1 {
                print!("{} ", display_text);
            } else {
                println!("{}", display_text);
            }
        }
    }else if args[0].starts_with("-") {

        println!("-------{:?}", args);
    }

    Ok(())
}
