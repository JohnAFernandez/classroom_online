mod db_init;
mod db_insert;
mod db_verify;
mod db_delete;
mod db_retrieve;
mod db_update;

mod db_types;
mod db_object_to_row;
mod db_row_to_object;
mod rest_api;

mod tests;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlite;
use std::path::PathBuf;
use std::fs;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let location = ".//src//db//db.sql";
    match fs::remove_file(location) { Ok(_) => (), Err(_) => ()};

    println!("1");
    // init the database if it has not been inited already
    let connection = db_init::init_database(PathBuf::from(location));

    println!("2");

    HttpServer::new(|| {
        App::new()
            .service(ping_the_server)
            .service(rest_api::post_user)
            .service(rest_api::post_organization)
            .service(rest_api::post_school)
        })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn ping_the_server() -> HttpResponse {
    HttpResponse::Ok().body("You've reached the classroom online rest api.  Please submit your users.")
}

