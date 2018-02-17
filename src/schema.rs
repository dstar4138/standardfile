table! {
    users (uuid) {
        uuid -> Text,
        email -> Text,
        pw_func -> Text,
        pw_alg -> Text,
        pw_cost -> Integer,
        pw_key_size -> Integer,
        pw_nonce -> Text,
        encrypted_password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        pw_salt -> Text,
        version -> Text,
    }
}

table! {
    items (uuid) {
        uuid                -> Text,
        content             -> Binary,
        content_type        -> Text,
        enc_item_key        -> Text,
        auth_hash           -> Text,
        user_uuid           -> Text,
        deleted             -> Bool,
        created_at          -> Timestamp,
        updated_at          -> Timestamp,
        last_user_agent     -> Text,
    }
}