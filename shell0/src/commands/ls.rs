use colored::Colorize;
use std::env::current_dir;
use std::fs::{self, Metadata};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::time::SystemTime;

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
            files.push(arg);
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
        } else if path.is_dir() {
            let mut dir_content = Vec::new();
            for entry in fs::read_dir(path).map_err(|e|e.to_string())?{
                let entry = entry.map_err(|e| e.to_string())?;
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy().to_string();
                if !show_all && file_name_str.starts_with("."){
                    continue;
                }
                dir_content.push(entry.path());
                
            }
            dir_content.sort();
            if long_format {
                for entry in dir_content{
                    display(&entry, long_format, classify)?;
                }
            }
            //println!("====> : {:?}", dir_content);
        }
    }
    Ok(())
}

fn display(path: &std::path::Path, long_format: bool, classify: bool) -> Result<(), String> {
    let metadata = fs::metadata(path).map_err(|e| e.to_string())?;
    let file_name = path.file_name().and_then(|f| f.to_str()).unwrap_or("");
    if long_format {
        let permissions = format_permissions(&metadata);
        let size = metadata.len();
        let modified = metadata.modified().map_err(|er|er.to_string())?;
        let datetime = format_time(modified);
        // println!("==>:m {:?}", modified);
        // println!("==>:d {:?}", datetime);
        // println!("------ {} -----", size);
        let is_dir = metadata.is_dir();
        let mut name_display = if is_dir {
            file_name.bold().blue().to_string()
        } else {
            file_name.to_string()
        };
        if classify {
            if is_dir {
                name_display.push('/');
            }else if is_executable(&metadata) {
                name_display.push('*');                
            }
        }
        // println!("{:?}",metadata);
        println!("{} {:>8} {} {}", permissions, size,datetime, name_display);
    }
    Ok(())
}

fn is_executable(meta_file : &fs::Metadata)-> bool{
    if meta_file.permissions().mode() & 0o111 != 0 {
        true
    }else {
        false
    }
}
fn format_time(time: SystemTime) -> String {
    use chrono::{DateTime, Local, Datelike};
    
    let datetime: DateTime<Local> = time.into();
    let now = Local::now();
    
    if datetime.year() == now.year() {
        datetime.format("%b %e %H:%M").to_string()
    } else {
        datetime.format("%b %e  %Y").to_string()
    }
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
