use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{
    models,
    storage::{self, db::establish_connection},
};

pub async fn create(data: web::Json<models::NewHost>) -> HttpResponse {
    let conn = &mut establish_connection();
    let host = data.save(conn);
    HttpResponse::Ok().json(host)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindQueryParams {
    token: Option<String>,
}
pub async fn find(path: web::Query<FindQueryParams>) -> HttpResponse {
    let conn = &mut establish_connection();

    if let Some(token) = path.token.as_deref() {
        let host = storage::db::hosts::find_host_by_token(conn, &token);
        HttpResponse::Ok().json(host)
    } else {
        println!("No token in req");
        HttpResponse::InternalServerError().into()
    }
}

pub async fn connect_to_function(path: web::Path<(i32, i32)>) -> HttpResponse {
    let (host_id, function_id) = path.into_inner();

    let conn = &mut establish_connection();
    let host_function =
        storage::db::hosts::connect_host_to_function(conn, &host_id, &function_id).unwrap();

    HttpResponse::Ok().json(host_function)
}

#[derive(Serialize)]
struct ConnectionResponse {
    is_online: bool,
}
pub async fn connect(path: web::Path<i32>) -> HttpResponse {
    let id = path.into_inner();

    let conn = &mut establish_connection();
    let host = storage::db::hosts::find_by_id(conn, &id);
    host.online(conn).unwrap();
    HttpResponse::Ok().json(ConnectionResponse { is_online: true })
}
pub async fn disconnect(path: web::Path<i32>) -> HttpResponse {
    let id = path.into_inner();
    let conn = &mut establish_connection();
    let host = storage::db::hosts::find_by_id(conn, &id);
    host.offline(conn).unwrap();
    HttpResponse::Ok().json(ConnectionResponse { is_online: false })
}
