use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};

use syn::{self, spanned::Spanned, Field, Ident};
use syn::{Fields, Fields::Named, Variant};

use super::context::Context;
use super::RustlerAttr;

// FIXME should work exceptions?
pub fn transcoder_decorator(ast: &syn::DeriveInput, _add_exception: bool) -> TokenStream {
    let ctx = Context::from_ast(ast);

    let expect_message = "NifEnumStruct can only be used with structs or enums";
    // let decoders: Vec<TokenStream> = ctx
    //     .variants
    //     .as_ref()
    //     .expect(expect_message)
    //     .into_iter()
    //     .map(|variant| {
    //         super::tagged_enum::gen_named_decoder(ctx.ident, struct_fields(variant), &variant.ident)
    //     })
    //     .collect();
    // let decoder_body = quote! {
    //     #(#decoders)*
    // };

    let atom_modules = ctx
        .variants
        .as_ref()
        .expect(expect_message)
        .into_iter()
        .map(|variant| atoms_module_from_variant(ctx.ident, variant));

    // // let elixir_module = get_module_from_variant(variant);

    // let field_atoms = field_atoms(&struct_fields);

    // let atom_defs = quote! {
    //     rustler::atoms! {
    //         atom_module = #elixir_module,
    //         #(#field_atoms)*
    //     }
    // };

    // let atoms_module_name = Ident::new(
    //     &format!("RUSTLER_ATOMS_{}_{}", ctx.ident, variant.ident),
    //     Span::call_site(),
    // );

    // let decoder = if ctx.decode() {
    //     gen_decoder(&ctx, &struct_fields, &atoms_module_name)
    // } else {
    //     quote! {}
    // };

    // let encoder = if ctx.encode() {
    //     gen_encoder(&ctx, &struct_fields, &atoms_module_name, false)
    // } else {
    //     quote! {}
    // };

    // let gen = quote! {
    //     #[allow(non_snake_case)]
    //     mod #atoms_module_name {
    //         #atom_defs
    //     }

    //     #decoder

    //     #[allow(clippy::needless_borrow)]
    //     #encoder
    // };

    // gen
    quote! {
        mod atoms {
            #(#atom_modules)*
        }
    }
}

fn gen_decoder(ctx: &Context, fields: &[&Field], atoms_module_name: &Ident) -> TokenStream {
    let struct_name = ctx.ident;
    let struct_name_str = struct_name.to_string();

    let idents: Vec<_> = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
        .collect();

    let (assignments, field_defs): (Vec<TokenStream>, Vec<TokenStream>) = fields
        .iter()
        .zip(idents.iter())
        .enumerate()
        .map(|(index, (field, ident))| {
            let atom_fun = Context::field_to_atom_fun(field);
            let variable = Context::escape_ident_with_index(&ident.to_string(), index, "struct");

            let assignment = quote_spanned! { field.span() =>
                let #variable = try_decode_field(term, #atom_fun())?;
            };

            let field_def = quote! {
                #ident: #variable
            };

            (assignment, field_def)
        })
        .unzip();

    super::encode_decode_templates::decoder(
        ctx,
        quote! {
            use #atoms_module_name::*;
            use ::rustler::Encoder;

            fn try_decode_field<'a, T>(
                term: rustler::Term<'a>,
                field: rustler::Atom,
                ) -> ::rustler::NifResult<T>
                where
                    T: rustler::Decoder<'a>,
                {
                    use rustler::Encoder;
                    match ::rustler::Decoder::decode(term.map_get(&field)?) {
                        Err(_) => Err(::rustler::Error::RaiseTerm(Box::new(format!(
                                        "Could not decode field :{:?} on %{}{{}}",
                                        field, #struct_name_str
                        )))),
                        Ok(value) => Ok(value),
                    }
                }

            let module: ::rustler::types::atom::Atom = term.map_get(rustler::types::atom::__struct__())?.decode()?;
            if module != atom_module() {
                return Err(::rustler::Error::RaiseAtom("invalid_struct"));
            }

            #(#assignments);*

            Ok(#struct_name { #(#field_defs),* })
        },
    )
}

fn gen_encoder(
    ctx: &Context,
    fields: &[&Field],
    atoms_module_name: &Ident,
    add_exception: bool,
) -> TokenStream {
    let mut keys =
        vec![quote! { ::rustler::Encoder::encode(&rustler::types::atom::__struct__(), env) }];
    let mut values = vec![quote! { ::rustler::Encoder::encode(&atom_module(), env) }];
    if add_exception {
        keys.push(quote! { ::rustler::Encoder::encode(&atom_exception(), env) });
        values.push(quote! { ::rustler::Encoder::encode(&true, env) });
    }
    let (mut data_keys, mut data_values): (Vec<_>, Vec<_>) = fields
        .iter()
        .map(|field| {
            let field_ident = field.ident.as_ref().unwrap();
            let atom_fun = Context::field_to_atom_fun(field);
            (
                quote! { ::rustler::Encoder::encode(&#atom_fun(), env) },
                quote! { ::rustler::Encoder::encode(&self.#field_ident, env) },
            )
        })
        .unzip();
    keys.append(&mut data_keys);
    values.append(&mut data_values);

    super::encode_decode_templates::encoder(
        ctx,
        quote! {
            use #atoms_module_name::*;
            ::rustler::Term::map_from_term_arrays(env, &[#(#keys),*], &[#(#values),*]).unwrap()
        },
    )
}

// FIXME reduce duplication
fn get_module_from_variant(enum_name: &Ident, variant: &Variant) -> String {
    variant
        .attrs
        .iter()
        .flat_map(Context::get_rustler_attrs)
        .find_map(|attr| match attr {
            RustlerAttr::Module(ref module) => Some(module.clone()),
            _ => None,
        })
        .expect(&format!(
            "NifEnumStruct requires a 'module' attribute on every variant, but the {variant_ident} variant from {enum_name} didn't have one.",
            variant_ident = variant.ident
        ))
}

fn atoms_module_from_variant(enum_name: &Ident, variant: &Variant) -> TokenStream {
    let atom_module_name = variant.ident.to_string().to_lowercase();
    let elixir_module = get_module_from_variant(enum_name, variant);

    let fields = match &variant.fields {
        Named(fields_named) => fields_named.named.iter().map(|field| {
            field
                .ident
                .as_ref()
                .expect("NifEnumStruct expected field to have an identifier, but it didn't.")
        }),
        _ => panic!("NifEnumStruct requires all enum variants to be in struct syntax"),
    };

    let code = quote! {
        pub mod #atom_module_name {
            rustler::atoms! {
                __struct__ = #elixir_module,
                #(#fields),*


            }
        }
    };

    code
}

pub fn field_atoms(fields: &[&Field]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| {
            let atom_fun = Context::field_to_atom_fun(field);

            let ident = field.ident.as_ref().unwrap();
            let ident_str = ident.to_string();
            let ident_str = Context::remove_raw(&ident_str);

            quote! {
                #atom_fun = #ident_str,
            }
        })
        .collect()
}

fn struct_fields(variant: &Variant) -> Vec<&Field> {
    match &variant.fields {
        Fields::Named(named_fields) => named_fields.named.iter().collect(),
        _ => panic!("When using NifEnumStruct, all variants must be named (struct syntax)"),
    }
}
