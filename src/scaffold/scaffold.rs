use core::hash::Hash;
use core::hash::Hasher;

use alloc::boxed::Box;
use alloc::vec::Vec;

use ahash::AHasher;

// use bumpalo::Bump;

// use crate::surface::Surface;
// use crate::surface::SurfaceError;
// use crate::scaffold::Scaffold;
// use crate::scaffold::ScaffoldError;
use crate::element::Element;
use crate::element::ElementError;
use crate::element::ElementNode;
use crate::element::Renderable;
// use crate::element::DrawFn;
use crate::element::UUID;
use crate::event::Event;
use crate::event::EventKind;
use crate::event::EventHandlerFn;
use crate::style::StyleSheet;
// use crate::style::StyleProperty;
use crate::style::StyleValue;
// use crate::xtra::HashMap;

//---
/// Represents a lightweight single-pass builder for an Element node.
#[derive(Debug)]
pub struct Scaffold<'scaffold, H: Hasher = AHasher> {
    /// The node currently being built.
    element: Option<Box<dyn Element + 'scaffold>>,
    
    /// The actual state of the hash after final calculation.
    hash: Option<u64>,
    
    /// Hash of the element's internal state.
    hasher: H,
    
    /// TODO
    stylesheet: Option<StyleSheet>,
    
    /// TODO
    slots: Vec<()>,
    
    /// TODO
    children: Vec<Scaffold<'scaffold>>,
}

impl<'scaffold> Scaffold<'scaffold, AHasher> {
    /// TODO
    pub fn new() -> Self {
        Scaffold {
            element: None,
            hasher: AHasher::default(),
            hash: None,
            stylesheet: None,
            slots: Vec::new(),
            children: Vec::new(),
        }
    }
    
    /// TODO
    pub fn with_element(mut self, element: impl Element + 'scaffold) -> Self {
        self.element = Some(Box::new(element));
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

impl<'scaffold> Scaffold<'scaffold> {
    /// TODO
    pub fn try_from_draw_fn<F>(draw_fn: F) -> Result<Self, ScaffoldError>
    where
        F: FnOnce(&mut Scaffold<'scaffold>) -> Result<(), ScaffoldError>,
    {
        let mut scaffold = Scaffold::new();
        draw_fn(&mut scaffold)?;
        Ok(scaffold)
    }
}

impl<'scaffold> Scaffold<'scaffold> {
    /// Provides immutable access to the element node of this Scaffold.
    pub fn get_element(&self) -> Option<&Box<dyn Element + 'scaffold>> {
        self.element.as_ref()
    }
    
    /// Provides immutable access to the element node of this Scaffold.
    pub fn get_element_mut(&mut self) -> Option<&mut Box<dyn Element + 'scaffold>> {
        self.element.as_mut()
    }
    
    /// TODO
    pub fn take_element(&mut self) -> Option<Box<dyn Element + 'scaffold>> {
        self.element.take()
    }
    
    /// TODO
    pub fn stylesheet_mut(&mut self) -> &mut StyleSheet {
        self.stylesheet.get_or_insert_with(StyleSheet::new)
    }
    
    /// TODO
    pub fn get_stylesheet_mut(&mut self) -> Option<&mut StyleSheet> {
        self.stylesheet.as_mut()
    }
    
    /// TODO
    pub fn take_stylesheet(&mut self) -> Option<StyleSheet> {
        self.stylesheet.take()
    }
    
    //--
    /// Provides immutable access to the children of this Scaffold.
    pub fn children(&self) -> &Vec<Scaffold<'scaffold>> {
        self.children.as_ref()
    }
    
    /// Provides mutable access to the children of this Scaffold.
    pub fn children_mut(&mut self) -> &mut Vec<Scaffold<'scaffold>> {
        &mut self.children
    }
}

impl<'scaffold> Scaffold<'scaffold> {
    /// TODO
    pub fn add<E: Element + Renderable + Hash + 'scaffold>(&mut self, element: E) -> Result<&mut Self, ScaffoldError> {
        let mut new_scaffold = Scaffold::new();
        element.hash(&mut new_scaffold.hasher);
        new_scaffold.element = Some(Box::new(element));
        
        self.children.push(new_scaffold);
        self.children.last_mut().ok_or(ScaffoldError::CursorOutOfBounds)
    }
    
    /// TODO
    pub fn with_event_attr<E: Event>(&mut self, event_kind: EventKind, _event_fn: EventHandlerFn<E>) -> Result<&mut Self, ScaffoldError> {
        tracing::warn!("TODO: #[event({:?}, ..)]", event_kind);
        Ok(self) // etc..
    }
    
    /// TODO
    pub fn with_style_attr<V: StyleValue + core::fmt::Debug + 'static>(&mut self, style_value: V) -> Result<&mut Self, ScaffoldError> {
        self.stylesheet_mut().push(style_value);
        Ok(self) // etc..
    }
    
    /// TODO
    pub fn with_class_attr<F: Fn(&mut StyleSheet)>(&mut self, class_fn: F) -> Result<&mut Self, ScaffoldError> {
        class_fn(self.stylesheet_mut());
        Ok(self) // etc..
    }
    
    /// TODO
    pub fn with_children<F>(&mut self, child_builder_fn: F) -> Result<&mut Self, ScaffoldError>
    where
        F: FnOnce(&mut Scaffold<'scaffold>) -> Result<(), ScaffoldError>
    {
        child_builder_fn(self)?;
        Ok(self)
    }
    
    pub fn build(&mut self) -> Result<&mut Self, ScaffoldError> {
        let final_hash = self.hasher.finish();
        self.hash = Some(final_hash);
        
        #[cfg(feature = "verbose")]
        tracing::debug!("Built Scaffold with Hash({:?})", self.hash);
        
        Ok(self)
    }
}

impl<'scaffold> Scaffold<'scaffold> {
    /// TODO
    pub fn hash(&self) -> Option<u64> {
        if self.hash == None {
            tracing::warn!("Hash not yet built!");
        }
        self.hash
    }
    
    /// TODO
    pub fn has_changes(&self, element_node: &ElementNode<'scaffold>) -> Result<bool, ScaffoldError> {
        let scaffold_hash = self.hash().ok_or(ScaffoldError::Unknown("Hash not yet built!"))?;
        let element_hash = element_node.hash();
        Ok(scaffold_hash != element_hash)
    }
}

/// TODO
#[derive(Debug)]
pub enum ScaffoldError {
    /// An item wasn't at the expected index. This is almost always a logical
    /// error and is likely a bug in the Scaffold behavior iteself.
    CursorOutOfBounds,
    
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
            ScaffoldError::CursorOutOfBounds => write!(f, "TerminalError::CursorOutOfBounds"),
            ScaffoldError::HashMissing => write!(f, "TerminalError::HashUnavailable"),
            ScaffoldError::NodeMissing(error) => write!(f, "TerminalError::NodeMissing: {:}", error),
            ScaffoldError::ElementError(error) => write!(f, "TerminalError::ElementError: {:}", error),
            ScaffoldError::Unknown(error) => write!(f, "TerminalError::Unknown: {:}", error),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::element::tests::ElementTestImpl;
    use crate::scaffold::Scaffold;
    
    /// Test that Scaffold hashes are calculated correctly.
    #[test]
    fn test_scaffold_hash_eq() {
        // The outer scaffold appears superfluous, but it's here to test that
        // the recursive build patterns within the scaffold work as expected.
        let mut scaffold = Scaffold::new();
        let children: [usize; 4] = [0, 1, 1, 2];
        for child in children.iter() {
            scaffold
                // Add a child to the root scaffold ..
                .add(ElementTestImpl::default().with_number(*child)).unwrap()
                // .. and then build that child.
                .build().unwrap();
        }
        
        // Test that the items are expected values.
        assert_eq!(scaffold.children[0].hash(), Some(2853251017295103874));
        assert_eq!(scaffold.children[1].hash(), Some(501169195535462803));
        assert_eq!(scaffold.children[2].hash(), Some(501169195535462803));
        assert_eq!(scaffold.children[3].hash(), Some(3625697961063136066));
        
        // Test that hashes are only equal to each other when expected.
        assert_ne!(scaffold.children[0].hash(), scaffold.children[1].hash());
        assert_eq!(scaffold.children[1].hash(), scaffold.children[2].hash());
        assert_ne!(scaffold.children[2].hash(), scaffold.children[3].hash());
    }
}
