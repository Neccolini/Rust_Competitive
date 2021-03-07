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
        n:usize, m:usize,
        e:[(usize,usize);m],
    }
    let mut g = vec![vec![]; n];
    for (a, b) in &e{
        let (a_, b_) = (a-1, b-1);
        g[a_].push(b_);
    }
    //dp[v]:=vを始点としたときの最長パス
    let mut dp:Vec<i32> = vec![-1;n+1];
    let mut res = 0;
    for i in 0..n {
        res = max(res, dfs(i, &g, &mut dp));
    }

    println!("{}", res);
}

fn dfs(v: usize, g:&[Vec<usize>], dp:&mut [i32]) -> i32 {
    if dp[v] != -1 {
        return dp[v];
    }
    let mut res = 0;
    for &e in g[v].iter() {
        res = max(res, dfs(e, g, dp) + 1);
    }
    dp[v] = res;
    dp[v]
}