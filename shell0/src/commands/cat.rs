use std::fs::File;
use std::io::{self, Read, Write};

pub fn cat(args: &[&str]) -> Result<(), String> {
    if args.is_empty() {
        return stream_stdin_to_stdout().map_err(|e| format!("cat: {}", e));
    }
    for &arg in args {
        if arg == "-" {
            stream_stdin_to_stdout().map_err(|e| format!("cat: {}", e))?;
            continue;
        }
        let mut file = match File::open(arg) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("cat: cannot open '{}': {}", arg, e);
                continue;
            }
        };
        if let Err(e) = stream_reader_to_stdout(&mut file) {
            eprintln!("cat: error reading '{}': {}", arg, e);
        }
    }
    Ok(())
}

fn stream_stdin_to_stdout() -> io::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    stream_reader_to_stdout(&mut handle)
}

fn stream_reader_to_stdout<R: Read>(reader: &mut R) -> io::Result<()> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut buffer = [0u8; 8 * 1024];

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        out.write_all(&buffer[..n])?;
    }
    Ok(())
}
