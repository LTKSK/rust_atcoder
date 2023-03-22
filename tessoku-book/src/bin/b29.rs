use proconio::input;
fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let div = 1000000007;
    let mut ans = 1;
    let mut v = a;
    for i in 0..60 {
        let d = 1 << i;
        if (b / d) % 2 == 1 {
            ans = (ans * v) % div;
        }
        v = (v * v) % div;
    }

    println!("{}", ans);
}
