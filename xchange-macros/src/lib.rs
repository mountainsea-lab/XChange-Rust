// xchange-macros/src/lib.rs
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Attribute, FnArg, Ident, ItemFn, Lit, Pat, PatIdent, PathArguments, ReturnType, Type, TypePath,
    parse_macro_input,
};

/// Try to parse attribute macro arg as a string literal path: #[api_get("/path")]
fn parse_path_from_attr_args(attr_args: TokenStream) -> Option<String> {
    // Expect either empty or a single string literal
    if attr_args.is_empty() {
        return None;
    }
    // Try parse as LitStr
    let lit: Result<syn::LitStr, _> = syn::parse(attr_args);
    if let Ok(s) = lit {
        Some(s.value())
    } else {
        None
    }
}

/// Check if a type is Option<T> and return inner T if so
fn option_inner(ty: &Type) -> Option<Type> {
    if let Type::Path(TypePath { path, .. }) = ty {
        if path.segments.len() == 1 && path.segments[0].ident == "Option" {
            if let PathArguments::AngleBracketed(ab) = &path.segments[0].arguments {
                if let Some(syn::GenericArgument::Type(inner)) = ab.args.first() {
                    return Some(inner.clone());
                }
            }
        }
    }
    None
}

/// Check if a type is Vec<T> and return inner T
fn vec_inner(ty: &Type) -> Option<Type> {
    if let Type::Path(TypePath { path, .. }) = ty {
        if path.segments.len() == 1 && path.segments[0].ident == "Vec" {
            if let PathArguments::AngleBracketed(ab) = &path.segments[0].arguments {
                if let Some(syn::GenericArgument::Type(inner)) = ab.args.first() {
                    return Some(inner.clone());
                }
            }
        }
    }
    None
}

/// Detect HashMap<String,String> simple case
fn is_hashmap_string_string(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        if path.segments.len() == 1 && path.segments[0].ident == "HashMap" {
            if let PathArguments::AngleBracketed(ab) = &path.segments[0].arguments {
                let mut args = ab.args.iter();
                if let (Some(syn::GenericArgument::Type(k)), Some(syn::GenericArgument::Type(v))) =
                    (args.next(), args.next())
                {
                    if let Type::Path(TypePath { path: kp, .. }) = k {
                        if kp.segments.len() == 1 && kp.segments[0].ident == "String" {
                            if let Type::Path(TypePath { path: vp, .. }) = v {
                                if vp.segments.len() == 1 && vp.segments[0].ident == "String" {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

/// Extract the Ok type T from -> Result<T, E>, fall back to serde_json::Value
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

/// Parameter role discovered from attributes
#[derive(Clone)]
enum ParamRole {
    Query,
    Form,
    Json,
    Header(Option<String>),
    Timestamp(Option<String>), // optional key name
    Signature {
        key: Option<String>,
        header: Option<String>,
    },
    Auto,
}

/// Parse per-parameter attributes into ParamRole
fn parse_param_role(attrs: &[Attribute]) -> Option<ParamRole> {
    for attr in attrs {
        if attr.path().is_ident("query") {
            return Some(ParamRole::Query);
        } else if attr.path().is_ident("form") {
            return Some(ParamRole::Form);
        } else if attr.path().is_ident("json") || attr.path().is_ident("body") {
            return Some(ParamRole::Json);
        } else if attr.path().is_ident("timestamp") {
            // parse optional #[timestamp(name = "ts")]
            let mut name: Option<String> = None;
            let _ = attr.parse_args_with(|input: syn::parse::ParseStream| {
                while !input.is_empty() {
                    let ident: Ident = input.parse()?;
                    let _: syn::Token![=] = input.parse()?;
                    let lit: Lit = input.parse()?;
                    if ident == "name" {
                        if let Lit::Str(s) = lit {
                            name = Some(s.value());
                        }
                    }
                    if input.peek(syn::Token![,]) {
                        let _: syn::Token![,] = input.parse()?;
                    }
                }
                Ok(())
            });
            return Some(ParamRole::Timestamp(name));
        } else if attr.path().is_ident("sig") || attr.path().is_ident("signature") {
            // parse #[sig(key="signature", header="X-SIGN")]
            let mut key: Option<String> = None;
            let mut header: Option<String> = None;
            let _ = attr.parse_args_with(|input: syn::parse::ParseStream| {
                while !input.is_empty() {
                    let ident: Ident = input.parse()?;
                    let _: syn::Token![=] = input.parse()?;
                    let lit: Lit = input.parse()?;
                    if ident == "key" {
                        if let Lit::Str(s) = lit {
                            key = Some(s.value());
                        }
                    } else if ident == "header" {
                        if let Lit::Str(s) = lit {
                            header = Some(s.value());
                        }
                    }
                    if input.peek(syn::Token![,]) {
                        let _: syn::Token![,] = input.parse()?;
                    }
                }
                Ok(())
            });
            return Some(ParamRole::Signature { key, header });
        } else if attr.path().is_ident("header") {
            // parse #[header = "NAME"]
            let mut name: Option<String> = None;
            let _ = attr.parse_args_with(|input: syn::parse::ParseStream| {
                if !input.is_empty() {
                    let lit: Lit = input.parse()?;
                    if let Lit::Str(s) = lit {
                        name = Some(s.value());
                    }
                }
                Ok(())
            });
            return Some(ParamRole::Header(name));
        }
    }
    None
}

/// whether the HTTP method allows a request body
fn method_allows_body(method: &str) -> bool {
    matches!(method, "POST" | "PUT" | "PATCH")
}

/// Core expansion function used for GET/POST/DELETE/PATCH
fn expand_api_core(method: &str, attr_tokens: TokenStream, input: TokenStream) -> TokenStream {
    // parse optional path passed to attribute like #[api_post("/api/v3/order")]
    let path_opt = parse_path_from_attr_args(attr_tokens);

    let input_fn = parse_macro_input!(input as ItemFn);

    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    let fn_name = &fn_sig.ident;
    let fn_asyncness = &fn_sig.asyncness;
    let fn_inputs = &fn_sig.inputs;
    let fn_output = &fn_sig.output;

    // decide response ok type
    let ok_ty = extract_ok_type(fn_output);

    // collect parameter metadata
    struct ParamInfo {
        ident: Ident,
        ty: Type,
        role: ParamRole,
        is_option: bool,
        is_vec: bool,
        is_hashmap_ss: bool,
    }

    let mut params: Vec<ParamInfo> = Vec::new();

    for arg in fn_inputs.iter() {
        if let FnArg::Typed(pat_ty) = arg {
            // only support simple identifier patterns
            let ident = if let Pat::Ident(PatIdent { ident, .. }) = &*pat_ty.pat {
                ident.clone()
            } else {
                panic!("proc-macro: only simple identifier parameters supported (no patterns)");
            };
            let ty = (*pat_ty.ty).clone();
            let role = parse_param_role(&pat_ty.attrs).unwrap_or(ParamRole::Auto);
            let is_option = option_inner(&ty).is_some();
            let is_vec = vec_inner(&ty).is_some()
                || option_inner(&ty)
                    .and_then(|inner| vec_inner(&inner))
                    .is_some();
            let is_hashmap_ss = is_hashmap_string_string(&ty);

            params.push(ParamInfo {
                ident,
                ty,
                role,
                is_option,
                is_vec,
                is_hashmap_ss,
            });
        }
    }

    // Generate token streams that will push into explicit_query/form/headers
    let mut push_query_tokens = Vec::new();
    let mut push_form_tokens = Vec::new();
    let mut push_header_tokens = Vec::new();
    let mut json_body_ident: Option<Ident> = None;
    let mut timestamp_param: Option<(Ident, String)> = None;
    let mut signature_param: Option<(Ident, Option<String>, Option<String>)> = None;

    for p in &params {
        let name_str = p.ident.to_string();
        let ident = &p.ident;
        match &p.role {
            ParamRole::Header(name_opt) => {
                // header may have explicit name or use param name
                let header_key = if let Some(n) = name_opt {
                    n.clone()
                } else {
                    name_str.clone()
                };
                if p.is_option {
                    push_header_tokens.push(quote! {
                        if let Some(v) = &#ident {
                            explicit_headers.push((#header_key.to_string(), v.to_string()));
                        }
                    });
                } else {
                    push_header_tokens.push(quote! {
                        explicit_headers.push((#header_key.to_string(), #ident.to_string()));
                    });
                }
            }
            ParamRole::Query => {
                if p.is_hashmap_ss {
                    push_query_tokens.push(quote! {
                        for (k, v) in #ident.iter() {
                            explicit_query.push((k.clone(), v.clone()));
                        }
                    });
                } else if p.is_vec {
                    push_query_tokens.push(quote! {
                        for v in #ident.iter() {
                            explicit_query.push((#name_str.to_string(), v.to_string()));
                        }
                    });
                } else if p.is_option {
                    push_query_tokens.push(quote! {
                        if let Some(v) = &#ident {
                            explicit_query.push((#name_str.to_string(), v.to_string()));
                        }
                    });
                } else {
                    push_query_tokens.push(quote! {
                        explicit_query.push((#name_str.to_string(), #ident.to_string()));
                    });
                }
            }
            ParamRole::Form => {
                if p.is_hashmap_ss {
                    push_form_tokens.push(quote! {
                        for (k, v) in #ident.iter() {
                            explicit_form.push((k.clone(), v.clone()));
                        }
                    });
                } else if p.is_vec {
                    push_form_tokens.push(quote! {
                        for v in #ident.iter() {
                            explicit_form.push((#name_str.to_string(), v.to_string()));
                        }
                    });
                } else if p.is_option {
                    push_form_tokens.push(quote! {
                        if let Some(v) = &#ident {
                            explicit_form.push((#name_str.to_string(), v.to_string()));
                        }
                    });
                } else {
                    push_form_tokens.push(quote! {
                        explicit_form.push((#name_str.to_string(), #ident.to_string()));
                    });
                }
            }
            ParamRole::Json => {
                if json_body_ident.is_none() {
                    json_body_ident = Some(ident.clone());
                } else {
                    // ignore extras
                }
            }
            ParamRole::Timestamp(name_maybe) => {
                let key = name_maybe
                    .clone()
                    .unwrap_or_else(|| "timestamp".to_string());
                timestamp_param = Some((ident.clone(), key));
            }
            ParamRole::Signature { key, header } => {
                signature_param = Some((ident.clone(), key.clone(), header.clone()));
            }
            ParamRole::Auto => {
                if method_allows_body(method) {
                    // default to form
                    if p.is_hashmap_ss {
                        push_form_tokens.push(quote! {
                            for (k, v) in #ident.iter() {
                                explicit_form.push((k.clone(), v.clone()));
                            }
                        });
                    } else if p.is_vec {
                        push_form_tokens.push(quote! {
                            for v in #ident.iter() {
                                explicit_form.push((#name_str.to_string(), v.to_string()));
                            }
                        });
                    } else if p.is_option {
                        push_form_tokens.push(quote! {
                            if let Some(v) = &#ident {
                                explicit_form.push((#name_str.to_string(), v.to_string()));
                            }
                        });
                    } else {
                        push_form_tokens.push(quote! {
                            explicit_form.push((#name_str.to_string(), #ident.to_string()));
                        });
                    }
                } else {
                    // default to query
                    if p.is_hashmap_ss {
                        push_query_tokens.push(quote! {
                            for (k, v) in #ident.iter() {
                                explicit_query.push((k.clone(), v.clone()));
                            }
                        });
                    } else if p.is_vec {
                        push_query_tokens.push(quote! {
                            for v in #ident.iter() {
                                explicit_query.push((#name_str.to_string(), v.to_string()));
                            }
                        });
                    } else if p.is_option {
                        push_query_tokens.push(quote! {
                            if let Some(v) = &#ident {
                                explicit_query.push((#name_str.to_string(), v.to_string()));
                            }
                        });
                    } else {
                        push_query_tokens.push(quote! {
                            explicit_query.push((#name_str.to_string(), #ident.to_string()));
                        });
                    }
                }
            }
        }
    }

    // method token ident
    let method_ident = format_ident!("{}", method);
    // ok type tokens
    let ok_ty_tokens = ok_ty;
    // path string
    let path_literal = match path_opt {
        Some(p) => p,
        None => "/".to_string(),
    };

    // Build final generated function
    // Key steps:
    // 1. Build explicit_query, explicit_form, explicit_headers from params
    // 2. If timestamp param exists -> call factory.create_value() and push into explicit_query
    // 3. canonical_pairs = crate::client::build_canonical_pairs(explicit_query.clone(), explicit_form.clone())
    // 4. If signer exists -> call signer.digest(&RestInvocation{...}) and insert signature to query or header based on annotation
    // 5. Apply explicit collections to reqwest RequestBuilder, set form/json if needed
    // 6. Send & parse response into Ok type
    //
    let timestamp_insertion = if let Some((ref ts_ident, ref ts_key)) = timestamp_param {
        quote! {
            // call TimestampFactory.create_value()
            let ts_val = #ts_ident.create_value();
            explicit_query.push((#ts_key.to_string(), ts_val.to_string()));
        }
    } else {
        quote! {}
    };

    let signature_handling = if let Some((ref signer_ident, ref key_opt, ref header_opt)) =
        signature_param
    {
        // default signature key name
        let sig_key = key_opt
            .as_ref()
            .map(|s| s.clone())
            .unwrap_or_else(|| "signature".to_string());
        let header_name = header_opt.clone();
        quote! {
            // build canonical pairs for signing (runtime helper)
            let canonical_pairs = crate::client::build_canonical_pairs(explicit_query.clone(), explicit_form.clone());
            let invocation = crate::client::RestInvocation {
                method: #method.to_string(),
                path: #path_literal.to_string(),
                query: canonical_pairs.clone(),
                headers: explicit_headers.clone(),
                body: None,
            };
            // call digest
            let sig_val = #signer_ident.digest(&invocation);
            // insert signature into query or header
            match #header_name {
                Some(ref hname) => {
                    explicit_headers.push((hname.clone(), sig_val));
                },
                None => {
                    explicit_query.push((#sig_key.to_string(), sig_val));
                }
            }
        }
    } else {
        quote! {}
    };

    let json_body_setting = if let Some(ref body_ident) = json_body_ident {
        // if json body present: prefer json; remove any conflicting form keys
        quote! {
            // serialize JSON body
            let body_val = serde_json::to_value(&#body_ident).map_err(crate::client::HttpError::Reqwest)?;
            // set json body later with req = req.json(&body_val);
        }
    } else {
        quote! {}
    };

    // Now generate function body
    let gen_fun = quote! {
        #fn_vis #fn_asyncness fn #fn_name(&self, #fn_inputs) #fn_output {
            let url = format!("{}{}", self.base_url, #path_literal);
            let client = self.client.clone();

            client.execute(move |client_ref| {
                let client = client.clone();
                let url = url.clone();
                async move {
                    let mut explicit_query: Vec<(String,String)> = Vec::new();
                    let mut explicit_form: Vec<(String,String)> = Vec::new();
                    let mut explicit_headers: Vec<(String,String)> = Vec::new();

                    #(#push_query_tokens)*
                    #(#push_form_tokens)*
                    #(#push_header_tokens)*

                    #timestamp_insertion
                    #signature_handling

                    let mut req = client.http.request(reqwest::Method::#method_ident, &url);

                    for (k,v) in &explicit_query {
                        req = req.query(&[(k.as_str(),v.as_str())]);
                    }
                    for (k,v) in &explicit_headers {
                        req = req.header(k.as_str(), v.as_str());
                    }

                    #json_body_setting

                    if !explicit_form.is_empty() && method_allows_body(#method) {
                        req = req.form(&explicit_form);
                    }

                    let resp = req.send().await.map_err(crate::client::HttpError::Reqwest)?;
                    if resp.status().is_success() {
                        let parsed = resp.json::<#ok_ty_tokens>().await.map_err(crate::client::HttpError::Reqwest)?;
                        Ok(parsed)
                    } else {
                        Err(crate::client::HttpError::Io(
                            std::io::Error::new(
                                std::io::ErrorKind::Other,
                                format!("HTTP status {}", resp.status())
                            )
                        ))
                    }
                }
            })
        }
    };

    gen_fun.into()
}

/// Exported attribute macros
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
