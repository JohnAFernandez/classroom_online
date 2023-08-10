// file for data verfication
use sqlite;

pub struct V {}
impl V {
    pub const USERS : usize = 0;
    pub const ORGANIZATIONS : usize = 1;
    pub const ADMINISTRATORS : usize = 2;
    pub const SCHOOLS : usize = 3;
    pub const TEACHERS : usize = 4;
    pub const EMPLOYEES_SUPERVISORS : usize = 5;
    pub const SUBJECTS : usize = 6;
    pub const CLASSES : usize = 7;
    pub const STUDENTS : usize = 8;
    pub const FAMILIES : usize = 9;
    pub const FAMILY_MEMBERS : usize = 10;
    pub const ASSIGNMENTS : usize = 11;
    pub const SUBMISSIONS : usize = 12;
    pub const COMMENTS : usize = 13;
    pub const USER_CHANGE_LOG : usize = 14;

    const STRINGS : [(&str, &str); (V::USER_CHANGE_LOG + 1) as usize] = [
    ("users", "user_id"),
    ("organizations", "organization_id"),
    ("administrators", "administrator_id"),
    ("schools", "school_id"),
    ("teachers", "teacher_id"),
    ("employees_supervisors", "supervisory_id"),
    ("subjects", "subject_id"),
    ("classes", "class_id"),
    ("students", "student_id"),
    ("families", "family_id"),
    ("family_members", "member_id"),
    ("assignments", "assignment_id"),
    ("submissions", "submission_id"),
    ("comments", "comment_id"),
    ("user_change_log", "change_id"),
    ];

    pub fn check_id(        
        connection: &sqlite::Connection,
        id : i64, 
        table_id : usize) -> bool {
        if table_id > V::USER_CHANGE_LOG {
            println!("You dun goofed, and sent a bad id of {} to table_id.  Gotta CRASH!", table_id);
            panic!("FUNDAMENTAL ERROR IN YOUR SERVER PROGRAMMING! FIX ME!");
        }

        let query : String = "SELECT * FROM ".to_owned() + V::STRINGS[table_id].0 
        + " WHERE " + V::STRINGS[table_id].1 + " = " + &id.to_string() + " LIMIT 1" ; 

        let mut result = connection.prepare(query).unwrap();

        while let Ok(sqlite::State::Row) = result.next(){
            println!("{} {} Exists ", table_id, id);
            return true;
        } 

        println!("{} {} does not exist.", table_id, id);

        return false;
    }

    const AT : &str = "@";
    const PERIOD : &str = ".";

    pub fn check_email(to_check : String) -> bool{

        let check = to_check.find(V::AT);

        let loc : usize;

        // no at symbol, or could be empty.
        match check {
            Some(x) => loc = x,
            None => return false,
        }

        // bad email because there's no username
        if loc < 1 {return false};

        let check2 = to_check[loc..].find(V::PERIOD);
        let loc2 : usize;

        match check2 {
            Some(x) => loc2 = x,
            None => return false,
        }

        // bad domain name
        if loc2 < 2  || loc2 == to_check[loc..].len() - 1 { return false};

        // period at the end means bad domain name
        if &to_check[to_check.len() - 1..] == "." { return false};
 
        return true;
    }

}