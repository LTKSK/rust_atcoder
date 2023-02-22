use itertools::Itertools;
use proconio::input;

fn lower_bound(arr: &Vec<&usize>, target: usize) -> isize {
    // lower_なのでokは0
    let mut ok = 0_isize;
    let mut ng = arr.len() as isize;
    while (ng - ok).abs() > 1 {
        let mid = (ok + ng) / 2;
        if *arr[mid as usize] > target {
            ng = mid;
        } else {
            ok = mid;
        }
    }
    ok
}
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    // 少なくとも最大値はn以下になるはず
    // 最小値は1以上になるはず
    // 10,16,15みたいな並びは1,2,1,に圧縮できるが
    // 10,16,15,9,8みたいになると1,4,3,2,1 にしないといけない

    // 左右に自分より小さい値がどれだけあるかを覚えておけば良い？
    // 出力であるBiは、Aiである値以下の整数の種類を表示している
    // なのでAiをsortしてdedupした結果のindex+1がBiになるということらしい
    // で、リニアサーチだとN^2になってしまうので、bisectでbに当たるindexを探す
    let sorted = a.clone();
    let mut sorted = sorted.iter().collect_vec();
    sorted.sort();
    sorted.dedup();
    println!(
        "{}",
        a.iter().map(|v| lower_bound(&sorted, *v) + 1).join(" ")
    )
}
