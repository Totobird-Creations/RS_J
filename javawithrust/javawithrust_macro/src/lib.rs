#![feature(proc_macro_expand)]


use proc_macro::*;

use quote::{quote, ToTokens};
use syn::{
    {
        Token, Ident, TypePath, Stmt, Block, Visibility,
        braced,
        parenthesized
    },
    Result, Type
};
use syn::{
    DeriveInput,
    parse_macro_input,
    parse::{Parse, ParseStream}
};


struct JavaFunctions {
    struct_name : Ident,
    funcs       : Vec<JavaFunction>
}
impl Parse for JavaFunctions {
    fn parse(input : ParseStream) -> Result<Self> {
        input.parse::<Token![impl]>()?;

        let struct_name = input.parse::<Ident>()?;

        let funcs_content;
        braced!(funcs_content in input);
        let mut funcs = Vec::new();
        loop {
            let vis = if funcs_content.lookahead1().peek(Token![fn]) {
                None
            } else if funcs_content.lookahead1().peek(Token![pub]) {
                Some(funcs_content.parse::<Visibility>()?)
            } else {
                break;
            };
            funcs_content.parse::<Token![fn]>()?;

            let name = funcs_content.parse::<Ident>()?;

            let args_content;
            parenthesized!(args_content in funcs_content);
            let mut arg_names = Vec::new();
            let mut arg_types = Vec::new();
            let mut args      = quote::__private::TokenStream::new();
            loop {
                if args_content.lookahead1().peek(Token![mut]) {
                    extend(&mut args, &args_content.parse::<Token![mut]>()?)
                } else {
                    if ! args_content.lookahead1().peek(Ident) {
                        break;
                    }
                }
                let ident = args_content.parse::<Ident>()?;
                extend(&mut args, &ident);
                arg_names.push(ident);
                let colon = args_content.parse::<Token![:]>()?;
                extend(&mut args, &colon);
                let typ = args_content.parse::<Type>()?;
                extend(&mut args, &typ);
                arg_types.push(typ);
                if args_content.lookahead1().peek(Token![,]) {
                    extend(&mut args, &args_content.parse::<Token![,]>()?);
                } else {
                    break;
                }
            }
    
            let ret = if funcs_content.lookahead1().peek(Token![->]) {
                funcs_content.parse::<Token![->]>()?;
                Some(funcs_content.parse::<TypePath>()?)
            } else {
                None
            };
    
            let block = if funcs_content.lookahead1().peek(Token![;]) {
                funcs_content.parse::<Token![;]>()?;
                None
            } else {
                let block_content;
                braced!(block_content in funcs_content);
                Some(block_content.call(Block::parse_within)?)
            };

            funcs.push(JavaFunction {
                vis,
                name,
                arg_names,
                arg_types,
                args,
                ret,
                block
            });
        }

        return Ok(JavaFunctions {
            struct_name,
            funcs
        }); 
    }
}

struct JavaFunction {
    vis       : Option<Visibility>,
    name      : Ident,
    arg_names : Vec<Ident>,
    arg_types : Vec<Type>,
    args      : quote::__private::TokenStream,
    ret       : Option<TypePath>,
    block     : Option<Vec<Stmt>>
}
impl JavaFunction {

    pub fn generate_bridge(&self, struct_name : &Ident, tokens : &mut quote::__private::TokenStream) {
        if let None = self.block {
            return;
        }
        let name              = &self.name;
        let name_str          = name.to_string();
        let arg_names = &self.arg_names;
        let arg_types = &self.arg_types;
        tokens.extend::<quote::__private::TokenStream>(quote!{
            ::javawithrust::prelude::paste::item!{
                #[::javawithrust::call_from_java_expand(concat!([<__jwrs_classname_ #struct_name>]!(), ".", #name_str))]
                fn #name(#(#arg_names : ::javawithrust::prelude::Instance),*) -> Result<::javawithrust::prelude::Instance, String> {
                    let __jwrs_jvm = ::javawithrust::prelude::Jvm::attach_thread().unwrap();
                    #(let #arg_names =
                        __jwrs_jvm.to_rust::<#arg_types>(#arg_names)
                            .map_err(|error| format!("{}", error))?;
                    )*
                    return ::javawithrust::prelude::Instance::try_from(
                        ::javawithrust::prelude::InvocationArg::try_from(
                            #struct_name :: #name(#(#arg_names),*)?
                        )
                            .map_err(|error| format!("{}", error))?
                    )
                        .map_err(|error| format!("{}", error));
                }
            }
        });

    }


    fn generate_frontend(&self, struct_name : &Ident, tokens : &mut quote::__private::TokenStream) {

        // Main function.
        {
            if let Some(vis) = &self.vis {
                extend(tokens, vis);
            }
            let name = &self.name;
            let args = &self.args;
            tokens.extend::<quote::__private::TokenStream>(quote!{
                fn #name(#args)
            }.into());
            tokens.extend::<quote::__private::TokenStream>(
                if let Some(ret) = &self.ret {
                    quote!{
                        -> Result<#ret, String>
                    }.into()
                } else {
                    quote!{
                        -> Result<(), String>
                    }.into()
                }
            );
            let name_str  = name.to_string();
            let arg_names = &self.arg_names;
            let ret       = if let Some(ret) = &self.ret {
                quote!{#ret}
            } else {
                quote!{()}
            };
            tokens.extend::<quote::__private::TokenStream>(match &self.block {
                Some(block) => {
                    quote!{
                        {#(#block)*}
                    }
                },
                None => {quote!{
                    {
                        let __jwrs_jvm = ::javawithrust::prelude::Jvm::attach_thread().unwrap();
                        #(let #arg_names =
                            ::javawithrust::prelude::Instance::try_from(
                                ::javawithrust::prelude::InvocationArg::try_from(
                                    #arg_names
                                )
                                    .map_err(|error| format!("{}", error))?
                            )
                            .map_err(|error| format!("{}", error));
                        )*
                        ::javawithrust::prelude::paste::item!{
                            return __jwrs_jvm.to_rust::<#ret>(
                                __jwrs_jvm.invoke_static([<__jwrs_classname_ #struct_name>]!(), #name_str, &[#(#arg_names),*])
                                    .map_err(|error| format!("{}", error))?
                            )
                                .map_err(|error| format!("{}", error));
                        }
                    }
                }}
            }.into());
        }

    }

}


fn extend<T : ToTokens>(tokens : &mut quote::__private::TokenStream, value : &T) {
    let mut token = quote::__private::TokenStream::new();
    value.to_tokens(&mut token);
    tokens.extend(token);
}


/// Define a new Java class.
/// 
/// # Examples
/// 
/// ```
/// #[jclass(io.example.MyJavaClass)]
/// struct MyJavaClass;
/// ```
/// 
/// # Warning
/// 
/// Class name can not contain any special characters or underscores.
#[proc_macro_attribute]
pub fn jclass(attr : TokenStream, item : TokenStream) -> TokenStream {
    let parse_item = item.clone();
    let DeriveInput {
        attrs    : _struct_attrs,
        vis      : _struct_vis,
        ident    : struct_name,
        generics : _struct_generics,
        data     : _struct_data
    } = parse_macro_input!(parse_item);
    let     class_name = attr.to_string();
    let mut item2 = quote!{
        #[derive(::javawithrust::prelude::serde::Serialize)]
        #[derive(::javawithrust::prelude::serde::Deserialize)]
    };
    item2.extend::<quote::__private::TokenStream>(item.into());
    item2.extend::<quote::__private::TokenStream>(quote!{
        ::javawithrust::prelude::paste::item!{
            macro_rules! [<__jwrs_classname_ #struct_name>] {
                () => {#class_name}
            }
            use [<__jwrs_classname_ #struct_name>];
        }

    }.into());
    return item2.into();
}


#[proc_macro_attribute]
pub fn jfuncs(attr : TokenStream, item : TokenStream) -> TokenStream {
    if ! attr.is_empty() {
        panic!("`jfuncs` attribute takes 0 parameters.")
    }
    let parse_item = item.clone();
    let JavaFunctions {
        struct_name,
        funcs,
        ..
    } = parse_macro_input!(parse_item as JavaFunctions);

    let mut bridge_funcs   = quote::__private::TokenStream::new();
    let mut frontend_funcs = quote::__private::TokenStream::new();
    for func in funcs {
        let mut bridge_func   = quote::__private::TokenStream::new();
        let mut frontend_func = quote::__private::TokenStream::new();
        func.generate_bridge(&struct_name, &mut bridge_func);
        func.generate_frontend(&struct_name, &mut frontend_func);
        bridge_funcs.extend(bridge_func);
        frontend_funcs.extend(frontend_func);
    }

    return quote!{
        ::javawithrust::prelude::paste::item!{
            mod [<__jwrs_ #struct_name>] {
                use super::*;
                #bridge_funcs
            }
        }
        impl #struct_name {
            #frontend_funcs
        }
    }.into();
}


#[proc_macro_attribute]
pub fn call_from_java_expand(mut attr : TokenStream, item : TokenStream) -> TokenStream {
    attr = attr.expand_expr().unwrap();
    let mut attr2 = quote::__private::TokenStream::new();
    attr2.extend::<quote::__private::TokenStream>(attr.into());
    let mut tokens = TokenStream::new();
    tokens.extend::<TokenStream>(quote!{
        #[::javawithrust::prelude::call_from_java(#attr2)]
    }.into());
    tokens.extend(item);
    return tokens;
}
