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
const MOD: i64 = 1000000007;
#[allow(dead_code)]
const MAX: usize = 100010;
#[allow(dead_code)]
const INF: i64 = (1 << 62) - (1 << 31);

fn main() {
    input! {
        n:i64, a:i64, b:i64, c:i64
    }
    let mut m: i64 = 10000;
    for i in 0..10000i64 {
        for j in 0..10000i64 {
            let z: i64 = n - (a * i + b * j);
            if z >= 0 && z % c == 0 {
                m = min(z / c + i + j, m);
            }
        }
    }
    println!("{}", m);
}
