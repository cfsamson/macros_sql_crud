extern crate crudcreator;
use crudcreator::Sql;

#[derive(Sql, Debug)]
struct T {
    #[id]
    id: i32,
    name: String,
}
#[test]
fn bacics() {
    assert_eq!(T::create_sql("persons", "$"), "INSERT INTO persons (id, name) VALUES ($1,$2);");
    assert_eq!(T::update_sql("persons", "$"), "UPDATE persons SET (id = $1, name = $2);");
    assert_eq!(T::delete_sql("persons", "$"), "DELETE FROM persons WHERE id = $1;");
    assert_eq!(T::get_by_id_sql("persons", "$"), "SELECT id, name FROM persons WHERE id = $1;");
}
