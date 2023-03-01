use proconio::input;
use std::cmp::max;
fn main() {
    input! {
        n: usize,
        pa: [(usize, usize); n],
    };

    // 区間DPというらしい
    // dp[l][r]までのブロックがあるときに最大何点か、というのを考える
    let mut dp = vec![vec![0; n]; n];

    for l in 0..n {
        // 両端から取っていくときはこう
        for r in (l..n).rev() {
            if l > 0 {
                let (p, a) = pa[l - 1];
                // pの値は1originなので-1して調整
                let p = p - 1;
                // pの値はlより大きくてrより小さく無いと得点にならない
                let score = if l <= p && p <= r { a } else { 0 };
                // l-1からブロックを取るので、l-1の値にscoreを加算した結果が現在の値より高くなるかを比較
                dp[l][r] = max(dp[l][r], dp[l - 1][r] + score);
            }

            if r < n - 1 {
                let (p, a) = pa[r + 1];
                let p = p - 1;
                let score = if l <= p && p <= r { a } else { 0 };
                // ↑の逆
                dp[l][r] = max(dp[l][r], dp[l][r + 1] + score);
            }
        }
    }
    let mut ans = 0;
    for i in 0..n {
        ans = max(ans, dp[i][i]);
    }
    println!("{}", ans);
}
