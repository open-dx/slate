use core::alloc::Layout;
// use core::borrow::Borrow;
use core::hash::Hash;
use core::hash::Hasher;

use alloc::alloc::alloc;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::vec::Vec;

use ahash::AHasher;

use crate::element::Content;
use crate::element::Element;
use crate::element::ElementError;
use crate::element::ElementNode;
// use crate::element::DrawFn;
use crate::element::UUID;
use crate::event::ClickEvent;
use crate::event::Event;
use crate::event::EventPin;
use crate::event::EventHandlerFn;
use crate::event::EventStack;
use crate::style::StyleSheet;
// use crate::style::StyleProperty;
use crate::style::Style;
// use crate::xtra::HashMap;


#[cfg(feature = "bump")]
use crate::arena::Bump;
use crate::surface::Context;

//---
/// A lightweight single-pass builder for a tree of Element nodes.
/// 
/// Represents the intended state of a `Surface` for a.
/// 
/// ## Guide: Direct Scaffolding
/// 
/// In some cases you might want to build a scaffold directly (e.g., at runtime
/// on an embedded system, or in cases where reactivity isn't a primary goal).
/// 
/// In these cases you're encouraged to build a `Scaffold` directly and use it
/// to render the `Surface` in whatever way you need.
/// 
/// ```rust
/// let mut scaffold = Scaffold::new();
/// // TODO: etc..
/// ```
#[derive(Debug)]
pub struct Scaffold<'arena> {
    /// The node currently being built.
    element: Option<(&'arena dyn Element, Layout)>,
    
    /// The built stylesheet for this node.
    stylesheet: StyleSheet<'arena>,
    
    /// TODO
    #[cfg(feature = "bump")]
    slots: Vec<(), &'arena Bump>,
    
    /// TODO
    #[cfg(not(feature = "bump"))]
    slots: Vec<()>,
    
    /// TODO
    #[cfg(feature = "bump")]
    children: Vec<Scaffold<'arena>, &'arena Bump>,
    
    /// TODO
    #[cfg(not(feature = "bump"))]
    children: Vec<Scaffold<'arena>>,
    
    /// TODO
    events: EventStack<'arena>,
    
    /// The state of the hash after the last calculation.
    hash: Option<u64>,
    
    /// Hash of the element's internal state.
    hasher: AHasher,
    
    /// TODO
    #[cfg(feature = "bump")]
    arena: &'arena Bump,
}

impl<'arena> Scaffold<'arena> {
    /// Create a new Scaffold from a given Element.
    #[cfg(feature = "bump")]
    pub fn new_in(arena: &'arena Bump) -> Self {
        Scaffold {
            element: None,
            stylesheet: StyleSheet::new_in(arena),
            slots: Vec::new_in(arena),
            children: Vec::new_in(arena),
            events: EventStack::new_in(arena),
            hasher: AHasher::default(),
            hash: None,
            arena,
        }
    }
    
    #[cfg(not(feature = "bump"))]
    pub fn new() -> Self {
        Scaffold {
            element: None,
            stylesheet: StyleSheet::new(),
            slots: Vec::new(),
            children: Vec::new(),
            events: EventStack::new(),
            hasher: AHasher::default(),
            hash: None,
        }
    }
    
    /// TODO
    #[cfg(feature = "bump")]
    pub fn with_element<E: Element + Hash + 'arena>(mut self, mut element: E) -> Self {
        let element = self.arena.alloc(element);
        element.hash(&mut self.hasher);
        self.element = Some((element, Layout::new::<E>()));
        self // etc..
    }
    
    /// TODO
    #[cfg(not(feature = "bump"))]
    pub fn with_element<E: Element + Hash + 'arena>(mut self, mut element: E) -> Self {
        let element = Box::new(element);
        element.hash(&mut self.hasher);
        self.element = Some((element, Layout::new::<E>()));
        self // etc..
    }
    
    pub fn with_slot(mut self) -> Self {
        self.slots.push(());
        self // etc..
    }
    
    /// TODO
    pub fn is_empty(&self) -> bool {
        self.element.is_none() && self.children.is_empty()
    }
}

impl<'arena> Scaffold<'arena> {
    /// Provides immutable access to the element node of this Scaffold.
    #[cfg(feature = "bump")]
    pub fn get_element(&self) -> Option<&dyn Element> {
        self.element.map(|element| element.0)
    }
    
    /// Provides immutable access to the element node of this Scaffold.
    #[cfg(not(feature = "bump"))]
    pub fn get_element(&self) -> Option<&dyn Element> {
        self.element.as_ref().map(|element| &*element.0)
    }
    
    /// TODO
    pub fn take_element_boxed(&mut self) -> Option<Box<dyn Element>> {
        self.element.take().and_then(|(element, layout)| {
            unsafe {
                type DynPtr = (*mut u8, *const ());
                let (src_data_ptr, vtable_ptr): DynPtr = core::mem::transmute(element);
                
                let box_data_ptr = alloc(layout) as *mut u8;
                core::ptr::copy_nonoverlapping(src_data_ptr, box_data_ptr, layout.size());
                
                let element_ptr: *mut dyn Element = core::mem::transmute((box_data_ptr, vtable_ptr));
                let element_box = Box::from_raw(element_ptr);
                
                Some(element_box)
            }
        })
    }
    
    /// TODO
    pub fn stylesheet(&self) -> &StyleSheet<'arena> {
        &self.stylesheet
    }
    
    /// TODO
    pub fn stylesheet_mut(&mut self) -> &mut StyleSheet<'arena> {
        &mut self.stylesheet
    }
    
    /// TODO
    pub fn content(&self) -> Option<Content<'_>> {
        self.get_element().as_ref().and_then(|e| e.content())
    }
    
    //--
    /// Provides immutable access to the events of this Scaffold.
    #[cfg(feature = "bump")]
    pub fn events(&self) -> &Vec<Box<EventPin, &'arena Bump>, &'arena Bump> {
        self.events.as_ref()
    }
    
    /// Provides immutable access to the events of this Scaffold.
    #[cfg(not(feature = "bump"))]
    pub fn events(&self) -> &Vec<Box<EventPin>> {
        self.events.as_ref()
    }
    
    /// Provides mutable access to the events of this Scaffold.
    #[cfg(feature = "bump")]
    pub fn events_mut(&mut self) -> &mut Vec<Box<EventPin, &'arena Bump>, &'arena Bump> {
        self.events.as_mut()
    }
    
    /// Provides mutable access to the events of this Scaffold.
    #[cfg(not(feature = "bump"))]
    pub fn events_mut(&mut self) -> &mut Vec<Box<EventPin>> {
        self.events.as_mut()
    }
    
    //--
    /// Provides immutable access to the children of this Scaffold.
    #[cfg(feature = "bump")]
    pub fn children(&self) -> &Vec<Scaffold<'arena>, &'arena Bump> {
        self.children.as_ref()
    }
    
    /// Provides immutable access to the children of this Scaffold.
    #[cfg(not(feature = "bump"))]
    pub fn children(&self) -> &Vec<Scaffold<'_>> {
        self.children.as_ref()
    }
    
    /// Provides mutable access to the children of this Scaffold.
    #[cfg(feature = "bump")]
    pub fn children_mut(&mut self) -> &mut Vec<Scaffold<'arena>, &'arena Bump> {
        self.children.as_mut()
    }
    
    /// Provides mutable access to the children of this Scaffold.
    #[cfg(not(feature = "bump"))]
    pub fn children_mut(&mut self) -> &mut Vec<Scaffold<'arena>> {
        self.children.as_mut()
    }
}

impl<'arena> Scaffold<'arena> {
    /// TODO
    pub fn add<E>(&mut self, element: E) -> Result<&mut Self, ScaffoldError>
    where
        E: Element + Hash + 'arena,
    {
        let child_idx = self.children.len() + 1;
        
        #[cfg(feature = "bump")]
        let child = Scaffold::new_in(self.arena).with_element(element);
        
        #[cfg(not(feature = "bump"))]
        let child = Scaffold::new().with_element(element);
        
        self.children.push(child);
        self.children.last_mut()
            .ok_or(ScaffoldError::IndexOutOfBounds(child_idx))
    }
    
    /// TODO
    pub fn with_event_attr<H: Into<EventPin>>(&mut self, event_pin: H) -> Result<&mut Self, ScaffoldError> {
        self.events.push(Box::new_in(event_pin.into(), self.arena));
        Ok(self) // etc..
    }
    
    /// TODO
    pub fn with_style_attr<V: Style + 'static>(&mut self, style_value: V) -> Result<&mut Self, ScaffoldError> {
        self.stylesheet.push(style_value);
        Ok(self) // etc..
    }
    
    /// TODO
    pub fn with_class_attr<F: Fn(&mut StyleSheet<'arena>)>(&mut self, class_fn: F) -> Result<&mut Self, ScaffoldError> {
        class_fn(&mut self.stylesheet);
        Ok(self) // etc..
    }
    
    /// TODO
    pub fn with_children<F>(&mut self, child_builder_fn: F) -> Result<&mut Self, ScaffoldError>
    where
        F: FnOnce(&mut Scaffold) -> Result<(), ScaffoldError>
    {
        child_builder_fn(self)?;
        Ok(self)
    }
    
    /// TODO
    pub fn build(&mut self) -> Result<&mut Self, ScaffoldError> {
        if let Some(ref element) = self.get_element() {
            element.draw(Context::new_in(self.arena))(self)?;
        }
        
        self.hash = Some(self.hasher.finish());
        
        #[cfg(feature = "verbose")]
        tracing::debug!("Built Scaffold with Hash({:?})", self.hash);
        
        Ok(self)
    }
}

impl<'arena> Scaffold<'arena> {
    /// TODO
    pub fn hash(&self) -> Option<u64> {
        if self.hash == None {
            #[cfg(feature = "verbose")]
            tracing::warn!("Hash not yet built!");
        }
        self.hash
    }
    
    /// TODO
    pub fn has_changes(&self, element_node: &ElementNode) -> Result<bool, ScaffoldError> {
        let scaffold_hash = self.hash().ok_or(ScaffoldError::Unknown("Hash not yet built!"))?;
        let element_hash = element_node.hash();
        Ok(scaffold_hash != element_hash)
    }
}

// use core::cell::OnceCell;
// use bumpalo_herd::Herd;

// pub struct DebugHerd(Herd);

// #[cfg(debug_assertions)]
// impl DebugHerd {
//     /// A bump used in quick debug operations.
//     const HERD: OnceCell<Herd> = OnceCell::new();
    
//     /// Get a &Bump for debug operations.
//     /// TODO: There's probably a better way to do this.
//     pub(crate) fn member<'a>() -> &'a Member<'a> {
//         Self::HERD.get_or_init(|| Herd::new()).get().borrow()
//     }
// }

impl<'arena> Scaffold<'arena> {
    /// TODO
    #[cfg(feature = "bump")]
    pub fn try_from_draw_fn<F>(bump: &'arena Bump, draw_fn: F) -> Result<Self, ScaffoldError>
    where
        F: FnOnce(&mut Scaffold<'arena>) -> Result<(), ScaffoldError>,
    {
        let mut scaffold = Scaffold::new_in(bump);
        draw_fn(&mut scaffold)?;
        Ok(scaffold)
    }
    
    /// TODO
    #[cfg(not(feature = "bump"))]
    pub fn try_from_draw_fn<F>(draw_fn: F) -> Result<Self, ScaffoldError>
    where
        F: FnOnce(&mut Scaffold) -> Result<(), ScaffoldError>,
    {
        let mut scaffold = Scaffold::new();
        draw_fn(&mut scaffold)?;
        Ok(scaffold)
    }
}

/// TODO
#[derive(Debug)]
pub enum ScaffoldError {
    /// An item wasn't at the expected index. This is almost always a logical
    /// error and is likely a bug in the Scaffold behavior iteself.
    IndexOutOfBounds(usize),
    
    /// The hash was expected but is not available. This is almost always a
    /// programming error. It means you're trying to use a hash that hasn't
    /// been built yet (probably during some phase of change detection).
    HashMissing,
    
    /// TODO
    NodeMissing(UUID),
    
    /// TODO
    ElementError(ElementError),
    
    /// TODO
    Unknown(&'static str),
}

#[automatically_derived]
impl From<&'static str> for ScaffoldError {
    /// TODO
    fn from(error: &'static str) -> Self {
        ScaffoldError::Unknown(error)
    }
}

// TODO: Remove this when we bring back oops.
#[automatically_derived]
impl core::error::Error for ScaffoldError {}

// TODO: Remove this when we bring back oops.
#[automatically_derived]
impl core::fmt::Display for ScaffoldError {
    /// TODO
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ScaffoldError::IndexOutOfBounds(index) => write!(f, "TerminalError::CursorOutOfBounds: {:}", index),
            ScaffoldError::HashMissing => write!(f, "TerminalError::HashUnavailable"),
            ScaffoldError::NodeMissing(error) => write!(f, "TerminalError::NodeMissing: {:}", error),
            ScaffoldError::ElementError(error) => write!(f, "TerminalError::ElementError: {:}", error),
            ScaffoldError::Unknown(error) => write!(f, "TerminalError::Unknown: {:}", error),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::scaffold::{Scaffold, DebugHerd};
//     use crate::element::tests::ElementTestImpl;
    
//     /// Test that Scaffold hashes are calculated correctly.
//     #[test]
//     fn test_scaffold_hash_eq() {
//         // The outer scaffold appears superfluous, but it's here to test that
//         // the recursive build patterns within the scaffold work as expected.
//         let mut scaffold = Scaffold::new_in(DebugHerd::member().as_bump());
//         let children: [usize; 4] = [0, 1, 1, 2];
//         for child in children.iter() {
//             scaffold
//                 // Add a child to the root scaffold ..
//                 .add(ElementTestImpl::default().with_number(*child)).unwrap()
//                 // .. and then build that child.
//                 .build().unwrap();
//         }
        
//         // Test that the items are expected values.
//         assert_eq!(scaffold.children[0].hash(), Some(2853251017295103874));
//         assert_eq!(scaffold.children[1].hash(), Some(501169195535462803));
//         assert_eq!(scaffold.children[2].hash(), Some(501169195535462803));
//         assert_eq!(scaffold.children[3].hash(), Some(3625697961063136066));
        
//         // Test that hashes are only equal to each other when expected.
//         assert_ne!(scaffold.children[0].hash(), scaffold.children[1].hash());
//         assert_eq!(scaffold.children[1].hash(), scaffold.children[2].hash());
//         assert_ne!(scaffold.children[2].hash(), scaffold.children[3].hash());
//     }
// }
