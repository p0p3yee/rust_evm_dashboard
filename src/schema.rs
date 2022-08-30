table! {
    accounts (address) {
        address -> Text,
        name -> Nullable<Text>,
        private_key -> Nullable<Text>,
    }
}

table! {
    endpoints (id) {
        id -> Integer,
        name -> Text,
        url -> Text,
        symbol -> Text,
        chain_id -> Text,
        explorer_url -> Nullable<Text>,
    }
}

table! {
    settings (selected_endpoint_id) {
        selected_endpoint_id -> Integer,
    }
}

joinable!(settings -> endpoints (selected_endpoint_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    endpoints,
    settings,
);
