use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Attribute, Data, DeriveInput, LitStr, Token};

#[proc_macro_derive(MyDerive, attributes(dom))]
pub fn my_derive(input: TokenStream) -> TokenStream {
    let input1 = parse_macro_input!(input as DeriveInput);
    let name = input1.ident;
    let (impl_generics, ty_generics, where_clause) = input1.generics.split_for_impl();

    println!("{:#?}", input1.generics.to_token_stream());
    let body = gen_body(input1.data, &name);
    quote!(
        impl #impl_generics ::rofd::dom::ToElement for #name #ty_generics #where_clause {
            fn to_element<N: AsRef<str>, NS: Into<String>>(
                &self,
                name: N,
                ns: NS,
                prefix: Option<String>) -> ::minidom::Element {
                #body
            }
        }
    )
    .into()
}

#[derive(Debug, Default)]
struct Ano {
    rename: Option<String>,
}
impl Ano {
    fn from_ast(ast: &[Attribute]) -> Self {
        let mut ano = Ano::default();
        ast.iter().for_each(|attr| {
            println!("{:#?}", attr.meta.path().to_token_stream());
            let meta = &attr.meta;
            if !meta.path().is_ident("dom") {
                return;
            }
            if let syn::Meta::List(meta) = meta {
                if meta.tokens.is_empty() {
                    return;
                }
            }
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("rename") {
                    let lookahead = meta.input.lookahead1();
                    if lookahead.peek(Token![=]) {
                        let value: LitStr = meta.value()?.parse()?;
                        ano.rename = Some(value.value());
                    }
                    // println!("{:#?}",meta.path.to_token_stream());
                    // ano.rename =
                }
                Ok(())
            })
            .unwrap();
        });
        ano
    }
}

fn gen_body(data: Data, _name: &Ident) -> proc_macro2::TokenStream {
    match data {
        Data::Struct(data) => {
            let _stream: proc_macro2::TokenStream = data
                .fields
                .iter()
                .enumerate()
                .map(|(idx, f)| {
                    let field_name = if let Some(ident) = f.ident.as_ref() {
                        ident.to_string()
                    } else {
                        idx.to_string()
                    };
                    let ty = &f.ty;
                    let _ano = Ano::from_ast(&f.attrs);
                    println!("type = {:#?}", ty.to_token_stream());
                    quote! {
                        #field_name :
                    }
                })
                .collect();

            quote! {
                todo!()
            }
        }
        Data::Enum(_) => {
            todo!()
        }
        Data::Union(_) => {
            todo!()
        }
    }
}
