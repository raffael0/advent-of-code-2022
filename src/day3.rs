fn charToPriority(char: char) -> usize {
    if char.is_uppercase() {
        char as usize - 38
    } else {
        char as usize - 96
    }
}
fn day3a(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            let diff: char = a
                .chars()
                .filter(|char_a| b.contains(*char_a))
                .nth(0)
                .unwrap();
            return charToPriority(diff);
        })
        .sum()
}

fn day3b(input: &str) -> usize {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            let (a, b, c) = (chunk[0], chunk[1], chunk[2]);
            let diff: char = a
                .chars()
                .filter(|char_a| b.contains(*char_a))
                .filter(|char_a| c.contains(*char_a))
                .nth(0)
                .unwrap();
            return charToPriority(diff);
        })
        .sum()
}
pub fn day3() {
    println!("result : {}", day3b(include_str!("../inputs/day3")));
}
