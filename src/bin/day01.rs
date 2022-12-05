use itertools::Itertools;

fn main() {
    let data = include_str!("./day01_data.txt")
        .lines()
        .group_by(|item| item.is_empty());

    let calories: Vec<_> = data
        .into_iter()
        .map(|(_, c)| c.filter_map(|c| c.parse::<i64>().ok()))
        .map(|c| c.sum::<i64>())
        .collect();

    println!("{:?}", calories.into_iter().max().expect("cannot get"))
}
