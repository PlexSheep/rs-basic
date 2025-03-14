use std::fs;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;

fn main() {
    println!("cwd: {:?}", std::env::current_dir().unwrap());
    let unparsed_file = fs::read_to_string("pest-demo/data/example.csv").expect("cannot read file");
    let file = CSVParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails
                   // let mut field_sum: f64 = 0.0;
    let mut field_sum: f64 = 0.0;
    let mut record_count: u64 = 0;

    println!("{:=^80}", "");
    for record in file.into_inner() {
        match record.as_rule() {
            Rule::record => {
                print!("|");
                record_count += 1;

                for field in record.into_inner() {
                    field_sum += field.as_str().parse::<f64>().unwrap();
                    print!("{:^25}|", field.as_str());
                }
                println!();
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    println!("{:=^80}", "");

    println!("Sum of fields: {}", field_sum);
    println!("Number of records: {}", record_count);
}
