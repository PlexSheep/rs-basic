fn main() {
    // args_os does not check if the args are valid UTF-8. args will panic if not so
    println!("args_os:\n{:#?}", std::env::args_os());
}
