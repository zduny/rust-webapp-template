use serde::{Serialize, Deserialize};

pub mod message;

#[derive(Debug, Serialize, Deserialize)]
pub struct FibonacciInputError {}

impl std::fmt::Display for FibonacciInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "zero is not a right argument to fibonacci()")
    }
}

impl std::error::Error for FibonacciInputError {}

pub fn fibonacci(n: u128) -> Result<u128, FibonacciInputError> {
    if n == 0 {
        Err(FibonacciInputError {})
    } else if n == 1 {
        Ok(1)
    } else {
        let mut sum = 0;
        let mut last = 0;
        let mut current = 1;
        for _i in 1..n {
            sum = last + current;
            last = current;
            current = sum;
        }
        Ok(sum)
    }
}

pub fn factorial(number: u128) -> u128 {
    match number {
        0 | 1 => 1,
        _ => factorial(number - 1) * number,
    }
}
