use chrono::{DateTime, Datelike, Duration, Local};
use colored::Colorize;
use std::fs::{self};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use users::{get_group_by_gid, get_user_by_uid};

struct DirEntry {
    path: PathBuf,
    display_name: String,
}

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
        files.push(".");
    }

    let mut missing = Vec::new();
    let mut regular_files = Vec::new();
    let mut dirs = Vec::new();

    for f in &files {
        let path = Path::new(f);
        if !path.exists() {
            missing.push(f.to_string());
        } else if path.is_dir() {
            dirs.push(path.to_path_buf());
        } else {
            regular_files.push(path);
        }
    }

    for m in &missing {
        eprintln!("ls: cannot access '{}': No such file or directory", m.trim());
    }

    for f in &regular_files {
        let file_name = f
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or_else(|| f.to_str().unwrap_or(""));
        display(f, file_name, long_format, classify)?;
    }

    for dir in dirs.iter() {
        if !regular_files.is_empty() || dirs.len() > 1 || !missing.is_empty() {
            println!();
            println!("{}:", dir.display());
        }

        let mut dir_content: Vec<DirEntry> = Vec::new();

        if show_all {
            let abs_path = dir.canonicalize().map_err(|e| e.to_string())?;

            dir_content.push(DirEntry {
                path: abs_path.clone(),
                display_name: ".".to_string(),
            });

            if let Some(parent) = abs_path.parent() {
                dir_content.push(DirEntry {
                    path: parent.to_path_buf(),
                    display_name: "..".to_string(),
                });
            } else {
                dir_content.push(DirEntry {
                    path: Path::new("..").to_path_buf(),
                    display_name: "..".to_string(),
                });
            }
        }

        for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            if !show_all && name_str.starts_with('.') {
                continue;
            }

            dir_content.push(DirEntry {
                path: entry.path(),
                display_name: name_str.to_string(),
            });
        }

        dir_content.sort_by(|a, b| {
            let a_compare = a.display_name.trim_start_matches('.');
            let b_compare = b.display_name.trim_start_matches('.');
            a_compare.to_lowercase().cmp(&b_compare.to_lowercase())
        });

        if long_format {
            let mut total = 0;
            for entry in &dir_content {
                if let Ok(meta) = fs::symlink_metadata(&entry.path) {
                    total += meta.blocks()
                }
            }
            println!("total {}", total / 2);
            for entry in &dir_content {
                display(&entry.path, &entry.display_name, long_format, classify)?;
            }
        } else {
            for (j, entry) in dir_content.iter().enumerate() {
                let metadata = fs::symlink_metadata(&entry.path).map_err(|e| e.to_string())?;
                let mut display_text = colorize_name(&entry.display_name, &metadata);

                if classify {
                    if metadata.is_symlink() {
                        display_text.push('@');
                    } else if metadata.is_dir() {
                        display_text.push('/');
                    } else if is_executable(&metadata) {
                        display_text.push('*');
                    }
                }

                if j < dir_content.len() - 1 {
                    print!("{}  ", display_text);
                } else {
                    println!("{}", display_text);
                }
            }
        }
    }

    Ok(())
}

fn display(path: &std::path::Path, file_name: &str, long_format: bool, classify: bool) -> Result<(), String> {
    let metadata = fs::symlink_metadata(path).map_err(|e| e.to_string())?;

    if long_format {
        let permissions = format_permissions(path, &metadata);
        let count_links = metadata.nlink();
        let is_symlink = metadata.is_symlink();
        let modified = metadata.modified().map_err(|er| er.to_string())?;
        let datetime = format_time(modified);
        let file_type = metadata.mode() & libc::S_IFMT;

        let size_or_dev = if file_type == libc::S_IFCHR || file_type == libc::S_IFBLK {
            format!("{}, {}", major(metadata.rdev()), minor(metadata.rdev()))
        } else {
            metadata.len().to_string()
        };
        let username = get_user_by_uid(metadata.uid())
            .map(|n| n.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| metadata.uid().to_string());
        let group = get_group_by_gid(metadata.gid())
            .map(|n| n.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| metadata.gid().to_string());

        let is_dir = metadata.is_dir();
        let mut name_display = colorize_name(file_name, &metadata);

        if is_symlink {
            if let Ok(target) = std::fs::read_link(path) {
                let target_str = target.to_string_lossy();

                let target_suffix = if classify {
                    if let Ok(target_meta) = std::fs::metadata(path) {
                        let target_mode = target_meta.mode();
                        let target_file_type = target_mode & libc::S_IFMT;

                        if target_meta.is_dir() {
                            "/"
                        } else if target_file_type == libc::S_IFSOCK {
                            "="
                        } else if target_file_type == libc::S_IFIFO {
                            "|"
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
            permissions, count_links, username, group, size_or_dev, datetime, name_display
        );
    } else {
        let is_dir = metadata.is_dir();
        let is_symlink = metadata.is_symlink();

        let mut display_text = colorize_name(file_name, &metadata);

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

fn major(rdev: u64) -> u64 {
    (rdev >> 8) & 0xfff
}
fn minor(rdev: u64) -> u64 {
    (rdev & 0xff) | ((rdev >> 12) & 0xfff00)
}

fn colorize_name(file_name: &str, metadata: &fs::Metadata) -> String {
    if metadata.is_symlink() {
        file_name.bold().cyan().to_string()
    } else if metadata.is_dir() {
        file_name.bold().blue().to_string()
    } else if (metadata.mode() & libc::S_IFMT) == libc::S_IFCHR {
        file_name.bold().truecolor(255, 127, 0).to_string()
    } else if (metadata.mode() & libc::S_IFMT) == libc::S_IFSOCK {
        file_name.bold().magenta().to_string()
    } else if is_executable(&metadata) {
        file_name.bold().green().to_string()
    } else {
        file_name.to_string()
    }
}

fn is_executable(meta_file: &fs::Metadata) -> bool {
    meta_file.permissions().mode() & 0o111 != 0
}

fn format_time(time: SystemTime) -> String {
    let datetime: DateTime<Local> = time.into();
    let datetime = datetime + Duration::hours(1);
    let now = Local::now();

    if datetime.year() == now.year() {
        datetime.format("%b %e %H:%M").to_string()
    } else {
        datetime.format("%b %e  %Y").to_string()
    }
}

fn format_permissions(path: &Path, metadata: &fs::Metadata) -> String {
    let mode = metadata.mode();
    let file_type_bits = mode & libc::S_IFMT;

    let file_type = if metadata.is_symlink() {
        'l'
    } else if metadata.is_dir() {
        'd'
    } else if file_type_bits == libc::S_IFCHR {
        'c'
    } else if file_type_bits == libc::S_IFBLK {
        'b'
    } else if file_type_bits == libc::S_IFIFO {
        'p'
    } else if file_type_bits == libc::S_IFSOCK {
        's'
    } else {
        '-'
    };

    let acl_indicator = if has_acl(path) { '+' } else { ' ' };

    format!(
        "{}{}{}{}{}{}{}{}{}{}{}",
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
        acl_indicator
    )
}

fn has_acl(path: &Path) -> bool {
    if let Ok(attrs) = xattr::list(path) {
        for attr in attrs {
            if attr == "system.posix_acl_access" {
                return true;
            }
        }
    }
    false
}
