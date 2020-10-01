
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

    let mut dp = vec![0;capacity+1];
    for c in 1..=capacity {
        if data[0][0] <= c {
            dp[c] = data[0][1];
        }
    }

    for i in 1..num_items {
        for c in (1..=capacity).rev() {
            let (weight, profit) = (data[i][0], data[i][1]);
            if weight <= c {
                dp[c] = max(dp[c], profit + dp[c-weight]);
            }
        }
    }

    let result = dp.last().unwrap();
    println!("{}", result);
}
