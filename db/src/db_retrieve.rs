use sqlite::{Connection, Statement};

pub struct R {}

impl R {
    pub fn retrieve_classes_from_school(connection : &sqlite::Connection, school_id : i32) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT class_id FROM schools WHERE school_id = ".to_owned()
            + &school_id.to_string();
    
        return connection.prepare(query);
    }

    pub fn retrieve_administrator_from_user(connection : &sqlite::Connection, user_id : i32) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT admin_id FROM administrator WHERE user_id = ".to_owned()
            + &user_id.to_string();
    
        return connection.prepare(query);
    }

    pub fn retrieve_teacher_from_user(connection : &sqlite::Connection, user_id : i32) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT teacher_id FROM administrator WHERE user_id = ".to_owned()
            + &user_id.to_string();
    
        return connection.prepare(query);
    }

    pub fn retrieve_student_from_user(connection : &sqlite::Connection, user_id : i32) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT student_id FROM administrator WHERE user_id = ".to_owned()
            + &user_id.to_string();
    
        return connection.prepare(query);
    }

    pub fn retrieve_family_member_from_user(connection : &sqlite::Connection, user_id : i32) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT member_id FROM family_memebers WHERE user_id = ".to_owned()
            + &user_id.to_string();
    
        return connection.prepare(query);
    }

    pub fn retrieve_administrator_from_user(connection : &sqlite::Connection, user_id : i32) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT admin_id FROM administrator WHERE user_id = ".to_owned()
            + &user_id.to_string();
    
        return connection.prepare(query);
    }
}