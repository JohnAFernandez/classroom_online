mod db_init;
mod db_insert;
mod db_verify;
mod db_delete;
mod db_retrieve;
mod db_update;

mod db_types;
mod db_object_to_row;
mod db_row_to_object;
mod tests;
use std::path::PathBuf;

fn main() {

    // establish our database
    let _connection = db_init::init_database(PathBuf::from(".//src//db//main.sql"));

    // establish server's services here.
    //    loop {
    //       break;
    //    }

    println!("Connection listener for app section not yet implemented.  Aborting. ")
}


