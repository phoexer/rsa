pub fn inverse(a: i64, n: i64) -> i64 {
    let (mut t, mut new_t, mut r, mut new_r) = (0, 1, n, a);

    loop {
        if new_r == 0 {
            if r > 1 {
                panic!("a is not invertible");
            }
            if t < 0 {
                t = t + n;
            }
            return t;
        }
        let quotient = r / new_r;
        (t, new_t) = (new_t, t - quotient * new_t);
        (r, new_r) = (new_r, r - quotient * new_r);
    }
}

#[cfg(test)]
mod inverse_test {
    use super::*;

    #[test]
    fn test_inverse() {
        assert_eq!(inverse(3, 26), 9);
        assert_eq!(inverse(17, 780), 413);
    }
}
