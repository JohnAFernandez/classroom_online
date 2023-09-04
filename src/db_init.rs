use std::path::PathBuf;

pub async fn init_database(path: PathBuf) -> sqlite::Connection {

    let connection = sqlite::open(path).unwrap();

    // TODO!  WE need some anscillary tables that will have to be checked.
    // Not states, that can be done at the rust level.  But we will need to do zip codes
    // like that.  Also we need to do Disciplines .... Hmmmm

    // create users table. This is the basic information for all user types.
    // now, we usually can use email for logins, but many students will not have one.
    // The intention is to be able to use either email or username for logins.
    // But only family and student accounts can be registered without an email.
    // additionally, a username has to be unique. To enforce, usernames are assigned by
    // first and last name and numerical suffix.
    let mut query = "
        CREATE TABLE users (
            user_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            email STRING,
            username STRING NOT NULL UNIQUE,
            password STRING NOT NULL,
            first_name STRING NOT NULL,
            last_name STRING NOT NULL,
            birthday STRING_NOT_NULL,
            date_registered STRING NOT NULL,
            phone STRING,
            icon STRING,
            hidden INTEGER DEFAULT 0,
            deleted INTEGER DEFAULT 0
            );
    ";
    connection.execute(query).unwrap();

    // think school district
    query = "
        CREATE TABLE organizations (
            organization_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            name STRING NOT NULL UNIQUE,
            address1 STRING,
            address2 STRING,
            city STRING,
            state STRING,
            zip STRING,
            phone STRING,
            country STRING,
            deactivated INTEGER DEFAULT 0
        );
    ";
    connection.execute(query).unwrap();

    // we use supervisor name here in case there is no higher supervisor registered, but a supervisor does exist
    // the buisness logic for this will have to be tricky, because we need to tell organizations or schools when someone
    // doesn't have a supervisor marked
    query = "
        CREATE TABLE administrators (
            administrator_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            user_id INTEGER NOT NULL UNIQUE,
            level STRING NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(user_id)
        );
    ";
    connection.execute(query).unwrap();

    // super admins are the ones that are in charge of the account for the school.
    query = "
        CREATE TABLE schools (
            school_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            organization_id INTEGER NOT NULL,
            super_administrator_id INTEGER,
            icon STRING,
            name STRING NOT NULL,
            address1 STRING,
            address2 STRING,
            city STRING,
            state STRING,
            zip STRING,
            phone STRING,
            country STRING,
            FOREIGN KEY (organization_id) REFERENCES organizations(organization_id),
            FOREIGN KEY (super_administrator_id) REFERENCES administrators(administrator_id)
        );
    ";
    connection.execute(query).unwrap();

    // You can have regional administrators, therefore admins need to be able to be
    // associated with more than one school
    query = "
        CREATE TABLE administrators_schools (
            administrator_id INTEGER NOT NULL,
            school_id INTEGER NOT NULL,
            PRIMARY KEY (administrator_id, school_id),
            FOREIGN KEY (administrator_id) REFERENCES administrators(administrator_id)
            FOREIGN KEY (school_id) REFERENCES schools(school_id)
        );
    ";
    connection.execute(query).unwrap();

    query = "
        CREATE TABLE teachers (
            teacher_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            user_id INTEGER UNIQUE,
            icon STRING,
            FOREIGN KEY (user_id) REFERENCES users(user_id)
        );
    ";
    connection.execute(query).unwrap();

    // this is a little complicated because a supervisor may not have an account.
    // So, add a name field.  That user can confirm later, if necessary.
    query = "
        CREATE TABLE employees_supervisors (
            supervisory_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            user_id INTEGER,
            administrator_id INTEGER,
            supervisor_name STRING,
            organization_id INTEGER, 
            FOREIGN KEY (user_id) REFERENCES users(user_id),
            FOREIGN KEY (administrator_id) REFERENCES administrators(administrator_id),
            FOREIGN KEY (organization_id) REFERENCES organizations(organization_id)
        );
    ";
    connection.execute(query).unwrap();

    query = "
        CREATE TABLE teachers_schools (
            teacher_id INTEGER NOT NULL,
            school_id INTEGER NOT NULL,
            PRIMARY KEY (teacher_id, school_id),
            FOREIGN KEY (teacher_id) REFERENCES teachers(teacher_id)
            FOREIGN KEY (school_id) REFERENCES schools(school_id)
        );
    ";
    connection.execute(query).unwrap();

    // In the future we'll want to match this up with standards -- if this ends up being used by anyone, anyway
    query = "
        CREATE TABLE subjects (
            subject_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            name STRING UINQUE NOT NULL,
            ap INTEGER DEFAULT 0 NOT NULL,
            ib INTEGER DEFAULT 0 NOT NULL,
            target_year STRING,
            discipline STRING
        );
    ";
    connection.execute(query).unwrap();

    query = "
        CREATE TABLE classes (
            class_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            school_id INTEGER NOT NULL,
            subject_id INTEGER,
            grade INTEGER,
            start_day INTEGER,
            end_day INTEGER,
            start_time INTEGER,
            end_time INTEGER,
            days_scheduled STRING,
            FOREIGN KEY (subject_id) REFERENCES subjects (subject_id),
            FOREIGN KEY (school_id) REFERENCES schools (school_id)
        );
    ";
    connection.execute(query).unwrap();

    query = "
        CREATE TABLE teachers_classes (
            teacher_id INTEGER NOT NULL,
            class_id INTEGER NOT NULL,
            role STRING,
            FOREIGN KEY (teacher_id) REFERENCES teachers(teacher_id),
            FOREIGN KEY (class_id) REFERENCES classes(class_id)
        );
    ";
    connection.execute(query).unwrap();

    query = "
        CREATE TABLE students (
            student_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            user_id INTEGER UNIQUE,
            FOREIGN KEY (user_id) REFERENCES users(user_id)
        );
    ";
    connection.execute(query).unwrap();

    query = "
        CREATE TABLE students_classes (
            student_id INTEGER,
            class_id INTEGER,
            PRIMARY KEY (student_id, class_id),
            FOREIGN KEY (student_id) REFERENCES students(student_id),
            FOREIGN KEY (class_id) REFERENCES classes(class_id)
        )
    ";
    connection.execute(query).unwrap();

    // at first we had guardian accounts, but this allows us to add
    // arbitrary family members that want to be kept in the loop.
    // you can all track each other via the
    query = "
        CREATE TABLE families (
            family_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            name STRING
        );
    ";
    connection.execute(query).unwrap();

    // this is a little strange, but once you create a family, associate it with all family members
    query = "
        CREATE TABLE families_users (
            family_id INTEGER,
            user_id INTEGER,
            relationship STRING,
            PRIMARY KEY(family_id, user_id),
            FOREIGN KEY (family_id) REFERENCES families(family_id),
            FOREIGN KEY (user_id) REFERENCES users(user_id)
        );
    ";
    connection.execute(query).unwrap();

    // not all family members will have a user id assoicated
    query = "
        CREATE TABLE family_members (
            member_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            user_id INTEGER UNIQUE,
            notification_methods STRING,
            email STRING,
            phone STRING,
            FOREIGN KEY (user_id) REFERENCES users(user_id)
        );
    ";
    connection.execute(query).unwrap();

    // grade scale needs a good way to be handled.
    // We literally could have anything there and SQL doesn't like doing just anything.
    query = "
        CREATE TABLE assignments (
            assignment_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            class_id INTEGER,
            required INT DEFAULT 0,
            grade_scale STRING NOT NULL,
            name STRING NOT NULL,
            description STRING,
            template STRING,
            FOREIGN KEY (class_id) REFERENCES classes(class_id)
        );
    ";
    connection.execute(query).unwrap();

    query = "
        CREATE TABLE submissions (
            submission_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            user_id INTEGER NOT NULL,
            assignment_id INTEGER NOT NULL,
            contents STRING,
            grade STRING,
            FOREIGN KEY (user_id) REFERENCES users(user_id),
            FOREIGN KEY (assignment_id) REFERENCES assignments(assignment_id)
        )
    ";
    connection.execute(query).unwrap();

    query = "
        CREATE TABLE comments (
            comment_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            user_id INTEGER NOT NULL,
            assignment_id INTEGER NOT NULL,
            contents STRING,
            FOREIGN KEY (assignment_id) REFERENCES assignments(assignment_id)
        );
    ";
    connection.execute(query).unwrap();

    query = "
        CREATE TABLE user_change_log (
            change_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            source_name STRING,
            user_id INTEGER,
            type_of_change INT,
            old_value STRING,
            timestamp STRING,
            FOREIGN KEY (user_id) REFERENCES users(user_id)
        );
    ";
    connection.execute(query).unwrap();

    connection
}
