use actix_web::{web, Responder};
use serde::{Deserialize, Serialize};

use crate::storage;

#[derive(Deserialize, Debug)]
pub struct SignedUrlQueryParams {
    method: String,
    object_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignedUrlResponse {
    pub uri: String,
}
pub async fn signed_url_handler(query_params: web::Query<SignedUrlQueryParams>) -> impl Responder {
    // build signed url
    let uri = storage::gcs::generate_signed_url(
        "/Users/bjar/service-account.json".into(),
        "aspn_functions".into(),
        query_params.object_name.clone(),
        None,
        query_params.method.clone(),
        None,
    )
    .await
    .unwrap();

    web::Json(SignedUrlResponse { uri })
}
