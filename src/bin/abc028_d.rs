use proconio::input;

fn main() {
    input! {
        n:f64,k:f64
    }
    let ans_comb: f64 = 6.0 * (k - 1.0) * (n - k) + 1.0 + 3.0 * (n - 1.0);
    let ans: f64 = ans_comb / n / n / n;
    println!("{}", ans);
}
