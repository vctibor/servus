table! {
    jobs (id) {
        id -> Uuid,
        name -> Text,
        code -> Text,
        description -> Nullable<Text>,
        schedule -> Text,
        target -> Uuid,
        owner -> Uuid,
        last_update -> Timestamp,
        send_email -> Bool,
    }
}

table! {
    machines (id) {
        id -> Uuid,
        name -> Text,
        username -> Text,
        url -> Text,
        port -> Int4,
    }
}

table! {
    tx_log (id) {
        id -> Uuid,
        success -> Bool,
        time -> Timestamp,
        message -> Text,
        job -> Uuid,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        email -> Nullable<Text>,
    }
}

joinable!(jobs -> machines (target));
joinable!(jobs -> users (owner));
joinable!(tx_log -> jobs (job));

allow_tables_to_appear_in_same_query!(
    jobs,
    machines,
    tx_log,
    users,
);
