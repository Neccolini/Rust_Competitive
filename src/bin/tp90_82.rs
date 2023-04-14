//cargo run --bin
#![allow(unused_imports)]
use itertools::Itertools;
use num::clamp;
use proconio::{fastout, input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;
#[allow(dead_code)]
const MOD: u64 = 1000000007;
#[allow(dead_code)]
const MODI: i64 = 1000000007;
#[allow(dead_code)]
const MAX: usize = 100010;
#[allow(dead_code)]
const INF: i64 = (1 << 62) - (1 << 31);

fn modpow(a: i64, b: i64, m: i64) -> u64 {
    let mut p = 1;
    let mut q = a;
    for i in 0..30 {
        if (b & (1i64 << i)) != 0 {
            p *= q;
            p %= m;
        }
        q *= q;
        q %= m;
    }
    p as u64
}

fn div(a: u64, b: u64, m: u64) -> u64 {
    (a * modpow(b as i64, m as i64 - 2, m as i64)) % m
}

fn f(x: u64) -> u64 {
    let v1 = x % MOD;
    let v2 = (x + 1) % MOD;
    let v = v1 * v2 % MOD;
    div(v, 2, MOD)
}

fn main() {
    input! {
        l:u64, r:u64
    }
    let mut power10 = vec![0; 22];
    power10[0] = 1;
    for i in 0..20 {
        power10[i] = 10u64 * power10[i - 1];
    }
    let mut ans = 0;
    for i in 1..20 {
        let vl = max(l, power10[i - 1]);
        let vr = min(r, power10[i] - 1u64);
        if vl > vr {
            continue;
        }
        let val = (f(vr) as i64 - f(vl - 1) as i64 + MODI) % MODI;
        ans += 1 * i as u64 * val as u64;
        ans %= MOD;
    }
    println!("{}", ans);
}
