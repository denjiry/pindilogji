use actix_files as acfs;
use actix_web::http::StatusCode;
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder, Result};
use pindilogji::{format_sr, lightblue};
use serde::{Deserialize, Serialize};

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

type Lambda = String;
#[derive(Debug, Serialize, Deserialize)]
struct Term {
    word: String,
    lambda: Lambda,
}

#[post("/newterm")]
async fn newterm(item: web::Json<Term>) -> Result<HttpResponse> {
    let Term { word, lambda: _ } = item.into_inner();
    let lambda = match lightblue(&word) {
        Ok(lambda) => lambda,
        Err(error) => {
            let error_msg = format!("{:?}", error);
            return Ok(HttpResponse::InternalServerError().body(error_msg));
        }
    };
    let lambda = format_sr(&lambda);
    let ret_term = Term { word, lambda };
    Ok(HttpResponse::Ok().json(ret_term))
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
