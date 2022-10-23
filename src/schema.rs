// @generated automatically by Diesel CLI.

diesel::table! {
    agents (id) {
        id -> Int4,
        agent_id -> Varchar,
        agent_pid -> Varchar,
        agent_ip -> Varchar,
    }
}

diesel::table! {
    c2_tasks (id) {
        id -> Int4,
        created_at -> Timestamptz,
        executed_at -> Timestamptz,
        task -> Text,
        args -> Jsonb,
        result -> Nullable<Text>,
        implant_id -> Int4,
    }
}

diesel::table! {
    errands (id) {
        id -> Int4,
        task -> Varchar,
        result -> Varchar,
        time_stamp -> Varchar,
    }
}

diesel::table! {
    implants (id) {
        id -> Int4,
        name -> Varchar,
        pid -> Nullable<Varchar>,
        created_at -> Timestamptz,
        last_seen_at -> Timestamptz,
        ip -> Nullable<Varchar>,
    }
}

diesel::joinable!(c2_tasks -> implants (implant_id));

diesel::allow_tables_to_appear_in_same_query!(
    agents,
    c2_tasks,
    errands,
    implants,
);
