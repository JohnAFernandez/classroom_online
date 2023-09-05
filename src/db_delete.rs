use crate::db_retrieve::R;
use crate::db_verify::V;
use sqlite;

pub struct D {}

impl D {
    // ok, this one is kind of a red-herring.  Users are too fundamental to delete safely, and it would
    // delete grade records, so just mark a "deleted" flag, meaning that the user has cut themselves from access.
    // But the grades and other records should remain.
    pub async fn delete_user(connection: &sqlite::Connection, user_id: i64) -> (bool, String) {
        // if so, this user didn't really exist in the first place, go figure.
        if !V::check_id(connection, user_id, V::USERS).await {
            return (
                false,
                "User ID ".to_owned()
                    + &user_id.to_string()
                    + &" does not exist in the users table.",
            );
        }

        let query =
            "UPDATE users SET deleted = 1 WHERE user_id = ".to_owned() + &user_id.to_string();

        match connection.execute(query) {
            Ok(_) => return (true, "Success".to_owned()),
            Err(x) => {
                return (
                    false,
                    "User ID ".to_owned()
                        + &user_id.to_string()
                        + &" exists in users table, but deletion returned error of "
                        + &x.to_string(),
                )
            }
        }
    }

    pub async fn delete_administrator(
        connection: &sqlite::Connection,
        admin_id: i64,
    ) -> (bool, String) {
        // if so, this user didn't really exist in the first place, go figure.
        if !V::check_id(connection, admin_id, V::ADMINISTRATORS).await {
            return (
                false,
                "Admin ID ".to_owned()
                    + &admin_id.to_string()
                    + " does not exist in administrator table.",
            );
        }
        let query = "DELETE FROM administrators WHERE administrator_id = ".to_owned()
            + &admin_id.to_string();

        match connection.execute(query) {
            Ok(_) => {
                return (
                    true,
                    "Admin record ".to_owned() + &admin_id.to_string() + " successfully deleted.",
                )
            }
            Err(x) => {
                return (
                    false,
                    "Failed to deleted Admin record of ID ".to_owned()
                        + &admin_id.to_string()
                        + &" but deletion returned error of "
                        + &x.to_string(),
                )
            }
        }
    }

    pub async fn delete_school(connection: &sqlite::Connection, school_id: i64) -> (bool, String) {
        // if so, this user didn't really exist in the first place, go figure.
        if !V::check_id(connection, school_id, V::SCHOOLS).await {
            return (
                false,
                "School ID ".to_owned()
                    + &school_id.to_string()
                    + " does not exist in schools table.",
            );
        }

        let result: Result<sqlite::Statement<'_>, sqlite::Error> =
            R::retrieve_classes_from_school(connection, school_id).await;

        // only delete a school if there are no associated classes
        match result {
            Err(x) => {
                return (
                    false,
                    "Retrieving classes for School ID ".to_owned()
                        + &school_id.to_string()
                        + " encountered an error of "
                        + &x.to_string(),
                )
            }
            Ok(_) => (),
        }

        match result.into_iter().size_hint().1 {
            None => {
                return (
                    false,
                    "Could not delete School ID ".to_owned()
                        + &school_id.to_string()
                        + " because there are still classes associated with this school.",
                )
            }
            Some(_) => (),
        }

        let query = "DELETE FROM schools WHERE school_id = ".to_owned() + &school_id.to_string();

        match connection.execute(query) {
            Ok(_) => {
                return (
                    true,
                    "School record ".to_owned() + &school_id.to_string() + " successfully deleted.",
                )
            }
            Err(x) => {
                return (
                    false,
                    "Failed to deleted School record of ID ".to_owned()
                        + &school_id.to_string()
                        + &" but deletion returned error of "
                        + &x.to_string(),
                )
            }
        }
    }

    pub async fn delete_organization(
        connection: &sqlite::Connection,
        organization_id: i64,
    ) -> (bool, String) {
        // if so, this organization didn't really exist in the first place, go figure.
        if !V::check_id(connection, organization_id, V::ORGANIZATIONS).await {
            return (
                false,
                "Organization ID ".to_owned()
                    + &organization_id.to_string()
                    + " does not exist in organizations table.",
            );
        }

        let result: Result<sqlite::Statement<'_>, sqlite::Error> =
            R::retrieve_schools_from_organization(connection, organization_id).await;

        // only delete an organization if there are no associated schools
        match result {
            Err(x) => {
                return (
                    false,
                    "Retrieving schools for Organization ID ".to_owned()
                        + &organization_id.to_string()
                        + " encountered an error of "
                        + &x.to_string(),
                )
            }
            Ok(_) => (),
        }

        match result.into_iter().size_hint().1 {
            None => {
                return (
                    false,
                    "Could not delete Organization ID ".to_owned()
                        + &organization_id.to_string()
                        + " because there are still schools associated with this organization.",
                )
            }
            Some(_) => (),
        }

        let query = "DELETE FROM organizations WHERE organization_id = ".to_owned()
            + &organization_id.to_string();

        match connection.execute(query) {
            Ok(_) => {
                return (
                    true,
                    "Organization record ".to_owned()
                        + &organization_id.to_string()
                        + " successfully deleted.",
                )
            }
            Err(x) => {
                return (
                    false,
                    "Failed to deleted Organization record of Id ".to_owned()
                        + &organization_id.to_string()
                        + &"but deletion returned error of "
                        + &x.to_string(),
                )
            }
        }
    }

    pub async fn delete_teacher(
        connection: &sqlite::Connection,
        teacher_id: i64,
    ) -> (bool, String) {
        // if so, this user didn't really exist in the first place, go figure.
        if !V::check_id(connection, teacher_id, V::TEACHERS).await {
            return (
                false,
                "Teacher ID ".to_owned()
                    + &teacher_id.to_string()
                    + &" does not exist in the users table.",
            );
        }

        let query = "DELETE FROM teachers WHERE teacher_id = ".to_owned() + &teacher_id.to_string();

        match connection.execute(query) {
            Ok(_) => return (true, "Success".to_owned()),
            Err(x) => {
                return (
                    false,
                    "Techer ID ".to_owned()
                        + &teacher_id.to_string()
                        + &" exists in teacher table, but deletion returned error of "
                        + &x.to_string(),
                )
            }
        }
    }

    pub async fn delete_subject(
        connection: &sqlite::Connection,
        subject_id: i64,
    ) -> (bool, String) {
        // if so, this user didn't really exist in the first place, go figure.
        if !V::check_id(connection, subject_id, V::SUBJECTS).await {
            return (
                false,
                "Subject ID ".to_owned()
                    + &subject_id.to_string()
                    + &" does not exist in the subjects table.",
            );
        }

        let query = "DELETE FROM subjects WHERE subject_id = ".to_owned() + &subject_id.to_string();

        match connection.execute(query) {
            Ok(_) => return (true, "Success".to_owned()),
            Err(x) => {
                return (
                    false,
                    "Subject ID ".to_owned()
                        + &subject_id.to_string()
                        + &" exists in subjects table, but deletion returned error of "
                        + &x.to_string(),
                )
            }
        }
    }

    pub async fn delete_class(connection: &sqlite::Connection, class_id: i64) -> (bool, String) {
        // if so, this user didn't really exist in the first place, go figure.
        if !V::check_id(connection, class_id, V::CLASSES).await {
            return (
                false,
                "Class ID ".to_owned()
                    + &class_id.to_string()
                    + " does not exist in classes table.",
            );
        }

        // only delete a school if there are no associated classes
        match R::retrieve_assignments_from_class(connection, class_id).await {
            Ok(values) => {
                if values.into_iter().next().is_some() {
                    return (
                        false,
                        "Could not delete Class ID ".to_owned()
                            + &class_id.to_string()
                            + " because there are still assignments associated with this class.",
                    );
                }
            }
            Err(x) => {
                return (
                    false,
                    "Retrieving classes for School ID ".to_owned()
                        + &class_id.to_string()
                        + " encountered an error of "
                        + &x.to_string(),
                )
            }
        }

        let query = "DELETE FROM classes WHERE class_id = ".to_owned() + &class_id.to_string();

        match connection.execute(query) {
            Ok(_) => {
                return (
                    true,
                    "Class record ".to_owned() + &class_id.to_string() + " successfully deleted.",
                )
            }
            Err(x) => {
                return (
                    false,
                    "Failed to deleted Class record of ID ".to_owned()
                        + &class_id.to_string()
                        + &"but deletion returned error of "
                        + &x.to_string(),
                )
            }
        }
    }

    pub async fn delete_teacher_class(connection: &sqlite::Connection, teacher_id: i64, class_id: i64) -> (bool, String){
        if !V::check_id_pair(connection, teacher_id, class_id, V::TEACHERS_CLASSES).await{
            return (false, format!("No matching record of teacher id {} and class {} exists to delete in the teacher-class table.", teacher_id, class_id))
        }

        let query: String = format!("DELETE FROM teacher_classes WHERE teacher_id = {} AND class_id = {}", teacher_id, class_id);

        match connection.execute(query){
            Ok(_) => return (true, "Success!".to_string()),
            Err(x) => return (false, x.to_string())
        }
    }

}


