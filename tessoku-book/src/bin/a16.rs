use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-2],
    }

    let mut res = vec![0; n];
    // 部屋0から部屋1に移動するのに、a[0]だけかかる
    res[1] = a[0];
    for i in 2..n {
        // 配列の長さに差があるのでindexを調整している
        let ai = i - 1;
        let bi = i - 2;
        // 部屋i-1から部屋iまではa[ai]かかる
        // 部屋i-2から部屋iまではb[bi]かかる
        // 最短時間を記録
        res[i] = (res[i - 1] + a[ai]).min(res[i - 2] + b[bi]);
    }

    // 最後の部屋の値をみれば、最短時間がわかる
    println!("{:?}", res[n - 1]);
}
