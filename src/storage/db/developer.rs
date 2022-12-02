use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models::{Developer, NewDeveloper};
pub fn save(conn: &mut PgConnection, developer: &NewDeveloper) -> usize {
    use crate::schema::developers;

    diesel::insert_into(developers::table)
        .values(developer)
        .on_conflict(developers::id)
        .do_update()
        .set(developers::project_id.eq(0))
        .execute(conn)
        .expect("Could not update the developer")
}

pub fn find_by_id(conn: &mut PgConnection, id: &i32) -> Result<Developer> {
    use crate::schema::developers::dsl;

    let query = dsl::developers.filter(crate::schema::developers::id.eq(id));

    let developer = query
        .first::<Developer>(conn)
        .expect("Can't find developer");

    Ok(developer)
}

pub fn find_by_token(conn: &mut PgConnection, token: &str) -> Result<Developer> {
    use crate::schema::developers::dsl;

    let query = dsl::developers.filter(crate::schema::developers::auth_token.eq(token));

    let developer = query
        .first::<Developer>(conn)
        .expect("Can't find developer");

    Ok(developer)
}
