use super::basic::{gcd, int_powmod, ln_epsilon};
use bit_vec::BitVec;
use rand::Rng;

// Fermat primality test
pub fn prime_test(n: i64, mut tests: i64) -> bool {
    if n <= 1 || n == 4 {
        return false;
    } else if n <= 3 {
        return true;
    }

    while tests > 0 {
        let a = rand::thread_rng().gen_range(1..n);

        if gcd(n, a) != 1 || int_powmod(a, n, n) != a {
            return false; //n and a are not co-prime, or a^n doesn't pass fermat test
        }

        tests -= 1;
    }
    true
}

//An implementation of the Sieve of Eratosthenes, inspired by haskell
pub fn prime_sieve(max: i64) -> Vec<i64> {
    // precondition, max > 2
    let approx_primes = (1.375f64 * (max as f64) / ln_epsilon(max as f64, 0.01)).ceil() as usize; //should allocate enough positions in almost every case
    let mut primes = Vec::with_capacity(approx_primes);
    primes.push(2); //start with 2 right off the bat
    let mut current = 3; //first calculated value is 3
    while current < max {
        let mut i = 0;
        let mut flag = true;
        while flag && primes[i] <= approx_int_sqrt(current) {
            flag = current % primes[i] != 0;
        }
        if flag {
            primes.push(current);
        }
        current += 2; //since every even number is automatically out, increment by 2
    }
    primes.shrink_to_fit();
    primes
}

//A more traditional implementation of the Sieve of Eratosthenes
pub fn traditional_sieve(max: i64) -> Vec<i64> {
    // initialize the bitvec with all even numbers disqualified automatically for efficiency
    let mut bits = BitVec::from_bytes(vec![0b01010101; max as usize].as_slice());
    bits.set(1, false);
    bits.set(2, true);
    for i in 0..bits.len() {
        if bits[i] {
            let mut k = i * 2;
            while max > k as i64 {
                bits.set(k, false);
                k += i;
            }
        }
    }
    let approx_primes = (1.375f64 * (max as f64) / ln_epsilon(max as f64, 0.01)).ceil() as usize;
    let mut out = Vec::with_capacity(approx_primes);
    out.extend(
        bits.iter()
            .enumerate()
            .filter_map(|(i, b)| if b { Some(i as i64) } else { None }),
    );
    out
}

//helper functions

// fast square root approximation for large integers, O(1)
// Uses the Newton/Babylonian method detailed further in basic.rs
fn approx_int_sqrt(n: i64) -> i64 {
    let mut x = n.ilog2() as i64;
    for _ in 0..10 {
        let t = (x + (n / x)) >> 1;
        if t - x < 1 {
            return x;
        }
        x = t;
    }
    x + 1
}
