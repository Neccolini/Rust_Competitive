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
        n: usize,
        xy: [(i64, i64);n]
    }
    let (mut x, mut y)= (Vec::with_capacity(n), Vec::with_capacity(n));
    for i in 0..n {
        x.push(xy[i].0);
        y.push(xy[i].1);
    }
    x.sort();
    y.sort();
    let md = (x[n/2], y[n/2]);
    let mut ans = 0;
    for (x, y) in xy {
        ans += (x - md.0).abs() + (y - md.1).abs();
    }
    println!("{}", ans);
}