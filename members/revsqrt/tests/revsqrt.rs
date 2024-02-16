use std::iter::zip;



use cucumber::{gherkin::Step, given, then, when, World};


/// stores the current information for each scenario
#[derive(Debug, Default, World)]
struct NumWorld {
    numbers: Vec<(f32, f32)>,
}

/// is n about the same as m?
///
/// This is actually not so easy! How do you measure *about same*ness?
/// Also, I don't think it is transitive, as 1 ≈ 1.1 ≈ 1.2 ≈ 1.3 ≈ ... ≈ 2 ≈ ... ≈ 3 ≈ ... ≈ infinity
#[inline]
fn about_same(n: f32, m: f32) -> bool {
    (n - m).abs() <= calc_gate(n, m)
}

#[inline]
fn calc_gate(n: f32, m: f32) -> f32 {
    0.01 + ((n.abs().sqrt().min(m.abs().sqrt())).abs() / 10f32)
}

#[given(regex = r"the number n")]
async fn give_specific_number(world: &mut NumWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) {
            // NOTE: skip header
            let n = row[0].parse::<f32>().unwrap();
            world.numbers.push((n, f32::NAN));
        }
    }
}

#[given("a number")]
async fn give_rand_number(world: &mut NumWorld) {
    world.numbers.push(rand::random());
}

#[when("we calculate the inverted square root of it using the fast inverted square root algorithm")]
async fn calc_fast_inv_sqrt(world: &mut NumWorld) {
    for pair in &mut world.numbers {
        pair.1 = revsqrt::fast_inverse_sqrt(pair.0)
    }
}

#[when("we calculate the inverted square root of it normally")]
async fn calc_reg_inv_sqrt(world: &mut NumWorld) {
    for pair in &mut world.numbers {
        pair.1 = revsqrt::regular_inverse_sqrt(pair.0)
    }
}

#[then("the result is about the same as if we calculate it normally")]
async fn comp_result_with_normal(world: &mut NumWorld) {
    for pair in &mut world.numbers {
        assert!(about_same(pair.1, revsqrt::regular_inverse_sqrt(pair.0)));
    }
}

#[then("the result can be calculated")]
async fn can_be_calculated(world: &mut NumWorld) {
    for pair in &mut world.numbers {
        assert!(!pair.0.is_nan());
        assert!(!pair.1.is_nan());
        assert!(pair.0.is_finite());
        assert!(pair.1.is_finite());
    }
}

#[then(regex = r"the result is m")]
async fn result_is(world: &mut NumWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        for (row, i) in zip(table.rows.iter().skip(1), 0..table.rows.len() - 1) {
            // NOTE: skip header
            let m = row[0].parse::<f32>().unwrap();
            assert_eq!(world.numbers[i].1, m);
        }
    }
}

#[then(regex = r"the result is about the same as m")]
async fn result_is_about(world: &mut NumWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        for (row, i) in zip(table.rows.iter().skip(1), 0..table.rows.len() - 1) {
            // NOTE: skip header
            let m = row[0].parse::<f32>().unwrap();
            assert!(
                about_same(world.numbers[i].1, m),
                "{} and {} are not about the same!",
                world.numbers[i].1,
                m
            );
        }
    }
}

#[then("they are about the same")]
async fn they_are_about_the_same(world: &mut NumWorld) {
    let mut still_same = true;
    let mut last_num = world.numbers[0].0;
    for tup in &world.numbers {
        still_same &= about_same(tup.0, last_num);
        assert!(
            still_same,
            "{} and {} are not about the same! (gate: {})",
            tup.0,
            last_num,
            calc_gate(tup.0, last_num)
        );
        last_num = tup.0;
    }
}

#[then("they are not about the same")]
async fn they_are_not_about_the_same(world: &mut NumWorld) {
    let mut found_a_same = false;
    let mut last_num = f32::NAN;
    for tup in &world.numbers {
        found_a_same |= about_same(tup.0, last_num);
        assert!(
            !found_a_same,
            "{} and {} are about the same! (gate: {})",
            tup.0,
            last_num,
            calc_gate(tup.0, last_num)
        );
        last_num = tup.0;
    }
}

#[tokio::main]
async fn main() {
    NumWorld::run("tests/features/book/revsqrt.feature").await;
}
