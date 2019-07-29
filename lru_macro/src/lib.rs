extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn hola(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    //let ast: syn::DeriveInput = syn::parse(item).unwrap();

    let gen = quote!{
        fn hello(){
            println!("Yo bro!");
        }
    };
    
    gen.into()
}

#[proc_macro_attribute]
pub fn wip(_attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("Item: {}", item);
    let ast: syn::ItemFn = syn::parse(item).unwrap();

    let gen = quote!{
        #ast
        fn sherp(){
            println!("Yo bro!");
        }
    };
    
    gen.into()
}
