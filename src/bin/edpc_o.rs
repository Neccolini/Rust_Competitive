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
const INF: i64 = (1 << 62) - (1 << 31);

fn main() {
    input! {
        n:usize,
        a:[[usize;n];n],
    }
    // dp[s]:={s:女性の集合}がカップルとなっている場合の数
    let mut dp = vec![0; (1 << n) + 10];
    dp[0] = 1;
    for s in 0..(1 << n) {
        let idx = (s as u64).count_ones() as usize;
        for j in 0..n {
            if ((s >> j) & 1) == 0 && a[idx][j] == 1 {
                dp[s | 1 << j] += dp[s];
                dp[s | 1 << j] %= MOD;
            }
        }
    }
    println!("{}", dp[(1 << n) - 1]);
}
