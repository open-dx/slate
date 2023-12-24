use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::collections::HashMap;

use syn::*;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;
// use syn::token::Brace;
// use syn::punctuated::Punctuated;

use quote::*;

use crate::element::ElementClassAttribute;
use crate::element::ElementEventAttribute;
use crate::element::ElementProp;
use crate::element::ElementStyleAttribute;
use crate::util::parse::get_attr_ident;

//---
/// Global element counter. Used to build unique identifiers for generated
/// names, doc comments, debug info, etc.
static ELEMENT_COUNT: AtomicUsize = AtomicUsize::new(0);

//---
/// TODO
#[derive(Default)]
pub struct UIxBlock {
    /// TODO
    roots: Vec<UIxElement>,
    
    /// TODO
    styles: HashMap<Ident, Vec<Expr>>,
}

impl UIxBlock {
    /// TODO
    pub fn new() -> Self {
        Self {
            roots: Vec::new(),
            styles: HashMap::new(),
        }
    }
}

impl Parse for UIxBlock {
    /// Lines starting with `#` are used to annotate statements within 
    /// the block and may be used to alter how the block is built.
    fn parse(token_buf: ParseStream) -> Result<Self> {
        let mut block = UIxBlock::new();
        
        while !token_buf.is_empty() {
            if token_buf.peek(syn::Token![.]) {
                let mut class_names = Vec::new();
                
                loop {
                    if token_buf.peek(syn::Token![.]) {
                        token_buf.parse::<syn::Token![.]>()?;
                        class_names.push(token_buf.parse::<syn::Ident>()?);
                        continue;
                    }
                    if token_buf.peek(syn::Token![,]) {
                        token_buf.parse::<syn::Token![,]>()?;
                        continue;
                    }
                    
                    // Nothing else allowed in the list.
                    // TODO: Better error here.
                    break;
                }
                
                let content;
                braced!(content in token_buf);
                let parsed_styles = content.parse_terminated(Expr::parse, syn::Token![,])?;
                
                for expr in parsed_styles {
                    for class_name in &class_names {
                        #[cfg(feature = "verbose")]
                        println!("Parsed Class Name: {:?}", class_name);
                                
                        block.styles.entry(class_name.clone())
                            .or_insert_with(Vec::new)
                            .push(expr.clone());
                    }
                }
                
                #[cfg(feature = "inspect")]
                println!("Parsed styles: {:#?}", block.styles);
            }
            
            match UIxElement::parse(token_buf) {
                Ok(mut element) => {
                    element.root = true;
                    
                    block.roots.push(element);
                }
                Err(error) => {
                    // Return an error if unable to recover
                    return Err(token_buf.error(format!("Couldn't parse element: {:}", error)));
                }
            }
        }

        Ok(block)
    }
}

impl ToTokens for UIxBlock {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let roots = &self.roots;
        
        let styles = self.styles.iter().map(|(key, value)| {
            #[cfg(feature = "verbose")]
            println!("Printing Class Block Variable: {:?}", key);
            
            let values = value.iter().map(|value| {
                quote! {
                    stylesheet.push( #value );
                }
            });
            
            quote! {
                let #key = move |stylesheet: &mut StyleSheet| {
                    #(#values)*
                };
            }
        });        
        
        #[cfg(feature = "inspect")]
        for root_element in roots {
            println!("Root Element: {:#?}", root_element);
        }
        
        tokens.extend(quote! {
            #(#styles)*
            #(#roots)*
        })
    }
}

/// TODO
#[derive(Debug, Clone, PartialEq)]
pub struct UIxElement {
    /// TODO: Make private.
    pub ident: Ident,
    
    /// TODO: Make private.
    pub prefix: String,
    
    /// TODO: Make private.
    pub root: bool,
    
    /// TODO: Make private.
    pub index: usize,
    
    /// TODO: Make private.
    pub is_closed: bool,
    
    /// TODO: Make private.
    pub events: Vec<ElementEventAttribute>,
    
    /// TODO: Make private.
    pub classes: Vec<ElementClassAttribute>,
    
    /// TODO: Make private.
    pub styles: Vec<ElementStyleAttribute>,
    
    /// TODO: Make private.
    pub props: Vec<ElementProp>,
    
    /// TODO: Make private.
    pub children: Vec<UIxElement>,
}

impl UIxElement {
    /// TODO
    pub fn new(ident: Ident) -> Self {
        Self {
            ident,
            prefix: String::new(),
            root: false,
            index: ELEMENT_COUNT.fetch_add(1, Ordering::SeqCst),
            is_closed: false,
            events: Vec::new(),
            styles: Vec::new(),
            classes: Vec::new(),
            props: Vec::new(),
            children: Vec::new(),
        }
    }
}

impl UIxElement {
    pub fn name(&self) -> String {
        format!("{:}_{:05}", self.prefix, self.index)
    }
}

impl Parse for UIxElement {
    /// Parses a single element, including any attributes, children, etc.
    /// 
    /// ### Example (Input)
    /// ```rust
    /// #[on(Click, click_handler_fn)]
    /// #[style(BackgroundColor, hexa("#FF0000", 0.5))]
    /// #[class("my-class-name")]
    /// <ElementTestImpl name="Second Root" number=0usize>
    ///     etc ..
    /// </ElementTestImpl>
    fn parse(token_buf: ParseStream) -> Result<Self> {
        #[cfg(feature = "verbose")]
        println!("---\nParsing a new Element!");
        
        let mut events = Vec::new();
        let mut classes = Vec::new();
        let mut styles = Vec::new();
        
        // Parse attributes, like:
        // - Event handlers: `#[on:click(..)]`
        // - Inline Styles: `#[style(background-color: #FF0000)]`
        // - Class Names: `#[class("my-class-name")]`
        loop {
            if !token_buf.peek(syn::Token![#]) {
                break;
            }
            // Parse attributes, like `#[attr]` ..
            let attributes = Attribute::parse_outer(token_buf)?;
            
            // .. and print them for debugging.
            #[cfg(feature = "inspect")]
            if attributes.len() > 0 {
                println!("Parsed Attributes: {:#?}", attributes);
            }
            
            for attr in attributes {
                match get_attr_ident(&attr) {
                    Some(attr_ident) => {
                        match attr_ident.to_string().as_str() {
                            // Found UUID, as in `#[uuid(3500dad6-cdef-442a-ba05-d20c1f3b1921)]`.
                            // Explicitly sets a unique identifier on the element.
                            "uuid" => {
                                #[cfg(feature = "verbose")]
                                println!("Parsed UUID Attribute: TODO");
                            },
                            // Found When, as in `#[when(n == 1)]`.
                            // Allows an element to be rendered conditionally.
                            "when" => {
                                #[cfg(feature = "verbose")]
                                println!("Parsed While Attribute: TODO");
                            },
                            // Found Each, as in `#[each(n in 0..4)]`.
                            // Allows an element to be rendered multiple times.
                            "each" => {
                                #[cfg(feature = "verbose")]
                                println!("Parsed For Attribute: TODO");
                            },
                            // Found Event, as in `#[event(Kind, fn1, fn2, etc ..)]`.
                            // Register's an event listener on the element.
                            "on" => events.push(ElementEventAttribute::try_from(&attr)?),
                            // Found Class, as in `#[class(class_expr)]`.
                            // Adds a class name to the element.
                            "class" => classes.push(ElementClassAttribute::try_from(&attr)?),
                            // Found Style, as in `#[style(Prop, Value)]`.
                            // Sets a style property on the element.
                            "style" => styles.push(ElementStyleAttribute::try_from(&attr)?),
                            // Found Documentation, as in `#[doc = "TODO"]` or `/// Something`.
                            // Set a line of documentation on the element.
                            "doc" => {
                                #[cfg(feature = "verbose")]
                                println!("Parsed Documentation Attribute: TODO");
                            },
                            _ => {
                                return Err(token_buf.error(format!("Unknown attribute ident: {:}", attr_ident)));
                            }
                        }
                    }
                    None => {
                        return Err(token_buf.error(format!("Failed to parse attribute ident: {:?}", attr)));
                    }
                }
            }
        }
        
        // Parse the initial leading-tag `<` token.
        token_buf.parse::<syn::Token![<]>()?;
        
        let name = Ident::parse(token_buf)?;
        
        let mut element = UIxElement::new(name);
        element.events.extend(events.drain(..));
        element.styles.extend(styles.drain(..));
        element.classes.extend(classes.drain(..));
        element.is_closed = false;
        
        loop {
            // Is this the end of the opening tag of the element? `>`
            if token_buf.peek(syn::Token![>]) {
                token_buf.parse::<syn::Token![>]>()?;
                break;
            }
            
            // Is this the end of the element? `/>`?
            if token_buf.peek(syn::Token![/]) && token_buf.peek2(syn::Token![>]) {
                token_buf.parse::<syn::Token![/]>()?;
                token_buf.parse::<syn::Token![>]>()?;
                element.is_closed = true;
                break;
            }
            
            // Successfully parsed, so advance the main parser
            element.props.push(token_buf.parse::<ElementProp>()?);
        }
        
        #[cfg(feature = "inspect")]
        eprintln!("Parsed props: {:#?}", element.props);

        if !element.is_closed {
            // We don't yet have a close condition, meaning no `/>` was found.
            // Parse either the closing tag or a set of children.
            
            #[cfg(feature = "verbose")]
            println!("Not a self-closing tag. Attempting to parse children or closing tag.");
            
            loop {
                // Check for a pair of </ tokens and parse the close tag ..
                if token_buf.peek(syn::Token![<]) && token_buf.peek2(syn::Token![/]) {
                    #[cfg(feature = "verbose")]
                    println!("Parsing Closing Tag ..");
                    
                    // Parse opening to the close tag as `</`.
                    token_buf.parse::<syn::Token![<]>()?;
                    token_buf.parse::<syn::Token![/]>()?;
                    
                    let closing_ident = token_buf.parse::<syn::Ident>()?;
                    if element.ident != closing_ident {
                        return Err(token_buf.error(format!("Expected closing tag for ident `{:}`; found `{:}`", element.ident, closing_ident)));
                    }
                    
                    token_buf.parse::<syn::Token![>]>()?;
                    
                    #[cfg(feature = "inspect")]
                    println!("Closed Tag: {:#?}", closing_ident);
                    
                    break; // Finished!
                } else {
                    #[cfg(feature = "verbose")]
                    println!("Parsing Child Element ..");
                    
                    // Otherwise, attempt to match a child element.
                    match UIxElement::parse(token_buf) {
                        Ok(child_element) => {
                            #[cfg(feature = "inspect")]
                            println!("Parsed child Element: {:#?}", child_element);
                            element.children.push(child_element);
                        }
                        Err(error) => {
                            #[cfg(feature = "debug")]
                            eprintln!("Failed to parse child Element: {:#?}", error);
                            return Err(token_buf.error(format!("Couldn't parse child of `{}`: {:}", element.ident, error)));
                        }
                    }
                }
            }
        }
        
        #[cfg(feature = "debug")]
        #[cfg(feature = "verbose")]
        println!("Parsed Element: {:#?}", element);
        
        Ok(element)
    }
}

impl ToTokens for UIxElement {
    /// Generate a token stream for UIxElement composition.
    /// 
    /// ### Example (Output)
    /// ```rust
    /// // Add the element to the scaffold ..
    /// scaffold.add({
    ///     ElementTestImpl::default()
    ///         .with_name("Second Root")
    ///         .with_number(0usize)
    /// })
    ///     // .. along with event handlers, styles, etc ..
    ///     .with_event_attr(EventKind::Click, click_handler_fn)
    ///     .with_style_attr(StyleProperty::BackgroundColor, hexa("#ff0000", 0.5))
    ///     // .. and finally, add children.
    ///     .with_children(|scaffold| {
    ///         etc..
    ///     });
    /// ````
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.ident;
        // let prefix = Ident::new(&self.prefix, ident.span());
        let var_name = Ident::new(&self.name(), ident.span());
        
        // Write props, where `prop="value"`, `prop={value}`, or `prop=[value]`
        // is converted to `.with_prop(value)`.
        let mut props = proc_macro2::TokenStream::new();
        for ElementProp { key, value } in &self.props {
            // Build the method name into a "with" method (for building the element's props).
            let method_name = Ident::new(&format!("with_{}", key), key.span());
            match value {
                Some(value) =>  props.extend(quote! { . #method_name ( #value ) }),
                None => props.extend(quote! { . #method_name ( #key ) }),
            }
        }
        
        // Construct `.with_event(..)` calls for each event.
        let mut events = proc_macro2::TokenStream::new();
        for event in &self.events {
            let event_kind = event.kind();
            for event_handler in event.handlers() {
                events.extend(quote! {
                    .with_event_attr(#event_kind, #event_handler)?
                });
            }
        }
        
        // Construct `.with_style(..)` calls for each style.
        let mut styles = proc_macro2::TokenStream::new();
        for style in &self.styles {
            let event_kind = style.expr();
            styles.extend(quote! {
                .with_style_attr(#event_kind)?
            });
        }
        
        // Construct `.with_style(..)` calls for each style.
        let mut classes = proc_macro2::TokenStream::new();
        for class_attrs in &self.classes {
            #[cfg(feature = "verbose")]
            println!("Class Expr: {:#?}", class_attrs);
            
            for class_expr in class_attrs.classes() {
                classes.extend(quote! {
                    .with_class_attr( #class_expr )?
                });
            }
        }
        
        // TODO: Write children, where a `<Child />` is converted to a scoped
        // block which builds additional nested elements.
        let children = &self.children;
        
        // TODO: Remove `.with_children()` when there are no children.
        tokens.extend(quote! {
            let #var_name = scaffold.add({
                #ident ::default() #props
            })?
                #events // .with_event_attr(..),*
                #styles // .with_style_attr(..),*
                #classes // .with_class_attr(..),*
                .with_children(|scaffold| {
                    #(#children)*
                    Ok(()) // TODO: Return a draw result.
                })?
                .build();
        });
    }
}
