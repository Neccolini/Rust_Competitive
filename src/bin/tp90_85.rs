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
        k: u128
    }

    let mut lst = Vec::new();
    let mut i = 1;
    while i * i <= k {
        if k % i == 0 {
            lst.push(i);
            lst.push(k / i);
        }
        i += 1;
    }
    let mut lst = lst.iter().unique().collect_vec();
    lst.sort();
    let mut cnt: u128 = 0;
    for i in 0..lst.len() {
        for j in i..lst.len() {
            let (x, y) = (*lst[i], *lst[j]);
            if k / x < y || k % (x * y) != 0u128 {
                continue;
            }
            let z = k / (x * y);
            if z < y {
                continue;
            }
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
