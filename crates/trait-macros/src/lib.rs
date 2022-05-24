#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate proc_macro;

use crate::syn::DeriveInput;    
use crate::proc_macro::TokenStream;

#[proc_macro_derive(GetRunnable)]
pub fn derive_get_runnable_trait(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let token = quote!(
        impl shell_props::GetRunnable for #name {
            fn create () -> std::boxed::Box<dyn shell_props::Runnable> {
                std::boxed::Box::from(Self::new())
            }
        }
    );
    token.into()
}
