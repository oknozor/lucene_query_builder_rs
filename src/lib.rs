extern crate proc_macro;

use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Field, Fields, Ident, ParenthesizedGenericArguments,
    PathArguments,
};

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
        .map(|field| {
            let renamed = field.attrs.iter().find(|attr| {
                format!("{}", attr.path.segments.first().unwrap().ident) == "query_builder_rename"
            });

            if let Some(name) = renamed {
                if let PathArguments::Parenthesized(arg) =
                    &name.path.segments.first().unwrap().arguments
                {
                    println!("SOMETHING HERE");
                };
            }
            field
        })
        .filter(|field| {
            !field
                .attrs
                .iter()
                .map(|attr| format!("{}", attr.path.segments.first().unwrap().ident))
                .collect::<String>()
                .contains("query_builder_ignore")
        })
        .map(|field| field.ident.unwrap())
        .collect();

    let field_idents_range: Vec<Ident> = field_idents
        .iter()
        .cloned()
        .map(|name| format_ident!("{}{}", name, "_range"))
        .collect();

    let field_idents_str: Vec<String> = field_idents
        .iter()
        .cloned()
        .map(|name| format!("{}", name))
        .collect();

    let output = quote! {
        use std::fmt;

        enum Operator {
            Or,
            And,
            End
        }

        struct QueryString(pub String);

        impl fmt::Display for QueryString {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if self.0.contains(" ") {
                    write!(f, "\"{}\"", self.0)
                } else {
                    write!(f, "{}", self.0)
                }
            }
        }


        impl fmt::Display for Operator {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    Self::Or => write!(f, " OR "),
                    Self::And => write!(f, " AND "),
                    Self::End => write!(f, ""),
                }

            }
        }

        impl #ident {
            pub fn query_builder() -> #builder_ident {
                #builder_ident {
                    query: vec![]
                }
            }
        }

        pub struct #builder_ident {
            query: Vec<(String, Operator)>
        }

        impl #builder_ident {
            #(fn #field_idents(&mut self, value: &str) -> &mut Self {
                let value = QueryString(value.into());

                let search = format!("{}:{}",#field_idents_str, value);
                if let Some(last) = self.query.last_mut() {
                    last.0 = search;
                } else {
                    self.query.push((search, Operator::End));
                }

                self
            })*

            #(fn #field_idents_range(&mut self, from: &str, to: &str) -> &mut Self {

                let from = QueryString(from.into());
                let to = QueryString(to.into());
                let search = format!("{}:[{} TO {}]", #field_idents_str, from, to);

                if let Some(last) = self.query.last_mut() {
                    last.0 = search;
                } else {
                    self.query.push((search, Operator::End));
                }

                self
            })*

            pub fn or(&mut self) -> &mut Self {
                if let Some(last) = self.query.last_mut() {
                    last.1 = Operator::Or;
                };

                self.query.push(("".into(), Operator::End));
                self
            }

            pub fn and(&mut self) -> &mut Self {
                if let Some(last) = self.query.last_mut() {
                    last.1 = Operator::And;
                };

                self.query.push(("".into(), Operator::End));
                self
            }

            pub fn proximity(&mut self, proximity: i32) -> &mut Self {
                if let Some(last) = self.query.last_mut() {
                    let prox = &format!("~{}", proximity);
                    last.0.push_str(prox);
                };

                self
            }

            pub fn build(&mut self) -> String {
                format!("query={}", self.to_string_query())
            }

            pub fn expr(&mut self, exp: &mut Self) -> &mut Self {
                let value = format!("({})", exp.to_string_query());
                self.query.push((value, Operator::End));
                self
            }

            fn to_string_query(&self) -> String {
                self.query.iter()
                    .map(|(value, separator)| format!("{}{}", value, separator))
                    .collect::<String>()
            }
        }
    };

    TokenStream::from(output)
}
