fn day4a(input: &str) -> usize {
    return input
        .lines()
        .filter(|line| {
            let (_p1, _p2) = line.split_once(",").unwrap();
            fn to_pair(inp: &str) -> (usize, usize) {
                (
                    usize::from_str_radix(inp.split_once("-").unwrap().0, 10).unwrap(),
                    usize::from_str_radix(inp.split_once("-").unwrap().1, 10).unwrap(),
                )
            }
            let (p1, p2) = (to_pair(_p1), to_pair(_p2));
            return (p1.0 <= p2.0 && p1.1 >= p2.1) || (p2.0 <= p1.0 && p2.1 >= p1.1);
        })
        .count();
}
fn day4b(input: &str) -> usize {
    return input
        .lines()
        .filter(|line| {
            let (_p1, _p2) = line.split_once(",").unwrap();
            fn to_pair(inp: &str) -> (usize, usize) {
                (
                    usize::from_str_radix(inp.split_once("-").unwrap().0, 10).unwrap(),
                    usize::from_str_radix(inp.split_once("-").unwrap().1, 10).unwrap(),
                )
            }
            let (p1, p2) = (to_pair(_p1), to_pair(_p2));
            let (mut r1, r2) = (
                std::ops::RangeInclusive::new(p1.0, p1.1),
                std::ops::RangeInclusive::new(p2.0, p2.1),
            );
            return r1.any(|num| r2.contains(&num));
        })
        .count();
}

pub fn day4() {
    println!("result : {}", day4b(include_str!("../inputs/day4")));
}
