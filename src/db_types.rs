use serde::{Deserialize, Serialize};

// need these to print results and make sure we can test against each other.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
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

pub async fn build_user(
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

//pub async fn build_empty_user() -> User {
//    let emp : String = "".to_string();
//    build_user(0, emp, emp.copy(), emp.copy(), emp, emp, emp, emp, emp, emp, false, false)
//}

// some getter functions, because I don't want data to get clobbered by accident.
impl User {
    pub async fn id(&self) -> i64 {
        return self.id;
    }

    pub async fn email(&self) -> String {
        return self.email.clone();
    }

    pub async fn username(&self) -> String {
        return self.username.clone();
    }

    // this should not be used to send information to a client. This should only be used
    // for inbound information being applied to the database
    pub async fn password(&self) -> String {
        return self.password.clone();
    }

    pub async fn first_name(&self) -> String {
        return self.first_name.clone();
    }

    pub async fn last_name(&self) -> String {
        return self.last_name.clone();
    }

    pub async fn birthday(&self) -> String {
        return self.birthday.clone();
    }

    pub async fn date_registered(&self) -> String {
        return self.date_registered.clone();
    }

    pub async fn phone(&self) -> String {
        return self.phone.clone();
    }

    pub async fn icon(&self) -> String {
        return self.icon.clone();
    }

    pub async fn hidden(&self) -> bool {
        return self.hidden;
    }

    pub async fn deleted(&self) -> bool {
        return self.deleted;
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
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

pub async fn build_organization(
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

impl Organization {
    pub async fn id(&self) -> i64 {
        return self.id;
    }

    pub async fn name(&self) -> String {
        return self.name.clone();
    }

    pub async fn address1(&self) -> String {
        return self.address1.clone();
    }

    pub async fn address2(&self) -> String {
        return self.address2.clone();
    }

    pub async fn city(&self) -> String {
        return self.city.clone();
    }

    pub async fn state(&self) -> String {
        return self.state.clone();
    }

    pub async fn zip(&self) -> String {
        return self.zip.clone();
    }

    pub async fn phone(&self) -> String {
        return self.phone.clone();
    }

    pub async fn country(&self) -> String {
        return self.country.clone();
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Administrator {
    id: i64,
    user_id: i64,
    level: String,
}

pub async fn build_administrator(id: i64, user_id: i64, level: String) -> Administrator {
    Administrator { id, user_id, level }
}

impl Administrator {
    pub async fn id(&self) -> i64 {
        return self.id;
    }

    pub async fn user_id(&self) -> i64 {
        return self.user_id;
    }

    pub async fn level(&self) -> String {
        return self.level.clone();
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
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

pub async fn build_school(
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

impl School {
    pub async fn id(&self) -> i64 {
        return self.id;
    }

    pub async fn organization_id(&self) -> i64 {
        return self.organization_id;
    }

    pub async fn super_administrator_id(&self) -> i64 {
        return self.super_administrator_id;
    }

    pub async fn icon(&self) -> String {
        return self.icon.clone();
    }

    pub async fn name(&self) -> String {
        return self.name.clone();
    }

    pub async fn address1(&self) -> String {
        return self.address1.clone();
    }

    pub async fn address2(&self) -> String {
        return self.address2.clone();
    }

    pub async fn city(&self) -> String {
        return self.city.clone();
    }

    pub async fn state(&self) -> String {
        return self.state.clone();
    }

    pub async fn zip(&self) -> String {
        return self.zip.clone();
    }

    pub async fn phone(&self) -> String {
        return self.phone.clone();
    }

    pub async fn country(&self) -> String {
        return self.country.clone();
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct AdministratorSchool {
    admin_id: i64,
    school_id: i64,
}

pub async fn build_administrator_school(admin_id: i64, school_id: i64) -> AdministratorSchool {
    AdministratorSchool {
        admin_id,
        school_id,
    }
}

impl AdministratorSchool {
    pub async fn admin_id(&self) -> i64 {
        return self.admin_id;
    }

    pub async fn school_id(&self) -> i64 {
        return self.school_id;
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Teacher {
    teacher_id: i64,
    user_id: i64,
}

pub async fn build_teacher(teacher_id: i64, user_id: i64) -> Teacher {
    Teacher {
        teacher_id,
        user_id,
    }
}

impl Teacher {
    pub async fn teacher_id(&self) -> i64 {
        return self.teacher_id;
    }

    pub async fn user_id(&self) -> i64 {
        return self.user_id;
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct EmployeeSupervisor {
    id: i64,
    user_id: i64,
    administrator_id: i64,
    supervisor_name: String,
    organization_id: i64,
}

pub async fn build_employee_supervisor(
    id: i64,
    user_id: i64,
    administrator_id: i64,
    supervisor_name: String,
    organization_id: i64,
) -> EmployeeSupervisor {
    EmployeeSupervisor {
        id,
        user_id,
        administrator_id,
        supervisor_name,
        organization_id,
    }
}

impl EmployeeSupervisor {
    pub async fn id(&self) -> i64 {
        return self.id;
    }

    pub async fn user_id(&self) -> i64 {
        return self.user_id;
    }

    pub async fn administrator_id(&self) -> i64 {
        return self.administrator_id;
    }

    pub async fn supervisor_name(&self) -> String {
        return self.supervisor_name.clone();
    }

    pub async fn set_supervisor_name(&mut self, name_in: String) {
        self.supervisor_name = name_in;
    }

    pub async fn organization_id(&self) -> i64 {
        return self.organization_id;
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct TeacherSchool {
    teacher_id: i64,
    school_id: i64,
}

pub async fn build_teacher_school(teacher_id: i64, school_id: i64) -> TeacherSchool {
    TeacherSchool {
        teacher_id,
        school_id,
    }
}

impl TeacherSchool {
    pub async fn teacher_id(&self) -> i64 {
        return self.teacher_id;
    }

    pub async fn school_id(&self) -> i64 {
        return self.school_id;
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Subject {
    id: i64,
    name: String,
    ap: bool,
    ib: bool,
    target_year: String,
    discipline: String,
}

pub async fn build_subject(
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

impl Subject {
    pub async fn id(&self) -> i64 {
        return self.id;
    }

    pub async fn name(&self) -> String {
        return self.name.clone();
    }

    pub async fn ap(&self) -> bool {
        return self.ap;
    }

    pub async fn ib(&self) -> bool {
        return self.ib;
    }

    pub async fn target_year(&self) -> String {
        return self.target_year.clone();
    }

    pub async fn discipline(&self) -> String {
        return self.discipline.clone();
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Class {
    id: i64,
    school_id: i64,
    subject_id: i64,
    year: String,
    start_day: String,
    end_day: String,
    start_time: i32,
    end_time: i32,
    days_scheduled: String,
}

pub async fn build_class(
    id: i64,
    school_id: i64,
    subject_id: i64,
    year: String,
    start_day: String,
    end_day: String,
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

impl Class {
    pub async fn id(&self) -> i64 {
        return self.id;
    }

    pub async fn school_id(&self) -> i64 {
        return self.school_id;
    }

    pub async fn subject_id(&self) -> i64 {
        return self.subject_id;
    }

    pub async fn year(&self) -> String {
        return self.year.clone();
    }

    pub async fn start_day(&self) -> String {
        return self.start_day.clone();
    }

    pub async fn end_day(&self) -> String {
        return self.end_day.clone();
    }

    pub async fn start_time(&self) -> i32 {
        return self.start_time;
    }

    pub async fn end_time(&self) -> i32 {
        return self.end_time;
    }

    pub async fn days_scheduled(&self) -> String {
        return self.days_scheduled.clone();
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct TeacherClass {
    teacher_id: i64,
    class_id: i64,
    role: String,
    active: bool,
}

pub async fn build_teacher_class(teacher_id: i64, class_id: i64, role: String, active: bool) -> TeacherClass {
    TeacherClass {
        teacher_id,
        class_id,
        role,
        active,
    }
}

impl TeacherClass {
    pub async fn teacher_id(&self) -> i64 {
        return self.teacher_id;
    }

    pub async fn class_id(&self) -> i64 {
        return self.class_id;
    }

    pub async fn role(&self) -> String {
        return self.role.clone();
    }

    pub async fn active(&self) -> bool {
        return self.active;
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Student {
    id: i64,
    user_id: i64,
}

pub async fn build_student(id: i64, user_id: i64) -> Student {
    Student { id, user_id }
}

impl Student {
    pub async fn id(&self) -> i64 {
        return self.id;
    }

    pub async fn user_id(&self) -> i64 {
        return self.user_id;
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct StudentClass {
    student_id: i64,
    class_id: i64,
}

pub async fn build_student_class(student_id: i64, class_id: i64) -> StudentClass {
    StudentClass {
        student_id,
        class_id,
    }
}

impl StudentClass {
    pub async fn student_id(&self) -> i64 {
        return self.student_id;
    }

    pub async fn class_id(&self) -> i64 {
        return self.class_id;
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Family {
    family_id: i64,
    name: String,
}

pub async fn build_family(family_id: i64, name: String) -> Family {
    Family { family_id, name }
}

impl Family {
    pub async fn family_id(&self) -> i64 {
        return self.family_id;
    }

    pub async fn name(&self) -> String {
        return self.name.clone();
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct FamilyUser {
    family_id: i64,
    user_id: i64,
    relationship: String,
}

pub async fn build_family_user(family_id: i64, user_id: i64, relationship: String) -> FamilyUser {
    FamilyUser {
        family_id,
        user_id,
        relationship,
    }
}

impl FamilyUser {
    pub async fn family_id(&self) -> i64 {
        return self.family_id;
    }

    pub async fn user_id(&self) -> i64 {
        return self.user_id;
    }

    pub async fn relationship(&self) -> String {
        return self.relationship.clone();
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct FamilyMember {
    id: i64,
    user_id: i64,
    notification_methods: String,
    email: String,
    phone: String,
}

pub async fn build_family_member(
    id: i64,
    user_id: i64,
    notification_methods: String,
    email: String,
    phone: String,
) -> FamilyMember {
    FamilyMember {
        id,
        user_id,
        notification_methods,
        email,
        phone,
    }
}

impl FamilyMember {
    pub async fn id(&self) -> i64 {
        return self.id;
    }

    pub async fn user_id(&self) -> i64 {
        return self.user_id;
    }

    pub async fn notification_methods(&self) -> String {
        return self.notification_methods.clone();
    }

    pub async fn email(&self) -> String {
        return self.email.clone();
    }

    pub async fn set_email(&mut self, email: String) {
        self.email = email;
    }

    pub async fn phone(&self) -> String {
        return self.phone.clone();
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Assignment {
    id: i64,
    class_id: i64,
    required: bool,
    grade_scale: String,
    name: String,
    description: String,
    template: String,
}

pub async fn build_assignment(
    id: i64,
    class_id: i64,
    required: bool,
    grade_scale: String,
    name: String,
    description: String,
    template: String,
) -> Assignment {
    Assignment {
        id,
        class_id,
        required,
        grade_scale,
        name,
        description,
        template,
    }
}

impl Assignment {
    pub async fn id(&self) -> i64 {
        return self.id;
    }

    pub async fn class_id(&self) -> i64 {
        return self.class_id;
    }

    pub async fn required(&self) -> bool {
        return self.required;
    }

    pub async fn grade_scale(&self) -> String {
        return self.grade_scale.clone();
    }

    pub async fn name(&self) -> String {
        return self.name.clone();
    }

    pub async fn description(&self) -> String {
        return self.description.clone();
    }

    pub async fn template(&self) -> String {
        return self.template.clone();
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Submission {
    id: i64,
    user_id: i64,
    assignment_id: i64,
    contents: String,
    grade: String,
}

pub async fn build_submission(
    id: i64,
    user_id: i64,
    assignment_id: i64,
    contents: String,
    grade: String,
) -> Submission {
    Submission {
        id,
        user_id,
        assignment_id,
        contents,
        grade,
    }
}

impl Submission {
    pub async fn id(&self) -> i64 {
        return self.id;
    }

    pub async fn user_id(&self) -> i64 {
        return self.user_id;
    }

    pub async fn assignment_id(&self) -> i64 {
        return self.assignment_id;
    }

    pub async fn contents(&self) -> String {
        return self.contents.clone();
    }

    pub async fn grade(&self) -> String {
        return self.grade.clone();
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Comment {
    id: i64,
    user_id: i64,
    assignment_id: i64,
    contents: String,
}

pub async fn build_comment(id: i64, user_id: i64, assignment_id: i64, contents: String) -> Comment {
    Comment {
        id,
        user_id,
        assignment_id,
        contents,
    }
}

impl Comment {
    pub async fn id(&self) -> i64 {
        return self.id;
    }

    pub async fn user_id(&self) -> i64 {
        return self.user_id;
    }

    pub async fn assignment_id(&self) -> i64 {
        return self.assignment_id;
    }

    pub async fn contents(&self) -> String {
        return self.contents.clone();
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ChangeLogItem {
    id: i64,
    source_name: String,
    change_type: i32,
    old_value: String,
    timestamp: String,
}

pub async fn build_change_log_item(
    id: i64,
    source_name: String,
    change_type: i32,
    old_value: String,
    timestamp: String,
) -> ChangeLogItem {
    ChangeLogItem {
        id,
        source_name,
        change_type,
        old_value,
        timestamp,
    }
}

impl ChangeLogItem {
    pub async fn id(&self) -> i64 {
        return self.id;
    }

    pub async fn source_name(&self) -> String {
        return self.source_name.clone();
    }

    pub async fn change_type(&self) -> i32 {
        return self.change_type;
    }

    pub async fn old_value(&self) -> String {
        return self.old_value.clone();
    }

    pub async fn timestamp(&self) -> String {
        return self.timestamp.clone();
    }
}
