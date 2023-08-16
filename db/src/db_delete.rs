use sqlite;
use crate::db_verify::V;


pub struct D {}

impl D {
    // ok, this one is kind of a red-herring.  Users are too fundamental to delete safely, and it would
    // delete grade records, so just mark a "deleted" flag, meaning that the user has cut themselves from access.
    // But the grades and other records should remain.
    pub fn delete_user(connection : &sqlite::Connection, user_id: i64) -> bool{

        // if so, this user didn't really exist in the first place, go figure.
        if !V::check_id(connection, user_id, V::USERS) {
            return false;
        }
        
        let query = "UPDATE users SET deleted = 1 WHERE user_id = ".to_owned()
        + &user_id.to_string();

        match connection.execute(query){
            Ok(_) => return true,
            Err(_) => return false
        }

    }

}
