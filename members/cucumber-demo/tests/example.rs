use cucumber::{given, then, when, World};

// These `Cat` definitions would normally be inside your project's code,
// not test code, but we create them here for the show case.
#[derive(Debug, Default)]
struct Cat {
    pub hungry: bool,
}

impl Cat {
    fn feed(&mut self) {
        self.hungry = false;
    }
}

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, World)]
// Accepts both sync/async and fallible/infallible functions.
// We can set a non default constructor like this vvvv
#[world(init = Self::new)]
pub struct AnimalWorld {
    cat: Cat,
}

// new constructor
impl AnimalWorld {
    fn new() -> Self {
        Self {
            cat: Cat { hungry: true },
        }
    }
}

// Steps are defined with `given`, `when` and `then` attributes.
#[given(regex = r"^a (hungry|satiated) cat$")]
async fn hungry_cat(world: &mut AnimalWorld, state: String) {
    match state.as_str() {
        "hungry" =>  world.cat.hungry = true,
        "satiated" =>  world.cat.hungry = false,
        _ => unreachable!(),
    }
}

#[when("I feed the cat")]
async fn feed_cat(world: &mut AnimalWorld) {
    world.cat.feed();
}

#[then("the cat is not hungry")]
async fn cat_is_fed(world: &mut AnimalWorld) {
    assert!(!world.cat.hungry);
}

#[tokio::main]
async fn main() {
    futures::executor::block_on(AnimalWorld::run("tests/features/book/animal.feature"));
}
