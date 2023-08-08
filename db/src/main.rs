use sqlite;


fn main() {

    let _connection = init_database();

    loop {

        break;
    }

    println!("Connection listener for app section not yet implemented.  Aborting. ")

}

fn init_database() -> sqlite::Connection{
    let connection = sqlite::open("/Users/johnbootcamp/Desktop/kotlin/classroom_online/db/src/db/dbmain_database.sql").unwrap();

    // create users table. This is the basic information for all user types.
    let mut query = "
        CREATE TABLE users (
            user_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            email STRING NOT NULL,
            password STRING NOT NULL,
            first_name STRING NOT NULL,
            last_name STRING NOT NULL,
            type STRING NOT NULL
            );
    ";
    connection.execute(query).unwrap();

    // think school district
    query = "
        CREATE TABLE organization (
            organization_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            name STRING NOT NULL UNIQUE,
            address1 STRING,
            address2 STRING,
            city STRING,
            state STRING,
            zip STRING,
            phone STRING,
            country STRING
        );
    ";
    connection.execute(query).unwrap();

    // we use supervisor name here in case there is no higher supervisor registered, but a supervisor does exist
    // the buisness logic for this will have to be tricky, because we need to tell organizations or schools when someone
    // doesn't have a supervisor marked
    query = "
        CREATE TABLE administrators (
            administrator_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            user_id INTEGER NOT NULL,
            level STRING NOT NULL,
            supervisor_name STRING NOT NULL,
            supervisor_id,
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
            user_id INTEGER,
            name STRING,
            icon STRING,
            FOREIGN KEY (user_id) REFERENCES users(user_id)
        );
    ";
    connection.execute(query).unwrap();

    query = "
        CREATE TABLE teachers_school (
            teacher_id INTEGER NOT NULL,
            school_id INTEGER NOT NULL,
            PRIMARY KEY (teacher_id, school_id),
            FOREIGN KEY (teacher_id) REFERENCES teachers(teacher_id)
            FOREIGN KEY (school_id) REFERENCES schools(school_id)
        );
    ";
    connection.execute(query).unwrap();

    // In the future we'll want to match this up with standards.
    query = "
        CREATE TABLE subjects (
            subject_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            name STRING UINQUE NOT NULL,
            ap INTEGER DEFAULT 0 NOT NULL,
            ib INTEGER DEFAULT 0 NOT NULL,
            
        )
    ";

    connection.execute(query).unwrap();

    query = "
        CREATE TABLE classes (
            class_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            subject STRING
        )
    ";

    connection.execute(query).unwrap();
    connection.execute(query).unwrap();


    connection
}