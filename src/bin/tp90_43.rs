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
    input! {
        h:usize, w:usize,
        s:(Usize1, Usize1),
        d:(Usize1, Usize1),
        m:[[char;w];h],
    }
    let mut dp = vec![vec![INF; w]; h];
    dp[s.0][s.1] = 0;
    let mut q = VecDeque::new();
    q.push_back(s);
    while let Some((y, x)) = q.pop_front() {
        let d = dp[y][x] + 1;
        for &(dy, dx) in [(1, 0), (0, 1), (!1, 0), (0, !1)].iter() {
            let mut y = y + dy;
            let mut x = x + dx;
            while y < h && x < w && m[y][x] == '.' {
                if dp[y][x] < d {
                    break;
                }
                if dp[y][x] > d {
                    dp[y][x] = d;
                    q.push_back((y, x));
                }
                x += dx;
                y += dy;
            }
        }
    }
    println!("{}", dp[d.0][d.1] - 1);
}
