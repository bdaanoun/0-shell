
pub fn echo(args: &[&str]) -> Result<(), String> {
    let output = args.join(" ");
    println!("{}", output);
    Ok(())
}