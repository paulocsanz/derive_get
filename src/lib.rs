use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2};
use proc_macro_error::{abort, abort_call_site, proc_macro_error, ResultExt};
use syn::{spanned::Spanned, DataStruct, DeriveInput, Field};
use quote::quote;

#[proc_macro_derive(Getters, attributes(skip, copy))]
#[proc_macro_error]
pub fn getters(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast: DeriveInput = syn::parse(input).expect_or_abort("Couldn't parse for getters");

    // Build the impl
    let gen = produce(&ast);

    // Return the generated impl
    gen.into()
}

fn produce(ast: &DeriveInput) -> TokenStream2 {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Is it a struct?
    if let syn::Data::Struct(DataStruct { ref fields, .. }) = ast.data {
        let generated = fields.iter().map(|f| implement(f));

        quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                #(#generated)*
            }
        }
    } else {
        // Nope. This is an Enum. We cannot handle these!
        abort_call_site!("#[derive(Getters)] is only defined for structs, not for enums!");
    }
}

fn implement(field: &Field) -> TokenStream2 {
    let fn_name = field
        .clone()
        .ident
        .unwrap_or_else(|| abort!(field.span(), "Fields must have a name"));

    let ty = field.ty.clone();

    let attrs = field.attrs.iter().filter(|v| {
        !v.parse_meta()
            .map(|meta| meta.path().is_ident("skip") || meta.path().is_ident("copy"))
            .unwrap_or(false)
    });

    let attr = field
        .attrs
        .iter()
        .filter(|meta| meta.path.is_ident("skip") || meta.path.is_ident("copy"))
        .last();

    match attr {
        // Generate nothing for skipped field.
        Some(meta) if meta.path.is_ident("skip") => quote! {},
        Some(meta) if meta.path.is_ident("copy") => quote! {
            #(#attrs)*
            #[inline(always)]
            pub fn #fn_name(&self) -> #ty {
                self.#fn_name
            }
        },
        Some(meta) => abort!(meta.span(), "Invalid attribute, should be unreachable"),
        None => quote! {
            #(#attrs)*
            #[inline(always)]
            pub fn #fn_name(&self) -> &#ty {
                &self.#fn_name
            }
        },
    }
}
