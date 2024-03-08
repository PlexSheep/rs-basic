use fluent::{FluentArgs, FluentBundle, FluentResource, FluentValue};
// Used to provide a locale for the bundle.
use unic_langid::LanguageIdentifier;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let source_en: FluentResource = FluentResource::try_new(
        std::fs::read_to_string("../data/en-US.ftl").expect("could not read"),
    )
    .expect("could not parse");
    let source_de: FluentResource = FluentResource::try_new(
        std::fs::read_to_string("../data/de-DE.ftl").expect("could not read"),
    )
    .expect("could not parse");

    let langid_en: LanguageIdentifier = "en-US".parse().expect("Parsing failed");
    let langid_de: LanguageIdentifier = "de-DE".parse().expect("Parsing failed");
    let mut bundle = FluentBundle::new(vec![langid_en, langid_de]);
    bundle
        .add_resource(source_en)
        .expect("Failed to add FTL resources to the bundle.");
    bundle
        .add_resource(source_de)
        .expect("Failed to add FTL resources to the bundle.");

    rocket::build().launch().await?;

    Ok(())
}
