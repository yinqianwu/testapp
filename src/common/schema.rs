table! {
    articles (id) {
        id -> Varchar,
        slug -> Text,
        title -> Text,
        description -> Text,
        body -> Text,
        market_type -> Int4,
        media_source -> Text,
        section_tag_list -> Array<Int4>,
        event_tag_list -> Array<Int4>,
        stakeholder_list -> Array<Text>,
        occurred_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}