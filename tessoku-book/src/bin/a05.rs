use proconio::input;
fn main() {
    input! {
        n: usize,
        k: usize
    }

    let mut res = 0;
    for u in 1..=n {
        for v in 1..=n {
            if u + v > k {
                continue;
            }
            let t = k - u - v;
            if t >= 1 && t <= n {
                res += 1;
            }
        }
    }
    println!("{}", res);
}
