use std::{
    error::Error,
    io::{Read, Write},
    process::{Command, Stdio},
};
fn main() {
    match do_the_thing() {
        Ok(_) => (),
        Err(reason) => panic!("oops! {}", reason),
    }
}
fn do_the_thing() -> Result<(), Box<dyn Error>> {
    let rofi = Command::new("rofi")
        .args(["-dmenu", "-p", "layout", "-i"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    rofi.stdin
        .ok_or("failed to write to stdin")?
        .write_all(include_bytes!("modes"))?;

    let mut mode = String::new();
    rofi.stdout
        .ok_or("could not read stdout of rofi")?
        .read_to_string(&mut mode)?;

    println!("{}", mode);
    Command::new("i3l").arg(mode.trim()).spawn()?;
    Ok(())
}
