use std::cmp;

pub fn gcd(a: i64, b: i64) -> i64 {
    if a == b {
        return a;
    }
    let (mut a, mut b) = (a, b);
    loop {
        if cmp::min(a, b) == 0 {
            return cmp::max(a, b);
        }
        (a, b) = if a > b { (a % b, b) } else { (a, b % a) };
    }
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a * (b / gcd(a, b))
}

#[cfg(test)]
mod lcm {
    use super::*;

    #[test]
    fn test_60_52() {
        assert_eq!(lcm(60, 52), 780);
    }

    #[test]
    fn test_12_15() {
        assert_eq!(lcm(12, 15), 60);
    }

    #[test]
    fn test_0() {
        assert_eq!(lcm(0, 0), 0);
        assert_eq!(lcm(0, 1), 0);
        assert_eq!(lcm(0, 100), 0);
    }
    #[test]
    fn test_equal() {
        assert_eq!(lcm(12, 12), 12);
    }
}

#[cfg(test)]
mod gcd {
    use super::*;

    #[test]
    fn test_2_2() {
        assert_eq!(gcd(2, 2), 2);
    }

    #[test]
    fn test_42_56() {
        assert_eq!(gcd(42, 56), 14);
        assert_eq!(gcd(56, 42), 14);
        assert_eq!(gcd(-56, 42), 14);
        assert_eq!(gcd(-56, -42), 14);
    }

    #[test]
    fn test_0_0() {
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn test_12_0() {
        assert_eq!(gcd(12, 0), 12);
        assert_eq!(gcd(0, 12), 12);
    }

    #[test]
    fn test_large() {
        assert_eq!(gcd(24826148, 45296490), 526);
        assert_eq!(gcd(45296490, 24826148), 526);
    }
}
