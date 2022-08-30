use rand::{thread_rng, Rng};

// Euclid's greatest common divisor algorithm
pub fn gcd(a: i64, b: i64) -> i64 {
    match a % b {
        0 => b,
        c => gcd(b, c),
    }
}

// fast integer exponentiation through recursion
pub fn int_pow(base: f64, exp: i64) -> f64 {
    if exp == 0 {
        1.0
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

pub fn ln_epsilon(x: f64, epsilon: f64) -> f64 {
    /*double yn = x - 1.0d; // using the first term of the taylor series as initial-value
    double yn1 = yn;

    do
    {
        yn = yn1;
        yn1 = yn + 2 * (x - System.Math.Exp(yn)) / (x + System.Math.Exp(yn));
    } while (System.Math.Abs(yn - yn1) > epsilon);

    return yn1; */

    let y_n = x - 1.0;
    let y_n1 = y_n;

    loop {
        y_n = y_n1;
        y_n1 = y_n + 2 * (x - exp_frac(y_n)) / (x + exp_frac(y_n));
        if (y_n - y_n1).abs() <= epsilon {
            break;
        }
    }
}

pub fn exp_frac(n: f64)->f64 {
    0.999966
        + 1.00039 * n
        + 0.498051 * int_pow(n, 2)
        + 0.171742 * int_pow(n, 3)
        + 0.0343485 * int_pow(n, 4)
        + 0.0137393 * int_pow(n, 5)
} //0.999966 + 1.00039 x + 0.498051 x^2 + 0.171742 x^3 + 0.0343485 x^4 + 0.0137393 x^5
