use std::{env::current_dir, fs::read_to_string, path::Path};

pub fn read_data(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let dir = current_dir()?;
    let path = dir.join(Path::new("src/bin").join(path));
    let result = read_to_string(path)?;
    Ok(result)
}
