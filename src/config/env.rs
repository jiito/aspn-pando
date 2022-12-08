pub fn database_url() -> String {
    std::env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:1CUhBqvLusL6dJF@pando-psql.internal:5432".to_string())
}
