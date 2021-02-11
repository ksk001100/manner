table! {
    users (id) {
        id -> Unsigned<Bigint>,
        name -> Text,
        email -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
