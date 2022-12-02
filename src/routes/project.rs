use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::{
    models::NewDeveloper,
    storage::{self, db::establish_connection},
};

#[derive(Deserialize)]
pub struct CreateProjectData {
    name: String,
    developer_id: i32,
}
pub async fn create(data: web::Json<CreateProjectData>) -> HttpResponse {
    let conn = &mut storage::db::establish_connection();
    // create project in databse
    let project = storage::db::project::create_project(conn, &data.name);

    // add project to developer accounts
    let developer = storage::db::developer::find(conn, &data.developer_id);

    let update_dev = NewDeveloper {
        project_id: project.id,
        name: developer.name,
    };
    let changed = storage::db::developer::save(conn, &update_dev);

    // return a response to the user

    HttpResponse::Ok().json(project)
}

pub async fn find(path: web::Path<i32>) -> HttpResponse {
    let project_id = path.into_inner();
    let conn = &mut establish_connection();
    let project = storage::db::project::find_project(conn, &project_id);
    HttpResponse::Ok().json(project)
}
