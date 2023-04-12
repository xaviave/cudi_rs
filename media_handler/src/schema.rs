// @generated automatically by Diesel CLI.

diesel::table! {
    format (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    media (format_id, tag_id) {
        url -> Varchar,
        format_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    tag (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(media -> format (format_id));
diesel::joinable!(media -> tag (tag_id));

diesel::allow_tables_to_appear_in_same_query!(format, media, tag,);
