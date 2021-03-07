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

fn main(){
    input!{
        S:Chars,
    }
    let n=S.len();
    for i in 0..n-1{
        if S[i]==S[i+1]{
            println!("{} {}",i+1,i+2);
            return;
        }
    }
    for i in 0..n-2{
        if S[i]==S[i+2]{
            println!("{} {}",i+1,i+3);
            return;
        }
    }
    println!("-1 -1");
    return;
}