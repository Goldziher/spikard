use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn without_gvl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;
    let mut anon_sig = sig.clone();
    anon_sig.ident = syn::Ident::new("__anon_wrapper", sig.ident.span());

    let params = sig.inputs.iter().map(|arg| match arg {
        FnArg::Typed(pat) => {
            let arg = &pat.pat;
            let ty = &pat.ty;
            quote!(#arg, #ty)
        }
        FnArg::Receiver(recv) => {
            let ty = if let Some((_and, lifetime)) = &recv.reference {
                let mutability = recv.mutability;
                lifetime.as_ref().map_or_else(
                    || quote!(& #mutability Self),
                    |lifetime| quote!(&#lifetime #mutability Self),
                )
            } else {
                quote!(Self)
            };
            quote!(self, #ty)
        }
    });

    let return_ty = match &sig.output {
        syn::ReturnType::Default => quote!(()),
        syn::ReturnType::Type(_, ty) => quote!(#ty),
    };

    let block = &input.block;

    quote!(
        #(#attrs)*
        #vis #sig {
            #anon_sig {
                #block
            }
            crate::call_without_gvl!(__anon_wrapper, args: (#(#params),*), return_type: #return_ty)
        }
    )
    .into()
}
