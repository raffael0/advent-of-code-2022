fn day1a(input: &str) -> usize {
    let a = input
        .split("\n\n")
        .map(|st| {
            st.lines()
                .map(|num| usize::from_str_radix(num, 10).unwrap())
                .sum::<usize>()
        })
        .max()
        .unwrap();
    return a;
}
fn day1b(input: &str) -> usize {
    let mut a = input
        .split("\n\n")
        .map(|st| {
            st.lines()
                .map(|num| usize::from_str_radix(num, 10).unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<usize>>();

    a.sort();

    return a.iter().rev().take(3).sum();
}
pub fn day1() {
    println!("result : {}", day1b(include_str!("../inputs/day1")));
}
