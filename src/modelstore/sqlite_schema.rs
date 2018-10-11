embed_migrations!("src/modelstore/migrations/sqlite/");
table! {
    models (id) {
        id -> Integer,
        doc -> Text,
    }
}
