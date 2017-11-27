use std::io;
use std::fs::File;
use std::io::Read;

pub fn read_in_file(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
