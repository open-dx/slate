extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;

use syn::DeriveInput;
// use syn::Generics;
use syn::parse_macro_input;
// use syn::parse_quote;

#[proc_macro_derive(StyleProperty)]
pub fn asdf_style_property(input: TokenStream) -> TokenStream {
    let _ast = parse_macro_input!(input as DeriveInput);
    // TODO: Generate code for new typemap feature
    let gen = quote! {
        // Generated code goes here
    };
    gen.into()
}

#[proc_macro_derive(Unit)]
pub fn asdf_unit(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    
    let gen = quote! {
        #[automatically_derived]
        impl #name {
            pub fn new<U: Into<Unit>>(value: U) -> Self {
                Self(value.into())
            }
            
            pub fn unit(&self) -> &Unit {
                &self.0
            }
        }

        #[automatically_derived]
        impl From<Unit> for #name {
            fn from(unit: Unit) -> Self {
                Self(unit)
            }
        }

        #[automatically_derived]
        impl Into<Unit> for #name {
            fn into(self) -> Unit {
                self.0
            }
        }

        #[automatically_derived]
        impl Deref for #name {
            type Target = Unit<f32>;
            
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[automatically_derived]
        impl Into<StyleValue> for #name {
            fn into(self) -> StyleValue {
                StyleValue::#name(self)
            }
        }

        #[automatically_derived]
        impl Style for #name {
            //..
        }
    };

    gen.into()
}

#[proc_macro_derive(Rect)]
pub fn derive_rect(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    
    let gen = quote! {
        #[automatically_derived]
        impl #name {
            /// TODO
            #[inline(always)]
            pub fn new<U: Into<Unit<f32>>>(all: U) -> Self {
                let all = all.into();
                #name ::all(all, all, all, all)
            }
            
            /// TODO
            #[inline(always)]
            pub fn all<U1: Into<Unit<f32>>, U2: Into<Unit<f32>>, U3: Into<Unit<f32>>, U4: Into<Unit<f32>>>(top: U1, right: U2, bottom: U3, left: U4) -> Self {
                #name(Rect::all(top, right, bottom, left))
            }
            
            /// TODO
            #[inline(always)]
            pub fn xy<U1: Into<Unit<f32>> + Copy, U2: Into<Unit<f32>> + Copy>(x: U1, y: U2) -> Self {
                #name(Rect::all(y, x, y, x))
            }
        }
        
        #[automatically_derived]
        impl #name {
            /// TODO
            #[inline(always)]
            pub fn rect<'rect>(&'rect self) -> &'rect Rect {
                &self.0
            }
        }
        
        #[automatically_derived]
        impl Deref for #name {
            type Target = Rect;
            
            /// TODO
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        
        #[automatically_derived]
        impl Into<Rect> for #name {
            /// TODO
            fn into(self) -> Rect {
                self.0
            }
        }
        
        #[automatically_derived]
        impl Into<StyleValue> for #name {
            /// TODO
            fn into(self) -> StyleValue {
                StyleValue::#name(self)
            }
        }
        
        #[automatically_derived]
        impl Style for #name {
            //..
        }
    };

    gen.into()
}

#[proc_macro_derive(Size2d)]
pub fn derive_size2d(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
        let name = &ast.ident;
    
    let gen = quote! {
        impl #name {
            /// TODO
            #[inline(always)]
            pub fn new<U1: Into<Unit<f32>>, U2: Into<Unit<f32>>>(x: U1, y: U2) -> Self {
                #name ::xy(x, y)
            }
            
            /// TODO
            #[inline(always)]
            pub fn both<U1: Into<Unit<f32>>, U2: Into<Unit<f32>>>(x: U1, y: U2) -> Self {
                #name ::xy(x, y)
            }
            
            /// TODO
            #[inline(always)]
            pub fn xy<U1: Into<Unit<f32>>, U2: Into<Unit<f32>>>(x: U1, y: U2) -> Self {
                #name(Size2d(x.into(), y.into()))
            }
        }
        
        impl Deref for #name {
            type Target = Size2d;
            
            /// TODO
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        
        impl #name {
            /// TODO
            #[inline(always)]
            pub fn get_size_2d<'rect>(&'rect self) -> &'rect Size2d {
                &self.0
            }
        }
        
        impl Into<StyleValue> for #name {
            /// TODO
            fn into(self) -> StyleValue {
                StyleValue::#name(self)
            }
        }
        
        impl Style for #name {
            //..
        }
    };

    gen.into()
}

#[proc_macro_derive(Color)]
pub fn derive_color(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // Extract the name of the struct
    let name = &ast.ident;
    
    // Generate code
    let gen = quote! {
        #[automatically_derived]
        impl #name {
            /// TODO
            #[inline(always)]
            pub fn hex(hex: &str) -> Self {
                #name(Color::hex(hex).unwrap_or(Color::Transparent))
            }
        }
        
        #[automatically_derived]
        impl #name {
            /// TODO
            pub fn color(&self) -> &Color {
                &self.0
            }
        }

        #[automatically_derived]
        impl Deref for #name {
            type Target = Color;
            
            /// TODO
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[automatically_derived]
        impl Into<StyleValue> for #name {
            /// TODO
            fn into(self) -> StyleValue {
                StyleValue::#name(self)
            }
        }

        #[automatically_derived]
        impl Style for #name {
            //..
        }
    };

    gen.into()
}
