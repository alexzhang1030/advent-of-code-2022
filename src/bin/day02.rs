use std::collections::HashMap;

fn main() {
    let mapping = HashMap::from([
        ("A", ("X", "Z", "Y")),
        ("B", ("Y", "X", "Z")),
        ("C", ("Z", "Y", "X")),
    ]);
    let score_mapping = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let scores = (6, 3, 0);
    let data = include_str!("./day02_data.txt");
    let data: Vec<_> = data.lines().collect();
    let data: Vec<_> = data
        .iter()
        .map(|item| item.split_ascii_whitespace().collect::<Vec<_>>())
        .collect();

    let mut all_score = 0;
    for item in data {
        let (opponent, me) = (item[0], item[1]);
        let (draw, defeat, beat) = mapping.get(opponent).expect("cannot find mapping");
        let (win_score, draw_score, defeat_score) = scores;
        let score = score_mapping.get(me).expect("cannot get score");

        if me.eq(*draw) {
            all_score += draw_score;
        }

        if me.eq(*beat) {
            all_score += win_score;
        }

        if me.eq(*defeat) {
            all_score += defeat_score;
        }

        all_score += score;
    }

    println!("{}", all_score)
}
