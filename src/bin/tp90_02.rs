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
    input! { n:usize }
    let mut stack = vec![];
    let mut ok = true;
    for bit in 0..1 << n {
        ok = true;
        for i in 0..n {
            if (bit >> i) & 1 == 1 {
                stack.push(true);
            } else {
                if stack.len() == 0 {
                    stack.clear();
                    ok = false;
                    break;
                }
                stack.pop();
            }
        }
        if ok == true && stack.len() == 0 {
            for i in (0..n).rev() {
                if (bit >> i) & 1 == 1 {
                    print!(")");
                } else {
                    print!("(");
                }
            }
            println!();
        }
        stack.clear();
    }
}
