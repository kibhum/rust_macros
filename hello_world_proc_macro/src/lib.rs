use proc_macro::TokenStream;
use quote::quote;

// we have #[proc_macro_derive(Hello)].
// a piece of metadata that informs Rust that something
// should happen. In our case, it tells Rust that this function is an entry point to
// a derive macro
// This annotation requires a name between parentheses - 'Hello'
// in our example - which will be used when invoking the macro: #
// [derive(Hello)]. The name of the function, on the other hand, is not used
// externally. So pick anything you want
#[proc_macro_derive(Hello)]
pub fn hello(_item: TokenStream) -> TokenStream {
    let add_hello_world = quote! {};
    add_hello_world.into()
}
