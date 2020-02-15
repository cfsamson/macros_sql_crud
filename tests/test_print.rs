extern crate crudcreator;
use crudcreator::Sql;

#[derive(Sql, Debug)]
struct T {
    #[id]
    id: i32,
    name: String,
}
#[test]
fn test_print() {

    println!("{}", T::create_sql("persons", "$"));
    println!("{}", T::update_sql("persons", "$"));
    println!("{}", T::delete_sql("persons", "$"));
    println!("{}", T::get_by_id_sql("persons", "$"));
}
