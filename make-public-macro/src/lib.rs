extern crate core;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::Parse, parse::ParseStream, parse_macro_input, punctuated::Punctuated, token::Colon,
    Data::Struct, DataStruct, DeriveInput, Field, Fields::Named, FieldsNamed, Ident, Type,
    Visibility,
};

// DeriveInput source code
// pub struct DeriveInput { #1
//     pub attrs: Vec<Attribute>, #2
//     pub vis: Visibility, #3
//     pub ident: Ident, #4
//     pub generics: Generics, #5
//     pub data: Data, #6
//     }

// Data source code
// pub enum Data {
//     Struct(DataStruct), #1
//     Enum(DataEnum), #1
//     Union(DataUnion), #1
//     }

// Struct is the variant we need. What does the DataStruct nested inside
// contain? Dig just a little deeper. You will see that it has a struct_token
// which contains the struct keyword (not useful), semi_token which is an
// optionally present semicolon (not interesting), and the fields of the struct.
// pub struct DataStruct {
// pub struct_token: Token![struct],
// pub fields: Fields,
// pub semi_token: Option<Token![;]>,
// }

// we have a different attribute: #
// [proc_macro_attribute] instead of #[proc_macro_derive]. And unlike
// before, we do not specify the name of the macro between parenthesis (#
// [proc_macro_derive(Hello)])
// Instead, the name of the function determines
// the attribute name. So in our case, we created a custom attribute #[public].
// You can also see that we receive an additional TokenStream - which we will
// ignore for the time being - that contains information about our attribute.

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only works for structs with named fields"),
    };
    // With quote we generate a TokenStream with the public prefix, the name, and the type of the
    // given field
    // By now, we know that anything that is not a literal should be prefixed by a
    // hashtag in our output. That way quote knows it needs to replace the given
    // value with the value inside the identically named variable.
    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { pub #name: #ty }
    });
    let public_version = quote! {
        pub struct #name {
    // #(#name-of-your-variable,)*. I.e. take this variable,
    // which contains zero or more values, and after every retrieved element add a
    // comma.
        #(#builder_fields,)*
        }
        };
    public_version.into()
}

// Alternative implementation
struct StructField {
    name: Ident,
    ty: Type,
}

impl StructField {
    fn new(field: &Field) -> Self {
        Self {
            name: field.ident.as_ref().unwrap().clone(),
            ty: field.ty.clone(),
        }
    }
}

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let n = &self.name;
        let t = &self.ty;
        quote!(pub #n: #t).to_tokens(tokens)
    }
}
impl Parse for StructField {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let _vis: Result<Visibility, _> = input.parse();
        let list = Punctuated::<Ident, Colon>::parse_terminated(input).unwrap();
        Ok(StructField {
            name: list.first().unwrap().clone(),
            ty: list.last().unwrap().clone(),
        })
    }
}

#[proc_macro_attribute]
pub fn public_alt(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only works for structs with named fields"),
    };
    let builder_fields = fields.iter().map(StructField::new);
    let public_version = quote! {
        pub struct #name {
    // #(#name-of-your-variable,)*. I.e. take this variable,
    // which contains zero or more values, and after every retrieved element add a
    // comma.
        #(#builder_fields,)*
        }
        };
    public_version.into()
}

#[proc_macro_attribute]
pub fn public_alt_b(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only works for structs with named fields"),
    };
    let builder_fields = fields
        .iter()
        .map(|f| syn::parse2::<StructField>(f.to_token_stream()).unwrap());
    let public_version = quote! {
        pub struct #name {
    // #(#name-of-your-variable,)*. I.e. take this variable,
    // which contains zero or more values, and after every retrieved element add a
    // comma.
        #(#builder_fields,)*
        }
        };
    public_version.into()
}
