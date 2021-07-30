
fn modpow(a:u64, b:u64, m: u64) -> u64 {
    let mut p = 1;
    let mut q = a;
    for i in 0..30 {
        if ( b & (1u64 << i) ) != 0 {
            p *= q;
            p %= m;
        }
        q *= q;
        q %= m;
    }
    p
}