use sqlite;
mod db_init;

fn main() {
    let _connection = db_init::init_database();

    loop {
        break;
    }

    println!("Connection listener for app section not yet implemented.  Aborting. ")
}
