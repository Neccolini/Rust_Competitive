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
    input!{n:i64}
    let mut i = 2;
    let mut cnt = 0;
    let mut k = n;
    while i*i <= n {
        while k % i == 0 {
            cnt += 1;
            k /= i;
        }
        i += 1;
    }
    if k != 1 { cnt += 1; }
    
    for i in 0..64 {
        if cnt <= 1<<i {
            println!("{}", i);
            return;
        }
    }
}