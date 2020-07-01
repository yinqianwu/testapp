table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        tag_list -> Array<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
