use proconio::input;
fn main(){
    input!{
        x:f64,y:f64,n:usize,
        points:[(f64,f64);n],
    }
    let mut ans_min:f64=std::f64::INFINITY;
    for i in 1..n+1{
        let a:f64=points[i%n].1-points[i-1].1;
        let b:f64=-(points[i%n].0-points[i-1].0);
        let c:f64=-points[i-1].0*(points[i%n].1-points[i-1].1)+points[i-1].1*(points[i%n].0-points[i-1].0);
        let distance:f64=(a*x+b*y+c).abs()/(a*a+b*b).sqrt();
        if distance<ans_min{
            ans_min=distance;
        }
    }
    println!("{}",ans_min);
}
