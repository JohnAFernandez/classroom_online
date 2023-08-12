use crate::db_verify::V;

#[cfg(test)]

#[test]
fn test_server_level_name_and_birthday(){

    V::check_org_school_name(&"Fish School of Doom".to_string());
    V::check_org_school_name(&"1".to_string());

    V::check_birthday(&"1".to_string());
    V::check_birthday(&"1000000000".to_string());
    V::check_birthday(&"10/0000000".to_string());
    V::check_birthday(&"ab/cd/effe".to_string());
    V::check_birthday(&"ab/cd/fe".to_string());
    V::check_birthday(&"13/01/01".to_string());
    V::check_birthday(&"13/01/2001".to_string());
    V::check_birthday(&"00/01/01".to_string());
    V::check_birthday(&"13/01/2001".to_string());
    V::check_birthday(&"12/32/2001".to_string());
    V::check_birthday(&"11/31/2001".to_string());
    V::check_birthday(&"02/32/2001".to_string());
    V::check_birthday(&"06/32/2001".to_string());
    V::check_birthday(&"02/29/2000".to_string());
    V::check_birthday(&"02/30/2000".to_string());
    V::check_birthday(&"02/29/2001".to_string());
    V::check_birthday(&"02/28/2001".to_string());
    V::check_birthday(&"02/30/2001".to_string());
    V::check_birthday(&"02/29/2000".to_string());
    V::check_birthday(&"02\\01\\2000".to_string());
    V::check_birthday(&"02/28/2100".to_string());
    V::check_birthday(&"02/20/1847".to_string());
    V::check_birthday(&"01/20/2024".to_string());
    V::check_birthday(&"01/20/2023".to_string());
    V::check_birthday(&"01/31/2023".to_string());
    V::check_birthday(&"01/32/2023".to_string());
    V::check_birthday(&"03/31/2023".to_string());
    V::check_birthday(&"03/32/2023".to_string());
    V::check_birthday(&"05/31/2023".to_string());
    V::check_birthday(&"05/32/2023".to_string());
    V::check_birthday(&"07/31/2023".to_string());
    V::check_birthday(&"07/32/2023".to_string());
    V::check_birthday(&"08/31/2023".to_string());
    V::check_birthday(&"08/32/2023".to_string());
    V::check_birthday(&"10/31/2023".to_string());
    V::check_birthday(&"10/32/2023".to_string());
    V::check_birthday(&"12/31/2023".to_string());
    V::check_birthday(&"12/32/2023".to_string());
    V::check_birthday(&"04/30/2023".to_string());
    V::check_birthday(&"04/31/2023".to_string());
    V::check_birthday(&"06/30/2023".to_string());
    V::check_birthday(&"06/31/2023".to_string());
    V::check_birthday(&"09/30/2023".to_string());
    V::check_birthday(&"09/31/2023".to_string());
    V::check_birthday(&"11/30/2023".to_string());
    V::check_birthday(&"11/31/2023".to_string());
    V::check_birthday(&"04/00/2023".to_string());
    V::check_birthday(&"04/05/0000".to_string());
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
    
    assert!(V::check_name(&"Bob".to_string()));
    assert!(!V::check_name(&"".to_string()));
    assert!(!V::check_name(&"Wolfeschlegel­steinhausen­bergerdorff­welche­vor­altern­waren­gewissenhaft­schafers­wessen­schafe­waren­wohl­gepflege­und­sorgfaltigkeit­beschutzen­vor­angreifen­durch­ihr­raubgierig­feinde­welche­vor­altern­zwolfhundert­tausend­jahres­voran­die­erscheinen­von­der­erste­erdemensch­der­raumschiff­genacht­mit­tungstein­und­sieben­iridium­elektrisch­motors­gebrauch­licht­als­sein­ursprung­von­kraft­gestart­sein­lange­fahrt­hinzwischen­sternartig­raum­auf­der­suchen­nachbarschaft­der­stern­welche­gehabt­bewohnbar­planeten­kreise­drehen­sich­und­wohin­der­neue­rasse­von­verstandig­menschlichkeit­konnte­fortpflanzen­und­sich­erfreuen­an­lebenslanglich­freude­und­ruhe­mit­nicht­ein­furcht­vor­angreifen­vor­anderer­intelligent­geschopfs­von­hinzwischen­sternartig­raum".to_string()));
    assert!(!V::check_name(&"123".to_string()));
    assert!(!V::check_name(&"Abby1".to_string()));
    assert!(V::check_name(&"Turambar".to_string()));
    assert!(!V::check_name(&"!!BOB!!".to_string()));
    assert!(!V::check_name(&"!!!!".to_string()));

}

