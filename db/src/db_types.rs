// need these to print results and make sure we can test against each other.
#[derive(PartialEq, Eq, Debug)]
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

pub fn build_user(
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

// some getter functions, because I don't want data to get clobbered by accident.
impl User {
    pub fn id(&self) -> i64 {
        return self.id;
    }

    pub fn email(&self) -> String {
        return self.email.clone();
    }

    pub fn username(&self) -> String {
        return self.username.clone();
    }

    // this should not be used to send information to a client. This should only be used
    // for inbound information being applied to the database
    pub fn password(&self) -> String {
        return self.password.clone();
    }

    pub fn first_name(&self) -> String {
        return self.first_name.clone();
    }

    pub fn last_name(&self) -> String {
        return self.last_name.clone();
    }

    pub fn birthday(&self) -> String {
        return self.birthday.clone();
    }

    pub fn date_registered(&self) -> String {
        return self.date_registered.clone();
    }

    pub fn phone(&self) -> String {
        return self.phone.clone();
    }

    pub fn icon(&self) -> String {
        return self.icon.clone();
    }

    pub fn hidden(&self) -> bool {
        return self.hidden;
    }

    pub fn deleted(&self) -> bool {
        return self.deleted;
    }
}

#[derive(PartialEq, Eq, Debug)]
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

pub fn build_organization(
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
    pub fn id(&self) -> i64 {
        return self.id;
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn address1(&self) -> String {
        return self.address1.clone();
    }

    pub fn address2(&self) -> String {
        return self.address2.clone();
    }

    pub fn city(&self) -> String {
        return self.city.clone();
    }

    pub fn state(&self) -> String {
        return self.state.clone();
    }

    pub fn zip(&self) -> String {
        return self.zip.clone();
    }

    pub fn phone(&self) -> String {
        return self.phone.clone();
    }

    pub fn country(&self) -> String {
        return self.country.clone();
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Administrator {
    id: i64,
    user_id: i64,
    level: String,
}

pub fn build_administrator(id: i64, user_id: i64, level: String) -> Administrator {
    Administrator { id, user_id, level }
}

impl Administrator {
    pub fn id(&self) -> i64 {
        return self.id;
    }

    pub fn user_id(&self) -> i64 {
        return self.user_id;
    }

    pub fn level(&self) -> String {
        return self.level.clone();
    }
}

#[derive(PartialEq, Eq, Debug)]
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

pub fn build_school(
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
    pub fn id(&self) -> i64 {
        return self.id;
    }

    pub fn organization_id(&self) -> i64 {
        return self.organization_id;
    }

    pub fn super_administrator_id(&self) -> i64 {
        return self.super_administrator_id;
    }

    pub fn icon(&self) -> String {
        return self.icon.clone();
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn address1(&self) -> String {
        return self.address1.clone();
    }

    pub fn address2(&self) -> String {
        return self.address2.clone();
    }

    pub fn city(&self) -> String {
        return self.city.clone();
    }

    pub fn state(&self) -> String {
        return self.state.clone();
    }

    pub fn zip(&self) -> String {
        return self.zip.clone();
    }

    pub fn phone(&self) -> String {
        return self.phone.clone();
    }

    pub fn country(&self) -> String {
        return self.country.clone();
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct AdministratorSchool {
    admin_id: i64,
    school_id: i64,
}

pub fn build_administrator_school(admin_id: i64, school_id: i64) -> AdministratorSchool {
    AdministratorSchool {
        admin_id,
        school_id,
    }
}

impl AdministratorSchool {
    pub fn admin_id(&self) -> i64 {
        return self.admin_id;
    }

    pub fn school_id(&self) -> i64 {
        return self.school_id;
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Teacher {
    teacher_id: i64,
    user_id: i64,
}

pub fn build_teacher(teacher_id: i64, user_id: i64) -> Teacher {
    Teacher {
        teacher_id,
        user_id,
    }
}

impl Teacher {
    pub fn teacher_id(&self) -> i64 {
        return self.teacher_id;
    }

    pub fn user_id(&self) -> i64 {
        return self.user_id;
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct EmployeeSupervisor {
    id: i64,
    user_id: i64,
    administrator_id: i64,
    supervisor_name: String,
    organization_id : i64,
}

pub fn build_employee_supervisor(
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
    pub fn id(&self) -> i64 {
        return self.id;
    }

    pub fn user_id(&self) -> i64 {
        return self.user_id;
    }

    pub fn administrator_id(&self) -> i64 {
        return self.administrator_id;
    }

    pub fn supervisor_name(&self) -> String {
        return self.supervisor_name.clone();
    }

    pub fn set_supervisor_name(&mut self, name_in : String) {
        self.supervisor_name = name_in;
    }

    pub fn organization_id(&self) -> i64 {
        return self.organization_id;
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct TeacherSchool {
    teacher_id: i64,
    school_id: i64,
}

pub fn build_teacher_school(teacher_id: i64, school_id: i64) -> TeacherSchool {
    TeacherSchool {
        teacher_id,
        school_id,
    }
}

impl TeacherSchool {
    pub fn teacher_id(&self) -> i64 {
        return self.teacher_id;
    }

    pub fn school_id(&self) -> i64 {
        return self.school_id;
    }

}

#[derive(PartialEq, Eq, Debug)]
pub struct Subject {
    id: i64,
    name: String,
    ap: bool,
    ib: bool,
    target_year: String,
    discipline: String,
}

pub fn build_subject(
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
    pub fn id(&self) -> i64 {
        return self.id;
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn ap(&self) -> bool {
        return self.ap;
    }

    pub fn ib(&self) -> bool {
        return self.ib;
    }

    pub fn target_year(&self) -> String {
        return self.target_year.clone();
    }

    pub fn discipline(&self) -> String {
        return self.discipline.clone();
    }

}

#[derive(PartialEq, Eq, Debug)]
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

pub fn build_class(
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
    pub fn id(&self) -> i64 {
        return self.id;
    }

    pub fn school_id(&self) -> i64 {
        return self.school_id;
    }

    pub fn subject_id(&self) -> i64 {
        return self.subject_id;
    }

    pub fn year(&self) -> String {
        return self.year.clone();
    }

    pub fn start_day(&self) -> String {
        return self.start_day.clone();
    }

    pub fn end_day(&self) -> String {
        return self.end_day.clone();
    }

    pub fn start_time(&self) -> i32 {
        return self.start_time;
    }

    pub fn end_time(&self) -> i32 {
        return self.end_time;
    }

    pub fn days_scheduled(&self) -> String {
        return self.days_scheduled.clone();
    }

}

#[derive(PartialEq, Eq, Debug)]
pub struct TeacherClass {
    teacher_id: i64,
    class_id: i64,
    role: String,
}

pub fn build_teacher_class(teacher_id: i64, class_id: i64, role: String) -> TeacherClass {
    TeacherClass {
        teacher_id,
        class_id,
        role,
    }
}

impl TeacherClass {
    pub fn teacher_id(&self) -> i64 {
        return self.teacher_id;
    }

        pub fn class_id(&self) -> i64 {
        return self.class_id;
    }

    pub fn role(&self) -> String {
        return self.role.clone();
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Student {
    id: i64,
    user_id: i64,
}

pub fn build_student(id: i64, user_id: i64) -> Student {
    Student { id, user_id }
}

impl Student {
    pub fn id(&self) -> i64 {
        return self.id;
    }

    pub fn user_id(&self) -> i64 {
        return self.user_id;
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct StudentClass {
    student_id: i64,
    class_id: i64,
}

pub fn build_student_class(student_id: i64, class_id: i64) -> StudentClass {
    StudentClass {
        student_id,
        class_id,
    }
}

impl StudentClass {
    pub fn student_id(&self) -> i64 {
        return self.student_id;
    }

    pub fn class_id(&self) -> i64 {
        return self.class_id;
    }

}

#[derive(PartialEq, Eq, Debug)]
pub struct Family {
    family_id: i64,
    name: String,
}

pub fn build_family(family_id: i64, name: String) -> Family {
    Family { family_id, name }
}

impl Family {
    pub fn family_id(&self) -> i64 {
        return self.family_id;
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

}

#[derive(PartialEq, Eq, Debug)]
pub struct FamilyUser {
    family_id: i64,
    user_id: i64,
    relationship: String,
}

pub fn build_family_user(family_id: i64, user_id: i64, relationship: String) -> FamilyUser {
    FamilyUser {
        family_id,
        user_id,
        relationship,
    }
}

impl FamilyUser {
    pub fn family_id(&self) -> i64 {
        return self.family_id;
    }

    pub fn user_id(&self) -> i64 {
        return self.user_id;
    }

    pub fn relationship(&self) -> String {
        return self.relationship.clone();
    }

}

#[derive(PartialEq, Eq, Debug)]
pub struct FamilyMember {
    id: i64,
    user_id: i64,
    notification_methods: String,
    email: String,
    phone: String,
}

pub fn build_family_member(
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
    pub fn id(&self) -> i64 {
        return self.id;
    }

    pub fn user_id(&self) -> i64 {
        return self.user_id;
    }

    pub fn notification_methods(&self) -> String {
        return self.notification_methods.clone();
    }

    pub fn email(&self) -> String {
        return self.email.clone();
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }

    pub fn phone(&self) -> String {
        return self.phone.clone();
    }

}

#[derive(PartialEq, Eq, Debug)]
pub struct Assignment {
    id: i64,
    class_id: i64,
    required: bool,
    grade_scale: String,
    name: String,
    description: String,
    template: String,
}

pub fn build_assignment(
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
    pub fn id(&self) -> i64 {
        return self.id;
    }

    pub fn class_id(&self) -> i64 {
        return self.class_id;
    }

    pub fn required(&self) -> bool {
        return self.required;
    }

    pub fn grade_scale(&self) -> String {
        return self.grade_scale.clone();
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn description(&self) -> String {
        return self.description.clone();
    }

    pub fn template(&self) -> String {
        return self.template.clone();
    }

}

#[derive(PartialEq, Eq, Debug)]
pub struct Submission {
    id: i64,
    user_id: i64,
    assignment_id: i64,
    contents: String,
    grade: String,
}

pub fn build_submission(id: i64, user_id: i64, assignment_id: i64, contents: String, grade: String) -> Submission {
    Submission {
        id,
        user_id,
        assignment_id,
        contents,
        grade,
    }
}

impl Submission {
    pub fn id(&self) -> i64 {
        return self.id;
    }

    pub fn user_id(&self) -> i64 {
        return self.user_id;
    }

    pub fn assignment_id(&self) -> i64{
        return self.assignment_id;
    }

    pub fn contents(&self) -> String {
        return self.contents.clone();
    }

    pub fn grade(&self) -> String {
        return self.grade.clone();
    }

}

#[derive(PartialEq, Eq, Debug)]
pub struct Comment {
    id: i64,
    user_id: i64,
    assignment_id: i64,
    contents: String,
}

pub fn build_comment(id: i64, user_id: i64, assignment_id: i64, contents: String) -> Comment {
    Comment {
        id,
        user_id,
        assignment_id,
        contents,
    }
}

impl Comment {
    pub fn id(&self) -> i64 {
        return self.id;
    }

    pub fn user_id(&self) -> i64 {
        return self.user_id;
    }

    pub fn assignment_id(&self) -> i64 {
        return self.assignment_id;
    }

    pub fn contents(&self) -> String {
        return self.contents.clone();
    }

}

#[derive(PartialEq, Eq, Debug)]
pub struct ChangeLogItem {
    id: i64,
    source_name: String,
    change_type: i32,
    old_value: String,
    timestamp: String,
}

pub fn build_change_log_item(
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
    pub fn id(&self) -> i64 {
        return self.id;
    }

    pub fn source_name(&self) -> String {
        return self.source_name.clone();
    }

    pub fn change_type(&self) -> i32 {
        return self.change_type;
    }

    pub fn old_value(&self) -> String {
        return self.old_value.clone();
    }

    pub fn timestamp(&self) -> String {
        return self.timestamp.clone();
    }
}
