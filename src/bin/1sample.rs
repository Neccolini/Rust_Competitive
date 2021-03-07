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
#[fastout()]
fn main(){
    input!{
        h:usize,w:usize,
        s:[Chars;h],
        a:[i64;w],
    }
    let mut A=vec![];
    let mut B=vec![];
        for _ in 0..w{
        input!{
            a:i64,b:i64,
        }
        A.push(a);B.push(b);
    }
    A.sort();//昇順
    B.sort_by(|a,b| b.cmp(a));//降順

}