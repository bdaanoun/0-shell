use std::fs;
use std::path::Path;

pub fn cp(args: &[&str]) -> Result<(), String> {
    if args.len() != 2 {
        return Err("cp: requires two arguments, a source and a destination".to_string());
    }

    let source = Path::new(args[0]);
    let destination = Path::new(args[1]);

    if !source.exists() {
        return Err(format!("cp: cannot copy '{}': No such file or directory", args[0]));
    }
    let final_destination = if destination.exists() && destination.is_dir() {
        let file_name = source.file_name().ok_or_else(|| format!("cp: cannot get file name from '{}'", args[0]))?;
        destination.join(file_name)
    } else {
        destination.to_path_buf()
    };
    
    if let Err(e) = fs::copy(source, &final_destination) {
        return Err(format!("cp: cannot copy '{}' to '{}': {}", args[0], final_destination.display(), e));
    }

    Ok(())
}