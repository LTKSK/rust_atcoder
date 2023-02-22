use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
        d: [usize; n],
    }

    let p = a.iter().cartesian_product(b).map(|v| v.0 + v.1);
    let q = c.iter().cartesian_product(d).map(|v| v.0 + v.1);
    // let mut q: Vec<usize> = a.iter().combinations(b.iter()).collect_vec();
    // a,b,c,dからランダムに1つずつ取り出したときに、Kになるものがあるかを判定する
    let p: HashSet<usize> = p.into_iter().collect();
    // bisectでもできるけど、set使ってもできる
    for qv in q {
        if p.contains(&(k - qv)) {
            println!("Yes");
            std::process::exit(0);
        }
    }
    println!("No");
}
