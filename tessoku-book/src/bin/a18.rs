use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n]
    };

    let mut dp = vec![false; s + 1];
    dp[0] = true;
    // 最終的にdp[s]がtrueになるときは答えがYes
    for v in a {
        // 一致してほしいsから下に遡っていく
        // このループはaをs回ループすることになる
        // でもって、ループのたびに、a[i]とjを足してsを超えなければ、dp[a[i]+j]をtrueにしていく
        // dpの配列はbooleanだけど、aで取れる値がindexに対応しているところがちょっと分かりづらい
        // aが2,2,3でsが7の時、dpの配列の長さはs+1で、2を選ぶと0と2番目がtrueになるし、次の2では0,2,4がtrueになる
        for j in (0..=s).rev() {
            if v + j > s {
                continue;
            }
            dp[v + j] =
                dp[j] /* a[i]を選ばなくてもj、つまりsになる組み合わせが作れている */ || dp[v + j]/* a[i-1]の時点で合計が s-a[i]であり、a[i]を選ぶ */;
        }
    }

    println!("{}", if dp[s] { "Yes" } else { "No" });
}
