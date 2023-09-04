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

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Route};
use sqlite;
use std::path::PathBuf;
use std::fs;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let location = ".//src//db//db.sql";
    match fs::remove_file(location) { Ok(_) => (), Err(_) => ()};

    // init the database if it has not been inited already
    db_init::init_database(PathBuf::from(location)).await;

    println!("Classroom Online Server has successfully started!  Bound to 127.0.0.1:8080",);

    HttpServer::new(|| {
        App::new()
            .service(rest_api::post_user)
            .service(rest_api::post_organization)
            .service(rest_api::post_school)
            .service(home_page)
        })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn home_page() -> HttpResponse {
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    let mut welcome: String = "<h1>Welcome to Classroom Online!</h1><br><br>Classroom online is a work-in-progress classroom management app, but the database can be interacted with via our REST API.<br><br>Current table statistics:<br><br>".to_string();
    let count = db_retrieve::R::retrieve_all_counts(&connection).await;

    for item in count {
        if item.1 < 0{
            welcome = welcome + &item.0 + &"<br>";
        } else {
            welcome = welcome + &item.0 + "&emsp;" + &item.1.to_string() + &"<br>";
        }
    }

    HttpResponse::Ok().body(welcome)
}

