# Sql Procedural Macro

Proc macro for creating verbose sql statements from structs

## Usage

```rust
#[derive(Sql, Debug)]
struct T {
    #[id]
    id: i32,
    name: String,
};

assert_eq!(T::create_sql("persons", "$"), "INSERT INTO persons (id, name) VALUES ($1,$2);");
assert_eq!(T::update_sql("persons", "$"), "UPDATE persons SET (id = $1, name = $2);");
assert_eq!(T::delete_sql("persons", "$"), "DELETE FROM persons WHERE id = $1;");
assert_eq!(T::get_by_id_sql("persons", "$"), "SELECT id, name FROM persons WHERE id = $1;");
```

Deriving `Sql` adds two methods to the struct: `create_sql` and `update_sql`.

The parameters `tbl_name` is the name of the table you want to generate for,
`param_prefix` is the prefix for parameters for the database provider you use
i.e. "$" for postgres or "@P" for mssql.