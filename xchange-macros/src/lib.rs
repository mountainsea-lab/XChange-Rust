// xchange-macros/src/lib.rs
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, parse::{Parse, ParseStream}, punctuated::Punctuated,
    token::Comma, Ident, ItemFn, LitStr, Expr, FnArg, Pat, PatIdent, PathArguments, Type, TypePath,
    ReturnType,
};

/// Macro parameter role definitions
#[derive(Debug)]
enum ParamRole {
    Query(Vec<Ident>),
    Form(Vec<Ident>),
    Json(Vec<Ident>),
    Header(Vec<(String, Ident)>), // (header_name, param_name)
}

/// Parse each macro role entry
struct RoleEntry {
    role: Ident,
    args: Punctuated<Ident, Comma>,
}

impl Parse for RoleEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let role: Ident = input.parse()?;
        let content;
        let _ = syn::parenthesized!(content in input);
        let args = Punctuated::parse_terminated(&content)?;
        Ok(RoleEntry { role, args })
    }
}

/// Parse macro input: path + role entries
struct ApiMacroInput {
    path: LitStr,
    roles: Vec<RoleEntry>,
}

impl Parse for ApiMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path: LitStr = input.parse()?;
        let mut roles = Vec::new();
        while input.peek(syn::Token![,]) {
            let _comma: syn::Token![,] = input.parse()?;
            if input.is_empty() { break; }
            let role: RoleEntry = input.parse()?;
            roles.push(role);
        }
        Ok(ApiMacroInput { path, roles })
    }
}

/// Extract Ok type from Result<T,E> fallback to serde_json::Value
fn extract_ok_type(rt: &ReturnType) -> proc_macro2::TokenStream {
    if let ReturnType::Type(_, boxed) = rt {
        if let Type::Path(TypePath { path, .. }) = boxed.as_ref() {
            for seg in &path.segments {
                if seg.ident == "Result" {
                    if let PathArguments::AngleBracketed(args) = &seg.arguments {
                        if let Some(syn::GenericArgument::Type(ok_ty)) = args.args.first() {
                            return quote! { #ok_ty };
                        }
                    }
                }
            }
        }
    }
    quote! { serde_json::Value }
}

/// Core expansion
fn expand_api_core(method: &str, attr_tokens: TokenStream, input: TokenStream) -> TokenStream {
    let api_input = parse_macro_input!(attr_tokens as ApiMacroInput);
    let path_literal = api_input.path.value();
    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    let fn_output = &fn_sig.output;

    let ok_ty = extract_ok_type(fn_output);

    // Build role maps
    let mut query_params = Vec::new();
    let mut form_params = Vec::new();
    let mut json_params = Vec::new();
    let mut header_params = Vec::new();

    for role_entry in api_input.roles {
        let role_name = role_entry.role.to_string().to_lowercase();
        match role_name.as_str() {
            "query" => query_params.extend(role_entry.args.into_iter()),
            "form" => form_params.extend(role_entry.args.into_iter()),
            "json" => json_params.extend(role_entry.args.into_iter()),
            "header" => {
                for arg in role_entry.args {
                    if let Some(name_str) = arg.to_string().strip_prefix("\"").and_then(|s| s.strip_suffix("\"")) {
                        header_params.push((name_str.to_string(), arg.clone()));
                    } else {
                        header_params.push((arg.to_string(), arg.clone()));
                    }
                }
            }
            _ => {}
        }
    }

    // Generate push tokens for function parameters
    let mut push_query_tokens = Vec::new();
    let mut push_form_tokens = Vec::new();
    let mut push_header_tokens = Vec::new();

    for arg in fn_sig.inputs.iter() {
        if let FnArg::Typed(pat_ty) = arg {
            let ident = if let Pat::Ident(PatIdent { ident, .. }) = &*pat_ty.pat { ident } else { continue; };
            if query_params.iter().any(|id| id == ident) {
                push_query_tokens.push(quote! {
                    explicit_query.push((stringify!(#ident).to_string(), #ident.to_string()));
                });
            }
            if form_params.iter().any(|id| id == ident) {
                push_form_tokens.push(quote! {
                    explicit_form.push((stringify!(#ident).to_string(), #ident.to_string()));
                });
            }
            if header_params.iter().any(|(_, id2)| id2 == ident) {
                for (hname, id2) in &header_params {
                    if id2 == ident {
                        push_header_tokens.push(quote! {
                            explicit_headers.push((#hname.to_string(), #ident.to_string()));
                        });
                    }
                }
            }
        }
    }

    let method_ident = format_ident!("{}", method);

    let gen_fun = quote! {
        #fn_vis #fn_sig {
            let url = format!("{}{}", self.base_url, #path_literal);
            let client_arc = self.client.clone();

            let mut explicit_query: Vec<(String,String)> = Vec::new();
            let mut explicit_form: Vec<(String,String)> = Vec::new();
            let mut explicit_headers: Vec<(String,String)> = Vec::new();

            #(#push_query_tokens)*
            #(#push_form_tokens)*
            #(#push_header_tokens)*

            self.client.execute(move || {
                let client_inner = client_arc.clone();
                let url_inner = url.clone();
                async move {
                    let mut req = client_inner.http.request(reqwest::Method::#method_ident, &url_inner);

                    for (k,v) in &explicit_query {
                        req = req.query(&[(k.as_str(), v.as_str())]);
                    }
                    for (k,v) in &explicit_headers {
                        req = req.header(k.as_str(), v.as_str());
                    }
                    if !explicit_form.is_empty() {
                        req = req.form(&explicit_form);
                    }

                    let resp = req.send().await.map_err(crate::client::HttpError::Reqwest)?;
                    if resp.status().is_success() {
                        let parsed = resp.json::<#ok_ty>().await.map_err(crate::client::HttpError::Reqwest)?;
                        Ok(parsed)
                    } else {
                        Err(crate::client::HttpError::Io(
                            std::io::Error::new(
                                std::io::ErrorKind::Other,
                                format!("HTTP {}", resp.status())
                            )
                        ))
                    }
                }
            }).await
        }
    };

    gen_fun.into()
}

/// Exported macros
#[proc_macro_attribute]
pub fn api_get(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_api_core("GET", attr, item)
}
#[proc_macro_attribute]
pub fn api_post(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_api_core("POST", attr, item)
}
#[proc_macro_attribute]
pub fn api_delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_api_core("DELETE", attr, item)
}
#[proc_macro_attribute]
pub fn api_patch(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_api_core("PATCH", attr, item)
}
