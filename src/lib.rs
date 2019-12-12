extern crate proc_macro;

use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Type};

#[proc_macro_derive(QueryBuilder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;
    let builder_ident = format_ident!("{}{}", ident, "LuceneQueryBuilder");

    let attributes: Vec<(Ident, Type)> = match input.data {
        Data::Struct(struct_) => match struct_.fields {
            Fields::Named(n) => n
                .named
                .iter()
                .cloned()
                .map(|field| (field.ident.unwrap().clone(), field.ty.clone()))
                .collect(),
            Fields::Unnamed(_) => panic!("Unexpected unnamed field"),
            Fields::Unit => panic!("Unexpected unit"),
        },
        Data::Enum(_) => panic!("The Builder macro is not to be used on enum"),
        Data::Union(_) => panic!("The Builder macro is not to be used on union"),
    };

    let attr_names: Vec<Ident> = attributes.iter().cloned().map(|(name, _)| name).collect();
    let attr_names_range: Vec<Ident> = attributes
        .iter()
        .cloned()
        .map(|(name, _)| format_ident!("{}{}", name, "_range"))
        .collect();

    let attr_names_str: Vec<String> = attributes
        .iter()
        .cloned()
        .map(|(name, _)| format!("{}", name))
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
            #(fn #attr_names(&mut self, value: &str) -> &mut Self {
                let value = QueryString(value.into());

                let search = format!("{}:{}",#attr_names_str, value);
                if let Some(last) = self.query.last_mut() {
                    last.0 = search;
                } else {
                    self.query.push((search, Operator::End));
                }

                self
            })*

            #(fn #attr_names_range(&mut self, from: &str, to: &str) -> &mut Self {

                let from = QueryString(from.into());
                let to = QueryString(to.into());
                let search = format!("{}:[{} TO {}]", #attr_names_str, from, to);

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
