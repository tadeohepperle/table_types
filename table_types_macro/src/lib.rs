extern crate proc_macro;
use std::fmt::Debug;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse_macro_input, Data, DataEnum, DeriveInput, Field, Fields, FieldsNamed, Ident, Type,
};

use itertools::Itertools;

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr = attr.to_string();
    let input = input.to_string();

    quote!(
        pub fn show() {
            println!("{}", #attr);
            println!("{}", #input);
        }
    )
    .into()
}

#[proc_macro_attribute]
pub fn table(attr: TokenStream, input: TokenStream) -> TokenStream {
    return table_internal(attr, input);
}

fn table_internal(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let Data::Enum(enum_data) = &ast.data else{
        return quote! {
            compile_error!("Item must be enum");
        }.into();
    };
    let horizontal_variants = match horizontal_variants(enum_data) {
        Ok(h) => h,
        Err(e) => {
            return e.into();
        }
    };
    let vertical_names = vertical_compounds_names(attr);

    if !horizontal_variants
        .iter()
        .all(|v| v.fields.len() == vertical_names.len())
    {
        return quote!(compile_error!("Number of fields specified in attribute needs to be the same as number of unnamed fields of each enum variant.", )).into();
    }

    let mut output: Vec<TokenStream2> = vec![];

    // create vertical slice types:
    for v_cols in vertical_names.iter().combinations(vertical_names.len()) {
        // create a struct for each vertical slice of the whole table:
        output.push(quote!(println!("{:?}", v_cols);));
    }

    // create vertical slice compound types:
    for v_col in vertical_names.iter() {}

    quote! {
        pub fn s() {
        #(#output)*
        }
    }
    .into()
}

// let h = format!("{:?}", horizontal_variants);
// let v = format!("{:?}", vertical_names);
// quote!(
//     pub fn s() {
//         println!("{:?}", #h);
//         println!("{:?}", #v);
//     }
// )
// .into()

struct HorizontalVariant {
    name: String,
    fields: Vec<Type>,
}

impl Debug for HorizontalVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HorizontalVariant")
            .field("name", &self.name)
            .finish()
    }
}

fn horizontal_variants(data_enum: &DataEnum) -> Result<Vec<HorizontalVariant>, TokenStream2> {
    let mut variants = Vec::<HorizontalVariant>::new();
    for v in &data_enum.variants {
        let name = v.ident.to_string();
        let Fields::Unnamed(unnamed) = &v.fields else{
                return Err(quote!(compile_error!("Only unnamed enum fields allowed!")));
            };
        let fields: Vec<Type> = unnamed.unnamed.iter().map(|f| f.ty.clone()).collect();
        variants.push(HorizontalVariant { name, fields })
    }
    Ok(variants)
}

fn vertical_compounds_names(attr: TokenStream) -> Vec<String> {
    let attr = attr
        .to_string()
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();
    attr
}

#[test]
fn test_macro() {
    let input = quote!(
        enum Table {
            A(u16, u8),
            B(u16, u8),
            C(u16, u8),
            D(u16, u8),
        }
    );
    let attr = quote!(R, S, T);
    let output = table_internal(attr.into(), input.into());
    println!("{}", output.to_string());
}
