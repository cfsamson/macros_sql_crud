extern crate crudcreator;
use crudcreator::Sql;

#[derive(Sql, Debug)]
struct MyStruct {
    id: i32,
    name: String,
}
#[test]
fn it_works() {
    let m = MyStruct {
        id: 1,
        name: "Leo".to_string(),
    };

    println!("{:?}", m);
}
