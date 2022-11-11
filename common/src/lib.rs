pub mod message;

use std::mem::replace;

use num_bigint::BigUint;
use num_traits::{One, Zero};

pub fn fibonacci(n: u128) -> BigUint {
    let mut f0 = Zero::zero();
    let mut f1 = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        // This is a low cost way of swapping f0 with f1 and f1 with f2.
        f0 = replace(&mut f1, f2);
    }
    f0
}

pub fn factorial(n: u128) -> BigUint {
    let mut result = One::one();
    for i in 1..=n {
        result = result * i;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::{factorial, fibonacci};

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(1), 1u32.into());
        assert_eq!(fibonacci(2), 1u32.into());
        assert_eq!(fibonacci(3), 2u32.into());
        assert_eq!(fibonacci(4), 3u32.into());
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1u32.into());
        assert_eq!(factorial(1), 1u32.into());
        assert_eq!(factorial(2), 2u32.into());
        assert_eq!(factorial(3), 6u32.into());
    }
}
