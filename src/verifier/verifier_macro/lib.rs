extern crate proc_macro;
use proc_macro::TokenStream;


// testing purposes
static KANI : bool = false;

// create custom macro to include #[kani-proof] and #[no mangle] on different make optios
#[proc_macro_attribute]
pub fn select(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let attribute = if KANI {
        "#[kani::proof]"
    } else {
        "#[no_mangle]"
    }; 
    let func : TokenStream = [attribute, &item.to_string()].join(" ").parse().unwrap();
    func
}
