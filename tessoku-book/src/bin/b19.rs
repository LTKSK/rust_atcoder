use proconio::input;
fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize,usize); n]
    }

    // v[i]の制約のn+1個分
    let V = (n + 1) * 1000;
    let init = std::usize::MAX;
    let mut dp = vec![vec![init; V + 1]; n + 1];
    // 初期化忘れずに
    dp[0][0] = 0;
    for i in 0..n {
        let (w, v) = wv[i];
        for j in 0..=V {
            // ここでinitの値が来たときは、+wの加算ができないのでcontinue
            // 初期値によっては計算できない組み合わせがあるので注意が必要
            // ここがinitの場合、選ばれることはなかった値ということになる
            if dp[i][j] == init {
                continue;
            }
            // i+1で計算すると、イテレーションを0から回せる
            // Vが最大になるように回すので、逆にdp[i][j]に入っているのは重さになる
            // そのため、minを取って一番少ない重みの要素を探すdpになる
            dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);
            // もしwv[i]を採用した場合の重さ
            dp[i + 1][j + v] = dp[i + 1][j + v].min(dp[i][j] + w);
        }
    }

    let mut ans = 0;
    for j in 0..=V {
        if dp[n][j] <= w {
            ans = j;
        }
    }
    println!("{}", ans);
}
