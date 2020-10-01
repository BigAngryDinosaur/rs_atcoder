use std::io;
use std::io::prelude::*;
use std::cmp::max;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines().map(|l| l.unwrap());

    let s1 = lines.next().unwrap().trim().as_bytes().to_owned();
    let s2 = lines.next().unwrap().trim().as_bytes().to_owned();

    let (m, n) = (s1.len(), s2.len());
    let mut dp: Vec<Vec<usize>> = vec![vec![0;n+1];m+1];

    let mut ans = 0;

    for i in 1..=m {
        for j in 1..=n {
            if s1[i-1] == s2[j-1] {
                dp[i][j] = dp[i-1][j-1] + 1;
            } else {
                dp[i][j] = max(dp[i-1][j], dp[i][j-1])
            }

            ans = max(ans, dp[i][j]);
        }
    }

    let (mut i, mut j, mut k) = (m, n, ans);
    let mut res: Vec<u8> = vec![0;ans];
    while i > 0 && j > 0 {
        if s1[i-1] == s2[j-1] {
            res[k-1] = s1[i-1];
            i -= 1;
            j -= 1;
            if k > 0 {k -= 1;}
        } else if dp[i-1][j] > dp[i][j-1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    println!("{}", String::from_utf8(res).expect(""));

}
