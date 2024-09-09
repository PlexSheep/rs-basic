fn main() {
    let x = 7;
    println!("{}", factorial(x));
}

fn factorial(n: u128) -> u128 {
    (1..n + 1).product()
}

// does the same as product from the stdlib
#[allow(dead_code)]
fn myprod<T>(iter: T) -> u128
where
    T: Iterator<Item = u128>,
{
    #[allow(clippy::unnecessary_fold)] // clippy detects that this is just Iter::product
    iter.fold(1, |acc, v| acc * v)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn works() {
        assert_eq!(factorial(7), 5040);
        assert_eq!(factorial(9), 362880);
    }

    #[test]
    fn myprod_same_as_real() {
        assert_eq!((1..5).product::<u128>(), myprod(1..5));
        assert_eq!((1..30).product::<u128>(), myprod(1..30));
        // assert_eq!((1..50).product::<u128>(), myprod(1..50)); // u128 overflows here
    }
}
