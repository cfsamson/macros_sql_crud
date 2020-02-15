//! Proc macro for creating verbose sql statements from structs
//! 
//! ## Usage
//! 
//! ```rust
//! # extern crate crudcreator;
//! use crudcreator::Sql;
//! 
//! #[derive(Sql)]
//! struct T {
//!     #[id]
//!     id: i32,
//!     name: String,
//! };
//! 
//! assert_eq!(T::create_sql("persons", "$"), "INSERT INTO persons (id, name) VALUES ($1,$2);");
//! assert_eq!(T::update_sql("persons", "$"), "UPDATE persons SET (id = $1, name = $2);");
//! assert_eq!(T::delete_sql("persons", "$"), "DELETE FROM persons WHERE id = $1;");
//! assert_eq!(T::get_by_id_sql("persons", "$"), "SELECT id, name FROM persons WHERE id = $1;");
//! ```
//! 
//! Deriving `Sql` adds two methods to the struct: `create_sql` and `update_sql`.
//! 
//! The parameters `tbl_name` is the name of the table you want to generate for,
//! `param_prefix` is the prefix for parameters for the database provider you use
//! i.e. "$" for postgres or "@P" for mssql.


extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned, format_ident};
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index, TypePath, Type};
use std::fmt::Write;

#[proc_macro_derive(Sql, attributes(id))]
pub fn sql(input: TokenStream) -> TokenStream {

    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let data = match input.data {
        Data::Struct(ds) => ds,
        _ => panic!("must be a struct"),
    };

    let fields = match data.fields {
        Fields::Named(named) => named,
        _ => panic!("must have named fields"),
    };

    let mut struct_fields: Vec<String> = vec![];
    let mut id_field: Option<(String, Type)> = None;

    for field in fields.named {
        if let Some(ident) = field.ident {
            let field_name = format!("{}", ident);

            let is_id = field.attrs.iter().any(|attr| {
                if let Some(ident) = attr.path.get_ident() {
                    let ident = format!("{}", ident);
                    if &ident == "id" {
                        return true;
                    }
                }
                false
            });



            if is_id {
                //let fieldtype = get_type(&field.ty);
                match &mut id_field {
                    None => id_field = Some((field_name.clone(),field.ty.clone())),
                    Some(_) => panic!("Can't have more than one id field."),
                };
            }

            struct_fields.push(field_name);
        }
    }

    // We got all the fields and types, now create the impls
    let mut sql_field_list = String::new();
    for sf in &struct_fields {
        write!(sql_field_list, "{}, ", sf).ok();
    }
    sql_field_list.pop(); // " "
    sql_field_list.pop(); // ","


    let fields_count = struct_fields.len();
    let create = quote!{
            fn create_sql(tbl_name: &str, param_prefix: &str) -> String {
                use std::fmt::Write;
                let mut s = String::new();
                write!(s, "INSERT INTO {} ({}) VALUES (", tbl_name, #sql_field_list).ok();
                for i in 1..#fields_count + 1 {
                    write!(s, "{}{},", param_prefix, i).ok();
                }
                s.pop(); // ","
                write!(s, ");").ok();
                s
            }
    };

    let update = quote!{
            fn update_sql(tbl_name: &str, param_prefix: &str) -> String {
                use std::fmt::Write;
                let mut s = String::new();
                write!(s, "UPDATE {} SET (", tbl_name).ok();
                // array of fields
                let fields = &[#(#struct_fields),*];
                for i in 1..#fields_count + 1 {
                    write!(s, "{} = {}{}, ", fields[i - 1], param_prefix, i).ok();
                }
                s.pop(); // " "
                s.pop(); // ","
                write!(s, ");").ok();
                s
            }
    };

    let (id_field, id_type) = match id_field {
        Some(id_tuple) => id_tuple,
        None => panic!("Must have an #[id] field."),
    };

    let delete = quote!{
        fn delete_sql(tbl_name: &str, param_prefix: &str) -> String {
            use std::fmt::Write;
            let mut s = String::new();
            write!(s, "DELETE FROM {} ", tbl_name).ok();
            write!(s, "WHERE {} = {}1;", #id_field, param_prefix).ok();
            s
        }
    };

    let get_by_id = quote!{
            fn get_by_id_sql(tbl_name: &str, param_prefix: &str) -> String {
                use std::fmt::Write;
                let mut s = String::new();
                write!(s, "SELECT {} FROM {} ", #sql_field_list, tbl_name).ok();
                write!(s, "WHERE {} = {}1;", #id_field, param_prefix).ok();
                s
            }
    };

    let ts = quote!{
        impl #name {
            #create

            #update

            #delete

            #get_by_id
        }
    };

    TokenStream::from(ts)
}

fn get_type(ty: &Type) -> String {
    let path = match ty {
        Type::Path(path) => path,
        _ => panic!("Unsupported type"),
    };


    let id = path.path.segments.last().unwrap();

    format!("{}", id.ident)
}
