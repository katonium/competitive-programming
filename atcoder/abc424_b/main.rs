// https://atcoder.jp/contests/abc416/tasks/abc416_a
use std::io;
use std::io::BufRead;
use std::collections::hash_map::Entry;

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
    // let n = numbers[0];
    let m = numbers[1];
    let k = numbers[2];

    // define map of {str:int}
    let mut msi: std::collections::HashMap<String, usize> = [].into();

    // list of id of the cleared people
    let mut cleared_ids: Vec<usize> = Vec::new();

    // i = 0 to k
    for i in 0..k {
        // read a single line
        let mut s = String::new();
        reader.read_line(&mut s).ok();
        let numbers: Vec<usize> = s
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("could not parse"))
            .collect();
        let ai = numbers[0];
        let ai_s = ai.to_string();

        // check msi[ai] exists
        if !msi.contains_key(&ai_s) {
            // check if m=1 (quick win)
            if m == 1 {
                cleared_ids.push(ai);
                continue;
            }
            // if not, create and set 1
            msi.insert(ai_s, 1);
        } else {
            // if exists, increment by 1
            let count = msi.get(&ai_s).unwrap();
            // check if user wins
            if count + 1 >= m {
                cleared_ids.push(ai);
                continue;
            }
            // if not, update
            msi.insert(ai_s, count + 1);
        }

        // TODO consider Entry API
        // match msi.entry(ai_s) {
        //     Entry::Vacant(entry) => {

    }
    // print cleared ids with whitespace
    let output = cleared_ids.iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
    println!("{}", output);
}
