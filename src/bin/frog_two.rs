use std::cmp::min;
use std::io;
use std::io::prelude::*;

fn main() {

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let line1: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().expect("error") ).collect();
    let num_stones = line1[0];
    let jumps = min(line1[1], num_stones-1);
    let stones: Vec<i32> = lines.next().unwrap().split_whitespace().map(|x| x.parse().expect("Parse failed") ).collect();

    let mut dp = vec![std::i32::MAX; num_stones];
    dp[0] = 0;

    for i in 0..num_stones {
        for j in 1..=jumps {
            if i + j < num_stones {
                dp[i+j] = min(dp[i+j],
                              dp[i] + (stones[i] - stones[i+j]).abs());
            }
        }
    }

    println!("{}", dp[num_stones-1]);
}
