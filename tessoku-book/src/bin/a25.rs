use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h]
    };

    let mut dp = vec![vec![0_usize; w]; h];
    // 出発点に向かう方法は一通り
    dp[0][0] = 1;

    for x in 0..w {
        for y in 0..h {
            if x != 0 && c[y][x] != '#' {
                dp[y][x] += dp[y][x - 1];
            }
            if y != 0 && c[y][x] != '#' {
                dp[y][x] += dp[y - 1][x];
            }
        }
    }

    println!("{}", dp[h - 1][w - 1]);
}
