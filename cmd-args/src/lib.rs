use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Expr, ExprLit, Fields, Ident, Lit, ItemFn, FnArg, Pat, Type};

#[proc_macro_attribute]
pub fn permission(attr: TokenStream, item: TokenStream) -> TokenStream {
    let permission_expr = parse_macro_input!(attr as Expr);
    let mut function = parse_macro_input!(item as ItemFn);
    let ctx_ident = function.sig.inputs.iter().find_map(|arg| {
        if let FnArg::Typed(pat) = arg {
            if let Pat::Ident(ident) = &*pat.pat {
                // Check if the type is `&Context`
                if let Type::Reference(tyref) = &*pat.ty {
                    if let Type::Path(path) = &*tyref.elem {
                        if path.path.segments.last().unwrap().ident == "Context" {
                            return Some(ident.ident.clone());
                        }
                    }
                }
            }
        }
        None
    }).expect("Expected &Context parameter"); 
    let injected = quote! {
        if !#ctx_ident.has_permissions(#permission_expr) {
            return common.reply("You don't have permission!");
        }
    };

    let original_block = &function.block;
    function.block = syn::parse_quote!({
        #injected
        #original_block
    });

    TokenStream::from(quote!(#function))
}
#[proc_macro_derive(CommandArgs, attributes(description, required))]
pub fn derive_command_args(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("CommandArgs can only be derived for structs with named fields"),
        },
        _ => panic!("CommandArgs can only be derived for structs"),
    };

    let mut field_processors = Vec::new();
    let mut field_extractor_match_arms = Vec::new();
    let mut field_bindings = Vec::new();
    let mut field_locals = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let field_str = field_name.to_string();
        let field_type_str = &field_type.into_token_stream().to_string();

        let is_option = if let syn::Type::Path(type_path) = field_type {
            if let Some(segment) = type_path.path.segments.first() {
                segment.ident == "Option"
            } else {
                false
            }
        } else {
            false
        };
        let is_required = !is_option;
        let mut description = String::from("No description provided");
        for attr in &field.attrs {
            if attr.path().is_ident("description") {
                if let Ok(Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. })) = attr.parse_args::<Expr>() {
                    description = lit_str.value();
                }
            }
        }
        description = format!("({}) {}", field_type_str, description);
        if is_option {
            description = format!("{}{}", if is_option {
                "(Optional) "
            } else {
                ""
            }, description);
        }

        let processor = quote! {
            options.push(
                serenity::all::CreateCommandOption::new(
                    <#field_type as cmd_args_ext::CommandOptionTypeExt>::get_option_type(),
                    #field_str,
                    #description
                )
                .required(#is_required)
            );
        };

        let local_var = syn::Ident::new(&format!("__{}_arg", field_name), field_name.span());

        let local_declaration = quote! {
            let mut #local_var = None;
        };

        let extractor_arm = quote! {
            #field_str => {
                #local_var = Some(<#field_type as cmd_args_ext::CommandOptionTypeExt>::from_option(Some(option)));
            }
        };

        // Handle the Result properly in the binding
        let binding = if is_required {
            quote! {
                #field_name: #local_var.ok_or_else(|| cmd_args_ext::CommandError::Argument(
                    String::from(#field_str),
                    "Missing field".to_string()
                ))??
            }
        } else {
            // For Option<T>, flatten the nested Option
            quote! {
                #field_name: #local_var.transpose()?.flatten()
            }
        };

        field_processors.push(processor);
        field_locals.push(local_declaration);
        field_extractor_match_arms.push(extractor_arm);
        field_bindings.push(binding);
    }

    let output = quote! {
        impl cmd_args_ext::CommandArgsExt for #struct_name {
            fn add_to_command(command: serenity::all::CreateCommand) -> serenity::all::CreateCommand {
                let mut options = Vec::new();
                #(#field_processors)*

                let mut cmd = command;
                for option in options {
                    cmd = cmd.add_option(option);
                }
                cmd
            }

            fn from_options(options: &[serenity::all::ResolvedOption]) -> Result<Self, cmd_args_ext::CommandError> {
                #(#field_locals)*

                for option in options {
                    match option.name {
                        #(#field_extractor_match_arms)*
                        _ => {}
                    }
                }

                Ok(Self {
                    #(#field_bindings),*
                })
            }

            fn from_command(command: &serenity::all::CommandInteraction) -> Result<Self, cmd_args_ext::CommandError> {
                Self::from_options(&command.data.options())
            }
        }
    };

    output.into()
}
#[proc_macro_derive(EnumArgs, attributes(alias, required))]
pub fn derive_enum_args(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the enum variants
    let enum_name = &input.ident;
    let enum_name_string = enum_name.to_string();
    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => panic!("EnumArgs can only be derived for enums"),
    };

    // Process each variant to extract attributes and type information
    let mut variant_extractors = Vec::new();
    let mut to_string_extractors = Vec::new();
    let mut to_vec_strings = Vec::new();
    let mut first_variant_name: Option<&Ident> = None;
    for variant in variants {
        let variant_name = &variant.ident;
        let mut alias = variant_name.to_string();
        if first_variant_name.is_none() {
            first_variant_name = Some(&variant.ident);
        }

        for attr in &variant.attrs {
            if attr.path().is_ident("alias") {
                if let Expr::Lit(ExprLit {
                    lit: Lit::Str(lit_str),
                    ..
                }) = attr.parse_args::<Expr>().unwrap()
                {
                    alias = lit_str.value();
                }
            }
        }

        // Generate code to extract this variant from options
        let extractor = quote! {
            #alias => Ok(#enum_name::#variant_name),
        };
        let to_string_extractor = quote! {
            #enum_name::#variant_name => {
                return String::from(#alias);
            },
        };
        let to_vec_extractor = quote! {
            String::from(#alias),
        };

        variant_extractors.push(extractor);
        to_string_extractors.push(to_string_extractor);
        to_vec_strings.push(to_vec_extractor);
    }
    let first_variant_name_res = first_variant_name.unwrap();
    // Generate the implementation
    let output = quote! {
        impl cmd_args_ext::CommandOptionTypeExt for #enum_name {
            fn get_option_type() -> serenity::all::CommandOptionType {
                serenity::all::CommandOptionType::String
            }

            fn from_option(option: Option<&serenity::all::ResolvedOption>) -> Result<Self, cmd_args_ext::CommandError> {
                if let Some(option) = option {
                    if let serenity::all::ResolvedValue::String(value) = &option.value {
                        match *value {
                            #(#variant_extractors)*
                            _ => Err(cmd_args_ext::CommandError::Argument(option.name.to_string(), format!("Invalid value for enum {} (run `/enum name:{}` for more info)", #enum_name_string, #enum_name_string))),
                        }
                    } else {
                        Err(cmd_args_ext::CommandError::Argument(option.name.to_string(), format!("Expected enum of {} (run `/enum name:{}` for more info)", #enum_name_string, #enum_name_string)))
                    }
                } else {
                    Err(cmd_args_ext::CommandError::Default(format!("Expected enum of {} (run `/enum name:{}` for more info)", #enum_name_string, #enum_name_string)))
                }
            }
        }
        impl std::default::Default for #enum_name {
            fn default() -> Self {
                #enum_name::#first_variant_name_res
            }
        }
        impl cmd_args_ext::EnumArgsExt for #enum_name {
            fn enum_name() -> String {
                String::from(#enum_name_string)
            }
            fn to_alias(&self) -> String {
                match self {
                    #(#to_string_extractors)*
                    _ => {
                        return String::from("Unknown");
                    },
                }
            }
            fn to_vec() -> Vec<String> {
                vec![
                    #(#to_vec_strings)*
                ]
            }
        }
    };

    output.into()
}
