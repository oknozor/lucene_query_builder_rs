use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::Field;

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

pub fn get_non_ignored_fields(field: &Field) -> bool {
    !field
        .attrs
        .iter()
        .map(|attr| format!("{}", attr.path.segments.first().unwrap().ident))
        .collect::<String>()
        .contains("query_builder_ignore")
}

pub fn idents_to_string(field_idents: &Vec<Ident>) -> Vec<String> {
    field_idents
        .iter()
        .cloned()
        .map(|name| format!("{}", name))
        .collect()
}

pub fn get_suffixed_idents(field_idents: &Vec<Ident>, suffix: &str) -> Vec<Ident> {
    field_idents
        .iter()
        .cloned()
        .map(|name| format_ident!("{}{}", name, suffix))
        .collect()
}
