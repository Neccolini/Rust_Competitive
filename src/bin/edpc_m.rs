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
const MAX: usize = 100010;
fn main() {
    input! {
        n:usize, k:usize,
        a:[i64;n],
    }
    // dp[i][j]:=i人目まででj個配る配り方
    let mut dp: Vec<Vec<i64>> = vec![vec![0; k + MAX]; n + 1];
    dp[0][0] = 1;
    let mut sum: Vec<i64> = vec![0; k + 10];

    for i in 0..n {
        for j in 0..=k {
            sum[j + 1] = (sum[j] + dp[i][j]) % MOD;
        }
        for j in 0..=k {
            dp[i + 1][j] += (sum[j + 1] - sum[max(0, j as i64 - a[i]) as usize] + MOD) % MOD;
            dp[i + 1][j] %= MOD;
        }
    }
    println!("{}", dp[n][k]);
}
