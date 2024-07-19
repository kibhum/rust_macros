use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use venial::{parse_declaration, Declaration, Enum, Struct};

// we have #[proc_macro_derive(Hello)].
// a piece of metadata that informs Rust that something
// should happen. In our case, it tells Rust that this function is an entry point to
// a derive macro
// This annotation requires a name between parentheses - 'Hello'
// in our example - which will be used when invoking the macro: #
// [derive(Hello)]. The name of the function, on the other hand, is not used
// externally. So pick anything you want

#[proc_macro_derive(Hello)]
pub fn hello(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let add_hello_world = quote! {
    impl #name {
    fn hello_world(&self) {
    println!("Hello world")
    }
    }
    };
    add_hello_world.into()
}

// without using quote and syn
#[proc_macro_derive(HelloA)]
pub fn hello_alt(item: TokenStream) -> TokenStream {
    fn ident_name(item: TokenTree) -> String {
        match item {
            TokenTree::Ident(i) => i.to_string(),
            _ => panic!("no ident"),
        }
    }
    let name = ident_name(item.into_iter().nth(1).unwrap());
    format!(
        "impl {} {{ fn hello_world(&self) \
{{ println!(\"Hello world\") }} }} ",
        name
    )
    .parse()
    .unwrap()
}

#[proc_macro_derive(HelloV)]
pub fn hello_v(item: TokenStream) -> TokenStream {
    let declaration = parse_declaration(item.into()).unwrap();
    let name = match declaration {
        Declaration::Struct(Struct { name, .. }) => name,
        Declaration::Enum(Enum { name, .. }) => name,
        _ => panic!("only implemented for struct and enum"),
    };
    let add_hello_world = quote! {
    impl #name {
    fn hello_world(&self) {
    println!("Hello world")
    }
    }
    };
    add_hello_world.into()
}
