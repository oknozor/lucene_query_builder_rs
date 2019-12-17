extern crate proc_macro;

mod gen;

use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Ident};

#[proc_macro_derive(QueryBuilder, attributes(query_builder_ignore, query_builder_rename))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;
    let builder_ident = format_ident!("{}{}", ident, "LuceneQueryBuilder");

    let fields: Vec<Field> = match input.data {
        Data::Struct(struct_) => match struct_.fields {
            Fields::Named(n) => n.named.iter().cloned().collect(),
            Fields::Unnamed(_) => panic!("Unexpected unnamed field"),
            Fields::Unit => panic!("Unexpected unit"),
        },
        Data::Enum(_) => panic!("The Builder macro is not to be used on enum"),
        Data::Union(_) => panic!("The Builder macro is not to be used on union"),
    };

    let field_idents: Vec<Ident> = fields
        .iter()
        .cloned()
        .filter(|field| gen::get_non_ignored_fields(field))
        .map(|field| field.ident.unwrap())
        .collect();

    let common_functions = gen::common_functions();
    let query_builder_fn = gen::query_builder_fn(ident, &builder_ident);
    let query_builder = gen::query_builder(&builder_ident);
    let query_field_fn = gen::query_field_fn(&field_idents);
    let range_query_field_fn = gen::range_query_field_fn(&field_idents);
    let prelude = gen::prelude();

    let output = quote! {

        #prelude
        #query_builder_fn
        #query_builder

        impl #builder_ident {
            #query_field_fn
            #range_query_field_fn
            #common_functions
        }
    };

    TokenStream::from(output)
}
