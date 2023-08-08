use sqlite


fn main() {

    let mut connection = init_database();

    while true {

        break;
    }

}

fn init_database() -> sqlite::Connection{
    let connection = sqlite::open(":memory:").unwrap();

let query = "
    CREATE TABLE users (name TEXT, age INTEGER);
    INSERT INTO users VALUES ('Alice', 42);
    INSERT INTO users VALUES ('Bob', 69);
";
connection.execute(query).unwrap();
}