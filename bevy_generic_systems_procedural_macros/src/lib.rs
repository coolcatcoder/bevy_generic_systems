use foldhash::{HashMap, HashMapExt};
use proc_macro::TokenStream as StdTokenStream;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Ident, Type, parse_macro_input};

#[proc_macro_derive(Behaviour)]
pub fn behaviour(input: StdTokenStream) -> StdTokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    behaviour_internal(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn behaviour_internal(input: DeriveInput) -> syn::Result<TokenStream> {
    let ident = input.ident;

    let mut field_types_and_idents = HashMap::<Type, Vec<Ident>>::new();

    match input.data {
        Data::Struct(data) => {
            data.fields.iter().enumerate().for_each(|(index, field)| {
                let idents = if field_types_and_idents.contains_key(&field.ty) {
                    field_types_and_idents.get_mut(&field.ty).unwrap()
                } else {
                    field_types_and_idents.insert(field.ty.clone(), vec![]);
                    field_types_and_idents.get_mut(&field.ty).unwrap()
                };

                idents.push(
                    field
                        .ident
                        .clone()
                        .unwrap_or(Ident::new(format!("_{index}").as_str(), Span::call_site())),
                );
            });
        }
        Data::Enum(data) => {
            return Err(syn::Error::new(
                data.enum_token.span,
                "Behaviour in enums is unsupported.",
            ));
        }
        Data::Union(data) => {
            return Err(syn::Error::new(
                data.union_token.span,
                "Behaviour in unions is unsupported.",
            ));
        }
    }

    let field_types = field_types_and_idents.keys();

    let field_idents = field_types_and_idents.values();

    Ok(quote! {
        #(
            impl bevy_generic_systems::ComponentContains<#field_types> for #ident {
                fn get_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut #field_types>
                where
                    #field_types: 'a,
                {
                    [
                        #(
                            &mut self.#field_idents
                        ),*
                    ].into_iter()
                }
            }

            bevy_generic_systems::app!(|app| {
                let maybe: bevy_generic_systems::MaybeApp<#field_types> = bevy_generic_systems::MaybeApp(std::marker::PhantomData);

                #[allow(unused_imports)]
                use bevy_generic_systems::HasComponentFieldBehaviour;
                #[allow(unused_imports)]
                use bevy_generic_systems::NoComponentFieldBehaviour;
                (&&maybe).maybe_app::<#ident>(app);
            });
        )*
    })
}
