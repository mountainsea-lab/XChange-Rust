use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

/// 自动生成 query/header/body 参数展开
fn generate_param_stmts(arg_name: &syn::Ident) -> proc_macro2::TokenStream {
    // 简单规则示例：Option<T>/basic 类型作为 query，其它复杂类型作为 body
    quote! {
        // query 参数
        if let Some(val) = &#arg_name {
            for (k, v) in convert_to_query(stringify!(#arg_name), val) {
                req = req.query(&[(k, v)]);
            }
        }
    }
}

/// 公共实现宏生成逻辑
fn generate_api(attr: &str, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_args = &input_fn.sig.inputs;
    let fn_vis = &input_fn.vis;
    let fn_output = &input_fn.sig.output;

    // HTTP 方法
    let method_ident = syn::Ident::new(attr, proc_macro2::Span::call_site());

    // 遍历参数生成 query/header/body 处理逻辑
    let mut param_stmts = Vec::new();
    for arg in fn_args {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                param_stmts.push(generate_param_stmts(&pat_ident.ident));
            }
        }
    }

    let expanded = quote! {
        #fn_vis async fn #fn_name(&self, #fn_args) #fn_output {
            let mut req = self.client.http.request(reqwest::Method::#method_ident, &format!("{}{}", self.base_url, stringify!(#fn_name)));

            #(#param_stmts)*

            let resp = req.send().await.map_err(HttpError::Reqwest)?;
            if resp.status().is_success() {
                resp.json().await.map_err(HttpError::Reqwest)
            } else {
                Err(HttpError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("HTTP status {}", resp.status())
                )))
            }
        }
    };

    TokenStream::from(expanded)
}

/// 宏入口
#[proc_macro_attribute]
pub fn api_get(_args: TokenStream, input: TokenStream) -> TokenStream {
    generate_api("GET", input)
}

#[proc_macro_attribute]
pub fn api_post(_args: TokenStream, input: TokenStream) -> TokenStream {
    generate_api("POST", input)
}

#[proc_macro_attribute]
pub fn api_delete(_args: TokenStream, input: TokenStream) -> TokenStream {
    generate_api("DELETE", input)
}

#[proc_macro_attribute]
pub fn api_patch(_args: TokenStream, input: TokenStream) -> TokenStream {
    generate_api("PATCH", input)
}
