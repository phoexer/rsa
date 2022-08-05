mod inverse;
mod mod_exp;
mod numbers;
mod primality;

use crate::inverse::inverse;
use crate::mod_exp::mod_exp;
use crate::numbers::lcm;
use crate::primality::is_prime;

fn main() {
    println!("Simple RSA");

    // Initial Primes
    let p: i64 = 61;
    let q: i64 = 53;
    assert!(is_prime(p));
    assert!(is_prime(q));

    let n = p * q;
    println!("n: {}", n);

    // Carmichael function λ(n) of modulus
    let lambda_n: i64 = lcm(p - 1, q - 1);
    println!("Lambda_n: {}", lambda_n);

    let e: i64 = 17;
    let d: i64 = inverse(e, lambda_n);

    println!("Public Key: (n={}, e={})", n, e);
    println!("Private Key: (c={}, d={})", n, d);

    let message: i64 = 65;
    let encrypted_message: i64 = mod_exp(message, e, n);
    println!("Encrypted message: {}", encrypted_message);

    let decrypted_message = mod_exp(encrypted_message, d, n);
    println!("Decrypted: {}", decrypted_message);
}
