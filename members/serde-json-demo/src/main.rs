#![allow(clippy::disallowed_names)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
struct Foo {
    name: String,
    q: Qux,
    t: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
struct Qux {
    i: i64,
    b: bool,
    j: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Color {
    Green,
    Yellow,
    Red,
    Custom(String),
}

use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Clone, Debug, PartialEq)]
#[repr(u8)]
enum CLike {
    Amida = 1,
    Yamda = 2,
    Omaba = 10,
}

fn main() -> anyhow::Result<()> {
    let qux_source = r#"{
        "some Key": [1, 2, 3],
        "deep": {
            "m": true
        }
    }"#;
    let foo = Foo {
        name: String::from("foo"),
        q: Qux {
            i: 19,
            b: false,
            j: serde_json::from_str(qux_source).unwrap(),
        },
        t: chrono::offset::Utc::now(),
    };
    let foostr = serde_json::to_string(&foo).unwrap();
    println!("foo:\n\n{}", foostr);
    let foo2: Foo = serde_json::from_str(&foostr).unwrap();
    println!("same?: {}", foo == foo2);

    let color_source = r#""yellow""#;
    let color: Color = serde_json::from_str(color_source)?;
    dbg!(&color);
    let color_str = serde_json::to_string(&color)?;
    dbg!(&color_str);

    let custom_color = Color::Custom("Morange".to_string());
    dbg!(serde_json::to_string(&custom_color).unwrap());

    let cl = CLike::Omaba;
    let clrepr = serde_json::to_string(&cl)?;
    dbg!(&clrepr);
    let cl_from = serde_json::from_str(&clrepr)?;
    dbg!(cl == cl_from);
    Ok(())
}
