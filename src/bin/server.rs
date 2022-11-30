use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use pando::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("./tmp")?;

    HttpServer::new(|| {
        App::new()
            .route(
                "/signed_url",
                web::get().to(pando::api::handlers::signed_url_handler),
            )
            .service(
                web::scope("/project")
                    .route("/", web::post().to(routes::project::create))
                    .route("/{projectId}", web::get().to(routes::project::find)),
            )
            .service(
                web::scope("/host")
                    .route("", web::post().to(routes::host::create))
                    .route("/{hostId}", web::get().to(routes::host::find))
                    .route("/{hostId}/connect", web::post().to(routes::host::connect)),
            )
            .service(
                web::scope("/function")
                    .route("", web::post().to(routes::function::create))
                    .route("", web::get().to(routes::function::find))
                    .route("/{functionId}", web::get().to(routes::function::find_by_id)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
