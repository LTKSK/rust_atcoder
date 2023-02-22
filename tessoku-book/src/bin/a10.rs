use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        d: usize,
        lr: [(usize, usize); d],
    };

    // A[i]は部屋に収容できる人数
    // d日目には、l[d]号室からr[d]号室までを使うことができない
    // 1~d日目までに使える最も大きい部屋はどれかを示す

    // 1次元配列でみた時、使える部屋の最大値は0~l[d]-1までの部屋と、r[d]~nまでの部屋のmax
    // なので、両端から中央に向かってmaxを取った結果を保持しておいて、それで最終的にmaxを求める
    let mut p = vec![0; n + 1];
    p[0] = a[0];
    for i in 1..n {
        p[i] = a[i].max(p[i - 1]);
    }
    let mut q = vec![0; n + 1];
    q[n - 1] = a[n - 1];
    for i in (0..n).rev() {
        q[i] = a[i].max(q[i + 1]);
    }

    for (l, r) in lr {
        let l = l - 1;
        let r = r - 1;
        println!("{}", p[l - 1].max(q[r + 1]));
    }
}
