 use std::io;
  use std::io::Write;
pub fn echo(args: &[&str]) -> Result<(), String> {
    let mut raw_string = args.join(" ");

    if raw_string.matches('"').count() % 2 != 0 {
        loop {
            print!("> ");
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

    let output = raw_string.replace('"', "");
    println!("{}", output);
    
    Ok(())
} 