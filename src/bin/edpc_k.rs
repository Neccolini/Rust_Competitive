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
#[allow(dead_code)]
#[allow(unused_variables)]
fn reverse(t:String)->String {
    t.chars().rev().collect()
}

fn main() {
    input!{
        N:usize,K:usize,
        a:[usize;N],
    }
    // dp[x]:=のこりx個のときに自分が勝つかどうか
    let mut dp = vec![false;K+10];
    //ループを逆にするとWA
    //
    for j in 0..=K {
        for i in 0..N {
            if j >= a[i] {
                dp[j] |= !dp[j-a[i]];
            }
        }
    }
    if dp[K] == true{
        println!("First");
    }else{
        println!("Second");
    }
}