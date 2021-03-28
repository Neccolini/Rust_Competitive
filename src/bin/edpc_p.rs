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
const MOD:u64 = 1000000007;
#[allow(dead_code)]
const MAX:usize = 100010;
#[allow(dead_code)]
const INF:i64 = (1<<62) - (1<<31);

fn main() {
    input!{
        n:usize,
        xy:[(usize, usize);n-1],
    }
    let mut g = vec![vec![];n];
    for (x, y) in xy {
        g[x-1].push(y-1);
        g[y-1].push(x-1);
    }
    let mut dp = vec![vec![0;2];n];
    //dp[i][j]:=iを色jで塗る時の、iを頂点とする部分木を塗り分ける場合の数
    //dp[0]を求める
    dfs(0,INF as usize, &mut dp, &g);
    let ans = (dp[0][0] + dp[0][1]) % MOD;
    println!("{}", ans);
}

fn dfs(u:usize, p:usize, dp:&mut Vec<Vec<u64>>, g: &Vec<Vec<usize>>){
    dp[u][0] = 1;
    dp[u][1] = 1;
    for i in g[u].iter() {
        if *i == p {
            continue;
        }
        dfs(*i, u, dp, g);
        dp[u][1] = (dp[u][1] * dp[*i][0]) % MOD;
        dp[u][0] = (dp[u][0] * (dp[*i][0]+dp[*i][1])%MOD)%MOD;
    }
}