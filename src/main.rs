mod primality;
mod numbers;
mod inverse;
mod mod_exp;

use crate::primality::is_prime;
use crate::inverse::inverse;
use crate::numbers::lcm;
use crate::mod_exp::mod_exp;

fn main() {
    println!("Simple RSA");

    // Initial Primes
    let p:i64 = 61;
    let q:i64 = 53;
    assert!(is_prime(p));
    assert!(is_prime(q));

    let modulus = p * q;
    println!("Modulus: {}", modulus);

    // Carmichael function λ(n) of modulus
    let lambda_modulus: i64 = lcm(p-1, q-1);
    println!("Lambda_modulus: {}", lambda_modulus);

    let e: i64 = 17;
    let d: i64 = inverse(e, lambda_modulus);

    println!("Public Key: (n={}, e={})", modulus, e);
    println!("Private Key: (c={}, d={})", modulus, d);

    let message: i64 = 65;
    let encrypted_message:i64  = mod_exp(message, e, modulus);
    println!("Encrypted message: {}", encrypted_message);


    let decrypted_message = mod_exp(encrypted_message, d, modulus);
    println!("Decrypted: {}", decrypted_message);
}
