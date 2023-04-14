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
#[allow(dead_code)]
#[allow(unused_variables)]
fn reverse(t: String) -> String {
    t.chars().rev().collect()
}

fn main() {
    input! {
        n:usize,k:usize,
        a:[usize;n],
    }
    // dp[x]:=のこりx個のときに自分が勝つかどうか
    let mut dp = vec![false; k + 10];
    //ループを逆にするとWA
    //
    for j in 0..=k {
        for i in 0..n {
            if j >= a[i] {
                dp[j] |= !dp[j - a[i]];
            }
        }
    }
    if dp[k] == true {
        println!("First");
    } else {
        println!("Second");
    }
}
