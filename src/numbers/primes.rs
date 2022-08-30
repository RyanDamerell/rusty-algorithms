use super::basic::{gcd, int_powmod};
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
        if gcd(n, a) != 1 {
            return false; //n and a are not co-prime, ergo n is not prime
        }

        if int_powmod(a, n, n) != a {
            return false;
        }

        tests -= 1;
    }
    true
}

//An efficient implementation of the Sieve of Eratosthenes
pub fn sieve(max: i64) -> Vec<i64> {
    // precondition, max > 2
    let approx_primes = (1.375f64 * max.into() / ln(max)).ceil() as usize; //should allocate enough positions in almost every case
    let mut primes = Vec::with_capacity(approx_primes as usize);
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

//helper functions

// fast square root approximation for large integers, O(1)
// Uses the Newton/Babylonian method detailed further in basic.rs
fn approx_int_sqrt(n: i64) -> i64 {
    let mut x = approx_int_log2(n);
    for _ in 0..10 {
        let t = (x + (n / x)) >> 1;
        if t - x < 1 {
            return x;
        }
        x = t;
    }
    x + 1
}

// lightning fast log2 approximation to an integer
fn approx_int_log2(n: i64) -> i64 {
    64 - n.leading_zeros() as i64
    // eventually will be able to replaced by n.log2(), currently experimental
}
