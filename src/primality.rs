pub fn is_prime(number: i64) -> bool {
    if number <= 3 {
        return number > 1;
    }
    if number % 2 == 0 || number % 3 == 0 {
        return false;
    }
    let upper_limit = (number as f64).sqrt().floor() as i64;
    for i in (5..=upper_limit).step_by(6){
        if number % i == 0 || number % (i + 2) == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_non_primes(){
        let non_primes = [
            4, 6, 8, 9, 10, 12, 14, 15,
            16, 18, 20, 21, 22, 24, 25,
            26, 27, 28, 30, 32, 33, 34,
            35, 36, 38, 39, 40, 42, 44,
            45, 46, 48, 49, 50, 51, 52,
            54, 55, 56, 57, 58, 60, 62,
            63, 64, 65, 66, 68, 69, 70,
            72, 74, 75, 76, 77, 78, 80,
            81, 82, 84, 85, 86, 87, 88,
            90, 91, 92, 93, 94, 95, 96,
            98, 99, 100
        ];
        for number in non_primes {
            assert!(!is_prime(number));
        }
    }

    #[test]
    fn test_primes() {
        let primes = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
            31, 37, 41, 43, 47, 53, 59, 61, 67,
            71, 73, 79, 83, 89, 97
        ];
        for number in primes {
            assert!(is_prime(number));
        }
    }

    #[test]
    fn one_is_not_prime() {
        assert!(!is_prime(1));
    }

    #[test]
    fn large_primes() {
        let _large_primes = [
            7829, 7841, 7853, 7867,
            7873, 7877, 7879, 7883,
            7901, 7907, 7919,
            479001599, 87178291199,
        ];
        for number in _large_primes {
            assert!(is_prime(number));
        }
    }
}
