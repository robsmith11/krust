use proc_macro::TokenStream;
use quote::*;

/// Generates a function `foo_to_ktable for the struct `Foo`
/// with named fields of type `Vec<ToKList>`.
#[proc_macro_derive(ToKTable)]
pub fn ktable_derive(input:TokenStream) -> TokenStream {
    let ast:syn::DeriveInput = syn::parse(input).unwrap();

    let name = ast.ident;
    let fname = syn::Ident::new(&format!("{}_to_ktable", &name).to_lowercase(), quote::__rt::Span::call_site());

    if let syn::Data::Struct(ref data) = ast.data {
        if let syn::Fields::Named(ref fields) = data.fields {
            let nf = fields.named.iter().map(|x| x.clone().ident.unwrap());
            let snf = fields.named.iter().map(|x| x.clone().ident.unwrap().to_string());
            
            let expanded = quote! {
                pub fn #fname(x:&[#name]) -> *const K {
                    vecs_to_table(vec![ #( #snf ,)* ],
                                  vec![ #(ToKList::to_klist(&mut x.iter().map(|y| y.#nf).collect()) ,)* ])
                }
            };

            return expanded.into();
        }
    }
    
    unimplemented!();
        
}
