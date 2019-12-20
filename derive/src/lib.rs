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

    let field_idents: Vec<Ident> = gen::get_field_idents(fields);

    let common_functions = gen::common_functions();
    let query_builder_fn = gen::new_query_builder_fn(&builder_ident);
    let query_builder_struct = gen::query_builder_struct(&builder_ident);
    let query_field_fn = gen::query_field_fn(&field_idents);
    let range_query_field_fn = gen::range_query_field_fn(&field_idents);

    let output = quote! {
        use lucene_query_builder::*;

        #query_builder_struct

        impl QueryBuilder for #ident {
            type Output = #builder_ident;
            #query_builder_fn
        }

        impl QueryBuilderImpl for #builder_ident {
            fn build(&self) -> String {
                self.build()
            }
        }

        impl #builder_ident {
            #query_field_fn
            #range_query_field_fn
            #common_functions
        }
    };

    TokenStream::from(output)
}
