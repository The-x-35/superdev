use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, Fields, Type};

#[proc_macro_derive(SerializeNumberStruct)]
pub fn serialise_number_struct(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let serialize_fields = match &ast.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    let field_serializations = fields.named.iter().map(|field| {
                        let field_name = &field.ident;
                        let field_type = &field.ty;
                        
                        let serialization = match field_type {
                            Type::Path(type_path) => {
                                let type_name = &type_path.path.segments.last().unwrap().ident;
                                match type_name.to_string().as_str() {
                                    "i8" => quote! { result.extend_from_slice(&self.#field_name.to_be_bytes()); },
                                    "i16" => quote! { result.extend_from_slice(&self.#field_name.to_be_bytes()); },
                                    "i32" => quote! { result.extend_from_slice(&self.#field_name.to_be_bytes()); },
                                    "i64" => quote! { result.extend_from_slice(&self.#field_name.to_be_bytes()); },
                                    "i128" => quote! { result.extend_from_slice(&self.#field_name.to_be_bytes()); },
                                    "u8" => quote! { result.extend_from_slice(&self.#field_name.to_be_bytes()); },
                                    "u16" => quote! { result.extend_from_slice(&self.#field_name.to_be_bytes()); },
                                    "u32" => quote! { result.extend_from_slice(&self.#field_name.to_be_bytes()); },
                                    "u64" => quote! { result.extend_from_slice(&self.#field_name.to_be_bytes()); },
                                    "u128" => quote! { result.extend_from_slice(&self.#field_name.to_be_bytes()); },
                                    "usize" => quote! { result.extend_from_slice(&(self.#field_name as u64).to_be_bytes()); },
                                    "isize" => quote! { result.extend_from_slice(&(self.#field_name as i64).to_be_bytes()); },
                                    "String" => quote! { 
                                        let bytes = self.#field_name.as_bytes();
                                        result.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
                                        result.extend_from_slice(bytes);
                                    },
                                    _ => panic!("Unsupported number type: {}", type_name),
                                }
                            }
                            _ => panic!("Unsupported field type"),
                        };
                        
                        serialization
                    });
                    quote! {
                        #(#field_serializations)*
                    }
                }
                _ => panic!("Only named fields are supported"),
            }
        }
        _ => panic!("Only structs are supported"),
    };

    let generated = quote! {
        impl serialize_macro_traits::Serialize for #name {
            fn serialize(&self) -> Vec<u8> {
                let mut result = Vec::new();
                #serialize_fields
                result
            }
        }
    };
    generated.into()
}

#[proc_macro_derive(DeserializeNumberStruct)]
pub fn deserialise_number_struct(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let (deserialize_fields, field_assignments) = match &ast.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    let mut field_deserializations = Vec::new();
                    let mut field_assignments = Vec::new();
                    
                    for field in &fields.named {
                        let field_name = &field.ident;
                        let field_type = &field.ty;
                        
                        let deserialization = match field_type {
                            Type::Path(type_path) => {
                                let type_name = &type_path.path.segments.last().unwrap().ident;
                                match type_name.to_string().as_str() {
                                    "i8" => quote! {
                                        let #field_name = {
                                            if offset + 1 > base.len() {
                                                return Err(serialize_macro_traits::SerializeError);
                                            }
                                            let bytes: [u8; 1] = base[offset..offset + 1]
                                                .try_into()
                                                .map_err(|_| serialize_macro_traits::SerializeError)?;
                                            offset += 1;
                                            i8::from_be_bytes(bytes)
                                        };
                                    },
                                    "i16" => quote! {
                                        let #field_name = {
                                            if offset + 2 > base.len() {
                                                return Err(serialize_macro_traits::SerializeError);
                                            }
                                            let bytes: [u8; 2] = base[offset..offset + 2]
                                                .try_into()
                                                .map_err(|_| serialize_macro_traits::SerializeError)?;
                                            offset += 2;
                                            i16::from_be_bytes(bytes)
                                        };
                                    },
                                    "i32" => quote! {
                                        let #field_name = {
                                            if offset + 4 > base.len() {
                                                return Err(serialize_macro_traits::SerializeError);
                                            }
                                            let bytes: [u8; 4] = base[offset..offset + 4]
                                                .try_into()
                                                .map_err(|_| serialize_macro_traits::SerializeError)?;
                                            offset += 4;
                                            i32::from_be_bytes(bytes)
                                        };
                                    },
                                    "i64" => quote! {
                                        let #field_name = {
                                            if offset + 8 > base.len() {
                                                return Err(serialize_macro_traits::SerializeError);
                                            }
                                            let bytes: [u8; 8] = base[offset..offset + 8]
                                                .try_into()
                                                .map_err(|_| serialize_macro_traits::SerializeError)?;
                                            offset += 8;
                                            i64::from_be_bytes(bytes)
                                        };
                                    },
                                    "i128" => quote! {
                                        let #field_name = {
                                            if offset + 16 > base.len() {
                                                return Err(serialize_macro_traits::SerializeError);
                                            }
                                            let bytes: [u8; 16] = base[offset..offset + 16]
                                                .try_into()
                                                .map_err(|_| serialize_macro_traits::SerializeError)?;
                                            offset += 16;
                                            i128::from_be_bytes(bytes)
                                        };
                                    },
                                    "u8" => quote! {
                                        let #field_name = {
                                            if offset + 1 > base.len() {
                                                return Err(serialize_macro_traits::SerializeError);
                                            }
                                            let bytes: [u8; 1] = base[offset..offset + 1]
                                                .try_into()
                                                .map_err(|_| serialize_macro_traits::SerializeError)?;
                                            offset += 1;
                                            u8::from_be_bytes(bytes)
                                        };
                                    },
                                    "u16" => quote! {
                                        let #field_name = {
                                            if offset + 2 > base.len() {
                                                return Err(serialize_macro_traits::SerializeError);
                                            }
                                            let bytes: [u8; 2] = base[offset..offset + 2]
                                                .try_into()
                                                .map_err(|_| serialize_macro_traits::SerializeError)?;
                                            offset += 2;
                                            u16::from_be_bytes(bytes)
                                        };
                                    },
                                    "u32" => quote! {
                                        let #field_name = {
                                            if offset + 4 > base.len() {
                                                return Err(serialize_macro_traits::SerializeError);
                                            }
                                            let bytes: [u8; 4] = base[offset..offset + 4]
                                                .try_into()
                                                .map_err(|_| serialize_macro_traits::SerializeError)?;
                                            offset += 4;
                                            u32::from_be_bytes(bytes)
                                        };
                                    },
                                    "u64" => quote! {
                                        let #field_name = {
                                            if offset + 8 > base.len() {
                                                return Err(serialize_macro_traits::SerializeError);
                                            }
                                            let bytes: [u8; 8] = base[offset..offset + 8]
                                                .try_into()
                                                .map_err(|_| serialize_macro_traits::SerializeError)?;
                                            offset += 8;
                                            u64::from_be_bytes(bytes)
                                        };
                                    },
                                    "u128" => quote! {
                                        let #field_name = {
                                            if offset + 16 > base.len() {
                                                return Err(serialize_macro_traits::SerializeError);
                                            }
                                            let bytes: [u8; 16] = base[offset..offset + 16]
                                                .try_into()
                                                .map_err(|_| serialize_macro_traits::SerializeError)?;
                                            offset += 16;
                                            u128::from_be_bytes(bytes)
                                        };
                                    },
                                    "usize" => quote! {
                                        let #field_name = {
                                            if offset + 8 > base.len() {
                                                return Err(serialize_macro_traits::SerializeError);
                                            }
                                            let bytes: [u8; 8] = base[offset..offset + 8]
                                                .try_into()
                                                .map_err(|_| serialize_macro_traits::SerializeError)?;
                                            offset += 8;
                                            u64::from_be_bytes(bytes) as usize
                                        };
                                    },
                                    "isize" => quote! {
                                        let #field_name = {
                                            if offset + 8 > base.len() {
                                                return Err(serialize_macro_traits::SerializeError);
                                            }
                                            let bytes: [u8; 8] = base[offset..offset + 8]
                                                .try_into()
                                                .map_err(|_| serialize_macro_traits::SerializeError)?;
                                            offset += 8;
                                            i64::from_be_bytes(bytes) as isize
                                        };
                                    },
                                    "String" => quote! {
                                        let #field_name = {
                                            if offset + 4 > base.len() {
                                                return Err(serialize_macro_traits::SerializeError);
                                            }
                                            let length_bytes: [u8; 4] = base[offset..offset + 4]
                                                .try_into()
                                                .map_err(|_| serialize_macro_traits::SerializeError)?;
                                            let length = u32::from_be_bytes(length_bytes) as usize;
                                            offset += 4;
                                            
                                            if offset + length > base.len() {
                                                return Err(serialize_macro_traits::SerializeError);
                                            }
                                            
                                            let string_bytes = &base[offset..offset + length];
                                            offset += length;
                                            String::from_utf8(string_bytes.to_vec())
                                                .map_err(|_| serialize_macro_traits::SerializeError)?
                                        };
                                    },
                                    _ => panic!("Unsupported number type: {}", type_name),
                                }
                            }
                            _ => panic!("Unsupported field type"),
                        };
                        
                        field_deserializations.push(deserialization);
                        field_assignments.push(quote! { #field_name });
                    }
                    
                    (field_deserializations, field_assignments)
                }
                _ => panic!("Only named fields are supported"),
            }
        }
        _ => panic!("Only structs are supported"),
    };

    let generated = quote! {
        impl serialize_macro_traits::Deserialize for #name {
            fn deserialize(base: &[u8]) -> Result<Self, serialize_macro_traits::SerializeError> {
                let mut offset = 0;
                
                #(#deserialize_fields)*
                
                Ok(#name {
                    #(#field_assignments,)*
                })
            }
        }
    };
    generated.into()
}