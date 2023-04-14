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
    input! { n:usize, k: usize }
    let mut cnt = vec![0; 10000009];
    for i in 2..=n {
        if cnt[i] > 0 {
            continue;
        }
        let mut j = i;
        while j <= n {
            cnt[j] += 1;
            j += i;
        }
    }
    let mut ans = 0;
    for i in 0..=n {
        if cnt[i] >= k {
            ans += 1;
        }
    }
    println!("{}", ans);
}
