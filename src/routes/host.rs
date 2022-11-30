use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::storage::{self, db::establish_connection};

pub async fn create() -> HttpResponse {
    todo!("implement the create host handler")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindQueryParams {
    token: Option<String>,
}
pub async fn find(path: web::Path<FindQueryParams>) -> HttpResponse {
    let conn = &mut establish_connection();

    if let Some(token) = path.token.as_deref() {
        let host = storage::db::hosts::find_host_by_token(conn, &token);
        HttpResponse::Ok().json(host)
    } else {
        HttpResponse::InternalServerError().into()
    }
}

pub async fn connect(path: web::Path<i32>) -> HttpResponse {
    let id = path.into_inner();

    let conn = &mut establish_connection();
    let host = storage::db::hosts::find_by_id(conn, &id);
    host.online(conn);
    HttpResponse::Ok().into()
}
pub async fn disconnect(path: web::Path<i32>) -> HttpResponse {
    let id = path.into_inner();
    let conn = &mut establish_connection();
    let host = storage::db::hosts::find_by_id(conn, &id);
    host.offline(conn);
    HttpResponse::Ok().into()
}
