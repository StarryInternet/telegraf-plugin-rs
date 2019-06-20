// Copyright 2019. Starry, Inc. All Rights Reserved.
//
// MIT License
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Software written by Preston Carpenter <pcarpenter@starry.com>
#![recursion_limit = "264"]

extern crate proc_macro;

use std::ffi::CString;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::{self, Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    Expr, ExprBlock, ItemFn, LitByteStr, LitStr, Token
};

#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables,
    unused_imports,
    dead_code
)]

struct Args {
    description: CString,
    sample_config: CString
}

impl Parse for Args {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let mut vars = Punctuated::<LitStr, Token![,]>::parse_terminated(input)?.into_iter();
        let description = vars
            .next()
            .map(|v| CString::new(v.value()).expect("Invalid CString"))
            .expect("Need two arguments: description and sample_config");
        let sample_config = vars
            .next()
            .as_ref()
            .map(|v| CString::new(v.value()).expect("Invalid CString"))
            .expect("Need two arguments: description and sample_config");
        Ok(Args {
            description,
            sample_config
        })
    }
}

#[proc_macro_attribute]
pub fn link_to_go(args: TokenStream, input: TokenStream) -> TokenStream {
    let fn_gather_user = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(args as Args);

    let description_string =
        LitByteStr::new(args.description.as_bytes_with_nul(), Span::call_site());
    let fn_description: ItemFn = parse_quote! {
        #[no_mangle]
        unsafe extern "C" fn description() -> *const libc::c_char {
            #description_string as *const _ as *const libc::c_char
        }
    };

    let sample_config_string =
        LitByteStr::new(args.sample_config.as_bytes_with_nul(), Span::call_site());
    let fn_sample_config: ItemFn = parse_quote! {
        #[no_mangle]
        unsafe extern "C" fn sample_config() -> *const libc::c_char {
            #sample_config_string as *const _ as *const libc::c_char
        }
    };

    let call_user_code = Expr::Block(ExprBlock {
        attrs: vec![],
        label: None,
        block: (*fn_gather_user.block).clone()
    });
    let fn_gather: ItemFn = parse_quote! {
        #[no_mangle]
        extern "C" fn gather() {
            #call_user_code
        }
    };

    let telegraf_api_decl: proc_macro2::TokenStream = parse_quote! {
        #[repr(C)]
        enum Type {
            TYPE_INT = 0
        }

        #[repr(C)]
        union Value {
            int_: libc::c_int
        }

        impl From<i32> for go_value {
            fn from(num: i32) -> Self {
                go_value {
                    type_: go_value_TYPE_INT,
                    value: go_value__bindgen_ty_2 {
                        int_: i32::from(num)
                    }
                }
            }
        }

        mod __hidden {
            pub fn add_generic(
                management: String,
                tags: std::collections::HashMap<String, String>,
                fields: std::collections::HashMap<String, super::go_value>,
                timestamp: Option<std::time::SystemTime>,
                add_func: unsafe extern "C" fn(
                    measurment: *mut libc::c_char,
                    tags: *mut super::tag,
                    tags_size: libc::c_int,
                    fields: *mut super::field,
                    fields_size: libc::c_int,
                    unix_sec: i64,
                    unix_nsec: i64)
            ) {
                let management = std::ffi::CString::new(management).unwrap();

                let tags = tags
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            std::ffi::CString::new(k).expect("Not a C string"),
                            std::ffi::CString::new(v).expect("Not a C string")
                        )
                    })
                    .collect::<std::collections::HashMap<std::ffi::CString, std::ffi::CString>>();
                let mut tags_list = Vec::with_capacity(tags.len());
                for (key, value) in tags.iter() {
                    tags_list.push(super::tag {
                        key: key.as_ptr() as *mut _,
                        value: value.as_ptr() as *mut _
                    })
                }

                let fields = fields
                    .into_iter()
                    .map(|(k, v)| (std::ffi::CString::new(k).expect("Not a C string"), v))
                    .collect::<std::collections::HashMap<std::ffi::CString, super::go_value>>();
                let mut fields_list = Vec::with_capacity(fields.len());
                for (key, &value) in fields.iter() {
                    fields_list.push(super::field {
                        key: key.as_ptr() as *mut _,
                        value
                    })
                }

                let unix_time = timestamp
                    .map(|timestamp| timestamp.duration_since(std::time::UNIX_EPOCH)
                         .expect("Time went backwards"))
                    .unwrap_or_else(|| std::time::Duration::new(0, 0));

                unsafe {
                    add_func(
                        management.as_ptr() as *mut _,
                        tags_list.as_mut_ptr(),
                        tags_list.len() as i32,
                        fields_list.as_mut_ptr(),
                        fields_list.len() as i32,
                        unix_time.as_secs() as i64,
                        unix_time.as_nanos() as i64
                    )
                }
            }
        }

        fn AddField(
            management: String,
            tags: std::collections::HashMap<String, String>,
            fields: std::collections::HashMap<String, go_value>,
            timestamp: Option<std::time::SystemTime>
        ) {
            self::__hidden::add_generic(management, tags, fields, timestamp, add_field);
        }

        fn AddGauge(
            management: String,
            tags: std::collections::HashMap<String, String>,
            fields: std::collections::HashMap<String, go_value>,
            timestamp: Option<std::time::SystemTime>
        ) {
            self::__hidden::add_generic(management, tags, fields, timestamp, add_gauge);
        }

        fn AddCounter(
            management: String,
            tags: std::collections::HashMap<String, String>,
            fields: std::collections::HashMap<String, go_value>,
            timestamp: Option<std::time::SystemTime>
        ) {
            self::__hidden::add_generic(management, tags, fields, timestamp, add_counter);
        }
    };
    let c_prelude: proc_macro2::TokenStream = syn::parse_str(include_str!("gen.rs")).unwrap();

    TokenStream::from(quote! {
        #c_prelude
        #fn_gather
        #telegraf_api_decl
        #fn_description
        #fn_sample_config
    })
}
