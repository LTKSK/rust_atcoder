use proconio::input;

fn gcd(mut a: usize, mut b: usize) -> usize {
    // どちらかが0になるまで続けて、0になった時のもう片方が答え
    while a > 0 && b > 0 {
        // 大きい値をmod取った値に更新する
        if a >= b {
            a = a % b;
        } else {
            b = b % a;
        }
    }
    if a != 0 {
        a
    } else {
        b
    }
}

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    println!("{}", gcd(a, b))
}
