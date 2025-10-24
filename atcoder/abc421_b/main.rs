// https://atcoder.jp/contests/abc421/tasks/abc421_b
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    // X Y
    let mut line = String::new();
    reader.read_line(&mut line).ok();
    let numbers: Vec<usize> = line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("could not parse"))
        .collect();

    let mut ai_2 = numbers[0];
    let mut ai_1 = numbers[1];
    for i in 3..11 {
        // println!("{}: {} {} -> {}({})", i, ai_1, ai_2, ai_1 + ai_2, rev(ai_1 + ai_2));
        let ai = rev(ai_1 + ai_2);
        ai_2 = ai_1;
        ai_1 = ai;
    }
    println!("{}", ai_1);
}

// rev reverse input integer
fn rev(n: usize) -> usize {
    let s: String = n.to_string().chars().rev().collect();
    s.parse().unwrap()
}
