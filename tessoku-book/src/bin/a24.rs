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
        // aを挿入できる最大のindexがposとして返る
        let pos = dp[0..=ans].lower_bound(&a);
        // dp[i]は最後の要素がaである部分列の最大の長さなので、dp[i]の要素よりaが小さかったら更新する
        if dp[pos] > a {
            dp[pos] = a;
        }
        // 最大値が伸ばせる場合は更新
        if pos > ans {
            ans = pos;
        }
    }
    println!("{}", ans);
}
