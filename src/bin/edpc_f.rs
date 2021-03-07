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
#[allow(unused_macros)]
#[allow(unused_variables)]

fn reverse(t:String)->String {
    t.chars().rev().collect()
}
fn main(){
    input!{
        s: Chars,
        t: Chars,
    }
    // dp[i][j]:=sのi文字目までとtのj文字目までをみたときのLCS
    let mut dp = vec![vec![0;t.len() + 1];s.len() + 1];
    for i in 0..s.len() {
        for j in 0..t.len() {
            if s[i] == t[j] {
                dp[i+1][j+1] = max(dp[i+1][j+1],dp[i][j] + 1);
            } else {
                dp[i+1][j+1] = max(dp[i+1][j], dp[i][j+1]);
            }
        }
    }
    let mut ans:String = String::new();
    let mut i = s.len();
    let mut j = t.len();
    while i > 0 && j > 0 {
        if dp[i-1][j] == dp[i][j] {
            i-=1;
        }
        else if dp[i][j-1] == dp[i][j] {
            j-=1;
        } else {
            ans.push(s[i-1]);
            i-=1;
            j-=1;
        }
    }
    println!("{}", reverse(ans));
}