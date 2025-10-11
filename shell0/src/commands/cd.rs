use std::env;
use std::path::Path;

pub fn cd(args: &[&str]) -> Result<(), String> {
    println!("your  now  in cd  ") ;
    let target = if args.is_empty() {
        env::var("HOME").map_err(|_| "HOME environment variable not set".to_string())?
    } else {
        args[0].to_string()
    };

    let path = Path::new(&target);
    if let Err(e) = env::set_current_dir(&path) {
        return Err(format!("cd: {}: {}", target, e));
    }

    Ok(())
}
