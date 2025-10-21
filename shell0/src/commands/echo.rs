use std::io;
use std::io::Write;

pub fn echo(args: &[&str]) -> Result<(), String> {
    let mut raw_string = args.join(" ");
     let  mut  in_dqoute_loop =  false  ; 

    if raw_string.matches('"').count() % 2 != 0 {
        
        if raw_string.starts_with('"') && raw_string.matches('"').count() == 1 {
            in_dqoute_loop = true ;
            raw_string.push('\n');
        }
        loop {
            print!("dqoute> ");
            io::stdout().flush().unwrap(); 

            let mut continuation_line = String::new();
            match io::stdin().read_line(&mut continuation_line) {
                Ok(0) => break, 
                Ok(_) => {
                    raw_string.push_str(&continuation_line);
                    if raw_string.matches('"').count() % 2 == 0 {
                        break;
                    }
                }
                Err(e) => {
                    return Err(format!("Error reading input: {}", e));
                }
            }
        }
         
    }
   if in_dqoute_loop && raw_string.ends_with('\n') {
        raw_string.pop();
        }
    let output = raw_string.replace('"', "");
    println!("{}", output);
    
    Ok(())
}