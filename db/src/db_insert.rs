// This file is for insertion of new information into the database.
// Data needs to be validated before these functions are called.

use sqlite;

pub struct I {}

impl I {
    const INSERT: &str = "INSERT INTO ";

    // The escaped quotation marks in these constants are for the quotation marks that SQL requires.

    const VALUES: &str = " VALUES (";
    const VALUES_S: &str = " VALUES (\"";

    // no string type for SQL
    const AND: &str = ",";
    // only left is sql string
    const S_AND: &str = "\",";
    // only right is sql string
    const AND_S: &str = ",\"";
    // left and right are sql strings
    const S_AND_S: &str = "\",\"";
    // end of value insertion list
    const END: &str = ");";
    // end of value insertion list where last item was sql string.
    const S_END: &str = "\");";

    pub fn insert_user(
        connection: &sqlite::Connection,
        email: &String,
        username: &String,
        password: &String,
        first_name: &String,
        last_name: &String,
        birthday: &String,
        date_registered: &String,
        phone: &String,
        icon: &String,
    ) {
        let query: String = I::INSERT.to_owned() + "users (email, username, password, first_name, last_name, birthday, date_registered, phone, icon)" 
        + I::VALUES_S + email + I::S_AND_S + username + I::S_AND_S + password + I::S_AND_S + first_name + I::S_AND_S + last_name + I::S_AND_S + birthday + I::S_AND_S + date_registered + I::S_AND_S + phone + I::S_AND_S + icon + I::S_END;

        connection.execute(query).unwrap();
    }

    pub fn insert_organization(
        connection: &sqlite::Connection,
        name: &String,
        address1: &String,
        address2: &String,
        city: &String,
        state: &String,
        zip: &String,
        phone: &String,
        country: &String,
    ) {
        let query: String = I::INSERT.to_owned()
            + "organizations (name, address1, address2, city, state, zip, phone, country)"
            + I::VALUES_S
            + name
            + I::S_AND_S
            + address1
            + I::S_AND_S
            + address2
            + I::S_AND_S
            + city
            + I::S_AND_S
            + state
            + I::S_AND_S
            + zip
            + I::S_AND_S
            + phone
            + I::S_AND_S
            + country
            + I::S_END;

        connection.execute(query).unwrap();
    }

    pub fn insert_administrator(connection: &sqlite::Connection, user_id: &String, level: &String) {
        let query: String = I::INSERT.to_owned()
            + "administrators (user_id, level)"
            + I::VALUES
            + user_id
            + I::AND_S
            + level
            + I::S_END;

        connection.execute(query).unwrap();
    }

    pub fn insert_school(
        connection: &sqlite::Connection,
        organization_id: &String,
        super_administrator_id: &String,
        icon: &String,
        name: &String,
        address1: &String,
        address2: &String,
        city: &String,
        state: &String,
        zip: &String,
        phone: &String,
        country: &String,
    ) {
        let query: String = I::INSERT.to_owned() + "schools (organization_id, super_administrator_id, icon, name, address1, address2, city, state, zip, phone, country)" 
        + I::VALUES + organization_id + I::AND + super_administrator_id + I::AND_S + icon + I::S_AND_S + name + I::S_AND_S + address1 + I::S_AND_S + address2 + I::S_AND_S + city + I::S_AND_S + state + I::S_AND_S + zip + I::S_AND_S + phone + I::S_AND_S + country + I::S_END;

        connection.execute(query).unwrap();
    }

    pub fn insert_teacher(connection: &sqlite::Connection, user_id: &String) {
        let query: String =
            I::INSERT.to_owned() + "teachers (user_id)" + I::VALUES + user_id + I::END;

        connection.execute(query).unwrap();
    }

    pub fn insert_subject(
        connection: &sqlite::Connection,
        name: &String,
        ap: &String,
        ib: &String,
        target_year: &String,
        discipline: &String,
    ) {
        let query: String = I::INSERT.to_owned()
            + "subjects (name, ap, ib, target_year, discipline)"
            + I::VALUES_S
            + name
            + I::S_AND
            + ap
            + I::AND
            + ib
            + I::AND_S
            + target_year
            + I::S_AND_S
            + discipline
            + I::S_END;

        connection.execute(query).unwrap();
    }

    pub fn insert_class(
        connection: &sqlite::Connection,
        school_id: &String,
        subject_id: &String,
        grade: &String,
        start_day: &String,
        end_day: &String,
        start_time: &String,
        end_time: &String,
        days_scheduled: &String,
    ) {
        let query: String = I::INSERT.to_owned() + "classes (school_id, subject_id, grade, start_day, end_day, start_time, end_time, days_scheduled)" + 
        I::VALUES + school_id + I::AND + subject_id + I::AND + grade + I::AND + start_day + I::AND + end_day + I::AND + start_time + I::AND + end_time + I::AND_S + days_scheduled + I::S_END;

        connection.execute(query).unwrap();
    }

    pub fn insert_student(connection: &sqlite::Connection, user_id: &String) {
        let query: String =
            I::INSERT.to_owned() + "students (user_id)" + I::VALUES + user_id + I::END;

        connection.execute(query).unwrap();
    }

    pub fn insert_family(connection: &sqlite::Connection, name: &String) {
        let query: String =
            I::INSERT.to_owned() + "families (name)" + I::VALUES_S + name + I::S_END;

        connection.execute(query).unwrap();
    }

    pub fn insert_family_member(
        connection: &sqlite::Connection,
        user_id: &String,
        notification_methods: &String,
        email: &String,
        phone: &String,
    ) {
        let query: String = I::INSERT.to_owned()
            + "family_members (user_id, notification_methods, email, phone)"
            + I::VALUES
            + user_id
            + I::AND_S
            + notification_methods
            + I::S_AND_S
            + email
            + I::S_AND_S
            + phone
            + I::S_END;

        connection.execute(query).unwrap();
    }

    pub fn insert_assignment(
        connection: &sqlite::Connection,
        class_id: &String,
        required: &String,
        grade_scale: &String,
        description: &String,
        template: &String,
    ) {
        let query: String = I::INSERT.to_owned()
            + "assignments (class_id, required, grade_scale, description, template)"
            + I::VALUES
            + class_id
            + I::AND
            + required
            + I::AND_S
            + grade_scale
            + I::S_AND_S
            + description
            + I::S_AND_S
            + template
            + I::S_END;

        connection.execute(query).unwrap();
    }

    pub fn insert_submission(
        connection: &sqlite::Connection,
        user_id: &String,
        assignment_id: &String,
        contents: &String,
        grade: &String,
    ) {
        let query: String = I::INSERT.to_owned()
            + "submissions (user_id, assignment_id, contents, grade)"
            + I::VALUES
            + user_id
            + I::AND
            + assignment_id
            + I::AND_S
            + contents
            + I::S_AND_S
            + grade
            + I::S_END;

        connection.execute(query).unwrap();
    }

    pub fn insert_comments(
        connection: &sqlite::Connection,
        user_id: &String,
        assignment_id: &String,
        contents: &String,
    ) {
        let query: String = I::INSERT.to_owned()
            + "comments (user_id, assignment_id, contents)"
            + I::VALUES
            + user_id
            + I::AND
            + assignment_id
            + I::AND_S
            + contents
            + I::S_END;

        connection.execute(query).unwrap();
    }

    pub fn insert_change_log(
        connection: &sqlite::Connection,
        source_name: &String,
        type_of_change: &String,
        old_value: &String,
        timestamp: &String,
    ) {
        let query: String = I::INSERT.to_owned()
            + "user_change_log (source_name, type_of_change, old_value, timestamp)"
            + I::VALUES_S
            + source_name
            + I::S_AND
            + type_of_change
            + I::AND_S
            + old_value
            + I::S_AND_S
            + timestamp
            + I::S_END;

        connection.execute(query).unwrap();
    }

    pub fn insert_administrator_school(
        connection: &sqlite::Connection,
        administrator_id: &String,
        school_id: &String,
    ) {
        let query: String = I::INSERT.to_owned()
            + "administrators_schools (administrator_id, school_id)"
            + I::VALUES
            + administrator_id
            + I::AND
            + school_id
            + I::END;

        connection.execute(query).unwrap();
    }

    pub fn insert_employee_supervisor(
        connection: &sqlite::Connection,
        user_id: &String,
        administrator_id: &String,
        supervisor_name: &String,
        organization_id: &String,
    ) {
        let query: String = I::INSERT.to_owned()
            + "employees_supervisors (user_id, administrator_id, supervisor_name, organization_id)"
            + I::VALUES
            + user_id
            + I::AND
            + administrator_id
            + I::AND_S
            + supervisor_name
            + I::S_AND
            + organization_id
            + I::END;

        connection.execute(query).unwrap();
    }

    pub fn insert_teachers_schools(
        connection: &sqlite::Connection,
        teacher_id: &String,
        school_id: &String,
    ) {
        let query: String = I::INSERT.to_owned()
            + "teachers_schools (teacher_id, school_id)"
            + I::VALUES
            + teacher_id
            + I::AND
            + school_id
            + I::END;

        connection.execute(query).unwrap();
    }

    pub fn insert_teachers_classes(
        connection: &sqlite::Connection,
        teacher_id: &String,
        class_id: &String,
        role: &String,
    ) {
        let query: String = I::INSERT.to_owned()
            + "teachers_classes (teacher_id, class_id, role)"
            + I::VALUES
            + teacher_id
            + I::AND
            + class_id
            + I::AND_S
            + role
            + I::S_END;

        connection.execute(query).unwrap();
    }

    pub fn insert_students_classes(
        connection: &sqlite::Connection,
        student_id: &String,
        class_id: &String,
    ) {
        let query: String = I::INSERT.to_owned()
            + "students_classes (student_id, class_id)"
            + I::VALUES
            + student_id
            + I::AND
            + class_id
            + I::END;

        connection.execute(query).unwrap();
    }

    pub fn insert_families_users(
        connection: &sqlite::Connection,
        family_id: &String,
        user_id: &String,
        relationship: &String,
    ) {
        let query: String = I::INSERT.to_owned()
            + "families_users (family_id, user_id, relationship)"
            + I::VALUES
            + family_id
            + I::AND
            + user_id
            + I::AND_S
            + relationship
            + I::S_END;

        connection.execute(query).unwrap();
    }
}
