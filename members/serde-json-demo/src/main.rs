use chrono;
use serde::{Deserialize, Serialize};
use serde_json;

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

fn main() {
    let qj = r#"{
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
            j: serde_json::from_str(qj).unwrap(),
        },
        t: chrono::offset::Utc::now(),
    };
    let foostr = serde_json::to_string(&foo).unwrap();
    println!("foo:\n\n{}", foostr);
    let foo2: Foo = serde_json::from_str(&foostr).unwrap();
    println!("same?: {}", foo == foo2);
}
