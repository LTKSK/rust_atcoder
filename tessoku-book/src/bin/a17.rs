use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-2],
    }

    let mut res = vec![];

    res.push(0);
    res.push(a[0]);
    for i in 2..n {
        // 配列の長さに差があるのでindexを調整している
        let ai = i - 1;
        let bi = i - 2;
        res.push((res[i - 1] + a[ai]).min(res[i - 2] + b[bi]));
    }

    let mut ans: Vec<usize> = vec![];
    let mut idx = n - 1;
    ans.push(idx + 1);
    while idx > 0 {
        // 配列の長さに差があるのでindexを調整している
        let ai = idx - 1;
        if res[idx] == (res[idx - 1] + a[ai]) {
            idx -= 1;
        } else {
            idx -= 2;
        }
        ans.push(idx + 1);
    }
    ans.reverse();
    println!("{:?}", ans.len());
    println!("{}", ans.iter().join(" "));
}
