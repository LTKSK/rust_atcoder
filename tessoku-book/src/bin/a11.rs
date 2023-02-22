use proconio::input;
fn main() {
    input! {
        n: i64,
        x: usize,
        a: [usize; n]
    };

    // okが条件を満たす最小になるのを目指す
    let mut ok = n;
    let mut ng = -1_i64;
    while (ng - ok).abs() > 1 {
        let mid = (ok + ng) / 2;
        if a[mid as usize] >= x {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    // indexは0basedでaの値は1basedなので合わせる
    println!("{}", ok + 1);
}
