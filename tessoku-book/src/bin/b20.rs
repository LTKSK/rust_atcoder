use proconio::input;
use proconio::marker::Chars;
use std::cmp::min;

fn main() {
    input! {
        s: Chars,
        t: Chars
    };

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    // 縦と横,0列0行目の初期化処理
    // 一致しなければ単純に値がインクリメントされていくだけ
    for i in 0..=s.len() {
        dp[i][0] = i;
    }
    for j in 0..=t.len() {
        dp[0][j] = j;
    }

    for i in 1..=s.len() {
        for j in 1..=t.len() {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1]) + 1;
            }
        }
    }
    println!("{}", dp[s.len()][t.len()])
}
