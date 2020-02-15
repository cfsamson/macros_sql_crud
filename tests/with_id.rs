extern crate crudcreator;
use crudcreator::Sql;

#[derive(Sql, Debug)]
struct MyStruct {
    #[id]
    id: i32,
    name: String,
}
#[test]
fn it_works() {
    let m = MyStruct {
        id: 1,
        name: "Leo".to_string(),
    };

    println!("{}", m.create_sql("persons", "$"));
    println!("{}", m.update_sql("persons", "$"));
}
