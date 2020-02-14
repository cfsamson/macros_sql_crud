
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index, TypePath, Type};

use proc_macro2;

#[proc_macro_derive(Sql)]
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

    let mut struct_fields = vec![];
    for field in fields.named {
        if let Some(ident) = field.ident {
            let field_name = format!("{}", ident);
            let field_type = get_type(field.ty);

            struct_fields.push(StructField::new(field_name, field_type));
        }
    }

    // We got all the fields and types, now create the impls

    // Remember we actually dont need types... We impl this for params 

    println!("{:?}", struct_fields);
    let ts = quote!{
        struct Hello { 
            id: String,
        }
    };

    TokenStream::from(ts)
}

fn get_type(ty: Type) -> String {
    let path = match ty {
        Type::Path(path) => path,
        _ => panic!("Unsupported type"),
    };


    let id = path.path.segments.last().unwrap();

    format!("{}", id.ident)
}


#[derive(Debug)]
struct StructField {
    name: String,
    kind: String,
}

impl StructField {
    fn new(name: String, kind: String) -> Self {
        StructField {name, kind}
    }
}
