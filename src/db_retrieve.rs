use sqlite::{Connection, Statement};

pub struct R {}

impl R {
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
    pub const USER_CHANGE_LOG: usize = 14;
    pub const FAMILIES_USERS: usize = 15;
    pub const STUDENTS_CLASSES: usize = 16;
    pub const TEACHER_CLASSES: usize = 17;
    pub const TEACHERS_SCHOOLS: usize = 18;
    pub const ADMINISTRATORS_SCHOOLS: usize = 19;

    const STRINGS: [(&str, &str); (R::USER_CHANGE_LOG + 1) as usize] = [
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

    const USER_FIELDS: [&str; 11] = [
        "user_id",
        "email",
        "username",
        "first_name",
        "last_name",
        "birthday",
        "date_registered",
        "phone",
        "icon",
        "hidden",
        "deleted",
    ];
    const ORGANIZATION_FIELDS: [&str; 9] = [
        "organization_id",
        "name",
        "address1",
        "address2",
        "city",
        "state",
        "zip",
        "phone",
        "country",
    ];
    const ADMINISTRATOR_FIELDS: [&str; 3] = ["administrator_id", "user_id", "level"];
    const SCHOOL_FIELDS: [&str; 12] = [
        "school_id",
        "organization_id",
        "super_administrator_id",
        "icon",
        "name",
        "address1",
        "address2",
        "city",
        "state",
        "zip",
        "phone",
        "country",
    ];
    const TEACHER_FIELDS: [&str; 3] = ["teacher_id", "user_id", "icon"];
    const EMPLOYEES_SUPERVISOR_FIELDS: [&str; 5] = [
        "supervisory_id",
        "user_id",
        "administrator_id",
        "supervisor_name",
        "organization_id",
    ];
    const SUBJECT_FIELDS: [&str; 6] = [
        "subject_id",
        "name",
        "ap",
        "ib",
        "target_year",
        "discipline",
    ];
    const CLASS_FIELDS: [&str; 9] = [
        "class_id",
        "school_id",
        "subject_id",
        "grade",
        "start_day",
        "end_day",
        "start_time",
        "end_time",
        "days_scheduled",
    ];
    const STUDENT_FIELDS: [&str; 2] = ["student_id", "user_id"];
    const FAMILY_FIELDS: [&str; 2] = ["family_id", "name"];
    const FAMILY_MEMBER_FIELDS: [&str; 5] = [
        "member_id",
        "user_id",
        "notification_methods",
        "email",
        "phone",
    ];
    const ASSIGNMENT_FIELDS: [&str; 6] = [
        "assignment_id",
        "class_id",
        "required",
        "grade_scale",
        "description",
        "template",
    ];
    const SUBMISSION_FIELDS: [&str; 5] = [
        "submission_id",
        "user_id",
        "assignment_id",
        "contents",
        "grade",
    ];
    const COMMENT_FIELDS: [&str; 4] = ["comment_id", "user_id", "assignment_id", "contents"];
    const USER_CHANGE_LOG_FIELDS: [&str; 6] = [
        "change_id",
        "source_name",
        "user_id",
        "type_of_change",
        "old_value",
        "timestamp",
    ];
    const FAMILIES_USERS_FIELDS: [&str; 3] = ["family_id", "user_id", "relationship"];
    const STUDENT_CLASS_FIELDS: [&str; 2] = ["student_id", "class_id"];
    const TEACHER_CLASS_FIELDS: [&str; 3] = ["teacher_id", "class_id", "role"];
    const TEACHER_SCHOOL_FIELDS: [&str; 2] = ["teacher_id", "school_id"];
    const ADMINISTRATOR_SCHOOL_FIELDS: [&str; 2] = ["administrator_id", "school_id"];

    pub async fn retrieve_details(
        connection: &sqlite::Connection,
        table_id: usize,
        item_id: String,
    ) -> Result<Statement<'_>, sqlite::Error> {
        let mut query: String = "SELECT ".to_string();

        match table_id {
            R::USERS => {
                for i in 0..R::USER_FIELDS.len() - 1 {
                    query = query + R::USER_FIELDS[i] + &", ".to_string()
                }
                query = query
                    + R::USER_FIELDS[R::USER_FIELDS.len() - 1]
                    + &" FROM ".to_string()
                    + R::STRINGS[R::USERS].0
                    + " WHERE "
                    + R::STRINGS[R::USERS].1
                    + " = "
                    + &item_id
            }
            R::ORGANIZATIONS => {
                for i in 0..R::ORGANIZATION_FIELDS.len() - 1 {
                    query = query + R::ORGANIZATION_FIELDS[i] + &", ".to_string()
                }
                query = query
                    + R::ORGANIZATION_FIELDS[R::ORGANIZATION_FIELDS.len() - 1]
                    + &" FROM ".to_string()
                    + R::STRINGS[R::ORGANIZATIONS].0
                    + " WHERE "
                    + R::STRINGS[R::ORGANIZATIONS].1
                    + " = "
                    + &item_id
            }
            R::ADMINISTRATORS => {
                for i in 0..R::ADMINISTRATOR_FIELDS.len() - 1 {
                    query = query + R::ADMINISTRATOR_FIELDS[i] + &", ".to_string()
                }
                query = query
                    + R::ADMINISTRATOR_FIELDS[R::ADMINISTRATOR_FIELDS.len() - 1]
                    + &" FROM ".to_string()
                    + R::STRINGS[R::ADMINISTRATORS].0
                    + " WHERE "
                    + R::STRINGS[R::ADMINISTRATORS].1
                    + " = "
                    + &item_id
            }
            R::SCHOOLS => {
                for i in 0..R::SCHOOL_FIELDS.len() - 1 {
                    query = query + R::SCHOOL_FIELDS[i] + &", ".to_string()
                }
                query = query
                    + R::SCHOOL_FIELDS[R::SCHOOL_FIELDS.len()]
                    + &" FROM ".to_string()
                    + R::STRINGS[R::SCHOOLS].0
                    + " WHERE "
                    + R::STRINGS[R::SCHOOLS].1
                    + " = "
                    + &item_id
            }
            R::TEACHERS => {
                for i in 0..R::TEACHER_FIELDS.len() - 1 {
                    query = query + R::TEACHER_FIELDS[i] + &", ".to_string()
                }
                query = query
                    + R::TEACHER_FIELDS[R::TEACHER_FIELDS.len()]
                    + &" FROM ".to_string()
                    + R::STRINGS[R::TEACHERS].0
                    + " WHERE "
                    + R::STRINGS[R::TEACHERS].1
                    + " = "
                    + &item_id
            }
            R::EMPLOYEES_SUPERVISORS => {
                for i in 0..R::EMPLOYEES_SUPERVISOR_FIELDS.len() - 1 {
                    query = query + R::EMPLOYEES_SUPERVISOR_FIELDS[i] + &", ".to_string()
                }
                query = query
                    + R::EMPLOYEES_SUPERVISOR_FIELDS[R::EMPLOYEES_SUPERVISOR_FIELDS.len()]
                    + &" FROM ".to_string()
                    + R::STRINGS[R::EMPLOYEES_SUPERVISORS].0
                    + " WHERE "
                    + R::STRINGS[R::EMPLOYEES_SUPERVISORS].1
                    + " = "
                    + &item_id
            }
            R::SUBJECTS => {
                for i in 0..R::SUBJECT_FIELDS.len() - 1 {
                    query = query + R::SUBJECT_FIELDS[i] + &", ".to_string()
                }
                query = query
                    + R::SUBJECT_FIELDS[R::SUBJECT_FIELDS.len()]
                    + &" FROM ".to_string()
                    + R::STRINGS[R::SUBJECTS].0
                    + " WHERE "
                    + R::STRINGS[R::SUBJECTS].1
                    + " = "
                    + &item_id
            }
            R::CLASSES => {
                for i in 0..R::CLASS_FIELDS.len() - 1 {
                    query = query + R::CLASS_FIELDS[i] + &", ".to_string()
                }
                query = query
                    + R::CLASS_FIELDS[R::CLASS_FIELDS.len()]
                    + &" FROM ".to_string()
                    + R::STRINGS[R::CLASSES].0
                    + " WHERE "
                    + R::STRINGS[R::CLASSES].1
                    + " = "
                    + &item_id
            }
            R::STUDENTS => {
                for i in 0..R::STUDENT_FIELDS.len() - 1 {
                    query = query + R::STUDENT_FIELDS[i] + &", ".to_string()
                }
                query = query
                    + R::STUDENT_FIELDS[R::STUDENT_FIELDS.len()]
                    + &" FROM ".to_string()
                    + R::STRINGS[R::STUDENTS].0
                    + " WHERE "
                    + R::STRINGS[R::STUDENTS].1
                    + " = "
                    + &item_id
            }
            R::FAMILIES => {
                for i in 0..R::FAMILY_FIELDS.len() - 1 {
                    query = query + R::FAMILY_FIELDS[i] + &", ".to_string()
                }
                query = query
                    + R::FAMILY_FIELDS[R::FAMILY_FIELDS.len()]
                    + &" FROM ".to_string()
                    + R::STRINGS[R::FAMILIES].0
                    + " WHERE "
                    + R::STRINGS[R::FAMILIES].1
                    + " = "
                    + &item_id
            }
            R::FAMILY_MEMBERS => {
                for i in 0..R::FAMILY_MEMBER_FIELDS.len() - 1 {
                    query = query + R::FAMILY_MEMBER_FIELDS[i] + &", ".to_string()
                }
                query = query
                    + R::FAMILY_MEMBER_FIELDS[R::FAMILY_MEMBER_FIELDS.len()]
                    + &" FROM ".to_string()
                    + R::STRINGS[R::FAMILY_MEMBERS].0
                    + " WHERE "
                    + R::STRINGS[R::FAMILY_MEMBERS].1
                    + " = "
                    + &item_id
            }
            R::ASSIGNMENTS => {
                for i in 0..R::ASSIGNMENT_FIELDS.len() - 1 {
                    query = query + R::ASSIGNMENT_FIELDS[i] + &", ".to_string()
                }
                query = query
                    + R::ASSIGNMENT_FIELDS[R::ASSIGNMENT_FIELDS.len()]
                    + &" FROM ".to_string()
                    + R::STRINGS[R::ASSIGNMENTS].0
                    + " WHERE "
                    + R::STRINGS[R::ASSIGNMENTS].1
                    + " = "
                    + &item_id
            }
            R::SUBMISSIONS => {
                for i in 0..R::SUBMISSION_FIELDS.len() - 1 {
                    query = query + R::SUBMISSION_FIELDS[i] + &", ".to_string()
                }
                query = query
                    + R::SUBMISSION_FIELDS[R::SUBMISSION_FIELDS.len()]
                    + &" FROM ".to_string()
                    + R::STRINGS[R::SUBMISSIONS].0
                    + " WHERE "
                    + R::STRINGS[R::SUBMISSIONS].1
                    + " = "
                    + &item_id
            }
            R::COMMENTS => {
                for i in 0..R::COMMENT_FIELDS.len() - 1 {
                    query = query + R::COMMENT_FIELDS[i] + &", ".to_string()
                }
                query = query
                    + R::COMMENT_FIELDS[R::COMMENT_FIELDS.len()]
                    + &" FROM ".to_string()
                    + R::STRINGS[R::COMMENTS].0
                    + " WHERE "
                    + R::STRINGS[R::COMMENTS].1
                    + " = "
                    + &item_id
            }
            R::USER_CHANGE_LOG => {
                for i in 0..R::USER_CHANGE_LOG_FIELDS.len() - 1 {
                    query = query + R::USER_CHANGE_LOG_FIELDS[i] + &", ".to_string()
                }
                query = query
                    + R::USER_CHANGE_LOG_FIELDS[R::USER_CHANGE_LOG_FIELDS.len()]
                    + &" FROM ".to_string()
                    + R::STRINGS[R::USER_CHANGE_LOG].0
                    + " WHERE "
                    + R::STRINGS[R::USER_CHANGE_LOG].1
                    + " = "
                    + &item_id
            }
            R::FAMILIES_USERS => panic!("Not yet implemented"),
            R::STUDENTS_CLASSES => panic!("Not yet implemented"),
            R::TEACHER_CLASSES => panic!("Not yet implemented"),
            R::TEACHERS_SCHOOLS => panic!("Not yet implemented"),
            R::ADMINISTRATORS_SCHOOLS => panic!("Not yet implemented"),
            _ => panic!("Bad table type of {} sent to retrieve_details", table_id),
        }

        println!("{}", query);

        return connection.prepare(query);
    }

    pub async fn retrieve_all_counts(connection: &sqlite::Connection) -> Vec<(String, i64)>{
        let mut info_out : Vec<(String, i64)> = Vec::new();

        for x in 0..R::USER_CHANGE_LOG {
            let query: String = "SELECT COUNT(*) as item_count FROM ".to_string() + &R::STRINGS[x].0;
            let mut string_out: String = R::STRINGS[x].0.to_owned() + &": ".to_string();
            let length = string_out.len();
            let mut count = -1;

            match connection.prepare(query) {
                Ok(x) => {
                    // retrieve the contents of the query.
                    for row in x.into_iter().map(|row| row.unwrap()) {
                        count = row.read::<i64,_>("item_count");
                        string_out = string_out + &count.to_string();
                    }
                },

                Err(x) => string_out = string_out + "Errored out because of " + &x.to_string(),
            }

            if length == string_out.len() {
                string_out += &" Did not get results, despite statement working.";
            }

            info_out.push((string_out, count));
        }
        
        info_out
    }

    pub async fn retrieve_classes_from_school(
        connection: &sqlite::Connection,
        school_id: i64,
    ) -> Result<Statement<'_>, sqlite::Error> {
        let query =
            "SELECT class_id FROM classes WHERE school_id = ".to_owned() + &school_id.to_string();

        return connection.prepare(query);
    }

    pub async fn retrieve_administrator_from_user(
        connection: &sqlite::Connection,
        user_id: i64,
    ) -> Result<Statement<'_>, sqlite::Error> {
        let query =
            "SELECT admin_id FROM administrator WHERE user_id = ".to_owned() + &user_id.to_string();

        return connection.prepare(query);
    }

    pub async fn retrieve_teacher_from_user(
        connection: &sqlite::Connection,
        user_id: i64,
    ) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT teacher_id FROM administrator WHERE user_id = ".to_owned()
            + &user_id.to_string();

        return connection.prepare(query);
    }

    pub async fn retrieve_student_from_user(
        connection: &sqlite::Connection,
        user_id: i64,
    ) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT student_id FROM administrator WHERE user_id = ".to_owned()
            + &user_id.to_string();

        return connection.prepare(query);
    }

    pub async fn retrieve_family_member_from_user(
        connection: &sqlite::Connection,
        user_id: i64,
    ) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT member_id FROM family_memebers WHERE user_id = ".to_owned()
            + &user_id.to_string();

        return connection.prepare(query);
    }

    pub async fn retrieve_schools_from_organization(
        connection: &sqlite::Connection,
        organization_id: i64,
    ) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT school_id FROM schools WHERE organization_id = ".to_owned()
            + &organization_id.to_string();

        return connection.prepare(query);
    }

    pub async fn retrieve_teachers_from_school(
        connection: &sqlite::Connection,
        school_id: i64,
    ) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT teacher_id FROM teachers_schools WHERE school_id = ".to_owned()
            + &school_id.to_string();

        return connection.prepare(query);
    }

    pub async fn retrieve_user_from_teacher(
        connection: &sqlite::Connection,
        teacher_id: i64,
    ) -> Result<Statement<'_>, sqlite::Error> {
        let query =
            "SELECT user_id FROM teachers WHERE teacher_id = ".to_owned() + &teacher_id.to_string();

        return connection.prepare(query);
    }

    pub async fn retrieve_user_from_administrator(
        connection: &sqlite::Connection,
        administrator_id: i64,
    ) -> Result<i64, sqlite::Error> {
        let query = "SELECT user_id FROM administrators WHERE administrator_id = ".to_owned()
            + &administrator_id.to_string();

        match connection.prepare(query) {
            Ok(x) => {
                for row in x.into_iter().map(|row| row.unwrap()) {
                    return Ok(row.read::<i64, _>(R::STRINGS[R::USERS].1));
                }

                panic!("retrieve_user_from_administrator had no results, but did not encounter an error.  Investigate!");
            }
            Err(x) => return Err(x),
        }
    }

    pub async fn retrieve_user_from_family_member(
        connection: &sqlite::Connection,
        member_id: i64,
    ) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT user_id FROM family_members WHERE member_id = ".to_owned()
            + &member_id.to_string();

        return connection.prepare(query);
    }

    pub async fn retrieve_user_from_student(
        connection: &sqlite::Connection,
        student_id: i64,
    ) -> Result<Statement<'_>, sqlite::Error> {
        let query =
            "SELECT user_id FROM students WHERE student_id = ".to_owned() + &student_id.to_string();

        return connection.prepare(query);
    }

    // copy of String may be necessary.
    pub async fn retrieve_organization_from_name(
        connection: &sqlite::Connection,
        name: String,
    ) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT user_id FROM students WHERE student_id = \"".to_owned()
            + &name
            + &"\"".to_string();

        return connection.prepare(query);
    }

    // copy of String may be necessary.
    pub async fn retrieve_schools_from_name(
        connection: &sqlite::Connection,
        name: String,
    ) -> Result<Statement<'_>, sqlite::Error> {
        let query =
            "SELECT school_id FROM schools WHERE name = \"".to_owned() + &name + &"\"".to_string();

        return connection.prepare(query);
    }

    pub async fn retrieve_administrators_from_school(
        connection: &sqlite::Connection,
        school_id: i64,
    ) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT administrator_id FROM administrators_schools WHERE school_id = "
            .to_owned()
            + &school_id.to_string();

        return connection.prepare(query);
    }

    pub async fn retrieve_super_administrator_from_school(
        connection: &sqlite::Connection,
        school_id: i64,
    ) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT super_administrator_id FROM administrators_schools WHERE school_id = "
            .to_owned()
            + &school_id.to_string();

        return connection.prepare(query);
    }

    pub async fn retrieve_assignments_from_class(
        connection: &sqlite::Connection,
        class_id: i64,
    ) -> Result<Statement<'_>, sqlite::Error> {
        let query = "SELECT assignment_id FROM assignments WHERE class_id = ".to_owned()
            + &class_id.to_string();

        return connection.prepare(query);
    }
}
