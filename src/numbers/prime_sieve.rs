//An efficient implementation of the Sieve of Eratosthenes

pub fn sieve(max: i64) -> Vec<i64> {
    // precondition, max > 2
    let approx_primes = 3 * max / (approx_int_ln(max) - 1); //should allocate enough positions in almost every case
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

//lightning fast natural log approximation
fn approx_int_ln(n: i64) -> i64 {
    (approx_int_log2(n) * 369) >> 8
    // 369 is used here because it's approximately 2^8/log_2(e)
    // Essentially, we're calculating log_2(n)*2^8*(1/log_2(e)), because we can divide by 2^8 in constant time
    // we're left with an approximation of log_2(n)/log_2(e), aka ln(n)
    // Since we're only using this value for allocation estimates, they can be very rough and general.
    // This application actually doesn't need to be super optimized for performance, since we're only
    // applying it once,but this is a lot easier to implement than a proper ln algorithm
}
