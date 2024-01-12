use cucumber::{given, then, when, World};
use rand;

#[derive(Debug, Default, World)]
pub struct NumWorld {
    number: f32,
    result: f32,
}

// is n about the same as m?
fn about_same(n: f32, m: f32) -> bool {
    (n - m) * 1000f32 < 1f32
}

// Steps are defined with `given`, `when` and `then` attributes.
#[given(regex = r"^a number$")]
async fn hungry_cat(world: &mut NumWorld) {
    world.number = rand::random();
}

#[when("We calculate the the inverted square root of a number using fast inverted square root")]
async fn calc_fast_inv_sqrt(world: &mut NumWorld) {
    world.result = revsqrt::fast_inverse_sqrt(world.number);
}

#[then("The result is about the same as if we calculate it normally")]
async fn comp_result_with_normal(world: &mut NumWorld) {
    assert!(about_same(world.number, world.result));
}

#[tokio::main]
async fn main() {
    futures::executor::block_on(NumWorld::run("tests/features/book/revsqrt.feature"));
}
