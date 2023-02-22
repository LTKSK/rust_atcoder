use proconio::input;
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(usize, usize); q]
    }
    let mut m = vec![];
    m.push(0);
    for (i, v) in a.iter().enumerate() {
        m.push(m[i] + v);
    }
    for (l, r) in lr {
        println!("{}", m[r] - m[l - 1]);
    }
}
