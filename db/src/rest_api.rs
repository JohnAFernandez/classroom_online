use crate::db_types as types;
use crate::db_retrieve as R;
use crate::db_object_to_row as otr;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::path::PathBuf;
use sqlite;

#[get("/user")]
async fn get_user(req_body: String) -> HttpResponse {
    let connection = sqlite::open(PathBuf::from(".//src//db//db.sql")).unwrap();

    ;

    HttpResponse::Ok().body(req_body)
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