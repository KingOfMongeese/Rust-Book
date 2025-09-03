use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("hello_marco_derive failed to read input");

    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let code = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Marco! My name is {}!", stringify!(#name))
            }
        }
    };
    code.into()
}
