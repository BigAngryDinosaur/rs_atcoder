use std::io;
use std::io::prelude::*;
use std::cmp::min;

fn main() {

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let num_stones: usize = lines.next().unwrap().trim().parse().unwrap();
    let stones: Vec<i32> = lines.next().unwrap().split_whitespace().map(|x| x.parse().expect("Parse failed") ).collect();

    if num_stones < 2 {
        println!("{}", 0);
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
