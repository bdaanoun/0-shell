use std::fs;
use std::path::Path;

pub fn mv(args: &[&str]) -> Result<(), String> {
    if args.len() != 2 {
        return Err("mv: requires two arguments, a source and a destination".to_string());
    }

    let source = Path::new(args[0]);
    let destination = Path::new(args[1]);

    if !source.exists() {
        return Err(format!("mv: cannot move '{}': No such file or directory", args[0]));
    }

    if destination.exists() && destination.is_dir() {
        let file_name = source.file_name().ok_or_else(|| format!("mv: cannot get file name from '{}'", args[0]))?;
        let new_dest = destination.join(file_name);
        if let Err(e) = fs::rename(source, new_dest) {
             return Err(format!("mv: cannot move '{}' to '{}': {}", args[0], args[1], e));
        }
    } else {
        if let Err(e) = fs::rename(source, destination) {
            return Err(format!("mv: cannot move '{}' to '{}': {}", args[0], args[1], e));
        }
    }

    Ok(())
}