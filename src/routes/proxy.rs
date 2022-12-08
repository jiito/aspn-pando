use crate::{
    models,
    storage::{self, db::establish_connection},
};
use anyhow::{Context, Result};
use reqwest::Response;

pub async fn proxy_request(function_id: &i32) -> Result<Response> {
    let conn = &mut &mut establish_connection();
    let connected_hosts = storage::db::functions::connected_hosts(conn, function_id)?;
    let first_host = &connected_hosts[0];
    let function = storage::db::functions::find_by_id(conn, function_id)?;

    proxy_request_to_host(first_host, &function).await
}

async fn proxy_request_to_host(
    host: &models::Host,
    function: &models::Function,
) -> Result<Response> {
    let client = reqwest::Client::new();

    let uri = uri(host, function);
    println!("[PROXY] {}", uri);

    client.get(uri).send().await.context("Sending request")
}

fn uri(host: &models::Host, function: &models::Function) -> String {
    format!("http://{}:4011/{}", host.ip_address.ip(), function.route)
}
