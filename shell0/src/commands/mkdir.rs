use std::fs;
use std::path::Path;

pub fn mkdir(args: &[&str]) -> Result<(), String> {
    if args.is_empty() {
        return Err("mkdir: missing operand".to_string());
    }
    for arg in args {
        let path = Path::new(arg);
        if let Err(e) = fs::create_dir_all(&path) {
            return Err(format!("mkdir: cannot create directory '{}': {}", arg, e));
        }
    }
    Ok(())
}