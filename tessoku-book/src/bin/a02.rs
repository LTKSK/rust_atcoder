use proconio::input;
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n]
    }
    for v in a {
        if v == x {
            println!("Yes");
            std::process::exit(0);
        }
    }
    println!("No");
}
