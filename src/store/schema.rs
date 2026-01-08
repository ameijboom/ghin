// @generated automatically by Diesel CLI.

diesel::table! {
    config (id) {
        id -> Integer,
        installation_directory -> Text,
        temp_directory -> Text,
    }
}

diesel::table! {
    repositories (id) {
        id -> Integer,
        owner -> Text,
        name -> Text,
        package -> Text,
        location -> Text,
        tag -> Text,
        locked -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(config, repositories,);
