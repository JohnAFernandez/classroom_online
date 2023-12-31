// file for data verfication from insertion
use chrono::Datelike;
use sqlite;

pub struct V {}

impl V {
    pub const USERS: usize = 0;
    pub const ORGANIZATIONS: usize = 1;
    pub const ADMINISTRATORS: usize = 2;
    pub const SCHOOLS: usize = 3;
    pub const TEACHERS: usize = 4;
    pub const EMPLOYEES_SUPERVISORS: usize = 5;
    pub const SUBJECTS: usize = 6;
    pub const CLASSES: usize = 7;
    pub const STUDENTS: usize = 8;
    pub const FAMILIES: usize = 9;
    pub const FAMILY_MEMBERS: usize = 10;
    pub const ASSIGNMENTS: usize = 11;
    pub const SUBMISSIONS: usize = 12;
    pub const COMMENTS: usize = 13;
    pub const ADMINISTRATORS_SCHOOLS: usize = 14;
    pub const TEACHERS_SCHOOLS: usize = 15;
    pub const TEACHERS_CLASSES: usize = 16;
    pub const STUDENTS_CLASSES: usize = 17;
    pub const FAMILIES_USERS: usize = 18;
    pub const STUDENTS_SCHOOLS: usize = 19;
    pub const USER_CHANGE_LOG: usize = 20;

    const STRINGS: [(&str, &str, &str); (V::USER_CHANGE_LOG) as usize] = [
        ("users", "user_id", ""),
        ("organizations", "organization_id", ""),
        ("administrators", "administrator_id", ""),
        ("schools", "school_id", ""),
        ("teachers", "teacher_id", ""),
        ("employees_supervisors", "supervisory_id", ""),
        ("subjects", "subject_id", ""),
        ("classes", "class_id", ""),
        ("students", "student_id", ""),
        ("families", "family_id", ""),
        ("family_members", "member_id", ""),
        ("assignments", "assignment_id", ""),
        ("submissions", "submission_id", ""),
        ("comments", "comment_id", ""),
        ("administrators_schools", "administrator_id", "school_id"),
        ("teachers_schools", "teacher_id", "school_id"),
        ("teachers_classes", "teacher_id", "class_id"),
        ("students_classes", "student_id", "class_id"),
        ("families_users", "family_id", "user_id"),
        ("students_schools", "student_id", "school_id"),
    ];

    pub async fn check_id(connection: &sqlite::Connection, id: i64, table_id: usize) -> bool {
        if table_id > V::USER_CHANGE_LOG - 1{
            panic!("FUNDAMENTAL ERROR IN YOUR SERVER PROGRAMMING! FIX ME! ID {}", table_id);
        }

        let query: String = "SELECT * FROM ".to_owned()
            + V::STRINGS[table_id].0
            + " WHERE "
            + V::STRINGS[table_id].1
            + " = "
            + &id.to_string()
            + " LIMIT 1";

        let mut result = connection.prepare(query).unwrap();

        while let Ok(sqlite::State::Row) = result.next() {
            return true;
        }

        return false;
    }

    pub async fn check_id_pair(
        connection: &sqlite::Connection,
        id_1: i64,
        id_2: i64,
        table_id: usize,
    ) -> bool {
        if table_id > V::USER_CHANGE_LOG {
            panic!("FUNDAMENTAL ERROR IN YOUR SERVER PROGRAMMING! FIX ME!");
        }

        let query: String = "SELECT * FROM ".to_owned()
            + V::STRINGS[table_id].0
            + " WHERE "
            + V::STRINGS[table_id].1
            + " = "
            + &id_1.to_string()
            + " AND "
            + V::STRINGS[table_id].2
            + " = "
            + &id_2.to_string()
            + " LIMIT 1";

        let mut result = connection.prepare(query).unwrap();

        while let Ok(sqlite::State::Row) = result.next() {
            return true;
        }

        return false;
    }

    const AT: &str = "@";
    const PERIOD: &str = ".";

    pub async fn check_email(to_check: String) -> bool {
        let check = to_check.find(V::AT);

        let loc: usize;

        // no at symbol, or could be empty.
        match check {
            Some(x) => loc = x,
            None => return false,
        }

        // bad email because there's no username
        if loc < 1 {
            return false;
        };

        let check2 = to_check[loc..].find(V::PERIOD);
        let loc2: usize;

        match check2 {
            Some(x) => loc2 = x,
            None => return false,
        }

        // bad domain name
        if loc2 < 2 || loc2 == to_check[loc..].len() - 1 {
            return false;
        };

        // period at the end means bad domain name
        if &to_check[to_check.len() - 1..] == "." {
            return false;
        };

        return true;
    }

    const MAX_NAME_LENGTH: usize = 256;

    pub async fn check_name(name: &String) -> bool {
        // if there's no text in a name, or it's bigger than 64 characters, then reject
        if name.is_empty() || name.len() > V::MAX_NAME_LENGTH {
            return false;
        }

        if !name
            .to_lowercase()
            .bytes()
            .all(|b| matches!(b, b'a'..=b'z'))
        {
            return false;
        }

        return true;
    }

    pub async fn check_org_school_name(name: &String) -> bool {
        if name.is_empty() || name.len() > V::MAX_NAME_LENGTH {
            return false;
        }

        if name.chars().all(|x| char::is_numeric(x)) {
            return false;
        }

        if !name.chars().all(|x| char::is_alphanumeric(x)) {
            return false;
        }

        return true;
    }

    const MMDDYY_LENGTH: usize = 8;
    const MMDDYYYY_LENGTH: usize = 10;

    pub async fn check_birthday(bd: &String) -> bool {
        if bd.len() != V::MMDDYY_LENGTH && bd.len() != V::MMDDYYYY_LENGTH {
            return false;
        }

        if &bd[2..3] != "/" && &bd[2..3] != "\\" {
            return false;
        }

        if &bd[5..6] != "/" && &bd[5..6] != "\\" {
            return false;
        }

        if !bd[0..2].chars().all(|x| x.is_numeric()) {
            return false;
        }

        if !bd[3..5].chars().all(|x| x.is_numeric()) {
            return false;
        }

        if !bd[6..].chars().all(|x| x.is_numeric()) {
            return false;
        }

        let month: usize = bd[0..2].to_string().parse().unwrap();

        if month < 1 || month > 12 {
            return false;
        }

        let mut year: usize = bd[6..].to_string().parse().unwrap();
        let mut leap_year: bool = false;

        if bd[6..].to_string().len() == 2 {
            if year > 23 {
                year += 1900;
            } else {
                year += 2000;
            }
        }

        let current_year: usize = chrono::Utc::now().year() as usize;

        if year < current_year - 120 {
            return false;
        } else if year > current_year {
            return false;
        }

        // NOTE if we use this server long enough (after 2100), you will have to switch to a more complicated formula to check
        // Every 100 years we skip a leap year, *except* every 400 years (untested):  if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
        // But the way it is now is just a convenient optimization.
        if year % 4 == 0 {
            leap_year = true;
        }

        let day: usize = bd[3..5].to_string().parse().unwrap();

        // sorry, but validating the day of the month is a little logically complicated
        if day < 1 {
            return false;

        // check february
        } else if month == 2 {
            // check leap year
            if leap_year && day > 29 {
                return false;
            // check regular february
            } else if !leap_year && day > 28 {
                return false;
            }
        // check 30 day months
        } else {
            if month == 4 || month == 6 || month == 9 || month == 11 {
                if day > 30 {
                    return false;
                }
            } else {
                if day > 31 {
                    return false;
                }
            }
        }

        return true;
    }
}
