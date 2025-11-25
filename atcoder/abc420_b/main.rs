// https://atcoder.jp/contests/abc421/tasks/abc420_b
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    // N M
    // N people vote M times
    let mut line = String::new();
    reader.read_line(&mut line).ok();
    let numbers: Vec<usize> = line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("could not parse"))
        .collect();
    let n = numbers[0];
    let m = numbers[1];

    // S[i][j]
    // ex) 1000101 -> vec![1,0,0,0,1,0,1]
    let mut s: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        let mut l = String::new();
        reader.read_line(&mut l).ok();
        let row: Vec<usize> = l
            .trim()
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        s.push(row);
    }

    // count each vote
    let mut points = vec![0; n];
    for j in 0..m {
        let mut count = 0;
        for i in 0..n {
            if s[i][j] == 1 {
                count += 1;
            }
        }
        // everyone votes to 1 or 0
        if count == 0 || count == n {
            // we do not need to update points
            continue;
        }

        if count > n - count {
            // people vote to 1 more than 0
            for i in 0..n {
                if s[i][j] == 0 {
                    points[i] += 1;
                }
            }
        } else {
            for i in 0..n {
                if s[i][j] == 1 {
                    points[i] += 1;
                }
            }
        }
    }

    // output points
    let max = points.iter().max().unwrap();
    let mut first = true;
    for i in 0..n {
        if points[i] == *max {
            if first {
                first = false;
            } else {
                print!(" ");
            }
            print!("{}", i + 1);
        }
    }
    println!();
}

