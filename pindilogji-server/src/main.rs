use actix_files as acfs;
use actix_web::http::StatusCode;
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder, Result};
use pindilogji::{format_sr, lightblue};

#[get("/")]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../pindilogji-ui/index.html")))
}

#[get("/pkg/package.js")]
async fn app() -> Result<acfs::NamedFile> {
    Ok(acfs::NamedFile::open("../pindilogji-ui/pkg/package.js")?)
}

#[get("/pkg/package_bg.wasm")]
async fn app2() -> Result<acfs::NamedFile> {
    Ok(acfs::NamedFile::open(
        "../pindilogji-ui/pkg/package_bg.wasm",
    )?)
}

#[post("/newterm")]
async fn newterm() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK).body("new"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(app)
            .service(app2)
            .service(newterm)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
