use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

fn main() {
    input! {
        s: Chars,
        t: Chars
    };

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    for (i, s) in s.iter().enumerate() {
        for (j, t) in t.iter().enumerate() {
            // 文字が同じだったら右下を+1
            // それ以外は、上と左から値が大きいものをもらってくる
            if t == s {
                dp[i + 1][j + 1] = max(dp[i][j] + 1, max(dp[i + 1][j], dp[i][j + 1]));
            } else {
                dp[i + 1][j + 1] = max(dp[i + 1][j], dp[i][j + 1]);
            }
        }
    }
    println!("{}", dp[s.len()][t.len()])
}
