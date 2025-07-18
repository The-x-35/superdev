use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data};

#[proc_macro_attribute]
pub fn todo_app(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    
    let fields = match input.data {
        Data::Struct(data) => data.fields,
        _ => panic!("todo_app can only be applied to structs"),
    };
    
    let field_attrs: Vec<_> = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let pascal_case_name = to_pascal_case(&field_name.as_ref().unwrap().to_string());
        let todo_app_name = format!("TodoApp{}", pascal_case_name);
        
        quote! {
            #[serde(rename = #todo_app_name)]
            #field_name: #field_type
        }
    }).collect();
    
    let output = quote! {
        #[derive(serde::Serialize, serde::Deserialize)]
        pub struct #name {
            #(#field_attrs),*
        }
    };
    
    output.into()
}

fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    
    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else {
            if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }
    }
    
    result
} 