use sqlite;
mod db_init;
mod db_insert;

fn main() {
    // establish our database
    let connection = db_init::init_database();

    // do some basic testing
    db_insert::I::insert_user(&connection, &"John@gmail.com".to_string(),  &"JF1995".to_string(), &"password123".to_string(), &"John".to_string(), &"Fernandez".to_string(), &"01/01/2010".to_string(), &"TODAY".to_string(), &"8675309".to_string(), &"".to_string());
    db_insert::I::insert_organization(&connection, &"John Fernandez Schools".to_string(),  &"123 Gandalf Way".to_string(), &"APT 11111".to_string(), &"John land".to_string(), &"State of John".to_string(), &"22222".to_string(), &"8675309".to_string(), &"Brazil".to_string());
    db_insert::I::insert_administrator(&connection, &"1".to_string(), &"John's boss.".to_string());
    db_insert::I::insert_school(&connection, &"1".to_string(), &"0".to_string(), &"".to_string(), &"JOHN FERNANDEZ SCHOOL".to_string(), &"123 ABC WAY".to_string(), &"UNIT 1".to_string(), &"Albaquerque".to_string(), &"NZ".to_string(), &"28318".to_string(), &"8675309".to_string(), &"NEW ZEALAND".to_string());
    db_insert::I::insert_teacher(&connection, &"1".to_string());

    db_insert::I::insert_subject(&connection, &"Math for Delinquents".to_string(), &"0".to_string(), &"0".to_string(), &"4".to_string(), &"MATH".to_string());
    db_insert::I::insert_class(&connection, &"1".to_string(), &"1".to_string(), &"7".to_string(), &"49".to_string(), &"99".to_string(), &"102".to_string(), &"110".to_string(), &"MWF".to_string());
    db_insert::I::insert_student(&connection, &"1".to_string());
    db_insert::I::insert_family(&connection, &"FERNANDEZ FAMILY".to_string());
    db_insert::I::insert_family_member(&connection, &"1".to_string(), &"sms;email".to_string(), &"johnfernandez@familymembers.org".to_string(), &"8675309".to_string());

    db_insert::I::insert_assignment(&connection, &"1".to_string(), &"1".to_string(),&"4 Point".to_string(),&"COUNT THE APPLES!".to_string(),&"".to_string());
    
//    loop {
//       break;
//    }

    println!("Connection listener for app section not yet implemented.  Aborting. ")
}
