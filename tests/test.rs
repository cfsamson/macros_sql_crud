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

    assert_eq!(&m.create_sql("persons", "$"), "INSERT INTO persons (id, name) VALUES ($1,$2);");

    assert_eq!(m.update_sql("persons", "$"), "UPDATE persons SET (id = $1, name = $2);");

    assert_eq!(&m.delete_sql("persons", "$"), "DELETE FROM persons WHERE id = $1;");

    assert_eq!(&m.get_by_id_sql("persons", "$"), "SELECT id, name FROM persons WHERE id = $1;");
}
