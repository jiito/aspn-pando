use crate::utils;
use crate::{models, storage::db::establish_connection};
use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::prelude::*;

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
        .get_result::<models::HostsFunctions>(conn)?;

    Ok(host_function)
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
            .execute(conn);
        Ok(())
    }
    pub fn offline(&self, conn: &mut PgConnection) -> Result<()> {
        diesel::update(self)
            .set(crate::schema::hosts::is_online.eq(false))
            .execute(conn);
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
