// shamelessly stolen from https://docs.rs/fluent/latest/fluent/index.html

use fluent::{FluentArgs, FluentBundle, FluentResource, FluentValue};
// Used to provide a locale for the bundle.
use unic_langid::LanguageIdentifier;

fn main() {
    let ftl_string = String::from(
        "
hello-world = Hello, world!
intro = Welcome, { $name }.
",
    );
    let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");

    let langid_en: LanguageIdentifier = "en-US".parse().expect("Parsing failed");
    let mut bundle = FluentBundle::new(vec![langid_en]);

    bundle
        .add_resource(res)
        .expect("Failed to add FTL resources to the bundle.");

    let msg = bundle
        .get_message("hello-world")
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(pattern, None, &mut errors);

    assert_eq!(&value, "Hello, world!");
    println!("{value}");

    let mut args = FluentArgs::new();
    args.set("name", FluentValue::from("John"));

    let msg = bundle.get_message("intro").expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(pattern, Some(&args), &mut errors);

    // The FSI/PDI isolation marks ensure that the direction of
    // the text from the variable is not affected by the translation.
    assert_eq!(value, "Welcome, \u{2068}John\u{2069}.");
    // which is very weird for printing to the terminal
    println!("{value}");
}
