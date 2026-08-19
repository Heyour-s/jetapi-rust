use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

fn route_macro(args: TokenStream, input: TokenStream, method: &str) -> TokenStream {
    let path = parse_macro_input!(args as LitStr);
    let handler = parse_macro_input!(input as ItemFn);
    let handler_name = &handler.sig.ident;
    let expanded = quote! {
        #handler

        #[doc(hidden)]
        pub fn __register_route(app: jetapi::App) -> jetapi::App {
            app.#method(#path, #handler_name)
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    route_macro(args, input, "get")
}
#[proc_macro_attribute]
pub fn post(args: TokenStream, input: TokenStream) -> TokenStream {
    route_macro(args, input, "post")
}
#[proc_macro_attribute]
pub fn put(args: TokenStream, input: TokenStream) -> TokenStream {
    route_macro(args, input, "put")
}
#[proc_macro_attribute]
pub fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
    route_macro(args, input, "delete")
}
#[proc_macro_attribute]
pub fn patch(args: TokenStream, input: TokenStream) -> TokenStream {
    route_macro(args, input, "patch")
}
#[proc_macro_attribute]
pub fn options(args: TokenStream, input: TokenStream) -> TokenStream {
    route_macro(args, input, "options")
}
#[proc_macro_attribute]
pub fn head(args: TokenStream, input: TokenStream) -> TokenStream {
    route_macro(args, input, "head")
}
#[proc_macro_attribute]
pub fn trace(args: TokenStream, input: TokenStream) -> TokenStream {
    route_macro(args, input, "trace")
}
#[proc_macro_attribute]
pub fn any(args: TokenStream, input: TokenStream) -> TokenStream {
    route_macro(args, input, "any")
}