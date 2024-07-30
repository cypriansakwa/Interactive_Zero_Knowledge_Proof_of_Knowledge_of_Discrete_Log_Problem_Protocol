extern crate num_bigint;
extern crate rand;

use num_bigint::{BigInt, RandBigInt};
use num_traits::One;
use rand::{thread_rng, Rng};

fn mod_exp(base: &BigInt, exponent: &BigInt, modulus: &BigInt) -> BigInt {
    base.modpow(exponent, modulus)
}

fn generate_random_bigint(n: &BigInt) -> BigInt {
    let mut rng = thread_rng();
    rng.gen_bigint_range(&BigInt::one(), n)
}

fn main() {
    // Public parameters
    let n = BigInt::parse_bytes(b"691752902839270113277", 10).unwrap(); // example value of n
    let e = BigInt::parse_bytes(b"65537", 10).unwrap(); // example value of e
    let c = BigInt::parse_bytes(b"123456789", 10).unwrap(); // example value of c

    // Cyprian's secret
    let m = BigInt::parse_bytes(b"987654321", 10).unwrap(); // example value of m
    // let _d = BigInt::parse_bytes(b"123456789", 10).unwrap(); // example value of d, unused

    // Step 1: Commitment Phase
    let r = generate_random_bigint(&n);
    let r_val = mod_exp(&r, &e, &n);

    println!("Cyprian sends R: {}", r_val);

    // Step 2: Challenge Phase
    let mut rng = thread_rng();
    let b: u8 = rng.gen_range(0..2); // Randomly selects 0 or 1
    println!("Alex sends challenge b: {}", b);

    // Step 3: Response Phase
    if b == 0 {
        // Cyprian sends r
        println!("Cyprian sends r: {}", r);
        // Alex verifies R ≡\u{2261} r^e mod n
        let check_r = mod_exp(&r, &e, &n);
        println!("Verification: R \u{2261} r^e mod n: {}", check_r == r_val);
    } else if b == 1 {
        // Cyprian sends s = r * M mod n
        let s = (&r * &m) % &n;
        println!("Cyprian sends s: {}", s);
        // Alex verifies R * C \u{2261} s^e mod n
        let check_s = mod_exp(&s, &e, &n);
        let check_rc = (&r_val * &c) % &n;
        println!("Verification: R * C \u{2261} s^e mod n: {}", check_s == check_rc);
    }
}




