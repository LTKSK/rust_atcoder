use proconio::input;
fn main() {
    input! {
        n: usize,
    }

    let mut d = n;
    let mut r: String = "".to_string();
    let mut c = 0;

    while d > 1 {
        r = r + &((d % 2).to_string());
        d = d / 2;
        c += 1;
    }

    let mut p = "".to_string();
    for _ in 0..(9 - c) {
        p += "0";
    }
    r = p + &(r + &((d % 2).to_string()))
        .chars()
        .rev()
        .collect::<String>();
    println!("{}", r);
}
