use proc_macro::TokenStream;

mod qvs20_row_mod;
mod qvs20_table_mod;

/// Derive implementation for the qvs20 format for the row struct.
/// The struct must be like this:
/// ```rust
/// #[derive(Qvs20Row)]
/// #[Qvs20TableName = "my_table"]
/// #[Qvs20Description = "my_description"]
/// pub struct MyRow {
///     pub field1: String,
///     #[Qvs20Type = "Decimal"]
///     pub field2: Decimal,
/// }
/// ```
///
#[proc_macro_derive(Qvs20Row, attributes(Qvs20Type, Qvs20TableName, Qvs20Description))]
pub fn qvs20_row(input: TokenStream) -> TokenStream {
    qvs20_row_mod::qvs20_row(input)
}

/// Derive implementation for the qvs20 format for the table newtype.
/// The newtype must be like this:
/// ```rust
/// #[derive(Qvs20Table)]
/// pub struct MyTable (Vec<MyRow>);
/// ```
///
#[proc_macro_derive(Qvs20Table)]
pub fn qvs20_table(input: TokenStream) -> TokenStream {
    qvs20_table_mod::qvs20_table(input)
}
