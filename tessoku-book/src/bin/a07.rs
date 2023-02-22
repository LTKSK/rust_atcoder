use proconio::input;
fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n]
    };

    let mut v = vec![0; d + 1];
    for (l, r) in lr {
        v[l - 1] += 1;
        v[r] -= 1;
    }
    let mut res = 0;
    for i in 0..d {
        res += v[i];
        println!("{}", res);
    }
}
