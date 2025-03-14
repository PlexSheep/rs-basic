// If you're confused what this is: it's an annotation that tells a common linting thing to shut up
// about this program calculating things that should normally be precalculated
//
// If you're still confused, feel free to ignore this, it's not important
#![allow(clippy::eq_op)] // thats the point
#![allow(clippy::identity_op)] // thats the point

fn main() {
    println!("1^1={}", 1 ^ 1);
    println!("0^1={}", 0 ^ 1);
}
