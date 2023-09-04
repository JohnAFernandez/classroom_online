use crate::db_insert::I;
use crate::db_retrieve::R;
use crate::db_types as types;
use crate::db_verify::V;

use sqlite;

pub async fn family_to_row(
    connection: &sqlite::Connection,
    family: types::Family,
) -> (bool, String) {
    if family.name().await.is_empty() {
        return (
            false,
            "Cannot add family without a family name.".to_string(),
        );
    }

    I::insert_family(connection, &family.name().await).await;

    (true, "Success!".to_string())
}

pub async fn school_to_row(
    connection: &sqlite::Connection,
    school: types::School,
) -> (bool, String) {
    if !V::check_id(connection, school.organization_id().await, V::ORGANIZATIONS).await {
        return (
            false,
            "School record cannot be added because the organization id of ".to_string()
                + &school.organization_id().await.to_string()
                + " is not in the organizations table.",
        );
    }

    if !V::check_name(&school.name().await).await {
        return (
            false,
            "School record cannot be added because schools need a name!".to_string(),
        );
    }

    (true, "Success!".to_string())
}

pub async fn organization_to_row(
    connection: &sqlite::Connection,
    organization: types::Organization,
) -> (bool, String) {
    if !V::check_org_school_name(&organization.name().await).await {
        return (
            false,
            "Adding an organization requires an organization name.".to_string(),
        );
    }

    // TODO, add address checks

    // TODO, add phone number check

    I::insert_organization(
        connection,
        &organization.name().await,
        &organization.address1().await,
        &organization.address2().await,
        &organization.city().await,
        &organization.state().await,
        &organization.zip().await,
        &organization.phone().await,
        &organization.country().await,
    ).await;

    return (true, "Success!".to_string());
}

pub async fn users_to_row(connection: &sqlite::Connection, user: types::User) -> (bool, String) {
    if !V::check_birthday(&user.birthday().await).await {
        return (
            false,
            "Birthday is not valid. A valid birthday must be submitted.".to_string(),
        );
    }

    if !V::check_email(user.email().await).await {
        return (
            false,
            "Email was invalid.  Create an account without an email, or provide a valid email."
                .to_string(),
        );
    }

    if user.first_name().await.is_empty() {
        return (
            false,
            "A first name must be specified for a new user.".to_string(),
        );
    }

    if user.last_name().await.is_empty() {
        return (
            false,
            "A last name must be specified for a new user.".to_string(),
        );
    }

    if user.password().await.is_empty() {
        return (false, "New users must have a password.  If using API, a randomly generated initial password is acceptable.".to_string());
    }
    // TODO Add phone number check

    let mut log: String = "User insertion log:\n".to_string();

    if !user.username().await.is_empty() {
        log = log + "Username is automatically generated, ignorning supplied username.\n";
    }

    if user.deleted().await {
        log = log + "You cannot specify a new account as deleted, ignoring.\n";
    }

    if user.hidden().await {
        log = log + "You cannot specify a new account as hidden, ignoring.\n";
    }

    I::insert_user(
        connection,
        &user.email().await,
        &user.password().await,
        &user.first_name().await,
        &user.last_name().await,
        &user.birthday().await,
        &user.date_registered().await,
        &user.phone().await,
        &user.icon().await,
    ).await;

    (true, log)
}

pub async fn class_to_row(connection: &sqlite::Connection, class: types::Class) -> (bool, String) {
    if !V::check_id(connection, class.school_id().await, V::SCHOOLS).await {
        return (
            false,
            "Not able to add class record, since school id ".to_string()
                + &class.school_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    if !V::check_id(connection, class.subject_id().await, V::CLASSES).await {
        return (
            false,
            "Not able to add class record, since subject id ".to_string()
                + &class.subject_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    // TODO, add more checks, like normal.  Time and days are probably next.

    I::insert_class(
        connection,
        &class.school_id().await.to_string(),
        &class.subject_id().await.to_string(),
        &class.year().await.to_string(),
        &class.start_day().await,
        &class.end_day().await,
        &class.start_time().await.to_string(),
        &class.end_time().await.to_string(),
        &class.days_scheduled().await,
    ).await;
    (true, "".to_string())
}

pub async fn administrator_to_row(
    connection: &sqlite::Connection,
    administrator: types::Administrator,
) -> (bool, String) {
    if !V::check_id(connection, administrator.user_id().await, V::ADMINISTRATORS).await {
        return (
            false,
            "Not able to add administrator record, since user id ".to_string()
                + &administrator.user_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    I::insert_administrator(
        connection,
        &administrator.user_id().await.to_string(),
        &administrator.level().await,
    ).await;
    (true, "".to_string())
}

pub async fn student_to_row(
    connection: &sqlite::Connection,
    student: types::Student,
) -> (bool, String) {
    if !V::check_id(connection, student.user_id().await, V::USERS).await {
        return (
            false,
            "Not able to add student record, since user id ".to_string()
                + &student.user_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    I::insert_student(connection, &student.user_id().await.to_string()).await;
    (true, "".to_string())
}

pub async fn teacher_to_row(
    connection: &sqlite::Connection,
    teacher: types::Teacher,
) -> (bool, String) {
    if !V::check_id(connection, teacher.user_id().await, V::USERS).await {
        return (
            false,
            "Not able to add teacher record, since user id ".to_string()
                + &teacher.user_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    I::insert_teacher(connection, &teacher.user_id().await.to_string()).await;
    (true, "".to_string())
}

pub async fn family_member_to_row(
    connection: &sqlite::Connection,
    mut family_member: types::FamilyMember,
) -> (bool, String) {
    if !V::check_id(connection, family_member.user_id().await, V::USERS).await {
        return (
            false,
            "Not able to add family member record, since user id ".to_string()
                + &family_member.user_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    let mut log: String = "".to_owned();

    if !family_member.email().await.is_empty() && !V::check_email(family_member.email().await).await {
        log = log
            + "Email address "
            + &family_member.email().await
            + " was rejected because it is invalid.\n";
        family_member.set_email("".to_string()).await;
    }

    // TODO Add a check phone number here

    if family_member.email().await.is_empty() && family_member.phone().await.is_empty() {
        log = log
            + "Because there is no valid contact method, we cannot add this family member record.";
        return (false, log);
    }

    I::insert_family_member(
        connection,
        &family_member.user_id().await.to_string(),
        &family_member.notification_methods().await,
        &family_member.email().await,
        &family_member.phone().await,
    ).await;

    (true, log)
}

pub async fn assignment_to_row(
    connection: &sqlite::Connection,
    assignment: types::Assignment,
) -> (bool, String) {
    if !V::check_id(connection, assignment.class_id().await, V::CLASSES).await {
        return (
            false,
            "Not able to add assignment record, since class id ".to_string()
                + &assignment.class_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    if assignment.name().await.is_empty() {
        return (
            false,
            "Assignment requires a name before it can be added to the assignment record."
                .to_string(),
        );
    }

    if assignment.grade_scale().await.is_empty() {
        return (
            false,
            "Assignment requires a grade scale before it can be added to the assignment record."
                .to_string(),
        );
    }

    I::insert_assignment(
        connection,
        &assignment.class_id().await.to_string(),
        if assignment.required().await {
            "1".to_string()
        } else {
            "0".to_string()
        },
        &assignment.grade_scale().await.to_string(),
        &assignment.name().await,
        &assignment.description().await,
        &assignment.template().await,
    ).await;

    return (true, "".to_string());
}

pub async fn submission_to_row(
    connection: &sqlite::Connection,
    submission: types::Submission,
) -> (bool, String) {
    if !V::check_id(connection, submission.user_id().await, V::USERS).await {
        return (
            false,
            "Not able to add comment record, since user id ".to_string()
                + &submission.user_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    if !V::check_id(connection, submission.assignment_id().await, V::ASSIGNMENTS).await {
        return (
            false,
            "Not able to add comment record, since assignment id ".to_string()
                + &submission.assignment_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    if submission.contents().await.is_empty() {
        return (
            false,
            "Not able to add submission record, since the submission is empty.".to_string(),
        );
    }

    I::insert_submission(
        connection,
        &submission.user_id().await.to_string(),
        &submission.assignment_id().await.to_string(),
        &submission.contents().await,
        &submission.grade().await,
    ).await;

    (true, "".to_string())
}

pub async fn comment_to_row(
    connection: &sqlite::Connection,
    comment: types::Comment,
) -> (bool, String) {
    if !V::check_id(connection, comment.user_id().await, V::USERS).await {
        return (
            false,
            "Not able to add comment record, since user id ".to_string()
                + &comment.user_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    if !V::check_id(connection, comment.assignment_id().await, V::ASSIGNMENTS).await {
        return (
            false,
            "Not able to add comment record, since assignment id ".to_string()
                + &comment.assignment_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    if comment.contents().await.is_empty() {
        return (
            false,
            "Not able to add comment because the comment has no contents.".to_string(),
        );
    }

    I::insert_comments(
        connection,
        &comment.user_id().await.to_string(),
        &comment.assignment_id().await.to_string(),
        &comment.contents().await,
    ).await;
    return (true, "".to_string());
}

pub async fn user_change_log_to_row(
    connection: &sqlite::Connection,
    log_item: types::ChangeLogItem,
) {
    I::insert_change_log(
        connection,
        &log_item.source_name().await,
        &log_item.change_type().await.to_string(),
        &log_item.old_value().await.to_string(),
        &log_item.timestamp().await,
    ).await
}

pub async fn subject_to_row(connection: &sqlite::Connection, subject: types::Subject) {
    // TODO create some tables and checks for this object type.

    I::insert_subject(
        connection,
        subject.name().await,
        if subject.ap().await {
            "1".to_string()
        } else {
            "0".to_string()
        },
        if subject.ib().await {
            "1".to_string()
        } else {
            "0".to_string()
        },
        subject.target_year().await,
        subject.discipline().await,
    ).await;
}

pub async fn employee_supervisor_to_row(
    connection: &sqlite::Connection,
    mut employee_supervisor: types::EmployeeSupervisor,
) -> (bool, String) {
    if !V::check_id(connection, employee_supervisor.user_id().await, V::USERS).await {
        return (
            false,
            "Not able to add employee_supervisor record, since user id ".to_string()
                + &employee_supervisor.user_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    if !V::check_id(
        connection,
        employee_supervisor.organization_id().await,
        V::ORGANIZATIONS,
    ).await {
        return (
            false,
            "Not able to add employee_supervisor record, since organization id ".to_string()
                + &employee_supervisor.user_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    let found_super_id = V::check_id(
        connection,
        employee_supervisor.administrator_id().await,
        V::ADMINISTRATORS,
    ).await;

    if !found_super_id && employee_supervisor.supervisor_name().await.is_empty() {
        return (
            false,
            "Not able to add employee_supervisor record, since administrator id ".to_string()
                + &employee_supervisor.user_id().await.to_string()
                + " does not have a corresponding record and the supervisor name field is empty.",
        );
    }

    let mut log: String = "employee_supervisor_to_row log:\n".to_string();

    if found_super_id && employee_supervisor.supervisor_name().await.is_empty() {
        // need to add some functionality here that attempts to populate the name friom the database
        match R::retrieve_user_from_administrator(
            connection,
            employee_supervisor.administrator_id().await,
        ).await {
            Ok(x) => match R::retrieve_details(connection, R::USERS, x.to_string()).await {
                Ok(x) => {
                    for row in x.into_iter().map(|row| row.unwrap()) {
                        let mut name: String = "".to_owned();
                        name += row.read::<&str, _>("first_name");

                        if !name.is_empty() {
                            name += " ";
                        }

                        name += row.read::<&str, _>("last_name");

                        if !name.is_empty() {
                            log = log + "Successfully found supervisor name \"" + &name + "\"";
                            employee_supervisor.set_supervisor_name(name).await;
                        }

                        break;
                    }
                }
                Err(x) => log = log
                    + "Tried to add supervisor name to employee_supervisor table, but we ran into "
                    + &x.to_string()
                    + ".\n",
            },
            Err(x) => {
                log = log
                    + "Tried to add supervisor name to employee_supervisor table, but we ran into "
                    + &x.to_string()
                    + ".\n"
            }
        }
    }

    I::insert_employee_supervisor(
        connection,
        &employee_supervisor.user_id().await.to_string(),
        &employee_supervisor.administrator_id().await.to_string(),
        &employee_supervisor.supervisor_name().await.to_string(),
        &employee_supervisor.organization_id().await.to_string(),
    ).await;

    return (true, log);
}

pub async fn family_user_to_row(
    connection: &sqlite::Connection,
    family_user: types::FamilyUser,
) -> (bool, String) {
    if !V::check_id(connection, family_user.family_id().await, V::FAMILIES).await {
        return (
            false,
            "Not able to add family-user record, since family id ".to_string()
                + &family_user.family_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    if !V::check_id(connection, family_user.user_id().await, V::USERS).await {
        return (
            false,
            "Not able to add family-user record, since user id ".to_string()
                + &family_user.user_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    if V::check_id_pair(
        connection,
        family_user.family_id().await,
        family_user.user_id().await,
        V::FAMILIES_USERS,
    ).await {
        return (false, "Not able to add family-user record, since that relationship already exists in the table.".to_string());
    }

    I::insert_families_users(
        connection,
        &family_user.family_id().await.to_string(),
        &family_user.user_id().await.to_string(),
        &family_user.relationship().await,
    ).await;

    return (true, "".to_string());
}

pub async fn student_class_to_row(
    connection: &sqlite::Connection,
    student_class: types::StudentClass,
) -> (bool, String) {
    if !V::check_id(connection, student_class.student_id().await, V::STUDENTS).await {
        return (
            false,
            "Not able to add student-class record, since student id ".to_string()
                + &student_class.student_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    if !V::check_id(connection, student_class.class_id().await, V::CLASSES).await {
        return (
            false,
            "Not able to add student-class record, since class id ".to_string()
                + &student_class.class_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    if V::check_id_pair(
        connection,
        student_class.student_id().await,
        student_class.class_id().await,
        V::STUDENTS_CLASSES,
    ).await {
        return (false, "Not able to add student-class record, since that relationship already exists in the table.".to_string());
    }

    I::insert_students_classes(
        connection,
        &student_class.student_id().await.to_string(),
        &student_class.class_id().await.to_string(),
    ).await;

    return (true, "".to_string());
}

pub async fn teacher_class_to_row(
    connection: &sqlite::Connection,
    teacher_class: types::TeacherClass,
) -> (bool, String) {
    if !V::check_id(connection, teacher_class.teacher_id().await, V::TEACHERS).await {
        return (
            false,
            "Not able to add teacher-class record, since teacher id ".to_string()
                + &teacher_class.teacher_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    if !V::check_id(connection, teacher_class.class_id().await, V::CLASSES).await {
        return (
            false,
            "Not able to add teacher-class record, since class id ".to_string()
                + &teacher_class.class_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    if V::check_id_pair(
        connection,
        teacher_class.teacher_id().await,
        teacher_class.class_id().await,
        V::TEACHERS_SCHOOLS,
    ).await {
        return (false, "Not able to add teacher-class record, since that relationship already exists in the table.".to_string());
    }

    I::insert_teachers_classes(
        connection,
        &teacher_class.teacher_id().await.to_string(),
        &teacher_class.class_id().await.to_string(),
        &teacher_class.role().await,
    ).await;

    return (true, "".to_string());
}

pub async fn teacher_school_to_row(
    connection: &sqlite::Connection,
    teacher_school: types::TeacherSchool,
) -> (bool, String) {
    if !V::check_id(connection, teacher_school.teacher_id().await, V::TEACHERS).await {
        return (
            false,
            "Not able to add teacher-school record, since teacher id ".to_string()
                + &teacher_school.teacher_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    if !V::check_id(connection, teacher_school.school_id().await, V::SCHOOLS).await {
        return (
            false,
            "Not able to add teacher-school record, since school id ".to_string()
                + &teacher_school.school_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    if V::check_id_pair(
        connection,
        teacher_school.teacher_id().await,
        teacher_school.school_id().await,
        V::TEACHERS_SCHOOLS,
    ).await {
        return (false, "Not able to add teacher-school record, since that relationship already exists in the table.".to_string());
    }

    I::insert_teachers_schools(
        connection,
        &teacher_school.teacher_id().await.to_string(),
        &teacher_school.school_id().await.to_string(),
    ).await;

    return (true, "".to_string());
}

pub async fn administrator_school_to_row(
    connection: &sqlite::Connection,
    admin_school: types::AdministratorSchool,
) -> (bool, String) {
    if !V::check_id(connection, admin_school.admin_id().await, V::ADMINISTRATORS).await {
        return (
            false,
            "Not able to add administrator-school record, since admin id ".to_string()
                + &admin_school.admin_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    if !V::check_id(connection, admin_school.school_id().await, V::SCHOOLS).await {
        return (
            false,
            "Not able to add administrator-school record, since school id ".to_string()
                + &admin_school.admin_id().await.to_string()
                + " does not have a corresponding record.",
        );
    }

    if V::check_id_pair(
        connection,
        admin_school.admin_id().await,
        admin_school.school_id().await,
        V::ADMINISTRATORS_SCHOOLS,
    ).await {
        return (false, "Not able to add administrator-school record, since that relationship already exists in the table.".to_string());
    }

    I::insert_administrator_school(
        connection,
        &admin_school.admin_id().await.to_string(),
        &admin_school.school_id().await.to_string(),
    ).await;

    return (true, "".to_string());
}
