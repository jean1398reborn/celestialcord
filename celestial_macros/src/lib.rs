extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn add_fn(function: TokenStream) -> TokenStream {
    format!(
        r#"Box::new(|returned, client| {}(returned, client).boxed())"#,
        function.to_string()
    )
    .as_str()
    .parse()
    .unwrap()
}
