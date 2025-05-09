// src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Reflection)]
pub fn derive_reflection(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident; // struct name
    let fields = match input.data {
        Data::Struct(s) => s.fields,
        _ => panic!("#[derive(Reflection)] only works on structs"),
    };

    let mut arms = Vec::new();

    if let Fields::Named(named) = fields {
        for f in named.named {
            let fname = f.ident.expect("named field");
            let key = fname.to_string();
            let ty = &f.ty;

            // Pick the reflect_rs::ReflValue variant based on the field type:
            let variant = if is_string(ty) {
                quote!(reflect_rs::ReflValue::Str(self.#fname.clone()))
            } else if is_integer(ty) {
                quote!(reflect_rs::ReflValue::Int(self.#fname as i32))
            } else if is_float(ty) {
                quote!(reflect_rs::ReflValue::Float(self.#fname as f32))
            } else {
                // unsupported → fall back to None
                quote! { return None; }
            };

            arms.push(quote! { #key => Some(#variant), });
        }
    }

    let expanded = quote! {
        impl reflect_rs::Reflection for #ident {
            fn get_field(&self, field_name: &str) -> Option<reflect_rs::ReflValue> {
                match field_name {
                    #(#arms)*
                    _ => None,
                }
            }
        }
    };
    TokenStream::from(expanded)
}

// helper predicates ---------------------------------------------------------
fn is_string(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(p) if p.path.is_ident("String"))
}
fn is_integer(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(p) if p.path.is_ident("i32"))
}
fn is_float(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(p) if p.path.is_ident("f32"))
}
