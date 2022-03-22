use rand::{thread_rng, Rng};

// Fermat primality test
pub fn prime_test(n: i64, mut tests: i64) -> bool {
    if n <= 1 || n == 4 {
        return false;
    } else if n <= 3 {
        return true;
    }

    while tests > 0 {
        let a = thread_rng().gen_range(1..n);
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

// Euclid's greatest common divisor algorithm
pub fn gcd(a: i64, b: i64) -> i64 {
    match a % b {
        0 => b,
        c => gcd(b, c),
    }
}

// fast integer exponentiation through recursion
pub fn int_pow(base: i64, exp: i64) -> i64 {
    if exp == 0 {
        1
    } else if exp & 1 == 1 {
        base * int_pow(base, exp - 1)
    } else {
        let t = int_pow(base, exp >> 1);
        t * t
    }
}

// calculates base^pow % modulo in O(logy)
pub fn int_powmod(base: i64, mut pow: i64, modulo: i64) -> i64 {
    let mut res = 1;
    let mut base = base % modulo;
    while pow > 0 {
        if pow & 1 != 0 {
            res = (res * base) % modulo;
        }
        pow >>= 1;
        base = (base * base) % modulo;
    }
    res
}

// integer square root using binary search method
pub fn int_sqrt(x: i64) -> i64 {
    if x == 0 || x == 1 {
        return x;
    }

    let mut lo = 1;
    let mut hi = x;
    let mut ans = 0;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        if mid * mid == x {
            return mid;
        } else if mid <= x / mid {
            lo = mid + 1;
            ans = mid;
        } else {
            hi = mid - 1;
        }
    }
    ans
}

// Default arguments square root, suitable for most cases
pub fn sqrt(n: f64) -> f64 {
    sqrt_params(n, 0.0078125, 50)
}

// Uses the Newton (sometimes called Babylonian or Herod's) method to calculate the square root
// will either go until it's within the margin of error or after the specified number of iterations.
pub fn sqrt_params(n: f64, error: f64, maxiter: usize) -> f64 {
    let mut x = n.log2();
    for _ in 0..maxiter {
        x = (x + (n / x)) / 2.0; // Newton's approximation
        if (n - x * x).abs() < error {
            return x;
        }
    }
    x
}

