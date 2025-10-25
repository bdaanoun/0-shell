use std::fs;
use std::path::Path;
use std::env;

pub fn rm(args: &[&str]) -> Result<(), String> {
    let mut recursive = false;
    let mut paths = Vec::new();
    for arg in args {
        if *arg == "-r" {
            recursive = true;
        } else if arg.starts_with('-') {
            return Err(format!("rm: invalid option -- '{}'", &arg[1..]));
        } else {
            paths.push(*arg);
        }
    }
    if paths.is_empty() {
        return Err("rm: missing operand".to_string());
    }

    let current_dir_result = env::current_dir().and_then(|p: std::path::PathBuf| p.canonicalize());

    for path_str in paths {
        let path = Path::new(path_str);
        if !path.exists() {
            eprintln!("rm: cannot remove '{}': No such file or directory", path_str);
            continue;
        }

        let mut deleting_current_dir = false;
        if let Ok(ref current_dir) = current_dir_result {
            if let Ok(absolute_path) = path.canonicalize() {
                if absolute_path == *current_dir {
                    deleting_current_dir = true;
                }
            }
        }

        if path.is_dir() {
            if recursive {
                if let Err  (e) = fs::remove_dir_all(path) {
                    eprintln!("rm: cannot remove directory '{}': {}", path_str, e);
                } else if deleting_current_dir {git config pull.rebase false
                    if let Err(e) = env::set_current_dir("..") {
                        eprintln!("rm: deleted current directory, but failed to cd to parent: {}", e);
                    }
                }
            } else {
                eprintln!("rm: cannot remove '{}': Is a directory", path_str);
            }
        } else {
            if let Err(e) = fs::remove_file(path) {
                eprintln!("rm: cannot remove file '{}': {}", path_str, e);
            }
        }
    }
    Ok(())
}