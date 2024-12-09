mod uix;

mod styles;

mod element;

mod util;

//---
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use quote::quote;

/// Derive Element for a struct.
#[proc_macro_derive(Element, attributes(element, prop, props, child, children, render))]
pub fn derive_element(token_buf: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let element = crate::element::ElementDeriveBuilder::new(syn::parse_macro_input!(token_buf));
    
    #[cfg(feature = "debug")]
    println!("Attempting to derive Element from: {:?}", element.ast.ident);
    
    element.build().into()
}

/// Write a UIx block using a JSX-like syntax.
#[proc_macro]
pub fn uix(token_buf: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match syn::parse::<crate::uix::UIxBlock>(token_buf) {
        Ok(uix_block) => {
            // TODO
            proc_macro::TokenStream::from(quote! {
                // Outer block helps keep the parent use-space pure.
                {
                    // Some Sensible imports for composability in the UIx block.
                    // TODO: Allow this to be set by the caller (with sensible defaults).
                    use slate::style::StyleSheet;
                    use slate::style::primitive::*;
                    use slate::style::primitive::Unit::*;
                    use slate::style::property::*;
                    use slate::event::EventPin::*;
                    
                    // Some complex types require encapsulation in braces, parens, etc.
                    // Allow a subset of these for convenience.
                    #[allow(unnecessary_braces)]
                    #[allow(unused_braces)]
                    #[allow(unused_parens)]
                    move |scaffold| {
                        #uix_block
                        Ok(())
                    }
                }
            })
        }
        Err(error) => {
            // Print the token_buf token stream for debugging
            // println!("Failed to parse the following token_buf: {:?}", input_clone);
            // Convert the syn::Error into a compiler error
            let compile_error = error.to_compile_error();
            compile_error.into() // </3
        }
    }
}

/// Write a UIx block using a JSX-like syntax.
#[proc_macro]
pub fn styles(token_buf: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match syn::parse::<crate::styles::StylesBlock>(token_buf) {
        Ok(styles_block) => {
            // TODO
            proc_macro::TokenStream::from(quote! {
                #styles_block
            })
        }
        Err(error) => {
            // Print the token_buf token stream for debugging
            // println!("Failed to parse the following token_buf: {:?}", input_clone);
            // Convert the syn::Error into a compiler error
            let compile_error = error.to_compile_error();
            compile_error.into() // </3
        }
    }
}

/// TODO
#[proc_macro_attribute]
pub fn render(_ts1: proc_macro::TokenStream, _ts2: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // _ts1
    proc_macro::TokenStream::new()
}
