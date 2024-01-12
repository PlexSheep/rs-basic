use std::{io::Write, panic};
fn inner() {
    let mut counter = 0u8;
    loop {
        // will eventually panic when overflowing?
        counter += 1;
        print!("{counter}\t");
        if counter % 8 == 0 { println!() }
        if counter == 255 {
            // so panic will look fancier :)
            println!()
        }
    }
}

fn main() {
    // will not catch all panics, only ones that unwind
    let panic = panic::catch_unwind(|| {
        inner();
    });
    if panic.is_err() {
        dbg!(&panic);
        dbg!(&panic.as_ref().unwrap_err().type_id());
        println!("recovered from a panic");
    }
    else {println!("no panic on the titanic")}
}
