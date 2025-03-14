//! How to calc the factorial n! with just a single line, no imperative style loops, no vars.
//!
//! See [factorial] for how it's done and [myprod] for how it works under the hood.

/// Just so there is something to be ran :)
pub fn main() {
    let x = 7;
    println!("{}", factorial(x));
}

/// Return the factorial of `n` which is defined as `1 * 2 * 3 * … * n`.
/// <https://en.wikipedia.org/wiki/Factorial>
///
/// Task from rustlings
///
/// Do not use:
/// - early returns (using the `return` keyword explicitly)
///
/// Try not to use:
/// - imperative style loops (for/while)
/// - additional variables
///
/// For an extra challenge, don't use:
/// - recursion
pub fn factorial(n: u128) -> u128 {
    (1..n + 1).product()
}

/// Does the same as [std::iter::Iterator::product] from the stdlib.
///
/// The code for [Iterator::product] can be hard to find because it is implemented with a trait and
/// then a macro. Here is a simple version of it.
#[allow(dead_code)]
pub fn myprod<T>(iter: T) -> u128
where
    T: Iterator<Item = u128>,
{
    #[allow(clippy::unnecessary_fold)] // clippy detects that this is just Iter::product
    iter.fold(1, |acc, v| acc * v)
}

/// That's just my unit tests to confirm it works
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    pub fn works() {
        assert_eq!(factorial(7), 5040);
        assert_eq!(factorial(9), 362880);
    }

    #[test]
    pub fn myprod_same_as_real() {
        assert_eq!((1..5).product::<u128>(), myprod(1..5));
        assert_eq!((1..30).product::<u128>(), myprod(1..30));
        // assert_eq!((1..50).product::<u128>(), myprod(1..50)); // u128 overflows here
    }
}
