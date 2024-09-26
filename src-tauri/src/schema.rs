// @generated automatically by Diesel CLI.

diesel::table! {
    belts (id) {
        id -> Integer,
        name -> Text,
        current_holder_id -> Nullable<Integer>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
    }
}

diesel::table! {
    wrestlers (id) {
        id -> Integer,
        name -> Text,
        gender -> Text,
        wins -> Integer,
        losses -> Integer,
    }
}

diesel::joinable!(belts -> wrestlers (current_holder_id));

diesel::allow_tables_to_appear_in_same_query!(
    belts,
    users,
    wrestlers,
);
