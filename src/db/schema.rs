pub mod messages {
    use diesel::{allow_tables_to_appear_in_same_query, joinable, table};

    table! {
        messages.message (id) {
            id -> Uuid,
            user_id -> Uuid,
            is_reply -> Bool,
            reply_to -> Nullable<Uuid>,
            created_datetime -> Timestamp,
            modified_datetime -> Timestamp,
        }
    }

    table! {
        messages.content (id) {
            id -> Uuid,
            message_id -> Uuid,
            data -> Text,
            created_datetime -> Timestamp,
            modified_datetime -> Timestamp,
        }
    }

    joinable!(content -> message (message_id));
    allow_tables_to_appear_in_same_query!(message,content);
}