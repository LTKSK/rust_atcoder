use proconio::input;

// t分のときに、kよりもprint枚数が多ければtrue
fn check(t: i64, a: &[i64], k: i64) -> bool {
    let mut sum = 0;
    for v in a {
        sum += t / v;
    }
    sum >= k
}
fn main() {
    input! {
        n: i64,
        k: i64,
        a: [i64; n],
    };

    let mut ok = 10_i64.pow(9) + 1;
    let mut ng = -1;

    while (ng - ok).abs() > 1 {
        let mid = (ok + ng) / 2;
        if check(mid, &a, k) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
