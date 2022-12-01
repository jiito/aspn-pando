use crate::{
    models,
    storage::{self, db::establish_connection},
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

pub async fn create(data: web::Json<models::NewFunction>) -> HttpResponse {
    let conn = &mut establish_connection();

    let function = storage::db::functions::save(conn, &data).unwrap();

    HttpResponse::Ok().json(function)
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FindQueryParams {
    project_id: Option<i32>,
}
pub async fn find_by_id(path: web::Path<i32>) -> HttpResponse {
    let id = path.into_inner();
    let conn = &mut establish_connection();
    let function = storage::db::functions::find_by_id(conn, &id);
    match function {
        Ok(function) => HttpResponse::Ok().json(function),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
pub async fn find(path: web::Query<FindQueryParams>) -> HttpResponse {
    let conn = &mut establish_connection();
    if let Some(id) = path.project_id {
        let function = storage::db::functions::find_by_project(conn, &id);
        match function {
            Ok(function) => HttpResponse::Ok().json(function),
            Err(_) => HttpResponse::InternalServerError().into(),
        }
    } else {
        HttpResponse::InternalServerError().into()
    }
}
