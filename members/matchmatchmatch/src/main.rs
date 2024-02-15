fn main() {
    let deep_type = Option::Some(Option::Some(Option::Some(())));
    match deep_type {
        Some(inner) => match inner {
            Some(inner) => match inner {
                Some(inner) => {
                    println!("{:?}", inner); return;
                }
                None => {}
            },
            None => {}
        },
        None => {}
    }
    unreachable!()
}
