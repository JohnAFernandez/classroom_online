use crate::db_insert::I;
use crate::db_retrieve::R;
use crate::db_verify::V;
use crate::db_types as types;

use sqlite;

pub fn submission_to_row(connection: &sqlite::Connection, submission: types::Submission) -> (bool, String){
    if !V::check_id(connection, submission.user_id(), V::USERS) {
        return (false, "Not able to add comment record, since user id ".to_string() + &submission.user_id().to_string() + " does not have a corresponding record." )
    }

    if !V::check_id(connection, submission.assignment_id(), V::ASSIGNMENTS) {
        return (false, "Not able to add comment record, since assignment id ".to_string() + &submission.assignment_id().to_string() + " does not have a corresponding record." )
    }

    if submission.contents().is_empty(){
        return (false, "Not able to add submission record, since the submission is empty.".to_string())
    }

    I::insert_submission(connection, &submission.user_id().to_string(), &submission.assignment_id().to_string(), &submission.contents(), &submission.grade());

    (true, "".to_string())

}

pub fn comment_to_row(connection: &sqlite::Connection, comment: types::Comment) -> (bool, String) {
    if !V::check_id(connection, comment.user_id(), V::USERS) {
        return (false, "Not able to add comment record, since user id ".to_string() + &comment.user_id().to_string() + " does not have a corresponding record." )
    }

    if !V::check_id(connection, comment.assignment_id(), V::ASSIGNMENTS) {
        return (false, "Not able to add comment record, since assignment id ".to_string() + &comment.assignment_id().to_string() + " does not have a corresponding record." )
    }

    if comment.contents().is_empty() {
        return (false, "Not able to add comment because the comment has no contents.".to_string())        
    }

    I::insert_comments(connection, &comment.user_id().to_string(), &comment.assignment_id().to_string(), &comment.contents());
    return (true, "".to_string());
}


pub fn user_change_log_to_row(connection: &sqlite::Connection, log_item: types::ChangeLogItem){
    I::insert_change_log(connection, &log_item.source_name(), &log_item.change_type().to_string(), &log_item.old_value().to_string(), &log_item.timestamp())

}

pub fn subject_to_row(connection: &sqlite::Connection, subject: types::Subject){
    // TODO create some tables and checks for this object type.

    I::insert_subject(connection, subject.name(), if subject.ap() {"1".to_string()} else {"0".to_string()}, if subject.ib() {"1".to_string()} else {"0".to_string()}, subject.target_year(), subject.discipline());

}

pub fn employee_supervisor_to_row(connection: &sqlite::Connection, mut employee_supervisor: types::EmployeeSupervisor) -> (bool, String) {
    if !V::check_id(connection, employee_supervisor.user_id(), V::USERS) {
        return (false, "Not able to add employee_supervisor record, since user id ".to_string() + &employee_supervisor.user_id().to_string() + " does not have a corresponding record." )
    }

    if !V::check_id(connection, employee_supervisor.organization_id(), V::ORGANIZATIONS) {
        return (false, "Not able to add employee_supervisor record, since organization id ".to_string() + &employee_supervisor.user_id().to_string() + " does not have a corresponding record." )
    }

    let found_super_id = V::check_id(connection, employee_supervisor.administrator_id(), V::ADMINISTRATORS);

    if !found_super_id && employee_supervisor.supervisor_name().is_empty() {
        return (false, "Not able to add employee_supervisor record, since administrator id ".to_string() + &employee_supervisor.user_id().to_string() + " does not have a corresponding record and the supervisor name field is empty." )
    }

    let mut log : String = "employee_supervisor_to_row log:\n".to_string();

    if found_super_id && employee_supervisor.supervisor_name().is_empty() {
        // need to add some functionality here that attempts to populate the name friom the database
        match R::retrieve_user_from_administrator(connection, employee_supervisor.administrator_id()){

            Ok(x)=> { 
                match R::retrieve_details(connection, R::USERS, x.to_string()) {
                    Ok(x) => for row in x.into_iter().map(|row| row.unwrap()){
                        let mut name : String = "".to_owned();
                        name += row.read::<&str, _>("first_name");
                        
                        if !name.is_empty(){
                            name += " ";
                        }
                        
                        name += row.read::<&str, _>("last_name");

                        if !name.is_empty(){
                            log = log + "Successfully found supervisor name \"" + &name + "\"";
                            employee_supervisor.set_supervisor_name(name);
                        }

                        break;
                    },
                    Err(x) => log = log + "Tried to add supervisor name to employee_supervisor table, but we ran into " + &x.to_string() + ".\n",
                }
            
            },
            Err(x) => log = log + "Tried to add supervisor name to employee_supervisor table, but we ran into " + &x.to_string() + ".\n",
        }
    }

    I::insert_employee_supervisor(connection, &employee_supervisor.user_id().to_string(), &employee_supervisor.administrator_id().to_string(), &employee_supervisor.supervisor_name().to_string(), &employee_supervisor.organization_id().to_string());

    return(true, log);

}

pub fn family_user_to_row(connection: &sqlite::Connection, family_user: types::FamilyUser) -> (bool, String) {
    if !V::check_id(connection, family_user.family_id(), V::FAMILIES) {
        return (false, "Not able to add family-user record, since family id ".to_string() + &family_user.family_id().to_string() + " does not have a corresponding record." )
    }

    if !V::check_id(connection, family_user.user_id(), V::USERS) {
        return (false, "Not able to add family-user record, since user id ".to_string() + &family_user.user_id().to_string() + " does not have a corresponding record." )
    }

    if V::check_id_pair(connection, family_user.family_id(), family_user.user_id(), V::FAMILIES_USERS){
        return (false, "Not able to add family-user record, since that relationship already exists in the table.".to_string())
    }

    I::insert_families_users(connection, &family_user.family_id().to_string(), &family_user.user_id().to_string(), &family_user.relationship());

    return(true, "".to_string());

}

pub fn student_class_to_row(connection: &sqlite::Connection, student_class: types::StudentClass) -> (bool, String) {
    if !V::check_id(connection, student_class.student_id(), V::STUDENTS) {
        return (false, "Not able to add student-class record, since student id ".to_string() + &student_class.student_id().to_string() + " does not have a corresponding record." )
    }

    if !V::check_id(connection, student_class.class_id(), V::CLASSES) {
        return (false, "Not able to add student-class record, since class id ".to_string() + &student_class.class_id().to_string() + " does not have a corresponding record." )
    }

    if V::check_id_pair(connection, student_class.student_id(), student_class.class_id(), V::STUDENTS_CLASSES){
        return (false, "Not able to add student-class record, since that relationship already exists in the table.".to_string())
    }

    I::insert_students_classes(connection, &student_class.student_id().to_string(), &student_class.class_id().to_string());

    return(true, "".to_string());

}

pub fn teacher_class_to_row(connection: &sqlite::Connection, teacher_class: types::TeacherClass) -> (bool, String) {
    if !V::check_id(connection, teacher_class.teacher_id(), V::TEACHERS) {
        return (false, "Not able to add teacher-class record, since teacher id ".to_string() + &teacher_class.teacher_id().to_string() + " does not have a corresponding record." )
    }

    if !V::check_id(connection, teacher_class.class_id(), V::CLASSES) {
        return (false, "Not able to add teacher-class record, since class id ".to_string() + &teacher_class.class_id().to_string() + " does not have a corresponding record." )
    }

    if V::check_id_pair(connection, teacher_class.teacher_id(), teacher_class.class_id(), V::TEACHERS_SCHOOLS){
        return (false, "Not able to add teacher-class record, since that relationship already exists in the table.".to_string())
    }

    I::insert_teachers_classes(connection, &teacher_class.teacher_id().to_string(), &teacher_class.class_id().to_string(), &teacher_class.role());

    return(true, "".to_string());

}

pub fn teacher_school_to_row(connection: &sqlite::Connection, teacher_school: types::TeacherSchool) -> (bool, String) {
    if !V::check_id(connection, teacher_school.teacher_id(), V::TEACHERS) {
        return (false, "Not able to add teacher-school record, since teacher id ".to_string() + &teacher_school.teacher_id().to_string() + " does not have a corresponding record." )
    }

    if !V::check_id(connection, teacher_school.school_id(), V::SCHOOLS) {
        return (false, "Not able to add teacher-school record, since school id ".to_string() + &teacher_school.school_id().to_string() + " does not have a corresponding record." )
    }

    if V::check_id_pair(connection, teacher_school.teacher_id(), teacher_school.school_id(), V::TEACHERS_SCHOOLS){
        return (false, "Not able to add teacher-school record, since that relationship already exists in the table.".to_string())
    }

    I::insert_teachers_schools(connection, &teacher_school.teacher_id().to_string(), &teacher_school.school_id().to_string());

    return(true, "".to_string());

}

pub fn administrator_school_to_row(connection: &sqlite::Connection, admin_school: types::AdministratorSchool) -> (bool, String) {
    if !V::check_id(connection, admin_school.admin_id(), V::ADMINISTRATORS) {
        return (false, "Not able to add administrator-school record, since admin id ".to_string() + &admin_school.admin_id().to_string() + " does not have a corresponding record." )
    }

    if !V::check_id(connection, admin_school.school_id(), V::SCHOOLS) {
        return (false, "Not able to add administrator-school record, since school id ".to_string() + &admin_school.admin_id().to_string() + " does not have a corresponding record." )
    }

    if V::check_id_pair(connection, admin_school.admin_id(), admin_school.school_id(), V::ADMINISTRATORS_SCHOOLS){
        return (false, "Not able to add administrator-school record, since that relationship already exists in the table.".to_string())
    }

    I::insert_administrator_school(connection, &admin_school.admin_id().to_string(), &admin_school.school_id().to_string());

    return(true, "".to_string());
}
