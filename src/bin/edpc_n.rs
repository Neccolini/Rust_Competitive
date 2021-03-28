//cargo run --bin
#![allow(unused_imports)]
use std::cmp::*;
use std::collections::*;
use itertools::Itertools;
use num::clamp;
use proconio::{input, marker::*, fastout};
use superslice::*;
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(unused_variables)]
#[allow(dead_code)]
const MOD:i64 = 1000000007;
#[allow(dead_code)]
const MAX:usize = 100010;
#[allow(dead_code)]
const INF:i64 = (1<<62) - (1<<31);

fn memo(l:usize, r:usize, dp:&mut Vec<Vec<i64>>, s:&Vec<i64>) ->i64{
        if l == r {
        return 0;
    }
    if dp[l][r] != INF {
        return dp[l][r];
    }
    let mut ans:i64 = INF;
    for i in l..r {
        ans = min(ans, memo(l, i, dp, s)+memo(i+1, r, dp, s));
    }
    dp[l][r] = ans + (s[r+1] - s[l]);
    dp[l][r]
}
fn main() {
    input!{
        n:usize,
        a:[i64;n],
    }
    let mut s = vec![0;n+1];
    for i in 0..n {
        s[i+1] = s[i] + a[i]; //累積和
    }
    // dp[l][r]:=区間[l,r]に相当するスライムが1匹にまとまっているとき、
    // それを分解するために必要な最小コスト
    let mut dp = vec![vec![INF;n+10];n+10];

    println!("{}", memo(0, n-1 ,&mut dp, &s));
}