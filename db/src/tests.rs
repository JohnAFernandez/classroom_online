use crate::db_verify::V;
use crate::db_init;
use crate::db_insert::I;
use crate::db_delete::D;
use std::path::PathBuf;


#[cfg(test)]

#[test]
fn test_database_creation_and_initialization(){
    let connection = db_init::init_database(PathBuf::from(".//src//db//test_creation.sql"));

    V::check_id(&connection, 1, V::ADMINISTRATORS);
    V::check_id(&connection, 1, V::ASSIGNMENTS);
    V::check_id(&connection, 1, V::CLASSES);
    V::check_id(&connection, 1, V::COMMENTS);
    V::check_id(&connection, 1, V::EMPLOYEES_SUPERVISORS);
    V::check_id(&connection, 1, V::FAMILIES);
    V::check_id(&connection, 1, V::FAMILY_MEMBERS);
    V::check_id(&connection, 1, V::ORGANIZATIONS);
    V::check_id(&connection, 1, V::SCHOOLS);
    V::check_id(&connection, 1, V::STUDENTS);
    V::check_id(&connection, 1, V::SUBJECTS);
    V::check_id(&connection, 1, V::SUBMISSIONS);
    V::check_id(&connection, 1, V::TEACHERS);
    V::check_id(&connection, 1, V::USERS);
    V::check_id(&connection, 1, V::USER_CHANGE_LOG);

    println!("\"Deleted\" user 1? {}", D::delete_user(&connection, 1));

    connection;
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
fn test_insertion_functions() {

    let connection = db_init::init_database(PathBuf::from(".//src//db//test_insertion.sql"));
    // do some basic testing
    I::insert_user(&connection, &"John@gmail.com".to_string(), &"JF1995".to_string(), &"password123".to_string(), &"John".to_string(), &"Fernandez".to_string(), &"01/01/2010".to_string(), &"TODAY".to_string(), &"8675309".to_string(), &"".to_string());
    
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
        &"Math for Delinquents".to_string(),
        &"0".to_string(),
        &"0".to_string(),
        &"4".to_string(),
        &"MATH".to_string(),
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

}