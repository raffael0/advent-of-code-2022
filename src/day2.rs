use std::str::Split;

fn calc_score_a(round: &str) -> usize {
    let moves = round.split_once(" ").unwrap();
    let mut result = match moves {
        ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
        ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
        _ => 0,
    };
    result += match moves.1 {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        &_ => 0,
    };
    return result;
}

fn calc_score_b(round: &str) -> usize {
    let moves = round.split_once(" ").unwrap();
    let mut my_move = "";
    let mut result = 0;
    match moves.1 {
        //lose
        "X" => {
            my_move = match moves.0 {
                "A" => "Z",
                "B" => "X",
                "C" => "Y",
                _ => "",
            };
            result = 0;
        }
        //  A X rock
        //  B Y paper
        //  C Z scissors
        //draw
        "Y" => {
            my_move = match moves.0 {
                "A" => "X",
                "B" => "Y",
                "C" => "Z",
                _ => "",
            };
            result = 3;
        }
        //win
        "Z" => {
            my_move = match moves.0 {
                "A" => "Y",
                "B" => "Z",
                "C" => "X",
                _ => "",
            };
            result = 6;
        }
        _ => {}
    }

    result += match my_move {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        &_ => 0,
    };
    println!("{result}");
    return result;
}

fn day2a(input: &str) -> usize {
    input.lines().map(|line| calc_score_a(line)).sum()
}

fn day2b(input: &str) -> usize {
    input.lines().map(|line| calc_score_b(line)).sum()
}

pub fn day2() {
    println!("result : {}", day2b(include_str!("../inputs/day2")));
}
