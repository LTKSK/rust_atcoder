use proconio::input;
fn main() {
    input! {
        n: usize,
        ta: [(char, i64); n]
    };

    let mut ans = 0_i64;

    for (t, a) in ta {
        // 愚直にやるとoverflowする
        // ans = match t {
        //     '+' => ans + a,
        //     '-' => ans - a,
        //     '*' => ans * a,
        //     _ => unreachable!(),
        // };

        ans = match t {
            '+' => ans + a,
            '-' => ans - a,
            '*' => ans * a,
            _ => unreachable!(),
        };
        // 引き算のときにマイナスになると計算がおかしくなるので、マイナスのときは10000を足す
        if ans < 0 {
            ans += 10000;
        }
        // 操作のたびにあまりを取っても結果は変わらない
        // overflow防止のためにloopごとにあまりを取る
        ans %= 10000;

        println!("{}", ans);
    }
}
