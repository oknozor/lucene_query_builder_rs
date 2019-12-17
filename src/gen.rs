use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

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
