
use std::io;
use std::io::prelude::*;
use std::cmp::{min, max};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines().map(|l| l.unwrap());

    let line1 = lines.next().unwrap().split_whitespace().map(|x| x.parse().expect("")).collect::<Vec<usize>>();
    let num_items = line1[0];
    let capacity = line1[1];
    let mut max_profit = 0;

    let mut data: Vec<Vec<usize>> = vec![];
    for _ in 0..num_items {
        let values: Vec<usize> = lines.next()
                                      .unwrap()
                                      .split_whitespace()
                                      .map(|x| x.parse().expect(""))
                                      .collect();

        max_profit += values[1];
        data.push(values);
    }

    let mut dp = vec![1000000009;max_profit+1];
    dp[0] = 0;

    let mut res = 0;

    for i in 0..num_items {
        let (weight, profit) = (data[i][0], data[i][1]);
        for p in (profit..=max_profit).rev() {
            dp[p] = min(dp[p], weight + dp[p-profit]);
            if dp[p] <= capacity {
                res = max(res, p);
            }
        }
    }
    println!("{}", res);
}
