#![allow(dead_code)]
#![allow(unused_variables)]

use proc_macro::TokenStream;
use quote::quote;

extern crate proc_macro;

/// 用于解析类似于:
/// 72631e54-78a4-11d0-bcf7-00aa00b7b32a
/// 或
/// "72631e54-78a4-11d0-bcf7-00aa00b7b32a"
/// 格式的 guid
#[proc_macro]
pub fn guid(input: TokenStream) -> TokenStream {
    guid_internal(input.into()).into()
}

// Used for internal and unit test
pub(crate) fn guid_internal(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let token_str = input.clone().to_string();
    let guid_args: guid::Guid = token_str.parse().unwrap();
    let data1 = guid_args.data1;
    let data2 = guid_args.data2;
    let data3 = guid_args.data3;
    let data4 = guid_args.data4;
    quote! {
        Guid{
            data1:#data1,
            data2:#data2,
            data3:#data3,
            data4:[#(#data4,)*]
        }
    }
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
