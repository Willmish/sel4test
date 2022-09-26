use proc_macro::TokenStream;
use quote::{ToTokens, quote, format_ident};
use syn::{FnArg, Type, ReturnType, parse_quote};

fn derive_test_wrapper(func: syn::ItemFn) -> TokenStream {
    let attributes = &func.attrs;
    let signature = &func.sig;
    let name = &signature.ident;
    let export_name = format_ident!("test_export_{}", name);
    let arguments = &signature.inputs;
    let return_type = match &signature.output {
        ReturnType::Default => ReturnType::Default,
        ReturnType::Type(_, ty) => match ty.as_ref() {
            Type::Path(ty) => {
                let type_ident = ty.path.segments.last().unwrap().ident.to_string();
                match &type_ident[..] {
                    "seL4_Result" => {
                        parse_quote! { -> isize }
                    }
                    _ => signature.output.clone()
                }
            }
            _ => {
                signature.output.clone()
            }
        }
    };

    let argument_names = arguments.iter().map(|arg| match arg {
        FnArg::Typed(ty) => {
            let last_token = ty.pat.to_token_stream().into_iter().last().unwrap();
            last_token
        },
        FnArg::Receiver(_) => panic!("unexpected self argument")
    });
    let function_invocation = quote! { #name(#(#argument_names,)*) };
    let body = if return_type == signature.output {
        quote! {
            #function_invocation
        }
    } else {
        quote! {
            match #function_invocation {
                Ok(_) => 0,
                Err(e) => e as _
            }
        }
    };

    TokenStream::from(quote! {
        #[export_name = stringify!(#name)]
        #(#attributes)*
        pub unsafe extern "C" fn #export_name(#arguments) #return_type {
            #body
        }
    })
}

#[proc_macro_attribute]
pub fn export_syscall(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let function: syn::ItemFn = syn::parse(input).expect("export_sycall argument should be function declaration");
    return derive_test_wrapper(function);
}

#[proc_macro]
pub fn export_syscalls(input: TokenStream) -> TokenStream {
    let filename_token: syn::LitStr = syn::parse(input).expect("export_syscalls argument should be string");
    let filename = filename_token.value();
    let file = std::fs::read_to_string(format!("{}/{}", std::env::var("OUT_DIR").unwrap(), filename)).unwrap();

    let mut output = TokenStream::new();
    let ast = syn::parse_file(&file).unwrap();
    for item in ast.items {
        if let syn::Item::Fn(item_fn) = item {
            let new_fn = derive_test_wrapper(item_fn);
            output.extend(new_fn);
        }
    }

    output
}
