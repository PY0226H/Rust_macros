use proc_macro::TokenStream;
use quote::quote;
//proc macro crate

//for enum, we'd like to generate From impls for each variant

#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    // println!("{:?}", input);
    //get the ident
    let ident = input.ident;
    //get enum variants
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom can only be derived for enums"),
    };
    //for each variant, get the ident and fields
    let from_impls = variants.iter().map(|variant| {
        let var = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                //only support one field
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().expect("Expected one field");
                    let ty = &field.ty;
                    quote! {
                        impl From<#ty> for #ident{
                            fn from (v: #ty) -> Self{
                                #ident::#var(v)
                            }
                        }
                    }
                }
            }
            syn::Fields::Unit => quote! {},
            syn::Fields::Named(_fields) => quote! {},
        }
    });
    //quote return proc_macro2 TokenStream, so we need to convert it into TokenStream
    quote! {
        #(#from_impls)*
    }
    .into()
}
