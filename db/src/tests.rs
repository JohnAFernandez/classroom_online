use sqlite::CursorWithOwnership;

use crate::db_verify::V;
use crate::db_init;
use crate::db_insert::I;
use crate::db_delete::D;
use crate::db_retrieve::R;
use crate::db_types as types;
use crate::db_row_to_object as rto;
use crate::db_object_to_row as otr;
use std::path::PathBuf;
use std::fs;
use std::env;


#[cfg(test)]

#[test]
fn test_database_creation_insertion_retrieval(){
    env::set_var("RUST_BACKTRACE", "1");
    let location = ".//src//db//test_creation.sql";

    match fs::remove_file(location) {
        Ok(_) => (),
        Err(_) => println!("Could not delete test database {}", location),
    };

    let connection = db_init::init_database(PathBuf::from(location));

    assert!(!V::check_id(&connection, 1, V::ADMINISTRATORS));
    assert!(!V::check_id(&connection, 1, V::ASSIGNMENTS));
    assert!(!V::check_id(&connection, 1, V::CLASSES));
    assert!(!V::check_id(&connection, 1, V::COMMENTS));
    assert!(!V::check_id(&connection, 1, V::EMPLOYEES_SUPERVISORS));
    assert!(!V::check_id(&connection, 1, V::FAMILIES));
    assert!(!V::check_id(&connection, 1, V::FAMILY_MEMBERS));
    assert!(!V::check_id(&connection, 1, V::ORGANIZATIONS));
    assert!(!V::check_id(&connection, 1, V::SCHOOLS));
    assert!(!V::check_id(&connection, 1, V::STUDENTS));
    assert!(!V::check_id(&connection, 1, V::SUBJECTS));
    assert!(!V::check_id(&connection, 1, V::SUBMISSIONS));
    assert!(!V::check_id(&connection, 1, V::TEACHERS));
    assert!(!V::check_id(&connection, 1, V::USERS));
    assert!(!V::check_id(&connection, 1, V::USER_CHANGE_LOG));

    I::insert_user(&connection, &"John@gmail.com".to_string(), &"JF1995".to_string(), &"password123".to_string(), &"John".to_string(), &"Fernandez".to_string(), &"01/01/2010".to_string(), &"TODAY".to_string(), &"(305)8675309".to_string(), &"".to_string());

    let test_user = types::build_user(1, "John@gmail.com".to_string(), "JF1995".to_string(), "".to_string(), "John".to_string(), "Fernandez".to_string(), "01/01/2010".to_string(), "TODAY".to_string(), "(305)8675309".to_string(), "".to_string(), false, false);

    I::insert_organization(
        &connection,
        &"John Fernandez Schools".to_string(),
        &"123 Gandalf Way".to_string(),
        &"APT 11111".to_string(),
        &"John land".to_string(),
        &"State of John".to_string(),
        &"z22222".to_string(),
        &"(305)8675309".to_string(),
        &"Brazil".to_string(),
    );

    let old_org:types::Organization = types::build_organization(1, "John Fernandez Schools".to_string(), "123 Gandalf Way".to_string(), "APT 11111".to_string(), "John land".to_string(), "State of John".to_string(), "z22222".to_string(), "(305)8675309".to_string(), "Brazil".to_string());

    I::insert_administrator(&connection, &"1".to_string(), &"John's boss.".to_string());

    let old_admin: types::Administrator = types::build_administrator(1, 1, "John's boss.".to_string());

    I::insert_school(
        &connection,
        &"1".to_string(),
        &"0".to_string(),
        &"".to_string(),
        &"JOHN FERNANDEZ SCHOOL".to_string(),
        &"123 ABC WAY".to_string(),
        &"UNIT 1".to_string(),
        &"Albaquerque".to_string(),
        &"NZ".to_string(),
        &"z28318".to_string(),
        &"(305)8675309".to_string(),
        &"NEW ZEALAND".to_string(),
    );
    I::insert_teacher(&connection, &"1".to_string());

    I::insert_subject(
        &connection,
        "Math for Delinquents".to_string(),
        "0".to_string(),
        "0".to_string(),
        "4".to_string(),
        "MATH".to_string(),
    );
    I::insert_class(
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
    I::insert_student(&connection, &"1".to_string());
    I::insert_family(&connection, &"FERNANDEZ FAMILY".to_string());
    I::insert_family_member(
        &connection,
        &"1".to_string(),
        &"sms;email".to_string(),
        &"johnfernandez@familymembers.org".to_string(),
        &"8675309".to_string(),
    );

    I::insert_assignment(
        &connection,
        &"1".to_string(),
        &"1".to_string(),
        &"4 Point".to_string(),
        &"COUNT THE APPLES!".to_string(),
        &"Look at the picture, then figure out how many apples there are!".to_string(),
        &"".to_string(),
    );
    I::insert_submission(
        &connection,
        &"1".to_string(),
        &"1".to_string(),
        &"I'm a submission of doom!".to_string(),
        &"".to_string(),
    );
    I::insert_comments(
        &connection,
        &"1".to_string(),
        &"1".to_string(),
        &"I'm a comment of doom!".to_string(),
    );
    I::insert_change_log(
        &connection,
        &"THE APP CREATOR SINGULARITUS".to_string(),
        &"1".to_string(),
        &"I'm just testing stuff.".to_string(),
        &"Pretty late....".to_string(),
    );
    I::insert_administrator_school(&connection, &"1".to_string(), &"1".to_string());

    I::insert_employee_supervisor(
        &connection,
        &"1".to_string(),
        &"1".to_string(),
        &"John's boss Bob".to_string(),
        &"1".to_string(),
    );
    I::insert_teachers_schools(&connection, &"1".to_string(), &"1".to_string());
    I::insert_teachers_classes(
        &connection,
        &"1".to_string(),
        &"1".to_string(),
        &"Just a teacher, lol".to_string(),
    );
    I::insert_students_classes(&connection, &"1".to_string(), &"1".to_string());

    I::insert_families_users(
        &connection,
        &"1".to_string(),
        &"1".to_string(),
        &"DAH BRO GUY".to_string(),
    );

    assert!(V::check_id(&connection, 1, V::ADMINISTRATORS));
    assert!(V::check_id(&connection, 1, V::ASSIGNMENTS));
    assert!(V::check_id(&connection, 1, V::CLASSES));
    assert!(V::check_id(&connection, 1, V::COMMENTS));
    assert!(V::check_id(&connection, 1, V::EMPLOYEES_SUPERVISORS));
    assert!(V::check_id(&connection, 1, V::FAMILIES));
    assert!(V::check_id(&connection, 1, V::FAMILY_MEMBERS));
    assert!(V::check_id(&connection, 1, V::ORGANIZATIONS));
    assert!(V::check_id(&connection, 1, V::SCHOOLS));
    assert!(V::check_id(&connection, 1, V::STUDENTS));
    assert!(V::check_id(&connection, 1, V::SUBJECTS));
    assert!(V::check_id(&connection, 1, V::SUBMISSIONS));
    assert!(V::check_id(&connection, 1, V::TEACHERS));
    assert!(V::check_id(&connection, 1, V::USERS));
    assert!(V::check_id(&connection, 1, V::USER_CHANGE_LOG));


    assert!(!otr::administrator_school_to_row(&connection, types::build_administrator_school(1, 1)).0);
    assert!(!otr::administrator_school_to_row(&connection, types::build_administrator_school(1, 5)).0);
    assert!(!otr::administrator_school_to_row(&connection, types::build_administrator_school(5, 1)).0);
    assert!(!otr::administrator_school_to_row(&connection, types::build_administrator_school(10, 10)).0);


    // test user retrieval
    let mut user : types::User = types::build_user(0, "John@gmail.com".to_string(), "JF1995".to_string(), "".to_string(), "John".to_string(), "Fernandez".to_string(), "01/01/2010".to_string(), "TODAY".to_string(), "(305)8675309".to_string(), "".to_string(), false, false);
    let mut result = R::retrieve_details(&connection,R::USERS, "1".to_string());

    match result {
        Ok(x) => {for row in x.into_iter().map(|row| row.unwrap()) {
            user = rto::row_to_user(&row);
        }},
        Err(_) => panic!("Retrieval panicked."),
    }

    assert!(user == test_user);

    let mut admin : types::Administrator = types::build_administrator(0, 0, "none".to_string());
    result = R::retrieve_details(&connection,R::ADMINISTRATORS, "1".to_string());

    match result {
        Ok(x) => {for row in x.into_iter().map(|row| row.unwrap()) {
            admin = rto::row_to_administrator(&row);
        }},
        Err(_) => panic!("Retrieval panicked."),
    }

    assert!(admin == old_admin);
    
    let mut org : types::Organization = types::build_organization(0, "none".to_string(), "none".to_string(), "none".to_string(), "none".to_string(), "none".to_string(), "none".to_string(), "none".to_string(), "none".to_string());
    result = R::retrieve_details(&connection,R::ORGANIZATIONS, "1".to_string());

    match result {
        Ok(x) => {for row in x.into_iter().map(|row| row.unwrap()) {
            org = rto::row_to_organization(&row);
        }},
        Err(_) => panic!("Retrieval panicked."),
    }

    println!("{:?}\n{:?}", org, old_org);
    assert!(org == old_org);
}

#[test]
fn test_sever_level_school_and_organization_name_verification(){
    assert!(!V::check_org_school_name(&"Fish School of Doom".to_string()));
    assert!(!V::check_org_school_name(&"1".to_string()));
    assert!(!V::check_org_school_name(&"-".to_string()));
    //assert!(V::check_org_school_name(&"Red-car-o-doom".to_string()));
}

#[test]
fn test_birthday(){
    assert!(!V::check_birthday(&"1".to_string()));
    assert!(!V::check_birthday(&"1000000000".to_string()));
    assert!(!V::check_birthday(&"10/0000000".to_string()));
    assert!(!V::check_birthday(&"ab/cd/effe".to_string()));
    assert!(!V::check_birthday(&"ab/cd/fe".to_string()));
    assert!(!V::check_birthday(&"13/01/01".to_string()));
    assert!(!V::check_birthday(&"13/01/2001".to_string()));
    assert!(!V::check_birthday(&"00/01/01".to_string()));
    assert!(!V::check_birthday(&"13/01/2001".to_string()));
    assert!(!V::check_birthday(&"12/32/2001".to_string()));
    assert!(!V::check_birthday(&"11/31/2001".to_string()));
    assert!(!V::check_birthday(&"02/32/2001".to_string()));
    assert!(!V::check_birthday(&"06/32/2001".to_string()));
    assert!(V::check_birthday(&"02/29/2000".to_string()));
    assert!(!V::check_birthday(&"02/30/2000".to_string()));
    assert!(!V::check_birthday(&"02/29/2001".to_string()));
    assert!(V::check_birthday(&"02/28/2001".to_string()));
    assert!(!V::check_birthday(&"02/30/2001".to_string()));
    assert!(V::check_birthday(&"02/29/2000".to_string()));
    assert!(V::check_birthday(&"02\\01\\2000".to_string()));
    assert!(!V::check_birthday(&"02/28/2100".to_string()));
    assert!(!V::check_birthday(&"02/20/1847".to_string()));
    assert!(!V::check_birthday(&"10/20/2024".to_string()));
    assert!(V::check_birthday(&"01/20/2023".to_string()));
    assert!(V::check_birthday(&"01/31/2023".to_string()));
    assert!(!V::check_birthday(&"01/32/2023".to_string()));
    assert!(V::check_birthday(&"03/31/2023".to_string()));
    assert!(!V::check_birthday(&"03/32/2023".to_string()));
    assert!(V::check_birthday(&"05/31/2023".to_string()));
    assert!(!V::check_birthday(&"05/32/2023".to_string()));
    assert!(V::check_birthday(&"07/31/2023".to_string()));
    assert!(!V::check_birthday(&"07/32/2023".to_string()));
    assert!(V::check_birthday(&"08/31/2023".to_string()));
    assert!(!V::check_birthday(&"08/32/2023".to_string()));
    assert!(V::check_birthday(&"10/31/2023".to_string()));
    assert!(!V::check_birthday(&"10/32/2023".to_string()));
    assert!(V::check_birthday(&"12/31/2023".to_string()));
    assert!(!V::check_birthday(&"12/32/2023".to_string()));
    assert!(V::check_birthday(&"04/30/2023".to_string()));
    assert!(!V::check_birthday(&"04/31/2023".to_string()));
    assert!(V::check_birthday(&"06/30/2023".to_string()));
    assert!(!V::check_birthday(&"06/31/2023".to_string()));
    assert!(V::check_birthday(&"09/30/2023".to_string()));
    assert!(!V::check_birthday(&"09/31/2023".to_string()));
    assert!(V::check_birthday(&"11/30/2023".to_string()));
    assert!(!V::check_birthday(&"11/31/2023".to_string()));
    assert!(!V::check_birthday(&"04/00/2023".to_string()));
    assert!(!V::check_birthday(&"04/05/0000".to_string()));
}


#[test]
fn test_sever_level_email_verification (){
    // Things that are supposed to fail
    assert!(!V::check_email("animal".to_string()));
    assert!(!V::check_email("animal@".to_string()));
    assert!(!V::check_email("@animal".to_string()));
    assert!(!V::check_email("@animal.com".to_string()));
    assert!(!V::check_email("animal@h".to_string()));
    assert!(!V::check_email("@".to_string()));
    assert!(!V::check_email("".to_string()));
    assert!(!V::check_email("b.e.s.t@abc.com.".to_string()));
    assert!(!V::check_email("fine@ab@c.com.".to_string()));
    assert!(!V::check_email("good.animal@bob.".to_string()));
    assert!(!V::check_email("good.animal@.com".to_string()));

    // Things that are supposed to pass.
    assert!(V::check_email("good.animal@bob.com".to_string()));
    assert!(V::check_email("animal@bob.com".to_string()));
    assert!(V::check_email("good.animal@bob.y".to_string()));
    assert!(V::check_email("good.animal@a.com".to_string()));
    assert!(V::check_email("b.e.s.t@abc.com".to_string()));
}

#[test]
fn test_sever_level_name_verification() {
    // Things that should pass
    assert!(V::check_name(&"Turambar".to_string()));    
    assert!(V::check_name(&"Bob".to_string()));

    // Things that should fail
    assert!(!V::check_name(&"".to_string()));
    assert!(!V::check_name(&"Wolfeschlegel­steinhausen­bergerdorff­welche­vor­altern­waren­gewissenhaft­schafers­wessen­schafe­waren­wohl­gepflege­und­sorgfaltigkeit­beschutzen­vor­angreifen­durch­ihr­raubgierig­feinde­welche­vor­altern­zwolfhundert­tausend­jahres­voran­die­erscheinen­von­der­erste­erdemensch­der­raumschiff­genacht­mit­tungstein­und­sieben­iridium­elektrisch­motors­gebrauch­licht­als­sein­ursprung­von­kraft­gestart­sein­lange­fahrt­hinzwischen­sternartig­raum­auf­der­suchen­nachbarschaft­der­stern­welche­gehabt­bewohnbar­planeten­kreise­drehen­sich­und­wohin­der­neue­rasse­von­verstandig­menschlichkeit­konnte­fortpflanzen­und­sich­erfreuen­an­lebenslanglich­freude­und­ruhe­mit­nicht­ein­furcht­vor­angreifen­vor­anderer­intelligent­geschopfs­von­hinzwischen­sternartig­raum".to_string()));
    assert!(!V::check_name(&"123".to_string()));
    assert!(!V::check_name(&"Abby1".to_string()));
    assert!(!V::check_name(&"!!BOB!!".to_string()));
    assert!(!V::check_name(&"!!!!".to_string()));
}

#[test]
fn test_deletion() {

    let location = ".//src//db//test_data_updates.sql";

    match fs::remove_file(location) {
        Ok(_) => (),
        Err(_) => println!("Could not delete test database {}", location),
    };

    let connection = db_init::init_database(PathBuf::from(location));
    // do some basic testing
    I::insert_user(&connection, &"John@gmail.com".to_string(), &"JF1995".to_string(), &"password123".to_string(), &"John".to_string(), &"Fernandez".to_string(), &"01/01/2010".to_string(), &"TODAY".to_string(), &"8675309".to_string(), &"".to_string());

    I::insert_user(&connection, &"John2@gmail.com".to_string(), &"JF1999".to_string(), &"password123".to_string(), &"John".to_string(), &"Fernandez".to_string(), &"01/01/2010".to_string(), &"TODAY".to_string(), &"8675309".to_string(), &"".to_string());

    I::insert_organization(
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
    I::insert_administrator(&connection, &"1".to_string(), &"John's boss.".to_string());
    I::insert_school(
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
    I::insert_teacher(&connection, &"1".to_string());

    I::insert_subject(
        &connection,
        "Math for Delinquents".to_string(),
        "0".to_string(),
        "0".to_string(),
        "4".to_string(),
        "MATH".to_string(),
    );
    I::insert_class(
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
    I::insert_student(&connection, &"1".to_string());
    I::insert_family(&connection, &"FERNANDEZ FAMILY".to_string());
    I::insert_family_member(
        &connection,
        &"1".to_string(),
        &"sms;email".to_string(),
        &"johnfernandez@familymembers.org".to_string(),
        &"8675309".to_string(),
    );

    I::insert_assignment(
        &connection,
        &"1".to_string(),
        &"1".to_string(),
        &"4 Point".to_string(),
        &"COUNT THE APPLES!".to_string(),
        &"Look at the picture, then figure out how many apples there are!".to_string(),
        &"".to_string(),
    );
    I::insert_submission(
        &connection,
        &"1".to_string(),
        &"1".to_string(),
        &"I'm a submission of doom!".to_string(),
        &"".to_string(),
    );
    I::insert_comments(
        &connection,
        &"1".to_string(),
        &"1".to_string(),
        &"I'm a comment of doom!".to_string(),
    );
    I::insert_change_log(
        &connection,
        &"THE APP CREATOR SINGULARITUS".to_string(),
        &"1".to_string(),
        &"I'm just testing stuff.".to_string(),
        &"Pretty late....".to_string(),
    );
    I::insert_administrator_school(&connection, &"1".to_string(), &"1".to_string());

    I::insert_employee_supervisor(
        &connection,
        &"1".to_string(),
        &"1".to_string(),
        &"John's boss Bob".to_string(),
        &"1".to_string(),
    );
    I::insert_teachers_schools(&connection, &"1".to_string(), &"1".to_string());
    I::insert_teachers_classes(
        &connection,
        &"1".to_string(),
        &"1".to_string(),
        &"Just a teacher, lol".to_string(),
    );
    I::insert_students_classes(&connection, &"1".to_string(), &"1".to_string());

    I::insert_families_users(
        &connection,
        &"1".to_string(),
        &"1".to_string(),
        &"DAH BRO GUY".to_string(),
    );

    // This user has accounts associated
    let mut result = D::delete_user(&connection, 1);
    //assert!(!result.0, "{}", result.1);

    // This user does not
    result = D::delete_user(&connection, 2);
    assert!(result.0, "{}", result.1);

    // This user should not exist
    result = D::delete_user(&connection, 3);
    assert!(!result.0, "{}", result.1);
    
    I::insert_user(&connection, &"John3@gmail.com".to_string(), &"JF2005".to_string(), &"password123".to_string(), &"John".to_string(), &"Fernandez".to_string(), &"01/01/2010".to_string(), &"TODAY".to_string(), &"8675309".to_string(), &"".to_string());
    I::insert_administrator(&connection, &"3".to_string(), &"Necromancer".to_string());
    
    result = D::delete_administrator(&connection, 2);
    assert!(result.0, "{}", result.1);

    I::insert_school(
        &connection,
        &"1".to_string(),
        &"1".to_string(),
        &"".to_string(),
        &"JOHN FERNANDEZ OTHER SCHOOL".to_string(),
        &"1234 BBQ ST".to_string(),
        &"UNIT 18".to_string(),
        &"Albaquerque".to_string(),
        &"NZ".to_string(),
        &"12345".to_string(),
        &"8675309".to_string(),
        &"NEW ZEALAND".to_string(),
    );

    // this school has no associated classes, and should be deletable
    result = D::delete_school(&connection, 2);
    assert!(result.0, "{}", result.1);
    
    // this school does not exist so deletion should fail.
    result = D::delete_school(&connection, 4);
    assert!(!result.0, "{}", result.1);


    I::insert_organization(
        &connection,
        &"John Fernandez's Other Schools".to_string(),
        &"1234 Gandalf Way".to_string(),
        &"APT 111".to_string(),
        &"John landIO".to_string(),
        &"State of BOB".to_string(),
        &"22222".to_string(),
        &"8675309".to_string(),
        &"Brazil".to_string(),
    );


    result = D::delete_organization(&connection, 2);
    assert!(result.0, "{}", result.1);

    result = D::delete_organization(&connection, 4);
    assert!(!result.0, "{}", result.1);


    I::insert_teacher(&connection, &"2".to_string());

    result = D::delete_teacher(&connection, 2);
    assert!(result.0, "{}", result.1);

    result = D::delete_teacher(&connection, 4);
    assert!(!result.0, "{}", result.1);

}

