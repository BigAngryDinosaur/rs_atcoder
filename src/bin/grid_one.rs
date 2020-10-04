use std::io;
use std::io::prelude::*;

#[derive(PartialEq)]
enum Part {
    Floor,
    Wall
}

type Grid = Vec<Vec<Part>>;

fn main() {

    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines().map(|l| l.unwrap());

    let line1: Vec<usize> = lines.next().unwrap()
                                      .split_whitespace().map(|n| n.parse().expect("")).collect();

    let rows = line1[0];
    // let columns = line1[1];

    let grid: Grid = lines.take(rows)
                                    .map(|row| row.chars()
                                         .map(|col| match col {
                                             '.' => Part::Floor,
                                             '#' => Part::Wall,
                                             _ => panic!("Invalid char")
                                         }).collect())
                                    .collect();

    let result = memoize(&grid);
    println!("{}", result);
}

#[allow(dead_code)]
fn backtrack(grid: &Grid) -> usize {

    fn go(grid: &Grid, row: usize, col: usize) -> usize {
        let (m, n) = (grid.len(), grid[0].len());

        if row == m || col == n || grid[row][col] == Part::Wall { return 0; }
        if row == m - 1 && col == n - 1 { return 1; }

        let x = 100_000_007;
        let mut count = 0;
        if grid[row][col] == Part::Floor {
            count += go(grid, row + 1, col) % x;
            count += go(grid, row, col + 1) % x;
        }

        count
    }

    go(grid, 0, 0)
}

#[allow(dead_code)]
fn memoize(grid: &Grid) -> i32 {

    fn go(grid: &Grid, row: usize, col: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        if row == m || col == n || grid[row][col] == Part::Wall { return 0; }
        if row == m - 1 && col == n - 1 { return 1; }

        if memo[row][col] == -1 {
            let x = 1_000_000_007;
            let mut count = 0;
            if grid[row][col] == Part::Floor {
                count += go(grid, row + 1, col, memo);
                count += go(grid, row, col + 1, memo);
            }

            memo[row][col] = count % x;
        }

        memo[row][col]
    }

    let (m, n) = (grid.len(), grid[0].len());
    let mut memo = vec![vec![-1;n];m];
    go(grid, 0, 0, &mut memo)
}
