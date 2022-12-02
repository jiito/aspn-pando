use crate::{
    models,
    storage::{self, db::establish_connection},
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

pub async fn create(data: web::Json<models::NewDeveloper>) -> HttpResponse {
    let conn = &mut establish_connection();

    let developer = storage::db::developer::save(conn, &data);
    if developer > 0 {
        let developer = storage::db::developer::find_by_token(
            conn,
            &data
                .auth_token
                .clone()
                .expect("No Auth token found for the user"),
        )
        .expect("Cannot find the developer with that token");
        HttpResponse::Ok().json(developer)
    } else {
        println!("Could not update the developer");
        HttpResponse::InternalServerError().into()
    }
}
pub async fn find_by_id(path: web::Path<i32>) -> HttpResponse {
    let id = path.into_inner();
    let conn = &mut establish_connection();
    let developer = storage::db::developer::find_by_id(conn, &id);
    match developer {
        Ok(developer) => HttpResponse::Ok().json(developer),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FindQueryParams {
    token: Option<String>,
}
pub async fn find(path: web::Query<FindQueryParams>) -> HttpResponse {
    let conn = &mut establish_connection();
    if let Some(token) = &path.token {
        let developer = storage::db::developer::find_by_token(conn, &token);
        match developer {
            Ok(developer) => HttpResponse::Ok().json(developer),
            Err(_) => HttpResponse::InternalServerError().into(),
        }
    } else {
        HttpResponse::InternalServerError().into()
    }
}
