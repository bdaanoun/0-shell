use std::fs;
use std::path::{Path, PathBuf};

pub fn cp(args: &[&str]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("cp: missing file operand".to_string());
    }

    let destination_str = args.last().unwrap();
    let destination = Path::new(destination_str);
    
    let sources = &args[0..args.len() - 1];
    
    let is_dir_destination = destination.exists() && destination.is_dir();
    let mut had_error = false;
    
    if sources.len() > 1 && !is_dir_destination {
        return Err(format!("cp: target '{}' is not a directory", destination_str));
    }
    
    for &source_str in sources {
        let source = Path::new(source_str);


        if !source.exists() {
            eprintln!("cp: cannot copy '{}': No such file or directory", source_str);
            had_error = true;
            continue
        }
        
        let final_dest: PathBuf;
        
        if is_dir_destination {
    
            let file_name = match source.file_name() {
                Some(name) => name,
                None => {
                    eprintln!("cp: cannot get file name from '{}'", source_str);
                    had_error = true;
                    continue;
                }
            };
            final_dest = destination.join(file_name);
        } else {
    
    
            final_dest = destination.to_path_buf();
        }
        

        if let Err(e) = fs::copy(source, &final_dest) {
            eprintln!("cp: cannot copy '{}' to '{}': {}", source_str, final_dest.display(), e);
            had_error = true;
        }
    }
    
    if had_error {
        Err("cp: encountered one or more errors".to_string())
    } else {
        Ok(())
    }
}