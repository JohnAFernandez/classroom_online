use crate::db_insert::I;
use crate::db_types as types;
use crate::db_verify::V;
use sqlite;

pub fn employee_supervisor_to_row(connection: &sqlite::Connection, employee_supervisor: types::EmployeeSupervisor) -> (bool, String) {
    if !V::check_id(connection, employee_supervisor.user_id(), V::USERS) {
        return (false, "Not able to add family-user record, since family id ".to_string() + &employee_supervisor.user_id().to_string() + " does not have a corresponding record." )
    }

    if !V::check_id(connection, employee_supervisor.administrator_id(), V::ADMINISTRATORS) {
    
        if employee_supervisor.supervisor_name().is_empty() {
            return (false, "Not able to add family-user record, since user id ".to_string() + &employee_supervisor.administrator_id().to_string() + " does not have a corresponding record." );
        }
    } else if employee_supervisor.supervisor_name().is_empty() {
        
    }

    if 

    if V::check_id_pair(connection, employee_supervisor.family_id(), employee_supervisor.user_id(), V::FAMILIES_USERS){
        return (false, "Not able to add family-user record, since that relationship already exists in the table.".to_string())
    }

    I::insert_employee_supervisor(connection, &employee_supervisor.user_id().to_string(), &employee_supervisor.administrator_id().to_string(), &employee_supervisor.supervisor_name().to_string(), employee_supervisor.);

    return(true, "".to_string());

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
