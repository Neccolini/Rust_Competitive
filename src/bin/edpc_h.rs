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
const MOD:u32 = 1000000007;
#[allow(dead_code)]
fn reverse(t:String)->String {
    t.chars().rev().collect()
}

fn main() {
    input!{
        h:usize, w:usize,
        grid:[Chars;h],
    }
    let mut dp = vec![vec![0;w+1];h+1];
    dp[0][0] = 1;
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '#' {
                continue;
            }
            if i >= 1 {
                if grid[i-1][j] == '.' {
                    dp[i][j] += dp[i-1][j];
                    dp[i][j] %= MOD;
                }
            }
            if j >= 1 {
                if grid[i][j-1] == '.' {
                    dp[i][j] += dp[i][j-1];
                    dp[i][j] %= MOD;
                }
            }
        }
    }
    println!("{}", dp[h-1][w-1]);
}