// https://atcoder.jp/contests/abc423/tasks/abc423_b
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    // N
    // L1,L2,..,LN

    // integer n
    let mut n = String::new();
    reader.read_line(&mut n).ok();
    let n: usize = n.trim().parse().expect("could not parse");

    let mut line = String::new();
    reader.read_line(&mut line).ok();
    let numbers: Vec<usize> = line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("could not parse"))
        .collect();

    // println!("numbers={:?}", numbers);

    // i = 0 to n-1
    let mut min1 = -1 as isize;
    let mut max1 = n as isize;
    for i in 0..n-1 {
        // println!("i={}", i);
        // hits 1 (close door), then store i
        if numbers[i] == 1 {
            min1 = i as isize;
            break;
        }
    }
    // println!("min1={}", min1);
    // if min1 == -1, meaning no closed door
    if min1 == -1 {
        println!("0");
        return;
    }
    // i = n-1 to 0
    // println!("rev");
    for i in (1..n).rev() { 
        // println!("i={}", i);
        // hits 1 (close door), then store i
        if numbers[i] == 1 {
            max1 = i as isize;
            break;
        }
    }
    // println!("max1={}", max1);
    // println!("max1={}", max1);
    // println!("min1={}", min1);

    // print closed room count
    println!("{}", max1 - min1);
}
