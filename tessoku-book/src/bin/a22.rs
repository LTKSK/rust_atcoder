use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-1],
    };

    let mut dp = vec![-std::isize::MAX; n + 1];
    dp[1] = 0;

    for i in 1..n {
        let a = a[i - 1];
        let b = b[i - 1];
        dp[a] = dp[a].max(100 + dp[i]);
        dp[b] = dp[b].max(150 + dp[i]);
    }

    println!("{}", dp[n]);
}
