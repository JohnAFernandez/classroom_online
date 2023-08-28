use crate::db_types as types;
use sqlite;

pub fn row_to_user(row : &sqlite::Row) -> types::User {

    types::build_user(row.read::<i64, _>("user_id"), row.read::<&str, _>("email").to_string(), row.read::<&str, _>("username").to_string(), "".to_string(), row.read::<&str, _>("first_name").to_string(), row.read::<&str, _>("last_name").to_string(), row.read::<&str, _>("birthday").to_string(), row.read::<&str, _>("date_registered").to_string(), row.read::<&str, _>("phone").to_string(), row.read::<&str, _>("icon").to_string(), if row.read::<i64, _>("hidden") == 0 {true } else { false }, if row.read::<i64, _>("deleted") == 0 {true } else { false })
}
