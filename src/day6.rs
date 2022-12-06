// 1 2 3 4 5 6 7 8
// 1 2 3 4
fn day6a(input: &str, window_size: usize) -> usize {
    calculate_position(input, 4)
}

fn day6b(input: &str) -> usize {
    calculate_position(input, 14)
}
fn calculate_position(input: &str, windowSize: usize) -> usize {
    let chars = input.chars().collect::<Vec<char>>();

    let pos = chars
        .windows(windowSize)
        .position(|chunk| {
            return chunk.into_iter().enumerate().all(|(pos, item)| {
                let mut copy = Vec::from(chunk.clone());
                copy.remove(pos);
                return !copy.contains(item);
            });
        })
        .unwrap()
        + windowSize;
    return pos;
}

pub fn day6() {
    println!("result : {}", day6b(include_str!("../inputs/day6")));
}
