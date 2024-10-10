// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        #[max_length = 16]
        id -> Binary,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
    }
}
