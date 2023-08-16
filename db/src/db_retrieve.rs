use sqlite::{Connection, Statement};

pub struct R {}

impl R {
    pub fn retrieve_classes_from_school(connection : &sqlite::Connection, school_id : i64) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT class_id FROM schools WHERE school_id = ".to_owned()
            + &school_id.to_string();
    
        return connection.prepare(query);
    }

    pub fn retrieve_administrator_from_user(connection : &sqlite::Connection, user_id : i64) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT admin_id FROM administrator WHERE user_id = ".to_owned()
            + &user_id.to_string();
    
        return connection.prepare(query);
    }

    pub fn retrieve_teacher_from_user(connection : &sqlite::Connection, user_id : i64) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT teacher_id FROM administrator WHERE user_id = ".to_owned()
            + &user_id.to_string();
    
        return connection.prepare(query);
    }

    pub fn retrieve_student_from_user(connection : &sqlite::Connection, user_id : i64) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT student_id FROM administrator WHERE user_id = ".to_owned()
            + &user_id.to_string();
    
        return connection.prepare(query);
    }

    pub fn retrieve_family_member_from_user(connection : &sqlite::Connection, user_id : i64) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT member_id FROM family_memebers WHERE user_id = ".to_owned()
            + &user_id.to_string();
    
        return connection.prepare(query);
    }

    pub fn retrieve_schools_from_organization(connection : &sqlite::Connection, organization_id : i64) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT school_id FROM schools WHERE organization_id = ".to_owned()
            + &organization_id.to_string();
    
        return connection.prepare(query);
    }

    pub fn retrieve_teachers_from_school(connection : &sqlite::Connection, school_id : i64) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT teacher_id FROM teachers_schools WHERE school_id = ".to_owned()
            + &school_id.to_string();
    
        return connection.prepare(query);
    }

    pub fn retrieve_user_from_teacher(connection : &sqlite::Connection, teacher_id : i64) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT user_id FROM teachers WHERE teacher_id = ".to_owned()
            + &teacher_id.to_string();
    
        return connection.prepare(query);
    }
    
    pub fn retrieve_user_from_administrator(connection : &sqlite::Connection, administrator_id : i64) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT user_id FROM administrators WHERE administrator_id = ".to_owned()
            + &administrator_id.to_string();
    
        return connection.prepare(query);
    }

    pub fn retrieve_user_from_family_member(connection : &sqlite::Connection, member_id : i64) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT user_id FROM family_members WHERE member_id = ".to_owned()
            + &member_id.to_string();
    
        return connection.prepare(query);
    }

    pub fn retrieve_user_from_student(connection : &sqlite::Connection, student_id : i64) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT user_id FROM students WHERE student_id = ".to_owned()
            + &student_id.to_string();
    
        return connection.prepare(query);
    }

    // copy of String may be necessary.
    pub fn retrieve_organization_from_name(connection : &sqlite::Connection, name : String) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT user_id FROM students WHERE student_id = \"".to_owned()
            + &name + &"\"".to_string();
    
        return connection.prepare(query);
    }

    // copy of String may be necessary.
    pub fn retrieve_schools_from_name(connection : &sqlite::Connection, name : String) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT school_id FROM schools WHERE name = \"".to_owned()
            + &name + &"\"".to_string();
        
        return connection.prepare(query);
    }    
    
    pub fn retrieve_administrators_from_school(connection : &sqlite::Connection, school_id : i64) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT administrator_id FROM administrators_schools WHERE school_id = ".to_owned()
            + &school_id.to_string();
    
        return connection.prepare(query);
    }

    pub fn retrieve_super_administrator_from_school(connection : &sqlite::Connection, school_id : i64) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT super_administrator_id FROM administrators_schools WHERE school_id = ".to_owned()
            + &school_id.to_string();
    
        return connection.prepare(query);
    }


}