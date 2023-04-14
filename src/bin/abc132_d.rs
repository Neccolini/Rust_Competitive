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
#[fastout()]
fn main() {
    input! {
        n:usize,k:usize,
    }
    let Mod = 1000000007;
    let mut c = vec![vec![0; 2500]; 2500];
    for i in 0..2500 {
        c[i][0] = 1;
    }
    for i in 0..2020 {
        for j in 0..2020 {
            c[i + 1][j + 1] = (c[i][j + 1] + c[i][j]) % Mod;
        }
    }
    for i in 1..k + 1 {
        let c1: i64 = c[n - k + 1][i] as i64;
        let c2: i64 = c[k - 1][i - 1] as i64;
        println!("{}", (c1 * c2) % Mod);
    }
}
