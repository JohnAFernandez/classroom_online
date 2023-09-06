use crate::db_types as types;
use crate::db_retrieve::R;
use crate::db_delete::D;
use crate::db_row_to_object as rto;
use crate::db_object_to_row as otr;

use actix_web::{get, post, delete, web, HttpResponse, Responder};
use std::path::PathBuf;
use sqlite;

#[get("/user/{id}")]
async fn get_user(path: web::Path<u64>) -> HttpResponse {
    let id = path.into_inner();

    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    match R::retrieve_details(&connection,R::USERS, id.to_string()).await {
        Ok(x) => { 
            let user: types::User;

            for row in x.into_iter().map(|row| row.unwrap()) {

                user = rto::row_to_user(&row).await;

                match serde_json::to_string(&user) {
                    Ok(x) => { 
                        otr::change_log_to_row(&connection, types::build_log_item(id as i64, -1, -1, "Not Implemented".to_string(), "REST get_user".to_string(), chrono::Utc::now().to_string()).await);
                        return HttpResponse::Ok().body(x.to_string()) },
                    Err(x) => return HttpResponse::InternalServerError().body(x.to_string()),
                }                
            }
            
        },
        Err(x) => return HttpResponse::BadRequest().body(x.to_string()),
    }

    return HttpResponse::InternalServerError().body("Unknown Server Error.");
}

#[get("/administrator/{id}")]
async fn get_administrator(path: web::Path<u64>) -> HttpResponse {
    let id = path.into_inner();

    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    match R::retrieve_details(&connection,R::ADMINISTRATORS, id.to_string()).await {
        Ok(x) => { 
            let object: types::Administrator;

            for row in x.into_iter().map(|row| row.unwrap()) {

                object = rto::row_to_administrator(&row).await;

                match serde_json::to_string(&object) {
                    Ok(x) => { return HttpResponse::Ok().body(x.to_string()) },
                    Err(x) => return HttpResponse::InternalServerError().body(x.to_string()),
                }                
            }
            
        },
        Err(x) => return HttpResponse::BadRequest().body(x.to_string()),
    }

    return HttpResponse::InternalServerError().body("Unknown Server Error.");
}

#[get("/organization/{id}")]
async fn get_organization(path: web::Path<u64>) -> HttpResponse {
    let id = path.into_inner();

    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    match R::retrieve_details(&connection,R::ORGANIZATIONS, id.to_string()).await {
        Ok(x) => { 
            let object: types::Organization;

            for row in x.into_iter().map(|row| row.unwrap()) {

                object = rto::row_to_organization(&row).await;

                match serde_json::to_string(&object) {
                    Ok(x) => { return HttpResponse::Ok().body(x.to_string()) },
                    Err(x) => return HttpResponse::InternalServerError().body(x.to_string()),
                }                
            }
            
        },
        Err(x) => return HttpResponse::BadRequest().body(x.to_string()),
    }

    return HttpResponse::InternalServerError().body("Unknown Server Error.");
}

#[get("/school/{id}")]
async fn get_school(path: web::Path<u64>) -> HttpResponse {
    let id = path.into_inner();

    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    match R::retrieve_details(&connection,R::SCHOOLS, id.to_string()).await {
        Ok(x) => { 
            let object: types::School;

            for row in x.into_iter().map(|row| row.unwrap()) {

                object = rto::row_to_school(&row).await;

                match serde_json::to_string(&object) {
                    Ok(x) => { return HttpResponse::Ok().body(x.to_string()) },
                    Err(x) => return HttpResponse::InternalServerError().body(x.to_string()),
                }                
            }
            
        },
        Err(x) => return HttpResponse::BadRequest().body(x.to_string()),
    }

    return HttpResponse::InternalServerError().body("Unknown Server Error.");
}


#[get("/teacher/{id}")]
async fn get_teacher(path: web::Path<u64>) -> HttpResponse {
    let id = path.into_inner();

    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    match R::retrieve_details(&connection,R::TEACHERS, id.to_string()).await {
        Ok(x) => { 
            let object: types::Teacher;

            for row in x.into_iter().map(|row| row.unwrap()) {

                object = rto::row_to_teacher(&row).await;

                match serde_json::to_string(&object) {
                    Ok(x) => { return HttpResponse::Ok().body(x.to_string()) },
                    Err(x) => return HttpResponse::InternalServerError().body(x.to_string()),
                }                
            }
            
        },
        Err(x) => return HttpResponse::BadRequest().body(x.to_string()),
    }

    return HttpResponse::InternalServerError().body("Unknown Server Error.");
}

#[get("/employee_supervisor/{id}")]
async fn get_employee_supervisor(path: web::Path<u64>) -> HttpResponse {
    let id = path.into_inner();

    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    match R::retrieve_details(&connection,R::EMPLOYEES_SUPERVISORS, id.to_string()).await {
        Ok(x) => { 
            let object: types::EmployeeSupervisor;

            for row in x.into_iter().map(|row| row.unwrap()) {

                object = rto::row_to_employee_supervisor(&row).await;

                match serde_json::to_string(&object) {
                    Ok(x) => { return HttpResponse::Ok().body(x.to_string()) },
                    Err(x) => return HttpResponse::InternalServerError().body(x.to_string()),
                }                
            }
            
        },
        Err(x) => return HttpResponse::BadRequest().body(x.to_string()),
    }

    return HttpResponse::InternalServerError().body("Unknown Server Error.");
}


#[get("/subject/{id}")]
async fn get_subject(path: web::Path<u64>) -> HttpResponse {
    let id = path.into_inner();

    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    match R::retrieve_details(&connection,R::SUBJECTS, id.to_string()).await {
        Ok(x) => { 
            let object: types::Subject;

            for row in x.into_iter().map(|row| row.unwrap()) {

                object = rto::row_to_subject(&row).await;

                match serde_json::to_string(&object) {
                    Ok(x) => { return HttpResponse::Ok().body(x.to_string()) },
                    Err(x) => return HttpResponse::InternalServerError().body(x.to_string()),
                }                
            }
            
        },
        Err(x) => return HttpResponse::BadRequest().body(x.to_string()),
    }

    return HttpResponse::InternalServerError().body("Unknown Server Error.");
}

#[get("/class/{id}")]
async fn get_class(path: web::Path<u64>) -> HttpResponse {
    let id = path.into_inner();

    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    match R::retrieve_details(&connection,R::CLASSES, id.to_string()).await {
        Ok(x) => { 
            let object: types::Class;

            for row in x.into_iter().map(|row| row.unwrap()) {

                object = rto::row_to_class(&row).await;

                match serde_json::to_string(&object) {
                    Ok(x) => { return HttpResponse::Ok().body(x.to_string()) },
                    Err(x) => return HttpResponse::InternalServerError().body(x.to_string()),
                }                
            }
            
        },
        Err(x) => return HttpResponse::BadRequest().body(x.to_string()),
    }

    return HttpResponse::InternalServerError().body("Unknown Server Error.");
}

#[get("/student/{id}")]
async fn get_student(path: web::Path<u64>) -> HttpResponse {
    let id = path.into_inner();

    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    match R::retrieve_details(&connection,R::STUDENTS, id.to_string()).await {
        Ok(x) => { 
            let object: types::Student;

            for row in x.into_iter().map(|row| row.unwrap()) {

                object = rto::row_to_student(&row).await;

                match serde_json::to_string(&object) {
                    Ok(x) => { return HttpResponse::Ok().body(x.to_string()) },
                    Err(x) => return HttpResponse::InternalServerError().body(x.to_string()),
                }                
            }
            
        },
        Err(x) => return HttpResponse::BadRequest().body(x.to_string()),
    }

    return HttpResponse::InternalServerError().body("Unknown Server Error.");
}

#[get("/family/{id}")]
async fn get_family(path: web::Path<u64>) -> HttpResponse {
    let id = path.into_inner();

    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    match R::retrieve_details(&connection,R::FAMILIES, id.to_string()).await {
        Ok(x) => { 
            let object: types::Family;

            for row in x.into_iter().map(|row| row.unwrap()) {

                object = rto::row_to_family(&row).await;

                match serde_json::to_string(&object) {
                    Ok(x) => { return HttpResponse::Ok().body(x.to_string()) },
                    Err(x) => return HttpResponse::InternalServerError().body(x.to_string()),
                }                
            }
            
        },
        Err(x) => return HttpResponse::BadRequest().body(x.to_string()),
    }

    return HttpResponse::InternalServerError().body("Unknown Server Error.");
}

#[get("/family-member/{id}")]
async fn get_family_member(path: web::Path<u64>) -> HttpResponse {
    let id = path.into_inner();

    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    match R::retrieve_details(&connection,R::FAMILY_MEMBERS, id.to_string()).await {
        Ok(x) => { 
            let object: types::FamilyMember;

            for row in x.into_iter().map(|row| row.unwrap()) {

                object = rto::row_to_family_member(&row).await;

                match serde_json::to_string(&object) {
                    Ok(x) => { return HttpResponse::Ok().body(x.to_string()) },
                    Err(x) => return HttpResponse::InternalServerError().body(x.to_string()),
                }                
            }
            
        },
        Err(x) => return HttpResponse::BadRequest().body(x.to_string()),
    }

    return HttpResponse::InternalServerError().body("Unknown Server Error.");
}

#[get("/assignment/{id}")]
async fn get_assignment(path: web::Path<u64>) -> HttpResponse {
    let id = path.into_inner();

    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    match R::retrieve_details(&connection,R::ASSIGNMENTS, id.to_string()).await {
        Ok(x) => { 
            let object: types::Assignment;

            for row in x.into_iter().map(|row| row.unwrap()) {

                object = rto::row_to_assignment(&row).await;

                match serde_json::to_string(&object) {
                    Ok(x) => { return HttpResponse::Ok().body(x.to_string()) },
                    Err(x) => return HttpResponse::InternalServerError().body(x.to_string()),
                }                
            }
            
        },
        Err(x) => return HttpResponse::BadRequest().body(x.to_string()),
    }

    return HttpResponse::InternalServerError().body("Unknown Server Error.");
}

#[get("/submission/{id}")]
async fn get_submission(path: web::Path<u64>) -> HttpResponse {
    let id = path.into_inner();

    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    match R::retrieve_details(&connection,R::SUBMISSIONS, id.to_string()).await {
        Ok(x) => { 
            let object: types::Submission;

            for row in x.into_iter().map(|row| row.unwrap()) {

                object = rto::row_to_submission(&row).await;

                match serde_json::to_string(&object) {
                    Ok(x) => { return HttpResponse::Ok().body(x.to_string()) },
                    Err(x) => return HttpResponse::InternalServerError().body(x.to_string()),
                }                
            }
            
        },
        Err(x) => return HttpResponse::BadRequest().body(x.to_string()),
    }

    return HttpResponse::InternalServerError().body("Unknown Server Error.");
}


#[get("/comment/{id}")]
async fn get_comment(path: web::Path<u64>) -> HttpResponse {
    let id = path.into_inner();

    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    match R::retrieve_details(&connection,R::COMMENTS, id.to_string()).await {
        Ok(x) => { 
            let object: types::Comment;

            for row in x.into_iter().map(|row| row.unwrap()) {

                object = rto::row_to_comment(&row).await;

                match serde_json::to_string(&object) {
                    Ok(x) => { return HttpResponse::Ok().body(x.to_string()) },
                    Err(x) => return HttpResponse::InternalServerError().body(x.to_string()),
                }                
            }
            
        },
        Err(x) => return HttpResponse::BadRequest().body(x.to_string()),
    }

    return HttpResponse::InternalServerError().body("Unknown Server Error.");
}


#[get("/administrator-school/{admin_id}/{school_id}")]
async fn get_administrator_school(path: web::Path<(u64, u64)>) -> HttpResponse {
    let (admin_id, school_id) = path.into_inner();
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();


    match R::retrieve_details_pair(&connection,R::COMMENTS, admin_id.to_string(), school_id.to_string()).await {
        Ok(x) => { 
            let object: types::Comment;

            for row in x.into_iter().map(|row| row.unwrap()) {

                object = rto::row_to_comment(&row).await;

                match serde_json::to_string(&object) {
                    Ok(x) => { return HttpResponse::Ok().body(x.to_string()) },
                    Err(x) => return HttpResponse::InternalServerError().body(x.to_string()),
                }                
            }
            
        },
        Err(x) => return HttpResponse::BadRequest().body(x.to_string()),
    }
    
    
    return HttpResponse::InternalServerError().body("Unknown Server Error.")

}

#[delete("/teacher-class/{teacher_id}/{class_id}")]
async fn delete_teacher_class(path: web::Path<(u64, u64)>) -> HttpResponse {
    let (teacher_id, class_id) = path.into_inner();
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    match D::delete_teacher_class(&connection, teacher_id as i64, class_id as i64).await{
        x if x.0 == true => return HttpResponse::Ok().body(x.1),
        x if x.0 == false => return HttpResponse::UnprocessableEntity().body(x.1),
        _=> return HttpResponse::InternalServerError().body("Unknown Server Error."),   
    }
}


#[post("/user")]
async fn post_user(req_body: String) -> HttpResponse {
    // connect to the database 
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    let new_record: types::User;

    // do we have a 
    match serde_json::from_str(&req_body) {
        Ok(x) => new_record = x,
        Err(x) => return HttpResponse::UnprocessableEntity().body("Bad format for post request. See serde error: ".to_string() + &x.to_string()),
    }

    match otr::users_to_row(&connection, new_record).await {
        x if x.0 == true => HttpResponse::Ok().body(x.1 + &req_body),
        x if x.0 == false => HttpResponse::UnprocessableEntity().body(x.1),
        _=> HttpResponse::Ok().body(req_body),
    }

}

#[post("/organization")]
async fn post_organization(req_body: String) -> impl Responder {
    // connect to the database 
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    let new_record: types::Organization;

    // do we have a 
    match serde_json::from_str(&req_body) {
        Ok(x) => new_record = x,
        Err(x) => return HttpResponse::UnprocessableEntity().body("Bad format for post request. See serde error: ".to_string() + &x.to_string()),
    }

    match otr::organization_to_row(&connection, new_record).await {
        x if x.0 == true => HttpResponse::Ok().body(x.1 + &req_body),
        x if x.0 == false => HttpResponse::UnprocessableEntity().body(x.1),
        _=> HttpResponse::Ok().body(req_body),
    }}

#[post("/school")]
async fn post_school(req_body: String) -> impl Responder {
    // connect to the database 
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    let new_record: types::School;

    // do we have a 
    match serde_json::from_str(&req_body) {
        Ok(x) => new_record = x,
        Err(x) => return HttpResponse::UnprocessableEntity().body("Bad format for post request. See serde error: ".to_string() + &x.to_string()),
    }

    match otr::school_to_row(&connection, new_record).await {
        x if x.0 == true => HttpResponse::Ok().body(x.1 + &req_body),
        x if x.0 == false => HttpResponse::UnprocessableEntity().body(x.1),
        _=> HttpResponse::Ok().body(req_body),
    }
}

#[post("/teacher")]
async fn post_teacher(req_body: String) -> impl Responder {
    // connect to the database 
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    let new_record: types::Teacher;

    // do we have a 
    match serde_json::from_str(&req_body) {
        Ok(x) => new_record = x,
        Err(x) => return HttpResponse::UnprocessableEntity().body("Bad format for post request. See serde error: ".to_string() + &x.to_string()),
    }

    match otr::teacher_to_row(&connection, new_record).await {
        x if x.0 == true => HttpResponse::Ok().body(x.1 + &req_body),
        x if x.0 == false => HttpResponse::UnprocessableEntity().body(x.1),
        _=> HttpResponse::Ok().body(req_body),
    }
}

#[post("/administrator")]
async fn post_administrator(req_body: String) -> impl Responder {
    // connect to the database 
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    let new_record: types::Administrator;

    // do we have a 
    match serde_json::from_str(&req_body) {
        Ok(x) => new_record = x,
        Err(x) => return HttpResponse::UnprocessableEntity().body("Bad format for post request. See serde error: ".to_string() + &x.to_string()),
    }

    match otr::administrator_to_row(&connection, new_record).await {
        x if x.0 == true => HttpResponse::Ok().body(x.1 + &req_body),
        x if x.0 == false => HttpResponse::UnprocessableEntity().body(x.1),
        _=> HttpResponse::Ok().body(req_body),
    }
}


#[post("/supervisory")]
async fn post_supervisory(req_body: String) -> impl Responder {
    // connect to the database 
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    let new_record: types::EmployeeSupervisor;

    // do we have a 
    match serde_json::from_str(&req_body) {
        Ok(x) => new_record = x,
        Err(x) => return HttpResponse::UnprocessableEntity().body("Bad format for post request. See serde error: ".to_string() + &x.to_string()),
    }

    match otr::employee_supervisor_to_row(&connection, new_record).await {
        x if x.0 == true => HttpResponse::Ok().body(x.1 + &req_body),
        x if x.0 == false => HttpResponse::UnprocessableEntity().body(x.1),
        _=> HttpResponse::Ok().body(req_body),
    }
}

#[post("/Subject")]
async fn post_subject(req_body: String) -> impl Responder {
    // connect to the database 
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    let new_record: types::Subject;

    // do we have a 
    match serde_json::from_str(&req_body) {
        Ok(x) => new_record = x,
        Err(x) => return HttpResponse::UnprocessableEntity().body("Bad format for post request. See serde error: ".to_string() + &x.to_string()),
    }

    otr::subject_to_row(&connection, new_record).await;
    HttpResponse::Ok().body(req_body)
}

#[post("/class")]
async fn post_class(req_body: String) -> impl Responder {
    // connect to the database 
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    let new_record: types::Class;

    // do we have a 
    match serde_json::from_str(&req_body) {
        Ok(x) => new_record = x,
        Err(x) => return HttpResponse::UnprocessableEntity().body("Bad format for post request. See serde error: ".to_string() + &x.to_string()),
    }

    match otr::class_to_row(&connection, new_record).await {
        x if x.0 == true => HttpResponse::Ok().body(x.1 + &req_body),
        x if x.0 == false => HttpResponse::UnprocessableEntity().body(x.1),
        _=> HttpResponse::Ok().body(req_body),
    }
}

#[post("/student")]
async fn post_student(req_body: String) -> impl Responder {
    // connect to the database 
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    let new_record: types::Student;

    // do we have a 
    match serde_json::from_str(&req_body) {
        Ok(x) => new_record = x,
        Err(x) => return HttpResponse::UnprocessableEntity().body("Bad format for post request. See serde error: ".to_string() + &x.to_string()),
    }

    match otr::student_to_row(&connection, new_record).await {
        x if x.0 == true => HttpResponse::Ok().body(x.1 + &req_body),
        x if x.0 == false => HttpResponse::UnprocessableEntity().body(x.1),
        _=> HttpResponse::Ok().body(req_body),
    }
}

#[post("/family")]
async fn post_family(req_body: String) -> impl Responder {
    // connect to the database 
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    let new_record: types::Family;

    // do we have a 
    match serde_json::from_str(&req_body) {
        Ok(x) => new_record = x,
        Err(x) => return HttpResponse::UnprocessableEntity().body("Bad format for post request. See serde error: ".to_string() + &x.to_string()),
    }

    match otr::family_to_row(&connection, new_record).await {
        x if x.0 == true => HttpResponse::Ok().body(x.1 + &req_body),
        x if x.0 == false => HttpResponse::UnprocessableEntity().body(x.1),
        _=> HttpResponse::Ok().body(req_body),
    }
}

#[post("/family-member")]
async fn post_family_member(req_body: String) -> impl Responder {
    // connect to the database 
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    let new_record: types::FamilyMember;

    // do we have a 
    match serde_json::from_str(&req_body) {
        Ok(x) => new_record = x,
        Err(x) => return HttpResponse::UnprocessableEntity().body("Bad format for post request. See serde error: ".to_string() + &x.to_string()),
    }

    match otr::family_member_to_row(&connection, new_record).await {
        x if x.0 == true => HttpResponse::Ok().body(x.1 + &req_body),
        x if x.0 == false => HttpResponse::UnprocessableEntity().body(x.1),
        _=> HttpResponse::Ok().body(req_body),
    }
}

#[post("/assignment")]
async fn post_assignment(req_body: String) -> impl Responder {
    // connect to the database 
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    let new_record: types::Assignment;

    // do we have a 
    match serde_json::from_str(&req_body) {
        Ok(x) => new_record = x,
        Err(x) => return HttpResponse::UnprocessableEntity().body("Bad format for post request. See serde error: ".to_string() + &x.to_string()),
    }

    match otr::assignment_to_row(&connection, new_record).await {
        x if x.0 == true => HttpResponse::Ok().body(x.1 + &req_body),
        x if x.0 == false => HttpResponse::UnprocessableEntity().body(x.1),
        _=> HttpResponse::Ok().body(req_body),
    }
}

#[post("/submission")]
async fn post_submission(req_body: String) -> impl Responder {
    // connect to the database 
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    let new_record: types::Submission;

    // do we have a 
    match serde_json::from_str(&req_body) {
        Ok(x) => new_record = x,
        Err(x) => return HttpResponse::UnprocessableEntity().body("Bad format for post request. See serde error: ".to_string() + &x.to_string()),
    }

    match otr::submission_to_row(&connection, new_record).await {
        x if x.0 == true => HttpResponse::Ok().body(x.1 + &req_body),
        x if x.0 == false => HttpResponse::UnprocessableEntity().body(x.1),
        _=> HttpResponse::Ok().body(req_body),
    }
}

#[post("/comment")]
async fn post_comment(req_body: String) -> impl Responder {
    // connect to the database 
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    let new_record: types::Comment;

    // do we have a 
    match serde_json::from_str(&req_body) {
        Ok(x) => new_record = x,
        Err(x) => return HttpResponse::UnprocessableEntity().body("Bad format for post request. See serde error: ".to_string() + &x.to_string()),
    }

    match otr::comment_to_row(&connection, new_record).await {
        x if x.0 == true => HttpResponse::Ok().body(x.1 + &req_body),
        x if x.0 == false => HttpResponse::UnprocessableEntity().body(x.1),
        _=> HttpResponse::Ok().body(req_body),
    }
}

#[post("/family-user")]
async fn post_family_user(req_body: String) -> impl Responder {
    // connect to the database 
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    let new_record: types::FamilyUser;

    // do we have a 
    match serde_json::from_str(&req_body) {
        Ok(x) => new_record = x,
        Err(x) => return HttpResponse::UnprocessableEntity().body("Bad format for post request. See serde error: ".to_string() + &x.to_string()),
    }

    match otr::family_user_to_row(&connection, new_record).await {
        x if x.0 == true => HttpResponse::Ok().body(x.1 + &req_body),
        x if x.0 == false => HttpResponse::UnprocessableEntity().body(x.1),
        _=> HttpResponse::Ok().body(req_body),
    }
}

#[post("/student-class")]
async fn post_student_class(req_body: String) -> impl Responder {
    // connect to the database 
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    let new_record: types::StudentClass;

    // do we have a 
    match serde_json::from_str(&req_body) {
        Ok(x) => new_record = x,
        Err(x) => return HttpResponse::UnprocessableEntity().body("Bad format for post request. See serde error: ".to_string() + &x.to_string()),
    }

    match otr::student_class_to_row(&connection, new_record).await {
        x if x.0 == true => HttpResponse::Ok().body(x.1 + &req_body),
        x if x.0 == false => HttpResponse::UnprocessableEntity().body(x.1),
        _=> HttpResponse::Ok().body(req_body),
    }
}

#[post("/teacher-class")]
async fn post_teacher_class(req_body: String) -> impl Responder {
    // connect to the database 
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    let new_record: types::TeacherClass;

    // do we have a 
    match serde_json::from_str(&req_body) {
        Ok(x) => new_record = x,
        Err(x) => return HttpResponse::UnprocessableEntity().body("Bad format for post request. See serde error: ".to_string() + &x.to_string()),
    }

    match otr::teacher_class_to_row(&connection, new_record).await {
        x if x.0 == true => HttpResponse::Ok().body(x.1 + &req_body),
        x if x.0 == false => HttpResponse::UnprocessableEntity().body(x.1),
        _=> HttpResponse::Ok().body(req_body),
    }
}

#[post("/administrator-school")]
async fn post_administrator_school(req_body: String) -> impl Responder {
    // connect to the database 
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    let new_record: types::AdministratorSchool;

    // do we have a 
    match serde_json::from_str(&req_body) {
        Ok(x) => new_record = x,
        Err(x) => return HttpResponse::UnprocessableEntity().body("Bad format for post request. See serde error: ".to_string() + &x.to_string()),
    }

    match otr::administrator_school_to_row(&connection, new_record).await {
        x if x.0 == true => HttpResponse::Ok().body(x.1 + &req_body),
        x if x.0 == false => HttpResponse::UnprocessableEntity().body(x.1),
        _=> HttpResponse::Ok().body(req_body),
    }
}

#[post("/teacher-school")]
async fn post_teacher_school(req_body: String) -> impl Responder {
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    let new_record: types::TeacherSchool;

    // do we have a 
    match serde_json::from_str(&req_body) {
        Ok(x) => new_record = x,
        Err(x) => return HttpResponse::UnprocessableEntity().body("Bad format for post request. See serde error: ".to_string() + &x.to_string()),
    }

    match otr::teacher_school_to_row(&connection, new_record).await {
        x if x.0 == true => HttpResponse::Ok().body(x.1 + &req_body),
        x if x.0 == false => HttpResponse::UnprocessableEntity().body(x.1),
        _=> HttpResponse::Ok().body(req_body),
    }}

