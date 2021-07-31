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

fn b_search(a: &Vec<i64>, x: i64) -> i64 {
    let (mut left, mut right) = (0, a.len() - 1);
    let mut mid;
    while left + 1 < right {
        mid = (left + right) / 2;
        if x < a[mid] {
            right = mid;
        }
        else {
            left = mid;
        }
    }
    min(a[right] - x, x - a[left]).abs()
}

fn main() {
    input!{
        n:usize,
        mut a:[i64;n],
        q: usize
    }
    a.sort();
    for _ in 0..q {
        input!{b:i64}
        println!("{}",b_search(&a, b));
    }
}