use actix_web::{web, HttpResponse};

pub async fn create() -> HttpResponse {
    todo!("implement the create host handler")
}
pub async fn find(path: web::Path<i32>) -> HttpResponse {
    todo!("implement the find host handler")
}