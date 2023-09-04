use crate::db_types as types;

use sqlite;

pub struct U {}

// always verify before calling these functions!
impl U {
    // always verify information integrity before calling these functions!
    pub async fn update_employee_supervisor(
        connection: &sqlite::Connection,
        info_in: &types::EmployeeSupervisor,
    ) {
        let query: String = "UPDATE employees_superviors SET supervisor_name = ".to_owned()
            + &info_in.supervisor_name().await.to_string()
            + " WHERE supervisory_id = "
            + &info_in.id().await.to_string()
            + ";";
    }
}
