use proc_macro::TokenStream;
use quote::quote;


#[proc_macro_attribute]
pub fn vlam(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: syn::ItemFn = syn::parse(item).unwrap();

    // Make sure we are in a simple function that is supported by the vla
    if let Some(asyncness) = item.sig.asyncness {
        return syn::Error::new_spanned(asyncness, "Not supported in an async fn")
            .to_compile_error().into();
    }
    if let Some(constness) = item.sig.constness {
        return syn::Error::new_spanned(constness, "Not supported in an async fn")
            .to_compile_error().into();
    }

    // Parse the ident for the context given in the attribute
    // eg. #[vlam(ctx)]
    let context_ident = syn::parse_macro_input!(attr as syn::Ident);


    // let generate the statement
    let ctx_decl = quote! {
        let #context_ident = unsafe {
            let #context_ident = vlam::VLACtx::init();
            core::pin::pin!(#context_ident)
        };
    };

    let sig =  &item.sig;
    let body = &item.block.stmts;
    let attrs = &item.attrs;
    let  vis = &item.vis;

    quote! {
        #(#attrs)*
        #[inline(never)]
        #vis #sig {
            #ctx_decl
            #(#body)*
        }
    }.into()
}