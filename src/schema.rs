// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Integer,
        uid -> BigUnsigned,
        #[max_length = 256]
        mnemonic -> Nullable<Varchar>,
        #[max_length = 256]
        address -> Nullable<Varchar>,
        #[max_length = 256]
        token -> Nullable<Varchar>,
    }
}
