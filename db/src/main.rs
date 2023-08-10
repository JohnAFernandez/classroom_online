use sqlite;
mod db_init;
mod db_insert;
mod db_verify;

fn main() {
    // establish our database
    let connection = db_init::init_database();

    test_insertion_functions(&connection);

    db_verify::V::check_id(&connection, 1, db_verify::V::ADMINISTRATORS);
    db_verify::V::check_id(&connection, 1, db_verify::V::ASSIGNMENTS);
    db_verify::V::check_id(&connection, 1, db_verify::V::CLASSES);
    db_verify::V::check_id(&connection, 1, db_verify::V::COMMENTS);
    db_verify::V::check_id(&connection, 1, db_verify::V::EMPLOYEES_SUPERVISORS);
    db_verify::V::check_id(&connection, 1, db_verify::V::FAMILIES);
    db_verify::V::check_id(&connection, 1, db_verify::V::FAMILY_MEMBERS);
    db_verify::V::check_id(&connection, 1, db_verify::V::ORGANIZATIONS);
    db_verify::V::check_id(&connection, 1, db_verify::V::SCHOOLS);
    db_verify::V::check_id(&connection, 1, db_verify::V::STUDENTS);
    db_verify::V::check_id(&connection, 1, db_verify::V::SUBJECTS);
    db_verify::V::check_id(&connection, 1, db_verify::V::SUBMISSIONS);
    db_verify::V::check_id(&connection, 1, db_verify::V::TEACHERS);
    db_verify::V::check_id(&connection, 1, db_verify::V::USERS);
    db_verify::V::check_id(&connection, 1, db_verify::V::USER_CHANGE_LOG);
    if cfg!(debug_assertions){
        test_sever_level_email_verification();
    }

    //    loop {
    //       break;
    //    }

    println!("Connection listener for app section not yet implemented.  Aborting. ")
}

fn test_insertion_functions(connection: &sqlite::Connection) {
    // do some basic testing
    db_insert::I::insert_user(
        &connection,
        &"John@gmail.com".to_string(),
        &"JF1995".to_string(),
        &"password123".to_string(),
        &"John".to_string(),
        &"Fernandez".to_string(),
        &"01/01/2010".to_string(),
        &"TODAY".to_string(),
        &"8675309".to_string(),
        &"".to_string(),
    );
    db_insert::I::insert_organization(
        &connection,
        &"John Fernandez Schools".to_string(),
        &"123 Gandalf Way".to_string(),
        &"APT 11111".to_string(),
        &"John land".to_string(),
        &"State of John".to_string(),
        &"22222".to_string(),
        &"8675309".to_string(),
        &"Brazil".to_string(),
    );
    db_insert::I::insert_administrator(&connection, &"1".to_string(), &"John's boss.".to_string());
    db_insert::I::insert_school(
        &connection,
        &"1".to_string(),
        &"0".to_string(),
        &"".to_string(),
        &"JOHN FERNANDEZ SCHOOL".to_string(),
        &"123 ABC WAY".to_string(),
        &"UNIT 1".to_string(),
        &"Albaquerque".to_string(),
        &"NZ".to_string(),
        &"28318".to_string(),
        &"8675309".to_string(),
        &"NEW ZEALAND".to_string(),
    );
    db_insert::I::insert_teacher(&connection, &"1".to_string());

    db_insert::I::insert_subject(
        &connection,
        &"Math for Delinquents".to_string(),
        &"0".to_string(),
        &"0".to_string(),
        &"4".to_string(),
        &"MATH".to_string(),
    );
    db_insert::I::insert_class(
        &connection,
        &"1".to_string(),
        &"1".to_string(),
        &"7".to_string(),
        &"49".to_string(),
        &"99".to_string(),
        &"102".to_string(),
        &"110".to_string(),
        &"MWF".to_string(),
    );
    db_insert::I::insert_student(&connection, &"1".to_string());
    db_insert::I::insert_family(&connection, &"FERNANDEZ FAMILY".to_string());
    db_insert::I::insert_family_member(
        &connection,
        &"1".to_string(),
        &"sms;email".to_string(),
        &"johnfernandez@familymembers.org".to_string(),
        &"8675309".to_string(),
    );

    db_insert::I::insert_assignment(
        &connection,
        &"1".to_string(),
        &"1".to_string(),
        &"4 Point".to_string(),
        &"COUNT THE APPLES!".to_string(),
        &"".to_string(),
    );
    db_insert::I::insert_submission(
        &connection,
        &"1".to_string(),
        &"1".to_string(),
        &"I'm a submission of doom!".to_string(),
        &"".to_string(),
    );
    db_insert::I::insert_comments(
        &connection,
        &"1".to_string(),
        &"1".to_string(),
        &"I'm a comment of doom!".to_string(),
    );
    db_insert::I::insert_change_log(
        &connection,
        &"THE APP CREATOR SINGULARITUS".to_string(),
        &"1".to_string(),
        &"I'm just testing stuff.".to_string(),
        &"Pretty late....".to_string(),
    );
    db_insert::I::insert_administrator_school(&connection, &"1".to_string(), &"1".to_string());

    db_insert::I::insert_employee_supervisor(
        &connection,
        &"1".to_string(),
        &"1".to_string(),
        &"John's boss Bob".to_string(),
    );
    db_insert::I::insert_teachers_schools(&connection, &"1".to_string(), &"1".to_string());
    db_insert::I::insert_teachers_classes(
        &connection,
        &"1".to_string(),
        &"1".to_string(),
        &"Just a teacher, lol".to_string(),
    );
    db_insert::I::insert_students_classes(&connection, &"1".to_string(), &"1".to_string());

    db_insert::I::insert_families_users(
        &connection,
        &"1".to_string(),
        &"1".to_string(),
        &"DAH BRO GUY".to_string(),
    );
}

fn test_sever_level_email_verification (){
    if db_verify::V::check_email("animal".to_string()){
        println!("Warning! animal passes" );
    }

    if db_verify::V::check_email("animal@".to_string()){
        println!("Warning! animal@ passes" );
    }

    if db_verify::V::check_email("@animal".to_string()){
        println!("Warning! @animal passes" );
    }

    if db_verify::V::check_email("@animal.com".to_string()){
        println!("Warning! @animal.com passes" );
    }

    if db_verify::V::check_email("animal@h".to_string()){
        println!("Warning! animal@h passes" );
    }

    if db_verify::V::check_email("@".to_string()){
        println!("@ passes" );
    } else {
        println!("@ fails");
    }

    if db_verify::V::check_email("".to_string()){
        println!("empty string passes" );
    } else {
        println!("empty string fails");
    }

    if db_verify::V::check_email("good.animal@bob.com".to_string()){
        println!("good.animal@bob.com passes" );
    } else {
        println!("good.animal@bob.com fails");
    }

    if db_verify::V::check_email("animal@bob.com".to_string()){
        println!("animal@bob.com passes" );
    } else {
        println!("animal@bob.com fails");
    }
    
    if db_verify::V::check_email("good.animal@bob.".to_string()){
        println!("good_animal@bob. passes" );
    } else {
        println!("good_animal@bob. fails");
    }

    if db_verify::V::check_email("good.animal@bob.y".to_string()){
        println!("good_animal@bob.y passes" );
    } else {
        println!("good_animal@bob.y fails");
    }

    if db_verify::V::check_email("good.animal@.com".to_string()){
        println!("good_animal@.com passes" );
    } else {
        println!("good_animal@.com fails");
    }

    if db_verify::V::check_email("good.animal@a.com".to_string()){
        println!("good_animal@a.com passes" );
    } else {
        println!("good_animal@a.com fails");
    }

    if db_verify::V::check_email("b.e.s.t@abc.com".to_string()){
        println!("b.e.s.t@abc.com passes" );
    } else {
        println!("b.e.s.t@abc.com fails");
    }

    if db_verify::V::check_email("b.e.s.t@abc.com.".to_string()){
        println!("b.e.s.t@abc.com. passes" );
    } else {
        println!("b.e.s.t@abc.com. fails");
    }

    if db_verify::V::check_email("fine@ab@c.com.".to_string()){
        println!("b.e.s.t@abc.com. passes" );
    } else {
        println!("b.e.s.t@abc.com. fails");
    }

}