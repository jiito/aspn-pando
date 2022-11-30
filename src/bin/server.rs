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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
