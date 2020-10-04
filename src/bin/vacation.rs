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

    let result = memoize(num_days, activities);
    println!("{}", result);
}

#[allow(dead_code)]
fn backtrack(num_days: usize, activities: Vec<Vec<usize>>) -> usize {

    fn go(num_days: usize, activities: &Vec<Vec<usize>>, day: usize, activity: i32) -> usize {
        if day == num_days { return 0 }

        let mut happiness = 0;
        for (i, a) in activities[day].iter().enumerate() {
            let i = i as i32;
            if i == activity { continue; }
            happiness = max(happiness, a + go(num_days, activities, day+1, i));
        }

        happiness
    }

    go(num_days, &activities, 0, -1)
}

#[allow(dead_code)]
fn memoize(num_days: usize, activities: Vec<Vec<usize>>) -> usize {

    fn go(num_days: usize, activities: &Vec<Vec<usize>>, day: usize, activity: usize, memo: &mut Vec<Vec<usize>>) -> usize {
        if day == num_days { return 0 }

        let mut activity = activity;
        if activity == std::usize::MAX || memo[day][activity] == std::usize::MAX {
            let mut happiness = 0;
            for (i, a) in activities[day].iter().enumerate() {
                if i == activity { continue; }
                happiness = max(happiness, a + go(num_days, activities, day+1, i, memo));
            }

            if activity == std::usize::MAX {
                activity = 0
            }

            memo[day][activity] = happiness;
        }

        memo[day][activity]
    }

    let mut memo = vec![vec![std::usize::MAX;activities[0].len()];num_days];
    go(num_days, &activities, 0, std::usize::MAX, &mut memo)
}

#[allow(dead_code)]
fn iter(num_days: usize, activities: Vec<Vec<usize>>) -> usize {

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

    return dp.iter().max().unwrap().clone();
}
