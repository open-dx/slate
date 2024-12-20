use core::any::Any;
use core::any::TypeId;
use core::fmt::Debug;
use core::ops::Deref;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use bumpalo::Bump;

use crate::surface::Context;
// use crate::surface::SurfaceError;
use crate::surface::SurfaceUpdate;
use crate::scaffold::Scaffold;
use crate::scaffold::ScaffoldError;
// use crate::style::Style;
// use crate::style::StyleProperty;
use crate::style::StyleSheet;
use crate::style::StyleSheet2;
use crate::event::EventPin;

//---
// Re-export the Element derive macro so it's available with the Element trait.
pub use chizel::Element;

pub enum Content<'content> {
    Text(&'content str),
    Image(&'content [u8]),
    WebView(&'content str),
}

/// Represents an item that can be drawn on a Surface.
pub trait Element: Send + Sync + Debug {
    /// Every element must have a unique identifier. This is typically
    /// auto-generated by the ElementNode upon creation but can be implemented
    /// directly for special cases.
    fn uuid(&self) -> Option<UUID> {
        None
    }
    
    /// Elements can provide a set of default meta values.
    fn meta(&self) -> Option<()> {
        None
    }
    
    /// TODO
    fn content(&self) -> Option<Content> {
        None
    }
    
    /// TODO
    #[inline(always)]
    fn draw(&self, ctx: Context) -> DrawFn {
        #[inline(always)]
        |_| Ok(())
    }
}

//---
/// TODO
pub trait ElementIndex: Copy + Clone + Debug + PartialEq + Eq + PartialOrd + Ord {
    //.
}

//---
/// TODO
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct UXID(UUID);

/// Style preference for working with UUIDs internally.
pub use uuid::Uuid as UUID;

impl ElementIndex for UUID {
    //..
}

//---
/// Wraps an ElementNode in the Scaffold'surface node graph.
#[derive(Debug)]
pub struct ElementNode<'element> {
    /// The uuid for this instance of an Element on a Surface.
    uuid: UUID, // TODO
    
    /// The inner element stored in a Scaffold'surface Element graph.
    element: Option<Box<dyn Element + 'element>>,
    
    /// The parent node of this node.
    events: Vec<Box<EventPin>>,
    
    /// The parent node of this node.
    stylesheet: StyleSheet2<'element>,
    
    /// A hash of the element's value.
    hash: u64,
}

impl<'element> ElementNode<'element> {
    /// Create a new instance of an ElementNode using the global allocator.
    /// 
    /// If the the element doesn't exist or doesn't provide a uuid, a new one
    /// is generated for the current node.
    pub fn new(element: Option<Box<dyn Element + 'element>>) -> Self {
        // Attempt to get the uuid from the element or generate a new one.
        let uuid = match element.as_ref() {
            Some(element) => match element.uuid() {
                Some(uuid) => uuid,
                None => UUID::new_v4()
            },
            
            // If the element doesn't exist, generate a new uuid.
            None => UUID::default(),
        };
        
        ElementNode {
            uuid,
            element,
            events: Vec::new(),
            stylesheet: StyleSheet2::new(),
            hash: 0,
        }
    }
}

impl<'element> core::fmt::Display for ElementNode<'element> {
    /// TODO
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ElementNode#{}", self.uuid())
    }
}

impl<'element> ElementNode<'element> {
    /// TODO: Should we return a reference here instead?
    pub fn uuid(&self) -> UUID {
        self.uuid
    }
    
    /// TODO
    pub fn hash(&self) -> u64 {
        self.hash
    }
    
    /// TODO
    pub fn set_hash(&mut self, hash: u64) {
        #[cfg(feature = "verbose")]
        tracing::debug!("Setting hash for {:} to {:}", self, hash);
        self.hash = hash;
    }
    
    /// TODO
    pub fn element(&self) -> Option<&dyn Element> {
        self.element.as_ref().map(|element| &**element)
    }
    
    /// TODO
    pub fn element_mut(&'element mut self) -> Option<&'element mut dyn Element> {
        self.element.as_mut().map(move |element| &mut **element)
    }
    
    /// TODO
    pub fn events(&self) -> &Vec<Box<EventPin>> {
        self.events.as_ref()
    }
    
    /// TODO
    pub fn events_mut(&mut self) -> &mut Vec<Box<EventPin>> {
        self.events.as_mut()
    }
    
    pub fn take_events(&mut self, other: &mut Vec<Box<EventPin, &Bump>, &Bump>) {
        self.events.extend(other.drain(..).map(|e| Box::new(*e)));
    }
    
    /// TODO
    pub fn set_element(&mut self, element: Option<Box<dyn Element + 'element>>) {
        self.element = element;
    }
    
    /// TODO
    pub fn alt(&self) -> Option<&str> {
        Some("TODO")
    }
    
    /// TODO
    pub fn content(&self) -> Option<Content<'_>> {
        self.element.as_ref().and_then(|e| e.content())
    }
    
    /// TODO
    pub fn stylesheet(&self) -> &StyleSheet2<'element> {
        &self.stylesheet
    }
    
    /// TODO
    pub fn stylesheet_mut(&mut self) -> &mut StyleSheet2<'element> {
        &mut self.stylesheet
    }
}

//---
/// TODO
#[derive(Copy, Clone, Debug)]
pub enum ElementNodeRel {
    /// TODO
    Parent,
    
    /// TODO
    Child,
}

//---
/// TODO
#[derive(Debug)]
pub enum ElementError {
    /// TODO
    Unknown(&'static str),
}

impl From<&'static str> for ElementError {
    /// TODO
    fn from(error: &'static str) -> Self {
        ElementError::Unknown(error)
    }
}

// TODO: Remove this when we bring back oops.
#[automatically_derived]
impl core::error::Error for ElementError {}

// TODO: Remove this when we bring back oops.
#[automatically_derived]
impl core::fmt::Display for ElementError {
    /// TODO
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ElementError::Unknown(error) => write!(f, "TerminalError::Unknown: {:}", error),
        }
    }
}

pub type DrawFn = fn(&mut Scaffold<'_>) -> Result<(), ScaffoldError>;

/// TODO
#[derive(Default, Debug)]
pub enum DrawReport {
    /// TODO
    Success(Vec<SurfaceUpdate>),
    
    /// TODO
    #[default]
    Noop,
}

//---
pub mod tests {
    // TODO: Remove this when we can set the UIxBlock.prelude from Chizel.
    use crate as slate;
    
    use alloc::boxed::Box;
    
    // use crate::surface::Surface;
    // use crate::surface::SurfaceError;
    // use crate::scaffold::Scaffold;
    
    use super::Element;
    use super::DrawFn;
    // use crate::element::DrawResult;
    use super::UUID;
    
    //---
    /// Example implementation of the Element type.
    /// 
    /// TODO: Make this private once Chalk provides some elements for examples.
    #[derive(Element, Default, Clone, Hash, Debug)]
    pub struct ElementTestImpl {
        /// The display name for the test element instance.
        #[prop(name: Into<String>)] // TODO
        name: Option<alloc::string::String>,
        
        /// An arbitrary numeric field for testing state change detection.
        number: usize,
    }

    #[automatically_derived]
    impl ElementTestImpl {
        /// Set the name of the Element.
        /// TODO: Auto-generated for each #[prop] when not supplied.
        pub fn with_name<N: Into<alloc::string::String>>(mut self, name: N) -> Self {
            self.name = Some(name.into());
            self // etc..
        }
    }

    impl ElementTestImpl {
        /// Implemented by one of the following scenarios:
        /// - The developer defines a `#[uuid]` attribute on the struct.
        /// - The developer defines a `#[uuid]` attribute on a field.
        /// - The developer defines a `#[prop(uuid)]` attribute.
        /// - The developer defines a `with_uuid(..)` method.
        /// 
        /// TODO: Evaluate this for safety (lol).
        pub fn with_uuid(self, _uuid: UUID) -> Self {
            // tracing::debug!("TODO: Apply custom UUID {:}!", _uuid);
            self // etc..
        }
        
        /// Props can also be defined by the developer explicitly for greater
        /// control over the element's inputs.
        pub fn with_number(mut self, number: usize) -> Self {
            self.number = number;
            self // etc..
        }
    }
    
    impl ElementTestImpl {
        /// TODO
        pub fn name(&self) -> Option<&str> {
            self.name.as_ref().map(|name| name.as_str())
        }
        
        /// TODO
        pub fn number(&self) -> usize {
            self.number
        }
    }
    
    // impl ElementTestImpl {
    //     /// The `chizel::render` attribute is used to generate a render function 
    //     /// for the element using the `chizel::uix!` macro. It does a number of 
    //     /// things, including simplifying the method signature.
    //     /// 
    //     /// Note: Until this is implemented, the `chizel::render` attribute is just 
    //     ///   throwing it all away (you can't call it).
    //     /// 
    //     /// Use this space to freely design the render api.
    //     // #[chizel::render]
    //     fn render<E: Element>(&self, _children: &[E]) -> DrawFn {
    //         chizel::uix! {
    //             <ElementTestImpl name="First Child of ElementTestImpl">
    //                 <ElementTestImpl name="First Child of First Child of ElementTestImpl" />
    //                 <ElementTestImpl name="Second Child of First Child of ElementTestImpl" />
    //                 // {_children}
    //             </ElementTestImpl>
    //         }
    //     }
    // }
    
    #[test]
    fn test_element() {
        let element = ElementTestImpl::default()
            .with_uuid(UUID::new_v4())
            .with_name("Test Element")
            .with_number(42);
        
        // Assert that the uuid gets set.
        // assert_ne!(element.uuid(), Some(UUID::default()));
        assert_eq!(element.uuid(), None);
        
        // Assert that the name gets set.
        assert_eq!(element.name(), Some("Test Element"));
        
        // Assert that the number gets set.
        assert_eq!(element.number, 42);
    }
}
