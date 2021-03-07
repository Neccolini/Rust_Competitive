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
#[allow(dead_code)]
#[allow(unused_variables)]
fn reverse(t:String)->String {
    t.chars().rev().collect()
}

fn main() {
    input!{
        N:usize, W:usize,
        wv:[(usize,usize);N],
    }

    let mut dp = vec![vec![0;W+10];N+10];
    for i in 0..N {
        for i_w in 0..W {
            let (weight, value) = wv[i];
            dp[i+1][i_w] = max(dp[i+1][i_w], dp[i][i_w]);
            if i_w + weight<=W {
                dp[i+1][i_w+weight] = max(dp[i+1][i_w], dp[i][i_w] + value);
            }
        }
    }
    let mut ans = 0;
    for i in 0..=W {
        ans = max(ans, dp[N][i]);
    }
    println!("{}", ans);
}