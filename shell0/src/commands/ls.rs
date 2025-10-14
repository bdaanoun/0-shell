use colored::Colorize;
use std::fs::{self};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::time::SystemTime;
use users::{get_group_by_gid, get_user_by_uid};

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
            );
            continue;
        }
        if path.is_file() || path.is_symlink() {
            display(path, long_format, classify)?;
        } else if path.is_dir() {
            let mut dir_content = Vec::new();
            for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
                let entry = entry.map_err(|e| e.to_string())?;
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy().to_string();
                if !show_all && file_name_str.starts_with(".") {
                    continue;
                }
                dir_content.push(entry.path());
            }

            dir_content.sort();

            if long_format {
                for entry in dir_content {
                    display(&entry, long_format, classify)?;
                }
            } else {
                for (i, file_path) in dir_content.iter().enumerate() {
                    let file_name = file_path
                        .file_name()
                        .and_then(|f| f.to_str())
                        .unwrap_or("");
                    
                    let metadata = fs::symlink_metadata(&file_path).map_err(|e| e.to_string())?;
                    let is_dir = metadata.is_dir();
                    let is_symlink = metadata.is_symlink();
                    
                    let mut display_text = if is_symlink {
                        file_name.bold().cyan().to_string()
                    } else if is_dir {
                        file_name.bold().blue().to_string()
                    } else if is_executable(&metadata) {
                        file_name.bold().green().to_string()
                    } else {
                        file_name.to_string()
                    };

                    if classify {
                        if is_symlink {
                            display_text.push('@');
                        } else if is_dir {
                            display_text.push('/');
                        } else if is_executable(&metadata) {
                            display_text.push('*');
                        }
                    }
                    
                    if i < dir_content.len() - 1 {
                        print!("{}  ", display_text);
                    } else {
                        println!("{}", display_text)
                    }
                }
            }
        }
    }
    Ok(())
}

fn display(path: &std::path::Path, long_format: bool, classify: bool) -> Result<(), String> {
    let metadata = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    let file_name = path.file_name().and_then(|f| f.to_str()).unwrap_or("");
    
    if long_format {
        let permissions = format_permissions(&metadata);
        let count_links = metadata.nlink();
        let is_symlink = metadata.is_symlink();
        let size = metadata.len();
        let modified = metadata.modified().map_err(|er| er.to_string())?;
        let datetime = format_time(modified);

        let username = get_user_by_uid(metadata.uid())
            .map(|n| n.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| metadata.uid().to_string());
        let group = get_group_by_gid(metadata.gid())
            .map(|n| n.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| metadata.gid().to_string());

        let is_dir = metadata.is_dir();
        
        let mut name_display = if is_symlink {
            file_name.bold().cyan().to_string()
        } else if is_dir {
            file_name.bold().blue().to_string()
        } else if is_executable(&metadata) {
            file_name.bold().green().to_string()
        } else {
            file_name.to_string()
        };

        if is_symlink {
            if let Ok(target) = std::fs::read_link(path) {
                let target_str = target.to_string_lossy();

                let target_suffix = if let Ok(target_meta) = std::fs::metadata(path) {
                    if classify {
                        if target_meta.is_dir() {
                            "/"
                        } else if is_executable(&target_meta) {
                            "*"
                        } else {
                            ""
                        }
                    } else {
                        ""
                    }
                } else {
                    ""
                };

                name_display = format!("{} -> {}{}", name_display, target_str, target_suffix);
            }
        } else {
            if classify {
                if is_dir {
                    name_display.push('/');
                } else if is_executable(&metadata) {
                    name_display.push('*');
                }
            }
        }

        println!(
            "{} {:>3} {:>8} {:>8} {:>8} {} {}",
            permissions, count_links, username, group, size, datetime, name_display
        );
    } else {
        let is_dir = metadata.is_dir();
        let is_symlink = metadata.is_symlink();
        
        let mut display_text = if is_symlink {
            file_name.bold().cyan().to_string()
        } else if is_dir {
            file_name.bold().blue().to_string()
        } else if is_executable(&metadata) {
            file_name.bold().green().to_string()
        } else {
            file_name.to_string()
        };

        if classify {
            if is_symlink {
                display_text.push('@');
            } else if is_dir {
                display_text.push('/');
            } else if is_executable(&metadata) {
                display_text.push('*');
            }
        }

        println!("{}", display_text);
    }
    Ok(())
}

fn is_executable(meta_file: &fs::Metadata) -> bool {
    meta_file.permissions().mode() & 0o111 != 0
}

fn format_time(time: SystemTime) -> String {
    use chrono::{DateTime, Datelike, Local};

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
    let file_type = if metadata.is_symlink() {
        'l'
    } else if metadata.is_dir() {
        'd'
    } else {
        '-'
    };

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