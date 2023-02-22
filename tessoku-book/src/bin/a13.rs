use proconio::input;
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mut ans: usize = 0;
    let mut r = 0;
    // lがスタート地点。rがlからスタートしてa[r] < a[l] + k以下になる値がどこまであるかのindex
    for l in 0..n {
        // aは単調増加なので、もしa[r] - a[l] <= kである時、はr-lの範囲の数は条件を満たしている
        // そのため、↓でansに追加して問題ない
        while r < n && a[r] - a[l] <= k {
            r += 1;
        }
        // いくつ進められるか == 答えになる数の個数を追加。-1は自身と同じindexを含まないようにするため
        ans += r - l - 1;
    }
    println!("{}", ans);
}
