use std::fs::File;
use std::io::Write;

pub fn add_to_suite(cmd: String) -> std::io::Result<()>{
    let mut file: File = File::open("test_commands.txt")?;

    file.write(cmd.as_bytes())?;

    Ok(())
}