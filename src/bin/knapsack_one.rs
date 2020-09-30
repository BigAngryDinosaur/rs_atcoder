
use std::io;
use std::io::prelude::*;
use std::cmp::max;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines().map(|l| l.unwrap());

    let line1 = lines.next().unwrap().split_whitespace().map(|x| x.parse().expect("")).collect::<Vec<usize>>();
    let num_items = line1[0];
    let capacity = line1[1];

    let data: Vec<Vec<usize>> = lines.take(num_items)
                                     .map(|line| line.split_whitespace()
                                          .map(|x| x.parse().expect(""))
                                          .collect())
                                     .collect();

    let mut dp = vec![vec![0;capacity+1];num_items+1];

    for i in 1..=num_items {
        for c in 1..=capacity {
            let (weight, profit) = (data[i-1][0], data[i-1][1]);
            dp[i][c] = dp[i-1][c];
            if weight <= c {
                dp[i][c] = max(dp[i][c], profit + dp[i-1][c-weight]);
            }
        }
    }

    let result = dp.last().unwrap().last().unwrap();
    println!("{}", result);
}
