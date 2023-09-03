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

use std::path::PathBuf;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() {

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;

    // establish our database
    let _connection = db_init::init_database(PathBuf::from(".//src//db//main.sql"));

    // establish server's services here.
    //    loop {
    //       break;
    //    }

    println!("Connection listener for app section not yet implemented.  Aborting. ")
}


