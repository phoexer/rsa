pub fn mod_exp(x: i64, n: i64, p: i64) -> i64{
    let mut ans: i64 = 1;
    let mut x: i64 = x % p;
    let mut n = n;
    while n > 0 {
        if n & 1 > 0 {
            ans = (ans * x) % p;
        }
        x = x.pow(2) % p;
        n = n >> 1;
    }
    ans
}
