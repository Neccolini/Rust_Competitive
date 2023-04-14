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
        n: usize, q: usize,
        mut a: [i64;n],
    }
    let mut shift_cnt: i64 = 0;
    for _ in 0..q {
        input! {
            t:usize, x:i64, y:i64,
        }
        let x: i64 = x - 1;
        let y: i64 = y - 1;
        if t == 1 {
            a.swap(
                ((x + shift_cnt) as usize) % n,
                ((y + shift_cnt) as usize) % n,
            );
        } else if t == 2 {
            shift_cnt = (shift_cnt + n as i64 - 1) % n as i64;
        } else {
            println!("{}", a[((x + shift_cnt) as usize) % n]);
        }
    }
}
