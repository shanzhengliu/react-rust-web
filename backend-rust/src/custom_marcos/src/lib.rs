extern crate proc_macro;


use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};


//marco for calculate the running time of a function which is pub async
#[proc_macro_attribute]
pub fn running_time_derive(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let func_name = &func.sig.ident;
    let func_inputs = &func.sig.inputs;
    let func_output = &func.sig.output;
    let func_block = &func.block;
    let func_attrs = &func.attrs;

    let expanded = quote! {
        #(#func_attrs)*
        pub async fn #func_name(#func_inputs) #func_output {
            let start_time = std::time::Instant::now();
            let result = async { #func_block }.await;
            let end_time = std::time::Instant::now();
            let duration = end_time.duration_since(start_time);
            println!("Function {} took {:?}", stringify!(#func_name), duration);
            result
        }
    };

    TokenStream::from(expanded)
}



