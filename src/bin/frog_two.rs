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

    let result = rec_memo(num_stones, jumps, stones);
    println!("{}", result);
}

#[allow(dead_code)]
fn rec(num_stones: usize, jumps: usize, stones: Vec<i32>) -> i32 {

    fn go(num_stones: usize, jumps: usize, stones: &Vec<i32>, stone: usize) -> i32 {
        if stone == num_stones - 1 {
            return 0
        }

        let mut cost = std::i32::MAX;

        for i in 1..=jumps {
            let j = stone + i;
            if j <= num_stones - 1 {
                let jump_cost = (stones[stone] - stones[j]).abs();
                cost = min(cost, jump_cost + go(num_stones, jumps, stones, stone+i));
            }
        }

        return cost
    }

    return go(num_stones, jumps, &stones, 0);
}

#[allow(dead_code)]
fn rec_memo(num_stones: usize, jumps: usize, stones: Vec<i32>) -> i32 {

    fn go(num_stones: usize, jumps: usize, stones: &Vec<i32>, stone: usize, dp: &mut Vec<i32>) -> i32 {
        if stone == num_stones - 1 {
            return 0
        }

        if dp[stone] == std::i32::MAX {

            let mut cost = std::i32::MAX;
            for i in 1..=jumps {
                let j = stone + i;
                if j <= num_stones - 1 {
                    let jump_cost = (stones[stone] - stones[j]).abs();
                    cost = min(cost, jump_cost + go(num_stones, jumps, stones, stone+i, dp));
                }
            }

            dp[stone] = cost;
        }

        return dp[stone];
    }

    let mut dp = vec![std::i32::MAX;num_stones+1];
    return go(num_stones, jumps, &stones, 0, &mut dp);
}


#[allow(dead_code)]
fn iter(num_stones: usize, jumps: usize, stones: Vec<i32>) -> i32 {

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

    return dp[num_stones-1]
}
