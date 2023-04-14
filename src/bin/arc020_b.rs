#![allow(unused_imports)]
use itertools::Itertools;
use num::clamp;
use proconio::{fastout, input, marker::*};
use std::cmp::*;
use std::cmp::*;
use std::collections::*;
use superslice::*;
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[fastout()]
fn main() {
    input! {
        n:usize,c:i64,
    }
    let mut A = vec![];
    let mut mx_even = -10000000;
    let mut mx_odd = -10000000;
    for i in 0..n {
        input! {
            a:i64,
        }
        if i % 2 == 0 {
            mx_even = max(mx_even, a);
        } else {
            mx_odd = max(mx_odd, a);
        }
        A.push(a);
    }
    let mut even = 0;
    let mut odd = 0;
    for i in 0..n {
        if i % 2 == 0 && A[i] == mx_even {
            even += 1;
        } else if i % 2 == 1 && A[i] == mx_odd {
            odd += 1;
        }
    }
    let ans = c as usize * (n / 2 - even + (n - n / 2 - odd));
    println!("{}", ans);
}
