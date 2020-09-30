use std::io;
use std::io::prelude::*;
use std::cmp::max;

fn main() {

    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines().map(|l| l.unwrap());

    let num_days: usize = lines.next().unwrap().parse().expect("failed to parse num days");
    let activities: Vec<Vec<usize>> = lines.take(num_days)
                          .map(|line| line.split(" ")
                               .map(|x| x.parse().expect("failed to parse activities"))
                               .collect::<Vec<usize>>())
                          .collect();


    let mut dp = activities[0].clone();

    for d in 1..num_days {
        let mut temp = vec![0;3];
        for i in 0..3 {
            for j in 0..3 {
                if i != j {
                    temp[i] = max(temp[i], dp[j] + activities[d][i])
                }
            }
        }
        dp = temp;
    }

    let result = dp.iter().max().unwrap();
    println!("{}", result);
}
