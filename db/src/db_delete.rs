use sqlite;
use crate::db_verify::V;
use crate::db_retrieve::R;

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

    pub fn delete_administrator(connection : &sqlite::Connection, admin_id: i64) -> bool {

        // if so, this user didn't really exist in the first place, go figure.
        if !V::check_id(connection, admin_id, V::ADMINISTRATORS) {
            return false;
        }
        let query = "DELETE FROM administrators WHERE admin_id = ".to_owned()
        + &admin_id.to_string();

        match connection.execute(query){
            Ok(_) => return true,
            Err(_) => return false
        }

    }

    pub fn delete_school(connection : &sqlite::Connection, school_id: i64) -> bool {

        // if so, this user didn't really exist in the first place, go figure.
        if !V::check_id(connection, school_id, V::SCHOOLS) {
            return false;
        }

        // only delete a school if there are no associated classes
        match R::retrieve_classes_from_school(connection, school_id) {
            Ok(values) => if values.into_iter().next().is_none() {return false},
            Err(_) =>  return false
        }

        let query = "DELETE FROM schools WHERE school_id = ".to_owned()
        + &school_id.to_string();

        match connection.execute(query){
            Ok(_) => return true,
            Err(_) => return false
        }

    }


}
