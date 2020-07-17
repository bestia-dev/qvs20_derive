use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn qvs20_table(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let table_type_name = input.ident;

    // region: row_type_name
    let mut row_type_name = None;
    // get the row type name from the newtype: pub struct MyTable (Vec<MyRow>);
    if let syn::Data::Struct(ref data_struct) = input.data {
        if let syn::Fields::Unnamed(fields_unnamed) = &data_struct.fields {
            if let syn::Type::Path(x) = &fields_unnamed.unnamed[0].ty {
                if let syn::PathArguments::AngleBracketed(a) = &x.path.segments[0].arguments {
                    if let syn::GenericArgument::Type(t) = &a.args[0] {
                        if let syn::Type::Path(p) = t {
                            let s = &p.path.segments[0];
                            row_type_name = Some(s.ident.clone());
                        }
                    }
                }
            }
        }
    };
    if row_type_name.is_none() {
        panic!("Wrong syntax. Must be like this: pub struct MyTable (Vec<MyRow>);");
    }
    let row_type_name = row_type_name.unwrap();
    // endregion: row_type_name

    let expanded = build_output(table_type_name, row_type_name);
    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

fn build_output(
    table_type_name: syn::Ident,
    row_type_name: syn::Ident,
) -> proc_macro2::TokenStream {
    // Build the output, possibly using quasi-quotation
    quote! {
        impl #table_type_name {
            /// constructor
            pub fn new() -> #table_type_name {
                //return
                #table_type_name (vec![] )
            }
            pub fn table_name()->&'static str{
                #row_type_name::table_name()
            }
            /// write schema
            pub fn write_schema(&self, file_name: &str) {
                let text = {
                    let mut wrt = WriterForQvs20::new();
                    self.write_schema_to_writer(&mut wrt, true);
                    wrt.return_and_finish()
                };
                fs::write(file_name, &text).unwrap();
                println!("write {}_schema.qvs20:",Self::table_name());
                println!("{}", text);
            }

            pub fn write_schema_to_writer(&self, wrt: &mut WriterForQvs20, schema_only:bool) {
                let schema = #row_type_name::get_schema();
                schema.write_schema_to_writer(wrt, schema_only);
            }
            /// write rows
            pub fn write_table_rows(&self, file_name: &str) {
                let text = {
                    let mut wrt = WriterForQvs20::new();
                    wrt.write_string("R");
                    wrt.write_string(&Self::table_name());
                    wrt.write_delimiter();
                    self.write_table_rows_to_writer(&mut wrt);
                    wrt.return_and_finish()
                };
                fs::write(file_name, &text).unwrap();
                println!("write {}_rows.qvs20:",Self::table_name());
                println!("{}", text);
            }

            fn write_table_rows_to_writer(&self, wrt: &mut WriterForQvs20) {
                for row in self.0.iter() {
                    row.write_row_to_writer(wrt);
                }
            }

            /// write one file for table
            pub fn write_one_file(&self, file_name: &str) {
                let text = {
                    let mut wrt = WriterForQvs20::new();
                    self.write_schema_to_writer(&mut wrt, false);
                    self.write_table_rows_to_writer(&mut wrt);
                    wrt.return_and_finish()
                };
                fs::write(file_name, &text).unwrap();
                println!("write {}.qvs20:",Self::table_name());
                println!("{}", text);
            }

            pub fn read_from_file(&mut self, file_name: &str) {
                let text = fs::read_to_string(file_name).unwrap();
                let mut rdr = ReaderForQvs20::new(text.as_bytes());
                let mut vec_of_string = rdr.next_row_as_vec_of_string().unwrap();
                // move out of vector. Warning: can be used only once!
                // The next time it will be the wrong value without any error.
                let file_type = std::mem::replace(&mut vec_of_string[0], String::new());
                let table_name = std::mem::replace(&mut vec_of_string[1], String::new());
                if table_name != Self::table_name() {
                    panic!("wrong table name");
                }
                // the next column defines if the files has the schema or not
                if file_type == "R" {
                    // only rows - data
                } else if file_type == "T" || file_type == "S" {
                    // schema with 5 rows
                    // move out of vector. Warning: can be used only once!
                    // The next time it will be the wrong value without any error.
                    let _description = std::mem::replace(&mut vec_of_string[1], String::new());
                    // drop the vector because it has not the originals values anymore
                    drop(vec_of_string);
                    let _vec_of_data_types = rdr.next_row_as_vec_of_string().unwrap();
                    let _vec_of_sub_table_schemas = rdr.next_row_as_vec_of_string().unwrap();
                    let _vec_of_additional_properties = rdr.next_row_as_vec_of_string().unwrap();
                    let _vec_of_column_names = rdr.next_row_as_vec_of_string().unwrap();
                } else {
                    panic!("first row is not correct");
                }

                #row_type_name::read_row_from_reader(&mut rdr, &mut self.0);
            }
            pub fn read_schema_from_file(&mut self, file_name: &str) -> TableSchema {
                let text = fs::read_to_string(file_name).unwrap();
                let schema = TableSchema::schema_from_qvs20_str(&text).unwrap();
                // return
                schema
            }
        }
    }
}
