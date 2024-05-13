use darling::ast::Data;
use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(deref))]
struct AutoDerefInfo {
  ident: syn::Ident,
  generics: syn::Generics,
  data: Data<(), FieldsInfo>,
  #[darling(default)]
  mutable: bool,
  #[darling(default)]
  field: Option<syn::Ident>,
}

#[derive(Debug, FromField)]
struct FieldsInfo {
  ident: Option<syn::Ident>,
  ty: syn::Type,
}

pub fn process_auto_deref(input: DeriveInput) -> TokenStream {
  let AutoDerefInfo {
    ident,
    generics,
    data: Data::Struct(fields),
    mutable,
    field,
  } = AutoDerefInfo::from_derive_input(&input).unwrap()
  else {
    panic!("AutoDeref only works on structs");
  };
  let (fd, ty) = if let Some(field) = field {
    match fields.iter().find(|f| f.ident.as_ref().unwrap() == &field) {
      Some(f) => (field, &f.ty),
      None => panic!("field {:?} not found", field),
    }
  } else {
    if fields.len() == 1 {
      let f = fields.iter().next().unwrap();
      (f.ident.as_ref().unwrap().clone(), &f.ty)
    } else {
      panic!("AUtoDeref only works on structs with 1 field with field attribute");
    }
  };
  let mut code = vec![quote! {
    impl #generics std::ops::Deref for #ident #generics {
      type Target = #ty;

      fn deref(&self) -> &Self::Target {
        &self.#fd
      }
    }
  }];
  if mutable {
    code.push(quote! {
      impl #generics std::ops::DerefMut for #ident #generics {
        fn deref_mut(&mut self) -> &mut Self::Target {
          &mut self.#fd
        }
      }
    });
  }
  quote! {
      #(#code)*
  }
  .into()
}

pub fn process_auto_debug(input: DeriveInput) -> TokenStream {
  quote! {}.into()
}
