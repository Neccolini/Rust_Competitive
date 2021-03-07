use proconio::input;
use std::cmp;
fn main() {
    input!{
        n:usize,k:i64,a:[i64;n],
    }
    let ans=solve(n,k,&a);
    println!("{}",ans);
}

fn ceil(a:i64,b:i64)->i64{
    if b==0{
        return 0;
    }
    (a+b-1)/b
}
fn solve(n:usize,k:i64,a:&[i64])->i64{
    let mut max_a:i64=0;
    for i in 0..n{
        max_a=cmp::max(max_a,a[i]);
    }
    let mut right:i64=max_a;
    let mut left:i64=0;
    while right-left>1{
        let x:i64=(right+left)/2;
        let mut cut_times:i64=0;
        for i in 0..n{
            cut_times+=ceil(a[i],x)-1;
        }
        if cut_times>k{
            left=x;
        }
        else {
            right=x;
        }
    }
    right
}