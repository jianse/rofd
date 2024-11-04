use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, LitStr, Token};

#[proc_macro_derive(MyDerive, attributes(dom))]
pub fn my_derive(input: TokenStream) -> TokenStream {
    let input1 = parse_macro_input!(input as DeriveInput);
    let name = input1.ident;
    let (impl_generics, ty_generics, where_clause) = input1.generics.split_for_impl();

    println!("{:#?}", input1.generics.to_token_stream());
    let body = gen_body(input1.data, &name);
    quote!(
        const _: () = {
        extern crate ofd_misc as _ofd_misc;
        impl #impl_generics _ofd_misc::ToElement for #name #ty_generics #where_clause {
            fn to_element<N: ::core::convert::AsRef<str>, NS: ::core::convert::Into<String>>(
                &self,
                name: N,
                ns: NS,
                prefix: ::core::option::Option<String>) -> ::core::option::Option<::minidom::Element> {
                #body
            }
        }
        };
    )
        .into()
}

fn gen_body(data: Data, _name: &Ident) -> proc_macro2::TokenStream {
    match data {
        Data::Struct(data) => {
            match data.fields {
                Fields::Named(_) => {}
                Fields::Unnamed(_) => {
                    todo!()
                }
                Fields::Unit => {}
            }
            let stream: proc_macro2::TokenStream = data
                .fields
                .iter()
                .map(|f| {
                    // let ty = &f.ty;
                    let ano = Ano::from_ast(f);
                    let ident = f.ident.as_ref().unwrap();
                    match ano.rename.unwrap().as_str() {
                        s if s.starts_with("@") => {
                            // attr
                            let name = s[1..].to_string();
                            quote! {
                                ele.set_attr(#name, self.#ident.clone());
                            }
                        }
                        "$text" => {
                            quote! {
                                if let Some(node) = self.#ident.clone().to_node(){
                                    ele.append_node(node);
                                }
                            }
                        }
                        s => {
                            //normal
                            quote! {
                                if let Some(e) = self.#ident.to_element(#s, &ns, None){
                                    ele.append_child(e);
                                }
                            }
                        }
                    }
                })
                .collect();

            quote! {
                let ns:String = ns.into();
                let name = name.as_ref();

                let mut ele = if prefix.is_some(){
                     ::minidom::Element::builder(name, &ns)
                        .prefix(prefix.clone(),&ns).unwrap()
                        .build()
                }else {
                    ::minidom::Element::bare(name, &ns)
                };

                #stream
                Some(ele)
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

#[derive(Debug, Default)]
struct Ano {
    rename: Option<String>,
}

impl Ano {
    fn from_ast(ast: &Field) -> Self {
        let mut ano = Ano::default();
        let name = ast.ident.as_ref().unwrap();
        let attrs = &ast.attrs;
        attrs.iter().for_each(|attr| {
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

        if ano.rename.is_none() {
            ano.rename = Some(name.to_string());
        }

        ano
    }
}
