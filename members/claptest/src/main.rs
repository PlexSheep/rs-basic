use clap::Parser;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// important list of strings
    #[arg(short, long, value_name = "String",num_args(0..), name="string")]
    pub strings: Vec<String>,
}
fn main() {
    println!("Hello, world!");
    let args = Args::parse();
    dbg!(args.strings);
}
