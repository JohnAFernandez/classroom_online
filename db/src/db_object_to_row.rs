use crate::db_insert::I;
use crate::db_retrieve::R;
use crate::db_verify::V;
use crate::db_types as types;

use sqlite;

pub fn family_to_row(connection: &sqlite::Connection, family: types::Family) -> (bool, String){
    if family.name().is_empty() {
        return (false, "Cannot add family without a family name.".to_string())
    }

    I::insert_family(connection, &family.name());

    (true, "".to_string())
}


pub fn school_to_row(connection: &sqlite::Connection, school: types::School) -> (bool, String) {
    if !V::check_id(connection, school.organization_id(), V::SCHOOLS){
        return (false, "School record cannot be added because the organization id of ".to_string() + &school.organization_id().to_string() + " is not in the organizations table.")
    }
    
    if !V::check_name(&school.name()) {
        return (false, "School record cannot be added because schools need a name!".to_string())
    }

    (true, "".to_string())
}

pub fn organization_to_row(connection: &sqlite::Connection, organization: types::Organization) -> (bool, String) {
    if !V::check_org_school_name(&organization.name()){
        return(false, "Adding an organization requires an organization name.".to_string());
    }

    // TODO, add address checks

    // TODO, add phone number check

    I::insert_organization(connection, &organization.name(), &organization.address1(), &organization.address2(), &organization.city(), &organization.state(), &organization.zip(), &organization.phone(), &organization.country());

    return (true, "".to_string())
}

pub fn users_to_row(connection: &sqlite::Connection, user: types::User) -> (bool, String) {
    if !V::check_birthday(&user.birthday()) {
        return (false, "Birthday is not valid. A valid birthday must be submitted.".to_string())
    }

    if !V::check_email(user.email()){
        return (false, "Email was invalid.  Create an account without an email, or provide a valid email.".to_string());
    }

    if user.first_name().is_empty() {
        return (false, "A first name must be specified for a new user.".to_string());
    }

    if user.last_name().is_empty() {
        return (false, "A last name must be specified for a new user.".to_string())
    }

    if user.password().is_empty() {
        return (false, "New users must have a password.  If using API, a randomly generated initial password is acceptable.".to_string());
    }
    // TODO Add phone number check

    let mut log: String = "User insertion log:\n".to_string();

    if !user.username().is_empty() {
        log = log + "Username is automatically generated, ignorning supplied username.\n";
    }

    if user.deleted() {
        log = log + "You cannot specify a new account as deleted, ignoring.\n";
    }

    if user.hidden() {
        log = log + "You cannot specify a new account as hidden, ignoring.\n";
    }

    I::insert_user(connection, &user.email(), &user.password(), &user.first_name(), &user.last_name(), &user.birthday(), &user.date_registered(), &user.phone(), &user.icon());

    (true, log)
}


pub fn class_to_row(connection: &sqlite::Connection, class: types::Class) -> (bool, String){
    if !V::check_id(connection, class.school_id(), V::SCHOOLS){
        return (false, "Not able to add class record, since school id ".to_string() + &class.school_id().to_string() + " does not have a corresponding record." );
    }

    if !V::check_id(connection, class.subject_id(), V::CLASSES){
        return (false, "Not able to add class record, since subject id ".to_string() + &class.subject_id().to_string() + " does not have a corresponding record." );
    }

    // TODO, add more checks, like normal.  Time and days are probably next.

    I::insert_class(connection, &class.school_id().to_string(), &class.subject_id().to_string(), &class.year().to_string(), &class.start_day(), &class.end_day(), &class.start_time().to_string(), &class.end_time().to_string(), &class.days_scheduled());
    (true, "".to_string())
}

pub fn administrator_to_row(connection: &sqlite::Connection, administrator: types::Administrator) -> (bool, String){
    if !V::check_id(connection, administrator.user_id(), V::ADMINISTRATORS){
        return (false, "Not able to add administrator record, since user id ".to_string() + &administrator.user_id().to_string() + " does not have a corresponding record." );
    }

    I::insert_administrator(connection, &administrator.user_id().to_string(), &administrator.level());
    (true, "".to_string())
}

pub fn student_to_row(connection: &sqlite::Connection, student: types::Student) -> (bool, String){
    if !V::check_id(connection, student.user_id(), V::USERS){
        return (false, "Not able to add student record, since user id ".to_string() + &student.user_id().to_string() + " does not have a corresponding record." );
    }

    I::insert_student(connection, &student.user_id().to_string());
    (true, "".to_string())
}

pub fn teacher_to_row(connection: &sqlite::Connection, teacher: types::Teacher) -> (bool, String){
    if !V::check_id(connection, teacher.user_id(), V::TEACHERS) {
        return (false, "Not able to add teacher record, since user id ".to_string() + &teacher.user_id().to_string() + " does not have a corresponding record." );
    }

    I::insert_teacher(connection, &teacher.user_id().to_string());
    (true, "".to_string())
}

pub fn family_member_to_row(connection: &sqlite::Connection, mut family_member: types::FamilyMember) -> (bool, String){

    if !V::check_id(connection, family_member.user_id(), V::USERS){
        return (false, "Not able to add family member record, since user id ".to_string() + &family_member.user_id().to_string() + " does not have a corresponding record." )
    }

    let mut log : String = "".to_owned();

    if !family_member.email().is_empty() && !V::check_email(family_member.email()){
        log = log + "Email address " + &family_member.email() + " was rejected because it is invalid.\n";
        family_member.set_email("".to_string());
    }

    // TODO Add a check phone number here

    if family_member.email().is_empty() && family_member.phone().is_empty(){
        log = log + "Because there is no valid contact method, we cannot add this family member record.";
        return (false, log);
    }

    I::insert_family_member(connection, &family_member.user_id().to_string(), &family_member.notification_methods(), &family_member.email(), &family_member.phone());

    (true, log)
}

pub fn assignment_to_row(connection: &sqlite::Connection, assignment: types::Assignment) -> (bool, String){
    if !V::check_id(connection, assignment.class_id(), V::CLASSES){
        return (false, "Not able to add assignment record, since class id ".to_string() + &assignment.class_id().to_string() + " does not have a corresponding record." )
    }

    if assignment.name().is_empty(){
        return (false, "Assignment requires a name before it can be added to the assignment record.".to_string())
    }

    if assignment.grade_scale().is_empty(){
        return (false, "Assignment requires a grade scale before it can be added to the assignment record.".to_string())
    }

    I::insert_assignment(connection, &assignment.class_id().to_string(), if assignment.required() {"1".to_string()} else {"0".to_string()}, &assignment.grade_scale().to_string(), &assignment.name(), &assignment.description(), &assignment.template());

    return (true, "".to_string());

}

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
