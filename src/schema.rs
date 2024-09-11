// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Int8,
        uid -> Int8,
        #[max_length = 256]
        mnemonic -> Nullable<Varchar>,
        #[max_length = 256]
        address -> Nullable<Varchar>,
        #[max_length = 256]
        token -> Nullable<Varchar>,
        feature -> Nullable<Bytea>,
        #[max_length = 256]
        seed -> Nullable<Varchar>,
        #[max_length = 256]
        note -> Nullable<Varchar>,
    }
}

diesel::table! {
    seedphrase (seedphrase_id) {
        seedphrase_id -> Int8,
        #[max_length = 256]
        seedphrase_address -> Nullable<Varchar>,
        #[max_length = 256]
        seedphrase_seed -> Nullable<Varchar>,
        #[max_length = 256]
        seedphrase_note -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    account,
    seedphrase,
);
