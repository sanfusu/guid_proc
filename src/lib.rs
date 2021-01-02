#![allow(dead_code)]
#![allow(unused_variables)]

use pest::Parser;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

extern crate pest;
extern crate proc_macro;
#[macro_use]
extern crate pest_derive;

pub(crate) mod guid_pest {
    #[derive(Parser)]
    #[grammar = "guid.pest"]
    pub(crate) struct Guid;
}

/// 用于解析类似于:
/// 72631e54-78a4-11d0-bcf7-00aa00b7b32a
/// 或
/// "72631e54-78a4-11d0-bcf7-00aa00b7b32a"
/// 格式的 guid
#[proc_macro]
pub fn guid_proc(input: TokenStream) -> TokenStream {
    guid_internal(input.into()).into()
}

// Used for internal and unit test
pub(crate) fn guid_internal(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let mut token_str = input.clone().to_string();
    token_str = token_str.replace(' ', "");
    token_str = token_str.replace('"', "");
    let guid_parsed = guid_pest::Guid::parse(guid_pest::Rule::guid, &token_str)
        .unwrap()
        .next()
        .unwrap();

    let mut data1: u32 = 0;
    let mut data2: u16 = 0;
    let mut data3: u16 = 0;
    let mut data4: Vec<u8> = Vec::new();
    for part in guid_parsed.into_inner() {
        match part.as_rule() {
            guid_pest::Rule::part1_u32 => {
                data1 = u32::from_str_radix(part.as_str(), 16).unwrap();
            }
            guid_pest::Rule::part2_u16 => {
                data2 = u16::from_str_radix(part.as_str(), 16).unwrap();
            }
            guid_pest::Rule::part3_u16 => {
                data3 = u16::from_str_radix(part.as_str(), 16).unwrap();
            }
            guid_pest::Rule::part4_u8_8 => {
                for byte in part.into_inner() {
                    data4.push(u8::from_str_radix(byte.as_str(), 16).unwrap());
                }
            }
            _ => {}
        }
    }
    quote! {
        Guid {
            data1:#data1,
            data2:#data2,
            data3:#data3,
            data4: [#(#data4,)*]
        }
    }
}

#[proc_macro_attribute]
pub fn guid(attr: TokenStream, item: TokenStream) -> TokenStream {
    let guid = guid_internal(attr.into());
    let struct_item: DeriveInput = syn::parse(item).unwrap();
    let ident = struct_item.clone().ident;
    let generics = struct_item.clone().generics;
    (quote! {
        #struct_item
        impl #generics #ident #generics {
            pub fn guid() -> Guid{
                #guid
            }
        }
    })
    .into()
}

#[cfg(test)]
mod test {
    use quote::quote;
    struct Guid {
        pub data1: u32,
        pub data2: u16,
        pub data3: u16,
        pub data4: [u8; 8],
    }
    #[test]
    fn test_guid() {
        let b = super::guid_internal(quote! {"72631e54-78a4-11d0-bcf7-00aa00b7b32a"});
        println!("{}", b.to_string());
        let a: Guid = Guid {
            data1: 2,
            data2: 3,
            data3: 4,
            data4: [1, 2, 3, 4, 5, 6, 7, 8],
        };
    }
}
