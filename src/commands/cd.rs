use std::env;
use std::path::{Path, PathBuf}; 

pub fn cd(args: &[&str]) -> Result<(), String> {
    
    let old_path = match env::current_dir() {
        Ok(path) => path,
        Err(e) => return Err(format!("cd: error getting current directory: {}", e)),
    };
    
    let is_cd_dash = args.len() > 0 && args[0] == "-";
    
    let target_str = if args.is_empty() || args[0] == "~" {
        env::var("HOME").map_err(|_| "cd: HOME not set".to_string())?
    } else if is_cd_dash {
        env::var("OLDPWD").map_err(|_| "cd: OLDPWD not set".to_string())?
    } else {
        args[0].to_string()
    };

    let target_path = Path::new(&target_str);

    let new_path_buf;
    
    if target_path.is_absolute() {
         new_path_buf = target_path.to_path_buf();
    } else {
         
         new_path_buf = old_path.join(target_path);
    }
    let new_path = match new_path_buf.canonicalize() {
         Ok(path) => path,
         Err(e) => return Err(format!("cd: {}: {}", e, target_str)),
    };
    if new_path == old_path {
        return Ok(()); 
    }
    if let Err(e) = env::set_current_dir(&new_path) {
        return Err(format!("cd: {}: {}", e, new_path.display()));
    }
    if let Some(path_str) = old_path.to_str() {
        unsafe {
            env::set_var("OLDPWD", path_str);
        }
    }

    Ok(())
}