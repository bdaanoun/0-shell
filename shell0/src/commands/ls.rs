use colored::Colorize;
use std::fs;
// use std::path::Path;
use std::env::current_dir;
use std::os::unix::fs::{MetadataExt, PermissionsExt};

pub fn ls(args: &[&str]) -> Result<(), String> {
    let mut show_all = false;
    let mut long_format = false;
    let mut classify = false;
    let mut files: Vec<&str> = Vec::new();

    for arg in args {
        if arg.contains("-") {
            for c in arg.chars().skip(1) {
                match c {
                    'a' => show_all = true,
                    'l' => long_format = true,
                    'F' => classify = true,
                    _ => return Err(format!("ls: invalid option: - '{}'", c)),
                }
            }
        } else {
            files.push(&arg);
        }
    }

    if files.is_empty() {
        files.push(".")
    }

    for (i, file_path) in files.iter().enumerate() {
        if i > 0 {
            println!()
        }
        if files.len() > 1 {
            println!("{}:", file_path);
        }
        let path = std::path::Path::new(file_path);
        if !path.exists() {
            println!(
                "ls: cannot access '{}': No such file or directory",
                file_path
            )
        }
        if path.is_file() {
            display(path, long_format, classify)?;
        }
        // println!("paath::  {:?}", path);
    }

    println!("---:  {:?}", files);
    Ok(())
}

fn display(path: &std::path::Path, long_format: bool, classify: bool) -> Result<(), String> {
    let metadata = fs::metadata(path).map_err(|e| e.to_string())?;
    let file_name = path.file_name().and_then(|f| f.to_str()).unwrap_or("");
    if long_format {
        let permissions = format_permissions(&metadata);
        let size = metadata.len();
        println!("{:?}", permissions);
        println!("------ {} -----", size);
    }
    Ok(())
}

fn format_permissions(metadata: &fs::Metadata) -> String {
    let mode = metadata.mode();
    let file_type = if metadata.is_dir() { 'd' } else { '-' };

    format!(
        "{}{}{}{}{}{}{}{}{}{}",
        file_type,
        if mode & 0o400 != 0 { 'r' } else { '-' },
        if mode & 0o200 != 0 { 'w' } else { '-' },
        if mode & 0o100 != 0 { 'x' } else { '-' },
        if mode & 0o040 != 0 { 'r' } else { '-' },
        if mode & 0o020 != 0 { 'w' } else { '-' },
        if mode & 0o010 != 0 { 'x' } else { '-' },
        if mode & 0o004 != 0 { 'r' } else { '-' },
        if mode & 0o002 != 0 { 'w' } else { '-' },
        if mode & 0o001 != 0 { 'x' } else { '-' },
    )
}
