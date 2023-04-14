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

fn main() {
    input! {
        mut S:String,
    }
    let m: Vec<&str> = ["dream", "dreamer", "erase", "eraser"].to_vec();
    'outer: loop {
        for s in &m {
            let S_cut = S.trim_end_matches(s);
            if S_cut != S {
                S = S_cut.to_string();
                continue 'outer;
            }
        }
        break;
    }
    println!("{}", if S == "" { "YES" } else { "NO" });
}
