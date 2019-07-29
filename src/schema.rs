table! {
    heads (table_name) {
        table_name -> Varchar,
        table_hash -> Varchar,
    }
}

table! {
    tables (hash) {
        hash -> Varchar,
        table_name -> Varchar,
        ts -> Timestamptz,
        data -> Jsonb,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        hash -> Varchar,
        ts -> Timestamptz,
        is_super -> Bool,
    }
}

joinable!(heads -> tables (table_hash));

allow_tables_to_appear_in_same_query!(
    heads,
    tables,
    users,
);
