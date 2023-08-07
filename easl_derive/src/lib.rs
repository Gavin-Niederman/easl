use proc_macro::TokenStream;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn with_inner_passthrough(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ItemFn { attrs, vis, sig, block } = syn::parse_macro_input!(input as ItemFn);
    quote::quote!(
        #(#attrs)* #vis #sig {
            let mut inner = input.clone().into_inner();
            if inner.len() == 1 {
                return Self::visit_expression(inner.next().unwrap());
            } #block
        }
    ).into()
}

#[proc_macro_attribute]
pub fn with_inner(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ItemFn { attrs, vis, sig, block } = syn::parse_macro_input!(input as ItemFn);
    let stmts = &block.stmts;
    quote::quote!(
        #(#attrs)* #vis #sig {
            let mut inner = input.clone().into_inner();
            #(#stmts)*
        }
    ).into()
}