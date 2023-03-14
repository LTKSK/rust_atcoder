use proconio::input;

fn is_prime(x: usize) -> bool {
    let mut d = 2;
    // 探索範囲は√x以下で十分
    while d * d <= x {
        if x % d == 0 {
            return false;
        }
        d += 1;
    }
    return true;
}

fn main() {
    input! {
        q: usize,
        x: [usize; q]
    };

    for v in x {
        if is_prime(v) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
