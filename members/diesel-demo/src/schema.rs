// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,             // what the hell is an Int4???? I guess it must
                                // be a 4 byte integer (u32?), no way it would be a nibble.
        title -> Varchar,
        body -> Text,           // What is the difference between Text and Varchar? I know Varchar
                                // from various DB stuff but idk Text
        published -> Bool,
    }
}
