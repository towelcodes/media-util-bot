// @generated automatically by Diesel CLI.

diesel::table! {
    llm_context (uid) {
        uid -> Unsigned<Bigint>,
        last_updated -> Timestamp,
        context -> Longtext,
    }
}

diesel::allow_tables_to_appear_in_same_query!(llm_context,);
