use proconio::input;
fn main() {
    input! {
        mut n:i64,
    }
    let v: Vec<i64> = func(n);
    let mut ans: i64 = digit(n);
    for i in v {
        ans = std::cmp::min(ans, digit(i));
    }
    println!("{}", ans);
}

fn digit(mut n: i64) -> i64 {
    let mut res: i64 = 0;
    while n > 0 {
        n /= 10;
        res += 1;
    }
    res
}

fn func(n: i64) -> Vec<i64> {
    let mut v: Vec<i64> = Vec::new();
    for i in 2..((n as f64).sqrt() as i64 + 1) {
        if n % i == 0 {
            v.push(n / i);
        }
    }
    v
}
