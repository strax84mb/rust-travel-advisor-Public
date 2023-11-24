// @generated automatically by Diesel CLI.

diesel::table! {
    airports (id) {
        id -> Bigint,
        city_id -> Bigint,
        name -> Varchar,
    }
}

diesel::table! {
    cities (id) {
        id -> Bigint,
        name -> Varchar,
    }
}

diesel::table! {
    comments (id) {
        id -> Bigint,
        user_id -> Bigint,
        city_id -> Bigint,
        text -> Varchar,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    routes (id) {
        id -> Bigint,
        start -> Bigint,
        finish -> Bigint,
        price -> Bigint,
    }
}

diesel::table! {
    users (id) {
        id -> Bigint,
        email -> Varchar,
        pass -> Varchar,
        roles -> Varchar,
    }
}

diesel::joinable!(airports -> cities (city_id));
diesel::joinable!(comments -> cities (city_id));
diesel::joinable!(comments -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    airports,
    cities,
    comments,
    routes,
    users,
);
