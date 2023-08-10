// This file is for insertion of new information into the database.
// Data needs to be validated before these functions are called.

use sqlite;

pub struct I {}

impl I {
    const INSERT : &str = "INSERT INTO ";

    // The escaped quotation marks in these constants are for the quotation marks that SQL requires.
    
    const VALUES : &str = " VALUES (";
    const VALUES_S : &str = " VALUES (\"";

    // no string type for SQL
    const AND : &str = ",";
    // only left is sql string
    const S_AND : &str = "\","; 
    // only right is sql string
    const AND_S : &str = ",\""; 
    // left and right are sql strings
    const S_AND_S : &str = "\",\"";
    // end of value insertion list
    const END : &str = ");";
    // end of value insertion list where last item was sql string.
    const S_END : &str = "\");";


    pub fn insert_user( connection : &sqlite::Connection, email : &String , username : &String, password : &String, first_name : &String, last_name : &String, birthday : &String, date_registered : &String, phone : &String, icon : &String) {
        let query: String = I::INSERT.to_owned() + "users (email, username, password, first_name, last_name, birthday, date_registered, phone, icon)" 
        + I::VALUES_S + email + I::S_AND_S + username + I::S_AND_S + password + I::S_AND_S + first_name + I::S_AND_S + last_name + I::S_AND_S + birthday + I::S_AND_S + date_registered + I::S_AND_S + phone + I::S_AND_S + icon + I::S_END;

        println!("{}", query);

        connection.execute(query).unwrap();
    }

    pub fn insert_organization( connection : &sqlite::Connection, name : &String , address1 : &String, address2 : &String, city : &String, state : &String, zip : &String, phone : &String, country : &String) {        
        let query: String = I::INSERT.to_owned() + "organization (name, address1, address2, city, state, zip, phone, country)" 
        + I::VALUES_S + name + I::S_AND_S + address1 + I::S_AND_S + address2 + I::S_AND_S + city + I::S_AND_S + state + I::S_AND_S + zip + I::S_AND_S + phone + I::S_AND_S + country + I::S_END;

        println!("{}", query);

        connection.execute(query).unwrap();
    }

    pub fn insert_administrator( connection : &sqlite::Connection, user_id : &String, level : &String) {     
        let query: String = I::INSERT.to_owned() + "administrators (user_id, level)" 
        + I::VALUES + user_id + I::AND_S + level + I::S_END;

        println!("{}", query);

        connection.execute(query).unwrap();
    }

    pub fn insert_school( connection : &sqlite::Connection, organization_id : &String, super_administrator_id : &String, icon : &String, name : &String, address1 : &String, address2 : &String, city : &String, state : &String, zip : &String, phone : &String, country : &String) {     
        let query: String = I::INSERT.to_owned() + "schools (organization_id, super_administrator_id, icon, name, address1, address2, city, state, zip, phone, country)" 
        + I::VALUES + organization_id + I::AND + super_administrator_id + I::AND_S + icon + I::S_AND_S + name + I::S_AND_S + address1 + I::S_AND_S + address2 + I::S_AND_S + city + I::S_AND_S + state + I::S_AND_S + zip + I::S_AND_S + phone + I::S_AND_S + country + I::S_END;

        println!("{}", query);

        connection.execute(query).unwrap();
    }

    pub fn insert_teacher( connection : &sqlite::Connection, user_id : &String) {       
        let query: String = I::INSERT.to_owned() + "teachers (user_id)" + I::VALUES + user_id + I::END;

        println!("{}", query);

        connection.execute(query).unwrap();
    }

    pub fn insert_subject( connection : &sqlite::Connection, name : &String, ap : &String, ib : &String, target_grade : &String, discipline : &String,) {       
        let query: String = I::INSERT.to_owned() + "subjects (name, ap, ib, target_grade, discipline)" + I::VALUES_S + name + I::S_AND + ap + I::AND + ib + I::AND + target_grade + I::AND_S + discipline + I::S_END;

        println!("{}", query);

        connection.execute(query).unwrap();
    }

    pub fn insert_class( connection : &sqlite::Connection, school_id : &String, subject_id : &String, grade : &String, start_day : &String, end_day : &String, start_time : &String, end_time : &String, days_scheduled : &String,) {
        let query: String = I::INSERT.to_owned() + "classes (school_id, subject_id, grade, start_day, end_day, start_time, end_time, days_scheduled)" + 
        I::VALUES + school_id + I::AND + subject_id + I::AND + grade + I::AND + start_day + I::AND + end_day + I::AND + start_time + I::AND + end_time + I::AND_S + days_scheduled + I::S_END;

        println!("{}", query);

        connection.execute(query).unwrap();
    }


}