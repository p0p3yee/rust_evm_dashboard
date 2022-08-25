table! {
    accounts (address) {
        address -> Text,
        name -> Nullable<Text>,
        private_key -> Nullable<Text>,
    }
}

table! {
    endpoints (name) {
        name -> Text,
        url -> Text,
        symbol -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    endpoints,
);
