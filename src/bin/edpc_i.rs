//cargo run --bin
#![allow(unused_imports)]
use itertools::Itertools;
use num::clamp;
use proconio::{fastout, input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(unused_variables)]
#[allow(dead_code)]
fn reverse(t: String) -> String {
    t.chars().rev().collect()
}

fn main() {
    input! {
        n:usize,
        p:[f64;n],
    }

    //dp[i][j]:=i枚目まで考えたときj枚が表の確率(i>=j)
    let mut dp = vec![vec![0.; n + 10]; n + 10];
    dp[0][0] = 1.;
    for i in 0..n {
        for j in 0..=i {
            dp[i + 1][j] += dp[i][j] * (1. - p[i]);
            dp[i + 1][j + 1] += dp[i][j] * p[i];
        }
    }
    let mut ans = 0.;
    for i in n / 2 + 1..=n {
        ans += dp[n][i];
    }
    println!("{:.10}", ans);
}
