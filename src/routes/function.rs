use actix_web::{web, HttpResponse};

pub async fn create() -> HttpResponse {
    todo!("Implement the create function for the functions")
}
pub async fn find(path: web::Path<i32>) -> HttpResponse {
    let id = path.into_inner();
    todo!("Implement the find function for the functions")
}
