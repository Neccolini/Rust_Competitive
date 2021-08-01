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

fn main() {
    input!{
        w: i64, n: usize,
        dish: [(i64, i64, i64);n]
    }

    let dp = vec![vec![0;100000];600];
    for i in 0..n {
        
    }
}