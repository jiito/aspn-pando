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
                web::scope("/developer")
                    .route("/", web::post().to(routes::developer::create))
                    .route("", web::get().to(routes::developer::find))
                    .route(
                        "/{developerId}",
                        web::get().to(routes::developer::find_by_id),
                    ),
            )
            .service(
                web::scope("/host")
                    .route("", web::post().to(routes::host::create))
                    //TODO: Move this to a sep service
                    .route(
                        "/{hostId}/{functionId}/connect",
                        web::post().to(routes::host::connect_to_function),
                    )
                    .route("", web::get().to(routes::host::find))
                    .route("/{hostId}/connect", web::post().to(routes::host::connect))
                    .route(
                        "/{hostId}/disconnect",
                        web::post().to(routes::host::disconnect),
                    ),
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
