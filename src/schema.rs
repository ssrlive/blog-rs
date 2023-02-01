// @generated automatically by Diesel CLI.

diesel::table! {
    blog_posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Nullable<Bool>,
    }
}
