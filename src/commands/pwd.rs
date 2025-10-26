use std::env;

pub fn pwd(args: &[&str]) -> Result<(), String> {
    if !args.is_empty() {
        return Err(format!("pwd: too many arguments"));
    }
    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
            Ok(())
        }
        Err(e) => Err(format!("pwd error: {}", e)),
    }
}
