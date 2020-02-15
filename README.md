# Sql Procedural Macro

Proc macro for creating verbose sql statements from structs

## Usage

```rust
#[derive(Sql, Debug)]
struct MyStruct {
    #[id]
    id: i32,
    name: String,
};

let m = MyStruct {
    id: 1,
    name: "Leo".to_string(),
};
println!("{}", m.create_sql("persons", "$"));
println!("{}", m.update_sql("persons", "$"));
```

Deriving `Sql` adds two methods to the struct: `create_sql` and `update_sql`.

The parameters `tbl_name` is the name of the table you want to generate for,
`param_prefix` is the prefix for parameters for the database provider you use
i.e. "$" for postgres or "@P" for mssql.