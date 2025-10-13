// https://atcoder.jp/contests/abc416/tasks/abc416_a
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut line = String::new();
    reader.read_line(&mut line).ok();
    let numbers: Vec<usize> = line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("could not parse"))
        .collect();
    let l = numbers[1];
    let r = numbers[2];

    let mut s = String::new();
    reader.read_line(&mut s).ok();

    // for loop from s[l] to s[r]
    for t in s.chars().skip(l - 1).take(r - l + 1) {
        if t != 'o' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
