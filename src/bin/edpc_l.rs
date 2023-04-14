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
const MOD: i64 = 1000000007;
#[allow(dead_code)]
const MAX: usize = 100010;
#[allow(dead_code)]
const INF: i64 = 1 << 62;

fn rec(l: usize, r: usize, n: usize, a: &[i64], dp: &mut Vec<Vec<i64>>) -> i64 {
    if r == l {
        return 0;
    }
    if dp[l][r] != INF {
        return dp[l][r];
    }
    if (n - (r - l)) % 2 == 0 {
        dp[l][r] = max(
            a[l] + rec(l + 1, r, n, &a, dp),
            rec(l, r - 1, n, &a, dp) + a[r - 1],
        );
    } else {
        dp[l][r] = min(
            rec(l + 1, r, n, a, dp) - a[l],
            rec(l, r - 1, n, a, dp) - a[r - 1],
        );
    }
    return dp[l][r];
}

fn main() {
    input! {
        n:usize,
        a:[i64;n],
    }
    //dp[l][r]:=区間[l,r]が残っていときから始めた時のX-Yの値
    let mut dp: Vec<Vec<i64>> = vec![vec![INF; 3010]; 3010];
    rec(0, n, n, &a, &mut dp);

    println!("{}", dp[0][n]);
}
