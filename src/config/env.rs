pub fn database_url() -> String {
    std::env::var("DATABASE_URL").unwrap_or("postgresql://localhost:5432/pando".to_string())
}
