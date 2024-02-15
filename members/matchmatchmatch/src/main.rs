fn main() {
    let deep_type = Option::Some(Option::Some(Option::Some(Option::Some(Option::Some(
        Option::Some(Option::Some(Option::Some(1337))),
    )))));
    match match match match match match match match deep_type {
        Some(inner) => inner,
        None => unreachable!(),
    } {
        Some(inner) => inner,
        None => unreachable!(),
    } {
        Some(inner) => inner,
        None => unreachable!(),
    } {
        Some(inner) => inner,
        None => unreachable!(),
    } {
        Some(inner) => inner,
        None => unreachable!(),
    } {
        Some(inner) => inner,
        None => unreachable!(),
    } {
        Some(inner) => inner,
        None => unreachable!(),
    } {
        Some(inner) => {
            println!("{:?}", inner);
            return;
        }
        None => unreachable!(),
    }
}
