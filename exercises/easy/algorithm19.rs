/*
    Nth Fibonacci Number
    Implement a function to calculate the `n`th Fibonacci number.
    The Fibonacci sequence is defined as follows:
    F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2) for n > 1.

    You need to implement the function `fib(n: i32) -> i32` to return the `n`th Fibonacci number.

    Hint: Consider using matrix exponentiation to solve the problem in O(log n) time complexity.
*/

use std::fmt::{self, Display, Formatter};

pub fn fib(n: i32) -> i32 {
    // TODO: Implement the logic to calculate the nth Fibonacci number using matrix exponentiation
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        // matrix_pow([[1, 1], [1, 0]], n - 2)[0][0] // is the [[1,1],[1,0]] ^ (n-2), need to left-multply [F(2), F(1)] = [1, 1]
        // matrix_pow([[1, 1], [1, 0]], n - 2)[0][0] + matrix_pow([[1, 1], [1, 0]], n - 2)[1][0]
        // =
        matrix_pow([[1, 1], [1, 0]], n - 1)[0][0]
    }
}

pub fn matrix_pow(matrix: [[i32; 2]; 2], mut num: i32) -> [[i32; 2]; 2] {
    let mut res = [[1, 0], [0, 1]]; 
    let mut base = matrix;

    while num > 0 {
        if num % 2 == 1 {
            res = matrix_mult(base, res);
        }
        base = matrix_mult(base, base);
        num /= 2
    }
    res
}

fn matrix_mult(a: [[i32; 2]; 2], b: [[i32; 2]; 2]) -> [[i32; 2]; 2] {
    let mut result = [[0, 0], [0, 0]];
    result[0][0] = a[0][0] * b[0][0] + a[0][1] * b[1][0];
    result[0][1] = a[0][0] * b[0][1] + a[0][1] * b[1][1];
    result[1][0] = a[1][0] * b[0][0] + a[1][1] * b[1][0];
    result[1][1] = a[1][0] * b[0][1] + a[1][1] * b[1][1];
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_1() {
        let result = fib(0);
        println!("Fibonacci of 0: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fib_2() {
        let result = fib(1);
        println!("Fibonacci of 1: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_3() {
        let result = fib(2);
        println!("Fibonacci of 2: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_4() {
        let result = fib(3);
        println!("Fibonacci of 3: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fib_5() {
        let result = fib(10);
        println!("Fibonacci of 10: {}", result);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_fib_6() {
        let result = fib(20);
        println!("Fibonacci of 20: {}", result);
        assert_eq!(result, 6765);
    }
}
