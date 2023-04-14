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
        n:usize,
    }
    let mut A = vec![];
    let mut B = vec![];
    for _ in 0..n {
        input! {
            a:i64,b:i64,
        }
        A.push(a);
        B.push(b);
    }
    A.sort();
    B.sort();
    if n % 2 == 1 {
        let mi = A[n / 2];
        let ma = B[n / 2];
        println!("{}", ma - mi + 1);
    } else {
        let mi = A[n / 2 - 1] + A[n / 2];
        let ma = B[n / 2 - 1] + B[n / 2];
        println!("{}", ma - mi + 1);
    }
}
