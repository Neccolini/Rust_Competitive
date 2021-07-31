use proconio::{input, marker::*};
fn main() {
    input!{
        n: usize, s: Chars
    }
    let s = s.to_vec();
    let all= (n * (n - 1) / 2) as i64;
    let mut cnt = 1;
    let mut ans = all;
    for i in 0..n-1 {
        if s[i] != s[i+1] {
            ans -= cnt * (cnt-1) / 2;
            cnt = 1;
        }
        else { cnt += 1; }
    }
    if cnt != 1 {
        ans -= cnt * (cnt-1) / 2;
    }

    println!("{}", ans);
}