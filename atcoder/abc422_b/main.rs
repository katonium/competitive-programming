// https://atcoder.jp/contests/abc423/tasks/abc423_b
use std::io;
use std::io::BufRead;


fn main() {
    let black: char = '#';

    let stdin = io::stdin();
    let mut reader = stdin.lock();

    // H W
    // S11 S12 ... S1W
    // S21 S22 ... S2W
    // ...
    // SH1 SH2 ... SHW


    // load integer h, w
    let mut hw = String::new();
    reader.read_line(&mut hw).ok();
    let mut hw_iter = hw.trim().split_whitespace();
    let h: usize = hw_iter
        .next()
        .unwrap()
        .parse()
        .expect("could not parse");
    let w: usize = hw_iter
        .next()
        .unwrap()
        .parse()
        .expect("could not parse");
    // println!("h={}, w={}", h, w);

    // load grid
    let mut grid: Vec<Vec<char>> = Vec::new();
    for _ in 0..h {
        let mut line = String::new();
        reader.read_line(&mut line).ok();
        let row: Vec<char> = line.trim().chars().collect();
        grid.push(row);
    }
    // println!("grid={:?}", grid);

    // evaluate grid
    for i in 0..h {
        for j in 0..w {
            // println!("i={}, j={}", i, j);
            // println!("grid[{}]={:?}", i, grid[i]);
            // println!("grid[{}][{}]={}", i, j, grid[i][j]);

            // black cell must have 2 or 4 black neighbors

            // if cell is white, skip
            if grid[i][j] != black {
                // println!("grid[{}][{}] is white, skip", i, j);
                continue;
            }

            let mut black_neighbors = 0;
            let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (di, dj) in directions.iter() {
                let ni = i as isize + di;
                let nj = j as isize + dj;
                if ni >= 0 && ni < h as isize && nj >= 0 && nj < w as isize {
                    if grid[ni as usize][nj as usize] == black {
                        black_neighbors += 1;
                    }
                }
            }
            if black_neighbors != 2 && black_neighbors != 4 {
                println!("No");
                return;
            }
            // println!("grid[{}][{}] has {} black neighbors", i, j, black_neighbors);
        }
    }
    println!("Yes");
}

