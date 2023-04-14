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

pub struct SCC<'a> {
    g: &'a [Vec<usize>],
    r_g: Vec<Vec<usize>>,
    post_order: VecDeque<usize>,
    used: Vec<bool>,
    pub order: Vec<usize>,
}
impl<'a> SCC<'a> {
    pub fn new(g: &'a [Vec<usize>]) -> Self {
        let n = g.len();
        let mut r_g = vec![vec![]; n];
        for u in 0..n {
            let conn = &g[u];
            for &v in conn {
                r_g[v].push(u);
            }
        }
        Self {
            g,
            r_g,
            post_order: VecDeque::new(),
            used: vec![false; n],
            order: vec![n; n],
        }
    }
    fn dfs(&mut self, u: usize) {
        self.used[u] = true;
        for i in 0..self.g[u].len() {
            let v = self.g[u][i];
            if !self.used[v] {
                self.dfs(v);
            }
        }
        self.post_order.push_front(u);
    }
    fn rdfs(&mut self, u: usize, k: usize) {
        self.used[u] = true;
        self.order[u] = k;
        for i in 0..self.r_g[u].len() {
            let v = self.r_g[u][i];
            if !self.used[v] {
                self.rdfs(v, k);
            }
        }
    }
    pub fn build(&mut self) {
        for v in 0..self.g.len() {
            if !self.used[v] {
                self.dfs(v);
            }
        }
        self.used = vec![false; self.g.len()];
        let mut k = 0;
        for i in 0..self.post_order.len() {
            let v = self.post_order[i];
            if !self.used[v] {
                self.rdfs(v, k);
                k += 1;
            }
        }
    }
}

fn main() {
    input! {
        n:usize, m:usize,
    }
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        input! { a:usize, b:usize }
        g[a - 1].push(b - 1);
    }
    let mut scc = SCC::new(&g);
    scc.build();
    let mut map = HashMap::new();
    for i in 0..n {
        *map.entry(scc.order[i]).or_insert(0) += 1;
    }
    let mut cnt: i64 = 0;
    for (_, v) in map {
        cnt += v * (v - 1) / 2;
    }
    println!("{}", cnt);
}
