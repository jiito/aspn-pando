use actix_web::{post, web, HttpResponse};
use anyhow::Result;
use serde::Deserialize;

use crate::{
    models,
    storage::{self, db::establish_connection},
};

#[derive(Deserialize)]
pub struct CreateProjectData {
    name: String,
    developer_id: i32,
}
pub async fn create(data: web::Json<CreateProjectData>) -> Result<HttpResponse> {
    let conn = &mut storage::db::establish_connection();
    // create project in databse
    let project = storage::db::create_project(conn, &data.name);

    // add project to developer accounts
    let developer = storage::db::find_developer(conn, &data.developer_id);

    // return a response to the user

    Ok(HttpResponse::Ok().json(project))
}

pub async fn find(path: web::Path<i32>) -> Result<models::Project> {
    let project_id = path.into_inner();
    let conn = &mut establish_connection();
    let project = storage::db::find_project(conn, &project_id);
    Ok(project)
}
