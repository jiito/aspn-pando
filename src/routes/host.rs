use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

pub async fn create() -> HttpResponse {
    todo!("implement the create host handler")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindQueryParams {
    token: Option<String>,
}
pub async fn find(path: web::Path<i32>) -> HttpResponse {
    let id = path.into_inner();
    todo!("implement the find host handler")
}
pub async fn find_by_id(path: web::Path<i32>) -> HttpResponse {
    let id = path.into_inner();
    todo!("implement the find host handler")
}

pub async fn connect(path: web::Path<i32>) -> HttpResponse {
    let id = path.into_inner();
    todo!("implement the find host handler")
}
