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
        a:[usize;n],
    }
    //dp[i][j][k]:=残り枚数(1,2,3)=(i,j,k)となるときから寿司がなくなる回数の期待値
    let mut dp = vec![vec![vec![0.; n + 10]; n + 10]; n + 10];
    let (mut one, mut two, mut three) = (0, 0, 0);
    for v in a {
        match v {
            1 => one += 1,
            2 => two += 1,
            3 => three += 1,
            _ => (),
        }
    }
    // i,j,kの順のfor文にすると通らない
    for k in 0..=n {
        for j in 0..=n {
            for i in 0..=n {
                if i + j + k == 0 {
                    continue;
                }
                if i + j + k > n {
                    continue;
                }
                dp[i][j][k] = n as f64;
                if i >= 1 {
                    dp[i][j][k] += (dp[i - 1][j][k]) * (i as f64);
                }
                if j >= 1 {
                    dp[i][j][k] += (dp[i + 1][j - 1][k]) * (j as f64);
                }
                if k >= 1 {
                    dp[i][j][k] += (dp[i][j + 1][k - 1]) * (k as f64);
                }
                dp[i][j][k] /= (i + j + k) as f64;
            }
        }
    }
    println!("{:.9}", dp[one][two][three]);
}
