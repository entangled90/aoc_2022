use std::{error::Error, fs::*, io::Read};

pub fn open_file(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents.lines().map(|s| s.to_string()).collect());
}
