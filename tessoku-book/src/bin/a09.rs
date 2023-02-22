use itertools::Itertools;
use proconio::{input, marker::Usize1};
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(Usize1, Usize1, Usize1, Usize1); n],
    };

    let mut field = vec![vec![0; w + 1]; h + 1];
    for (a, b, c, d) in abcd {
        field[a][b] += 1;
        field[c + 1][d + 1] += 1;
        field[a][d + 1] -= 1;
        field[c + 1][b] -= 1;
    }

    let mut res = vec![vec![0; w + 1]; h + 1];
    for y in 1..=h {
        for x in 1..=w {
            res[y][x] = field[y - 1][x - 1] + res[y][x - 1] + res[y - 1][x] - res[y - 1][x - 1];
        }
    }
    for r in res.iter().skip(1) {
        println!("{}", r.iter().skip(1).join(" "));
    }
}
