use proconio::input;
fn main() {
    input! {
        h: usize,
        w: usize,
        xy: [[usize;w]; h],
        q: usize,
        abcd: [(usize,usize,usize,usize); q]
    };

    let mut v = vec![vec![0_usize; w + 1]; h + 1];
    for x in 1..=h {
        for y in 1..=w {
            v[x][y] = v[x][y - 1] + xy[x - 1][y - 1];
        }
    }
    for x in 1..=h {
        for y in 1..=w {
            v[x][y] = v[x - 1][y] + v[x][y];
        }
    }

    // 今回の方法だと、1,1から始まる場合にエラーになっていまう。なのでvはw+1とh+1で作ってそこを0で埋めることで対応している
    for (a, b, c, d) in abcd {
        println!("{}", v[c][d] + v[a - 1][b - 1] - v[a - 1][d] - v[c][b - 1]);
    }
}
