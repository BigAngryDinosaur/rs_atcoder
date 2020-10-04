use std::io;
use std::io::prelude::*;
use std::cmp::min;

fn main() {

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let num_stones: usize = lines.next().unwrap().trim().parse().unwrap();
    let stones: Vec<i32> = lines.next().unwrap().split_whitespace().map(|x| x.parse().expect("Parse failed") ).collect();

    rec(num_stones, stones)
}

#[allow(dead_code)]
fn rec(num_stones: usize, stones: Vec<i32>) {

    fn go(num_stones: usize, stones: &Vec<i32>, stone: usize, dp: &mut Vec<i32>) -> i32 {

        if stone == num_stones - 1 {
            return 0
        }

        if dp[stone] == std::i32::MAX {

            let mut cost = std::i32::MAX;
            if stone < num_stones - 1 {
                let c1_cost = (stones[stone] - stones[stone+1]).abs();
                cost =  min(cost, c1_cost + go(num_stones, stones, stone + 1, dp));
            }

            if stone < num_stones - 2 {
                let c2_cost = (stones[stone] - stones[stone+2]).abs();
                cost =  min(cost, c2_cost + go(num_stones, stones, stone + 2, dp));
            }

            dp[stone] = cost;
        }

        return dp[stone]
    }

    let mut dp = vec![std::i32::MAX;num_stones];
    let result = go(num_stones, &stones, 0, &mut dp);
    println!("{}", result);
}

#[allow(dead_code)]
fn iter(num_stones: usize, stones: Vec<i32>) {

    if num_stones < 2 {
        return
    }

    let (mut x, mut y) = (0, (stones[1] - stones[0]).abs());

    for i in 2..num_stones {
        let temp = min(x + (stones[i-2] - stones[i]).abs(),
                       y + (stones[i-1] - stones[i]).abs());
        x = y;
        y = temp;
    }

    println!("{}", y);
}
