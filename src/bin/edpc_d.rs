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
#[allow(dead_code)]
#[allow(unused_variables)]
fn reverse(t: String) -> String {
    t.chars().rev().collect()
}

fn main() {
    input! {
        n:usize, w:usize,
        wv:[(usize,usize);n],
    }

    let mut dp = vec![vec![0; w + 10]; n + 10];
    for i in 0..n {
        for i_w in 0..w {
            let (weight, value) = wv[i];
            dp[i + 1][i_w] = max(dp[i + 1][i_w], dp[i][i_w]);
            if i_w + weight <= w {
                dp[i + 1][i_w + weight] = max(dp[i + 1][i_w], dp[i][i_w] + value);
            }
        }
    }
    let mut ans = 0;
    for i in 0..=w {
        ans = max(ans, dp[n][i]);
    }
    println!("{}", ans);
}
