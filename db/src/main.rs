mod db_init;
mod db_insert;
mod db_verify;
mod db_delete;
mod tests;
use clap::Parser;
use std::path::PathBuf;

fn main() {

    // establish our database
    let connection = db_init::init_database(PathBuf::from(".//src//db//main.sql"));

    // establish server's services here.
    //    loop {
    //       break;
    //    }

    println!("Connection listener for app section not yet implemented.  Aborting. ")
}


