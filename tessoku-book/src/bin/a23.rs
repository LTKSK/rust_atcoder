use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize;n]; m],
    }

    // m枚のクーポンを何枚使えばn種類の品物を回収できるか
    // なので、aの最小の組み合わせを見つける必要がある
    // dpはaからいくつか選んだときに無料で変える品物の集合
    // dp[i][S] Sが集合なので、これをbitを使ってうまいこと表現する

    let mut s = vec![];
    for i in 0..m {
        let mut v = 0;
        // ここで集合のbit表現を作る
        for j in 0..n {
            if a[i][j] == 1 {
                v += 1 << j;
            }
        }
        s.push(v);
    }

    let mut dp = vec![vec![1 << 9; 1 << n]; m + 1];
    dp[0][0] = 0;

    for i in 0..m {
        for j in 0..1 << n {
            dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);
            // j | s[i]で、番番目を選んだときのbit表現になる
            // クーポンを使う遷移なので + 1
            dp[i + 1][j | s[i]] = dp[i + 1][j | s[i]].min(dp[i][j] + 1);
        }
    }

    let ans = dp[m][(1 << n) - 1];
    if ans == 1 << 9 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
