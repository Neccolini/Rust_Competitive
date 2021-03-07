//cargo run --bin

#![allow(unused_imports)]
use std::cmp::*;
use std::num::*;
use std::collections::*;
use itertools::Itertools;
use num::clamp;
use proconio::{input, marker::*, fastout};
use superslice::*;
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[fastout()]
#[allow(unused_variables)]
fn main(){
    input!{
        n:usize,
    }
    let mut k:usize=0;
    for i in 1..n+1{
        if i*(i+1)==2*n{
            k=i;
            break;
        }
    }
    if k==0{
        println!("No");
        return;
    }
    println!("Yes\n{}",k+1);
    let mut ans:Vec<Vec<i32>>=vec![Vec::new();k+1];
    let mut num=1;
    for i in 0..k+1{
        for j in 0..k-i{
            ans[i].push(num);
            num+=1;
        }
    }
    num=1;
    for i in 0..k+1{
        for j in i+1..k+1{
            ans[j].push(num);
            num+=1;
        }
    }
    for i in 0..k+1{
        print!("{} ",k);
        for j in 0..k{
            print!("{} ",ans[i][j]);
        }
        println!();
    }
}