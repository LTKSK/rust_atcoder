use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut ans = 0;
    let mut dp = vec![std::usize::MAX; n + 1];
    dp[0] = 0;
    for a in a {
        let pos = dp[0..=ans].lower_bound(&a);
        if dp[pos] > a {
            dp[pos] = a;
        }
        if pos > ans {
            ans = pos;
        }
    }
    println!("{}", ans);
}
