use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use syn::{Lit, Meta};

pub fn qvs20_row(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let type_name = input.ident;

    // region: table_name and description from attrs
    let mut table_name = type_name.to_string();
    let mut table_description = String::new();
    for attr in input.attrs.iter() {
        let meta = attr.parse_meta().unwrap();
        match meta {
            Meta::Path(_) => (),
            Meta::List(_) => (),
            Meta::NameValue(meta_name_value) => {
                if meta_name_value.path.get_ident().unwrap().to_string() == "Qvs20TableName" {
                    match meta_name_value.lit {
                        Lit::Str(lit_str) => {
                            table_name = lit_str.value();
                        }
                        _ => {
                            panic!("Qvs20TableName must be a string");
                        }
                    };
                } else if meta_name_value.path.get_ident().unwrap().to_string()
                    == "Qvs20Description"
                {
                    match meta_name_value.lit {
                        Lit::Str(lit_str) => {
                            table_description = lit_str.value();
                        }
                        _ => {
                            panic!("Qvs20Description must be a string");
                        }
                    };
                }
            }
        };
    }
    // endregion: table_name and description from attr

    // region: extract names from fields
    let mut fields = vec![];
    if let syn::Data::Struct(ref data_struct) = input.data {
        if let syn::Fields::Named(ref fields_named) = data_struct.fields {
            for field in fields_named.named.iter() {
                let mut qvs20_type = String::new();
                for attr in field.attrs.iter() {
                    if let Meta::NameValue(m) = attr.parse_meta().unwrap() {
                        if m.path.get_ident().unwrap().to_string() == "Qvs20Type" {
                            if let Lit::Str(lit_str) = m.lit {
                                qvs20_type = lit_str.value();
                            }
                        }
                    }
                }
                if qvs20_type.is_empty() {
                    // TODO: if has not attr Qvs20Type, try to match the type
                    qvs20_type = "String".to_string();
                }

                // name and Qvs20Type of field
                if let Some(ident) = &field.ident {
                    fields.push((ident.clone(), qvs20_type.clone()));
                }
            }
        }
    }
    if fields.is_empty() {
        panic!("Struct is not in the correct format.");
    }
    let mut fields_write = quote! {};
    let mut fields_read = quote! {};
    let mut fields_list = quote! {};
    let mut fields_data_types = quote! {};
    let mut fields_sub_table_schemas = quote! {};
    let mut fields_additional_properties = quote! {};
    let mut fields_column_names = quote! {};
    for f in fields {
        let name = f.0;
        let qvs20_type = f.1;

        let field_write = if qvs20_type == "Decimal" {
            quote! {
                wrt.write_decimal(self.#name);
            }
        } else {
            // else is String
            quote! {
                wrt.write_string(&self.#name);
            }
        };
        fields_write = quote! {
            #fields_write
            #field_write
        };

        let field_read = if qvs20_type == "Decimal" {
            quote! {
                let #name = rdr.next_decimal().unwrap();
            }
        } else {
            quote! {
                let #name = rdr.next_string().unwrap();
            }
        };

        fields_read = quote! {
            #fields_read
            #field_read
        };

        fields_list = quote! {
            #fields_list
            #name ,
        };
        let field_data_types = if qvs20_type == "Decimal" {
            quote! {
                DataType::Decimal,
            }
        } else {
            quote! {
                DataType::String,
            }
        };
        fields_data_types = quote! {
            #fields_data_types
            #field_data_types
        };
        fields_sub_table_schemas = quote! {
            #fields_sub_table_schemas
            None ,
        };
        fields_additional_properties = quote! {
            #fields_additional_properties
            String::new(),
        };
        let column_name = format!(r#"{}"#, name);
        fields_column_names = quote! {
            #fields_column_names
            #column_name.to_string(),
        };
    }
    // end region: extract names from fields

    // region: Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl #type_name {
            pub fn table_name()->&'static str{
                #table_name
            }
            pub fn write_row_to_writer(&self, wrt: &mut WriterForQvs20) {
                #fields_write
                wrt.write_delimiter();
            }
            pub fn read_row_from_reader( rdr:&mut ReaderForQvs20,  rows:&mut Vec<#type_name>){
                while !rdr.peek_next_is_eof() {
                    #fields_read
                    rdr.next_row_delimiter().unwrap();
                    rows.push(#type_name { #fields_list });
                }
            }
            pub fn get_schema()->TableSchema{
                let schema = TableSchema{
                    table_name: #table_name.to_string(),
                    table_description:#table_description.to_string(),
                    data_types:vec![#fields_data_types],
                    sub_table_schemas:vec![#fields_sub_table_schemas],
                    additional_properties:vec![#fields_additional_properties],
                    column_names:vec![#fields_column_names],
                    row_delimiter:b'\n',
                    sub_table_row_delimiter:b'1',
                    ..Default::default()
                };
                schema
            }
        }
    };
    // endregion: Build the output, possibly using quasi-quotation

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
