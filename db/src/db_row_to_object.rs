use crate::db_types as types;
use sqlite;

pub fn row_to_user(row: &sqlite::Row) -> types::User {
    types::build_user(
        row.read::<i64, _>("user_id"),
        row.read::<&str, _>("email").to_string(),
        row.read::<&str, _>("username").to_string(),
        "".to_string(),
        row.read::<&str, _>("first_name").to_string(),
        row.read::<&str, _>("last_name").to_string(),
        row.read::<&str, _>("birthday").to_string(),
        row.read::<&str, _>("date_registered").to_string(),
        row.read::<&str, _>("phone").to_string(),
        row.read::<&str, _>("icon").to_string(),
        if row.read::<i64, _>("hidden") == 1 {
            true
        } else {
            false
        },
        if row.read::<i64, _>("deleted") == 11 {
            true
        } else {
            false
        },
    )
}

pub fn row_to_organization(row: &sqlite::Row) -> types::Organization {
    types::build_organization(
        row.read::<i64, _>("organization_id"),
        row.read::<&str, _>("name").to_string(),
        row.read::<&str, _>("address1").to_string(),
        row.read::<&str, _>("address2").to_string(),
        row.read::<&str, _>("city").to_string(),
        row.read::<&str, _>("state").to_string(),
        row.read::<&str, _>("zip").to_string(),
        row.read::<&str, _>("phone").to_string(),
        row.read::<&str, _>("country").to_string(),
    )
}

pub fn row_to_administrator(row: &sqlite::Row) -> types::Administrator {
    types::build_administrator(
        row.read::<i64, _>("administrator_id"),
        row.read::<i64, _>("user_id"),
        row.read::<&str, _>("level").to_string(),
    )
}

// not tested from here down....
pub fn row_to_school(row: &sqlite::Row) -> types::School {
    types::build_school(
        row.read::<i64, _>("school_id"),
        row.read::<i64, _>("organization_id"),
        row.read::<i64, _>("super_administrator_id"),
        row.read::<&str, _>("icon").to_string(),
        row.read::<&str, _>("name").to_string(),
        row.read::<&str, _>("address1").to_string(),
        row.read::<&str, _>("address2").to_string(),
        row.read::<&str, _>("city").to_string(),
        row.read::<&str, _>("state").to_string(),
        row.read::<&str, _>("zip").to_string(),
        row.read::<&str, _>("phone").to_string(),
        row.read::<&str, _>("country").to_string(),
    )
}

pub fn row_to_AdministratorSchool(row: &sqlite::Row) -> types::AdministratorSchool {
    types::build_administrator_school(
        row.read::<i64, _>("admin_id"),
        row.read::<i64, _>("school_id"),
    )
}

pub fn row_to_teacher(row: &sqlite::Row) -> types::Teacher {
    types::build_teacher(
        row.read::<i64, _>("teacher_id"),
        row.read::<i64, _>("user_id"),
    )
}

pub fn row_to_employee_supervisor(row: &sqlite::Row) -> types::EmployeeSupervisor {
    types::build_employee_supervisor(
        row.read::<i64, _>("supervisory_id"),
        row.read::<i64, _>("user_id"),
        row.read::<i64, _>("administrator_id"),
        row.read::<&str, _>("supervisor_name").to_string(),
    )
}

pub fn row_to_teacher_school(row: &sqlite::Row) -> types::TeacherSchool {
    types::build_teacher_school(
        row.read::<i64, _>("teacher_id"),
        row.read::<i64, _>("school_id"),
    )
}

pub fn row_to_subject(row: &sqlite::Row) -> types::Subject {
    types::build_subject(
        row.read::<i64, _>("subject_id"),
        row.read::<&str, _>("name").to_string(),
        if row.read::<i64, _>("ap") == 1 {
            true
        } else {
            false
        },
        if row.read::<i64, _>("ib") == 1 {
            true
        } else {
            false
        },
        row.read::<&str, _>("target_year").to_string(),
        row.read::<&str, _>("discipline").to_string(),
    )
}

pub fn row_to_class(row: &sqlite::Row) -> types::Class {
    types::build_class(
        row.read::<i64, _>("class_id"),
        row.read::<i64, _>("school_id"),
        row.read::<i64, _>("subject_id"),
        row.read::<&str, _>("year").to_string(),
        row.read::<&str, _>("start_day").to_string(),
        row.read::<&str, _>("end_day").to_string(),
        row.read::<i64, _>("start_time") as i32,
        row.read::<i64, _>("end_time") as i32,
        row.read::<&str, _>("days_scheduled").to_string(),
    )
}

pub fn row_to_student_class(row: &sqlite::Row) -> types::StudentClass {
    types::build_student_class(
        row.read::<i64, _>("student_id"),
        row.read::<i64, _>("class_id"),
    )
}

pub fn row_to_family(row: &sqlite::Row) -> types::Family {
    types::build_family(
        row.read::<i64, _>("family_id"),
        row.read::<&str, _>("name").to_string(),
    )
}

pub fn row_to_family_user(row: &sqlite::Row) -> types::FamilyUser {
    types::build_family_user(
        row.read::<i64, _>("family_id"),
        row.read::<i64, _>("user_id"),
        row.read::<&str, _>("relationship").to_string(),
    )
}

pub fn row_to_family_member(row: &sqlite::Row) -> types::FamilyMember {
    types::build_family_member(
        row.read::<i64, _>("member_id"),
        row.read::<i64, _>("user_id"),
        row.read::<&str, _>("notification_methods").to_string(),
        row.read::<&str, _>("email").to_string(),
        row.read::<&str, _>("phone").to_string(),
    )
}

pub fn row_to_assignment(row: &sqlite::Row) -> types::Assignment {
    types::build_assignment(
        row.read::<i64, _>("assignment_id"),
        row.read::<i64, _>("class_id"),
        if row.read::<i64, _>("required") == 1 {
            true
        } else {
            false
        },
        row.read::<&str, _>("grade_scale").to_string(),
        row.read::<&str, _>("description").to_string(),
        row.read::<&str, _>("template").to_string(),
    )
}

pub fn row_to_submission(row: &sqlite::Row) -> types::Submission {
    types::build_submission(
        row.read::<i64, _>("submission_id"),
        row.read::<i64, _>("user_id"),
        row.read::<&str, _>("contents").to_string(),
        row.read::<&str, _>("grade").to_string(),
    )
}

pub fn row_to_comment(row: &sqlite::Row) -> types::Comment {
    types::build_comment(
        row.read::<i64, _>("comment_id"),
        row.read::<i64, _>("user_id"),
        row.read::<i64, _>("assignment_id"),
        row.read::<&str, _>("contents").to_string(),
    )
}

pub fn row_to_log_item(row: &sqlite::Row) -> types::ChangeLogItem {
    types::build_change_log_item(
        row.read::<i64, _>("change_id"),
        row.read::<&str, _>("source_name").to_string(),
        row.read::<i64, _>("type") as i32,
        row.read::<&str, _>("old_value").to_string(),
    )
}
