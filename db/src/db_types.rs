// need these to print results and make sure we can test against each other.
#[derive(PartialEq, Eq, Debug)]
pub struct User {
    id: i64,
    email: String,
    username: String,
    password: String,
    first_name: String,
    last_name: String,
    birthday: String,
    date_registered: String,
    phone: String,
    icon: String,
    hidden: bool,
    deleted: bool,
}

pub fn build_user(
    id: i64,
    email: String,
    username: String,
    password: String,
    first_name: String,
    last_name: String,
    birthday: String,
    date_registered: String,
    phone: String,
    icon: String,
    hidden: bool,
    deleted: bool,
) -> User {
    User {
        id,
        email,
        username,
        password,
        first_name,
        last_name,
        birthday,
        date_registered,
        phone,
        icon,
        hidden,
        deleted,
    }
}

#[derive(PartialEq, Eq, Debug)]

pub struct Organization {
    id: i64,
    name: String,
    address1: String,
    address2: String,
    city: String,
    state: String,
    zip: String,
    phone: String,
    country: String,
}

pub fn build_organization(
    id: i64,
    name: String,
    address1: String,
    address2: String,
    city: String,
    state: String,
    zip: String,
    phone: String,
    country: String,
) -> Organization {
    Organization {
        id,
        name,
        address1,
        address2,
        city,
        state,
        zip,
        phone,
        country,
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Administrator {
    id: i64,
    user_id: i64,
    level: String,
}

pub fn build_administrator(id: i64, user_id: i64, level: String) -> Administrator {
    Administrator { id, user_id, level }
}

#[derive(PartialEq, Eq, Debug)]
pub struct School {
    id: i64,
    organization_id: i64,
    super_administrator_id: i64,
    icon: String,
    name: String,
    address1: String,
    address2: String,
    city: String,
    state: String,
    zip: String,
    phone: String,
    country: String,
}

pub fn build_school(
    id: i64,
    organization_id: i64,
    super_administrator_id: i64,
    icon: String,
    name: String,
    address1: String,
    address2: String,
    city: String,
    state: String,
    zip: String,
    phone: String,
    country: String,
) -> School {
    School {
        id,
        organization_id,
        super_administrator_id,
        icon,
        name,
        address1,
        address2,
        city,
        state,
        zip,
        phone,
        country,
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Administrator_School {
    admin_id: i64,
    school_id: i64,
}

pub fn build_administrator_school(admin_id: i64, school_id: i64) -> Administrator_School {
    Administrator_School {
        admin_id,
        school_id,
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Teacher {
    teacher_id: i64,
    user_id: i64,
}

pub fn build_teacher(teacher_id: i64, user_id: i64) -> Teacher {
    Teacher {
        teacher_id,
        user_id,
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Employee_Supervisor {
    id: i64,
    user_id: i64,
    administrator_id: i64,
    supervisor_name: String,
}

pub fn build_employee_supervisor(
    id: i64,
    user_id: i64,
    administrator_id: i64,
    supervisor_name: String,
) -> Employee_Supervisor {
    Employee_Supervisor {
        id,
        user_id,
        administrator_id,
        supervisor_name,
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Teacher_School {
    teacher_id: i64,
    school_id: i64,
}

pub fn build_teacher_school(teacher_id: i64, school_id: i64) -> Teacher_School {
    Teacher_School {
        teacher_id,
        school_id,
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Subject {
    id: i64,
    name: String,
    ap: bool,
    ib: bool,
    target_year: String,
    discipline: String,
}

pub fn build_subject(
    id: i64,
    name: String,
    ap: bool,
    ib: bool,
    target_year: String,
    discipline: String,
) -> Subject {
    Subject {
        id,
        name,
        ap,
        ib,
        target_year,
        discipline,
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Class {
    id: i64,
    school_id: i64,
    subject_id: i64,
    year: String,
    start_day: i32,
    end_day: i32,
    start_time: i32,
    end_time: i32,
    days_scheduled: String,
}

pub fn build_class(
    id: i64,
    school_id: i64,
    subject_id: i64,
    year: String,
    start_day: i32,
    end_day: i32,
    start_time: i32,
    end_time: i32,
    days_scheduled: String,
) -> Class {
    Class {
        id,
        school_id,
        subject_id,
        year,
        start_day,
        end_day,
        start_time,
        end_time,
        days_scheduled,
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Teacher_Class {
    teacher_id: i64,
    class_id: i64,
    role: String,
}

pub fn build_teacher_class(teacher_id: i64, class_id: i64, role: String) -> Teacher_Class {
    Teacher_Class {
        teacher_id,
        class_id,
        role,
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Student {
    id: i64,
    user_id: i64,
}

pub fn build_student(id: i64, user_id: i64) -> Student {
    Student { id, user_id }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Student_Class {
    student_id: i64,
    class_id: i64,
}

pub fn build_student_class(student_id: i64, class_id: i64) -> Student_Class {
    Student_Class {
        student_id,
        class_id,
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Family {
    family_id: i64,
    name: String,
}

pub fn build_family(family_id: i64, name: String) -> Family {
    Family { family_id, name }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Family_User {
    family_id: i64,
    user_id: i64,
    relationship: String,
}

pub fn build_family_user(family_id: i64, user_id: i64, relationship: String) -> Family_User {
    Family_User {
        family_id,
        user_id,
        relationship,
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Family_Member {
    id: i64,
    user_id: i64,
    notification_methods: String,
    email: String,
    phone: String,
}

pub fn build_family_member(
    id: i64,
    user_id: i64,
    notification_methods: String,
    email: String,
    phone: String,
) -> Family_Member {
    Family_Member {
        id,
        user_id,
        notification_methods,
        email,
        phone,
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Assignment {
    id: i64,
    class_id: i64,
    required: bool,
    grade_scale: String,
    description: String,
    template: String,
}

pub fn build_assignment(
    id: i64,
    class_id: i64,
    required: bool,
    grade_scale: String,
    description: String,
    template: String,
) -> Assignment {
    Assignment {
        id,
        class_id,
        required,
        grade_scale,
        description,
        template,
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Submission {
    id: i64,
    user_id: i64,
    contents: String,
    grade: String,
}

pub fn build_submission(id: i64, user_id: i64, contents: String, grade: String) -> Submission {
    Submission {
        id,
        user_id,
        contents,
        grade,
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Comment {
    id: i64,
    user_id: i64,
    assignment_id: i64,
    contents: String,
}

pub fn build_comment(id: i64, user_id: i64, assignment_id: i64, contents: String) -> Comment {
    Comment {
        id,
        user_id,
        assignment_id,
        contents,
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Change_Log_Item {
    id: i64,
    source_name: String,
    change_type: i32,
    old_value: String,
}

pub fn build_change_log_item(
    id: i64,
    source_name: String,
    change_type: i32,
    old_value: String,
) -> Change_Log_Item {
    Change_Log_Item {
        id,
        source_name,
        change_type,
        old_value,
    }
}
