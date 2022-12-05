use advent_of_code_2022::helper::read_data;

fn main() {
    let data = read_data("day01_data.txt").expect("error: cannot get data from file");
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
