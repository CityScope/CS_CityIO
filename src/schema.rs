table! {
    tables (id) {
        id -> Int4,
        title -> Varchar,
        data -> Jsonb,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        hash -> Varchar,
        ts -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    tables,
    users,
);
