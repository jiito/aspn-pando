use crate::models::{NewProject, Project};
use diesel::pg::PgConnection;
use diesel::prelude::*;
pub fn create_project(conn: &mut PgConnection, name: &str) -> Project {
    use crate::schema::projects;

    let new_project = NewProject { name };

    diesel::insert_into(projects::table)
        .values(&new_project)
        .get_result(conn)
        .expect("Error saving project")
}
pub fn get_projects(conn: &mut PgConnection) -> Vec<Project> {
    use crate::schema::projects::dsl;

    let connection = &mut establish_connection();

    let results = dsl::projects
        .load::<Project>(conn)
        .expect("Error loeading post");

    results
}
pub fn find_project(conn: &mut PgConnection, id: &i32) -> Project {
    use crate::schema::projects::dsl;

    let query = dsl::projects.filter(crate::schema::projects::id.eq(id));

    let project = query.first::<Project>(conn).expect("Can't find project");

    project
}
