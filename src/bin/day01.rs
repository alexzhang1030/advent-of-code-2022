use std::env::current_dir;
use std::fs::read_to_string;
use std::path::Path;

fn read_data() -> Result<String, Box<dyn std::error::Error>> {
    let dir = current_dir()?;
    let path = dir.join(Path::new("src/bin/day01_data.txt"));
    let result = read_to_string(path)?;
    Ok(result)
}

fn main() {
    let data = read_data().expect("error: cannot get data from file");
    let data: Vec<_> = data.lines().collect();
    let mut elf: Vec<i32> = Vec::new();
    let mut elves: Vec<i32> = Vec::new();

    for item in data {
        let str = item.trim();
        if str.is_empty() {
            elves.push(elf.iter().sum());
            elf.clear();
        } else {
            elf.push(str.parse::<i32>().expect("cannot parse to i32"));
        }
    }

    let result = elves.iter().max().expect("cannot find max value");

    println!("{}", result)
}
