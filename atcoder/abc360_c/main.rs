// https://atcoder.jp/contests/abc421/tasks/abc420_b
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    // N
    // N boxes and N items
    let mut line = String::new();
    reader.read_line(&mut line).ok();
    let numbers: Vec<usize> = line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("could not parse"))
        .collect();
    let n = numbers[0];

    // A[i]
    let mut line2 = String::new();
    reader.read_line(&mut line2).ok();
    let a: Vec<usize> = line2
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("could not parse"))
        .collect();

    // W[i]
    let mut line3 = String::new();
    reader.read_line(&mut line3).ok();
    let w: Vec<usize> = line3
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("could not parse"))
        .collect();

    // evaluate each item
    let mut heviest_items: HashMap<i32, i32> = HashMap::new();
    let mut result = 0;
    for i in 0..n {
        let ai = a[i] as i32;
        let wi = w[i] as i32;

        if !heviest_items.contains_key(&ai) {
            println!("{} First box {}: {}", result, ai, wi);
            heviest_items.insert(ai, wi);
            continue;
        }

        let current_weight = heviest_items.get(&ai).unwrap();
        if wi > *current_weight {
            // replace heaviest item and add current_weight to result
            println!("{} Replace box {}: {} -> {}", result, ai, current_weight, wi);
            result += *current_weight;
            heviest_items.insert(ai, wi);
        } else {
            // add wi to result
            println!("{} Keep box {}: {} >= {}", result, ai, current_weight, wi);
            result += wi;
        }
    }

    // output results
    println!("{}", result);
}
