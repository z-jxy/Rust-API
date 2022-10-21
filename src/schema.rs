// @generated automatically by Diesel CLI.

diesel::table! {
    errands (id) {
        id -> Int4,
        task -> Varchar,
        result -> Varchar,
        time_stamp -> Varchar,
    }
}

diesel::table! {
    agents (id) {
        id -> Int4,
        agent_id -> Varchar,
        agent_pid -> Varchar,
        agent_ip -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    errands,
    agents,
);
