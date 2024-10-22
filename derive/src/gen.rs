use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{Field, Lit, Meta, MetaNameValue};

pub fn common_functions() -> TokenStream2 {
    quote! {
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
}

pub fn get_field_idents(fields: Vec<Field>) -> Vec<(Ident, Ident)> {
    fields
        .iter()
        .cloned()
        .filter(|field| get_non_ignored_fields(field))
        .map(|field| {
            let attributes = get_fields_attrs_meta(&field);
            let opt_rename = attributes
                .iter()
                .map(|meta| parse_renamed(meta))
                .find(|rename_ident| rename_ident.is_some());

            let opt_field_rename = attributes
                .iter()
                .map(|meta| parse_lucene_field_renamed(meta))
                .find(|rename_field_ident| rename_field_ident.is_some());

            let function_name = if let Some(renamed) = opt_rename {
                renamed.unwrap()
            } else {
                field.ident.clone().unwrap()
            };

            let lucene_field = if let Some(field_renamed) = opt_field_rename {
                field_renamed.unwrap()
            } else {
                field.ident.unwrap()
            };

            (function_name, lucene_field)
        })
        .collect()
}

pub fn query_builder_fn(ident: &Ident, builder_ident: &Ident) -> TokenStream2 {
    quote! {
        impl #ident {
            pub fn query_builder() -> #builder_ident {
                #builder_ident {
                    query: vec![]
                }
            }
        }
    }
}

pub fn query_builder(builder_ident: &Ident) -> TokenStream2 {
    quote! {
        pub struct #builder_ident {
            query: Vec<(String, Operator)>
        }
    }
}

pub fn query_field_fn(field_idents: &[(Ident, Ident)]) -> TokenStream2 {
    let function_names: Vec<Ident> = field_idents
        .iter()
        .map(|(func, _)| func.to_owned())
        .collect();

    let function_names = function_names.as_slice();

    let lucene_field_names: Vec<Ident> =
        field_idents.iter().map(|(_, luc)| luc.to_owned()).collect();

    let lucene_field_names = lucene_field_names.as_slice();

    let field_idents_str = idents_to_string(lucene_field_names);

    quote! {
       #(pub fn #function_names(&mut self, value: &str) -> &mut Self {
            let value = QueryString(value.into());
            let search = format!("{}:{}", #field_idents_str, value);

            if let Some(last) = self.query.last_mut() {
                last.0 = search;
            } else {
                self.query.push((search, Operator::End));
            }

            self
        })*
    }
}

pub fn range_query_field_fn(field_idents: &[(Ident, Ident)]) -> TokenStream2 {
    let function_names: Vec<Ident> = field_idents.iter().map(|(func, _)| func.clone()).collect();
    let function_names = function_names.as_slice();
    let lucene_field_names: Vec<Ident> = field_idents.iter().map(|(_, luc)| luc.clone()).collect();
    let lucene_field_names = lucene_field_names.as_slice();

    let field_idents_range = get_suffixed_idents(function_names, "_range");
    let field_idents_str = idents_to_string(lucene_field_names);

    quote! {
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
    }
}

pub fn get_non_ignored_fields(field: &Field) -> bool {
    !field
        .attrs
        .iter()
        .map(|attr| format!("{}", attr.path.segments.first().unwrap().ident))
        .collect::<String>()
        .contains("query_builder_ignore")
}

pub fn idents_to_string(field_idents: &[Ident]) -> Vec<String> {
    field_idents
        .iter()
        .cloned()
        .map(|name| format!("{}", name))
        .collect()
}

pub fn get_suffixed_idents(field_idents: &[Ident], suffix: &str) -> Vec<Ident> {
    field_idents
        .iter()
        .cloned()
        .map(|name| format_ident!("{}{}", name, suffix))
        .collect()
}

pub fn get_fields_attrs_meta(field: &Field) -> Vec<Meta> {
    field
        .attrs
        .iter()
        .map(|attr| attr.parse_meta())
        .filter(|attr| attr.is_ok())
        .map(|attr| attr.unwrap())
        .collect()
}

pub fn parse_renamed(attr: &Meta) -> Option<Ident> {
    match attr {
        Meta::NameValue(MetaNameValue {
            lit: Lit::Str(ref s),
            path,
            ..
        }) => {
            if path.is_ident("query_builder_rename") {
                Some(format_ident!("{}", s.value()))
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn parse_lucene_field_renamed(attr: &Meta) -> Option<Ident> {
    match attr {
        Meta::NameValue(MetaNameValue {
            lit: Lit::Str(ref s),
            path,
            ..
        }) => {
            if path.is_ident("query_builder_field") {
                Some(format_ident!("{}", s.value()))
            } else {
                None
            }
        }
        _ => None,
    }
}
