use crate::storage;
use crate::{models, storage::db::establish_connection};
use anyhow::{Context, Result};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::upsert::*;

pub fn connect_host_to_function(
    conn: &mut PgConnection,
    host_id: &i32,
    func_id: &i32,
) -> Result<models::HostsFunctions> {
    use crate::schema::hosts_functions;

    let new_host_function = models::NewHostFunctionIDs {
        host_id: *host_id,
        function_id: *func_id,
    };

    let host_function = diesel::insert_into(hosts_functions::table)
        .values(&new_host_function)
        .on_conflict_do_nothing()
        .get_result::<models::HostsFunctions>(conn)
        .optional()
        .context("Saving host function")?;

    match host_function {
        Some(hf) => Ok(hf),
        None => {
            let h = hosts_functions::dsl::hosts_functions
                .filter(hosts_functions::function_id.eq(func_id))
                .filter(hosts_functions::host_id.eq(host_id))
                .first::<models::HostsFunctions>(conn)?;
            Ok(h)
        }
    }
}
pub fn find_hosts_connect_to_func(
    conn: &mut PgConnection,
    func: &models::Function,
) -> Result<Vec<models::Host>> {
    use crate::schema::hosts;
    use crate::schema::hosts_functions;

    let h: Vec<models::Host> = models::HostsFunctions::belonging_to(func)
        .inner_join(hosts::table)
        .select(hosts::all_columns)
        .load(conn)?;
    Ok(h)
}

pub fn find_by_id(conn: &mut PgConnection, id: &i32) -> models::Host {
    use crate::schema::hosts::dsl;

    let query = dsl::hosts.filter(crate::schema::hosts::id.eq(id));

    let host = query
        .first::<models::Host>(conn)
        .expect("Could not find host");

    host
}
pub fn find_host_by_token(conn: &mut PgConnection, token: &str) -> models::Host {
    use crate::schema::hosts::dsl;

    let query = dsl::hosts.filter(crate::schema::hosts::user_token.eq(token));

    let host = query
        .first::<models::Host>(conn)
        .expect("Could not find host");

    host
}

impl models::Host {
    pub fn online(&self, conn: &mut PgConnection) -> Result<()> {
        diesel::update(self)
            .set(crate::schema::hosts::is_online.eq(true))
            .execute(conn)?;
        Ok(())
    }
    pub fn offline(&self, conn: &mut PgConnection) -> Result<()> {
        diesel::update(self)
            .set(crate::schema::hosts::is_online.eq(false))
            .execute(conn)?;
        Ok(())
    }
}

impl models::NewHost {
    pub fn save(&self, conn: &mut PgConnection) -> models::Host {
        use crate::schema::hosts;

        diesel::insert_into(hosts::table)
            .values(self)
            .get_result::<models::Host>(conn)
            .expect("Could not save host")
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::db::{establish_connection, functions};

    use super::find_hosts_connect_to_func;

    #[test]
    fn get_hosts_for_functions() {
        let conn = &mut establish_connection();
        let func = functions::find_by_id(conn, &8).unwrap();
        let hosts = find_hosts_connect_to_func(conn, &func).unwrap();
        assert_eq!(hosts.len(), 2);
        println!("{:?}", hosts)
    }
}
