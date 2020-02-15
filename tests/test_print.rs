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
        name: "Abe".to_string(),
    };

    println!("{}", m.create_sql("persons", "$"));
    println!("{}", m.update_sql("persons", "$"));
    println!("{}", m.delete_sql("persons", "$"));
    println!("{}", m.get_by_id_sql("persons", "$"));
}
