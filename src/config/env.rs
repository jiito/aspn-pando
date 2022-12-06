pub fn database_url() -> String {
    std::env::var("DATABASE_URL").unwrap_or("localhost:8080".to_string())
}
