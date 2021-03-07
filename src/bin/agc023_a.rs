use proconio::input;
use std::collections::HashMap;
fn main() {
    input!{
        n:usize,mut a:[i64;n],
    }
    let mut s:Vec<i64>=vec![0;n+1];
    s[0]=0;
    for i in 0..n{
        s[i+1]=s[i]+a[i];
    }
    s.sort();
    let mut ans:i64=0;
    let mut map=HashMap::<i64,i64>::new();
    for &x in s.iter(){
        *map.entry(x).or_insert(0)+=1;
    }
    for (_,v) in &map{
        ans+=v*(v-1)/2;
    }
    println!("{}",ans);
}