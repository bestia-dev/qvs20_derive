[comment]: # (lmake_md_to_doc_comments segment start A)

# QVS20 - derive

A modern replacement for csv for the year 2020.  

[comment]: # (lmake_cargo_toml_to_md start)

**Derive crate for qvs20 - Modern replacement for csv for the year 2020**  
***version: 1.0  date: 2020-07-13 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/qvs20_derive)***  

[comment]: # (lmake_cargo_toml_to_md end)

[comment]: # (lmake_lines_of_code start)

[comment]: # (lmake_lines_of_code end)
  
My proposed format for import/export of 2 dimensional database tables.  
  
## Derive

Procedural macro to codegen (code generate) the implementation for serialize/deserialize qvs20.  
We need 3 derive macros:  

1. for the struct that represents the table (has one field that is for rows)
2. for the struct that represents the row
3. for the struct that represents the sub-table

Very interesting.  

## References

<https://dev.to/jeikabu/rust-derive-macros-o38>  

[comment]: # (lmake_md_to_doc_comments segment end A)
