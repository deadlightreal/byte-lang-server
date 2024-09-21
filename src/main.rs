use actix_web::{get, post, web, web::Data, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::io::Read;
use std::fs::File;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Deserialize)]
struct InstallPackageParams {
    package: String
}

#[post("/createPackage")]
async fn create_package(pool : web::Data<PgPool>) -> impl Responder {
    return HttpResponse::Ok().body(String::new());
}

#[get("/installPackage")]
async fn install_package(pool : web::Data<PgPool>, params : web::Query<InstallPackageParams>) -> impl Responder {
    let mut zip_package = File::open(format!("/Users/kokot/Desktop/byte-lang-server/packages/{}.zip", params.package)).unwrap();

    let mut buffer = Vec::new();
    zip_package.read_to_end(&mut buffer).unwrap();

    return HttpResponse::Ok().content_type("application/zip").body(buffer);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://kokot:122008@localhost:5432/bytelang")
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
        .app_data(Data::new(pool.clone()))
        .service(install_package)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
