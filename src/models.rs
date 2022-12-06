use crate::{schema::*, utils};
use anyhow::Result;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Queryable, Identifiable, Debug, Serialize, Hash)]
pub struct Project {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub name: &'a str,
}

#[derive(Queryable, Serialize, Associations, Identifiable, Debug)]
#[belongs_to(Project)]
#[diesel(table_name = developers)]
pub struct Developer {
    pub id: i32,
    pub name: String,
    pub project_id: Option<i32>,
    pub auth_token: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = developers)]
pub struct NewDeveloper {
    pub name: String,
    pub project_id: Option<i32>,
    pub auth_token: Option<String>,
}

#[derive(Queryable, Associations, Identifiable, Debug, Serialize)]
#[belongs_to(Project)]
#[diesel(table_name = functions)]
pub struct Function {
    pub id: i32,
    pub gcs_uri: String,
    pub route: String,
    pub project_id: i32,
}
#[derive(Insertable, Deserialize)]
#[diesel(table_name = functions)]
pub struct NewFunction {
    pub gcs_uri: String,
    pub route: String,
    pub project_id: i32,
}

#[derive(Queryable, Identifiable, Debug, Serialize)]
pub struct Host {
    pub id: i32,
    pub ip_address: ipnetwork::IpNetwork,
    pub user_token: String,
    pub is_online: bool,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = hosts)]
pub struct NewHost {
    pub ip_address: ipnetwork::IpNetwork,
    pub user_token: String,
}

#[derive(Associations, Queryable, Identifiable, Serialize, Debug)]
#[diesel(table_name = hosts_functions)]
#[belongs_to(Function)]
#[belongs_to(Host)]
#[primary_key(function_id, host_id)]
pub struct HostsFunctions {
    pub function_id: i32,
    pub host_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = hosts_functions)]
pub struct NewHostFunctionIDs {
    pub function_id: i32,
    pub host_id: i32,
}
