use core::fmt::Debug;

use alloc::vec::Vec;

use crate::scaffold::Scaffold;
use crate::scaffold::ScaffoldError;
// use crate::element::Element;
use crate::element::ElementNode;
use crate::element::DrawResult;
use crate::element::UUID;
use crate::x::HashMap;
// use crate::xtra::Entry;

//---
/// TODO
pub struct Surface<'surface> {
    /// The UUID of the surface.
    uuid: UUID,
    
    /// The nodes of the surface.
    nodes: Vec<ElementNode<'surface>>,
    
    /// An index to the element nodes of the scaffold.
    index: HashMap<UUID, usize>,
    
    /// An index to the root nodes of the scaffold.
    roots: Vec<usize>,
    
    /// An index to the root nodes of the scaffold.
    edges: HashMap<usize, Vec<usize>>,
}

impl<'surface> Surface<'surface> {
    /// Creates a new Surface using the Global allocator.
    pub fn new(uuid: Option<UUID>) -> Self {
        Surface {
            uuid: uuid.unwrap_or_else(|| UUID::new_v4()),
            nodes: Vec::new(),
            index: HashMap::new(),
            roots: Vec::new(),
            edges: HashMap::new(),
        }
    }
}

impl<'surface> Surface<'surface> {
    /// TODO
    pub fn get_node(&self, uuid: &UUID) -> Option<&ElementNode<'surface>> {
        self.nodes.get(*self.index.get(uuid)?)
    }
    
    /// TODO
    pub fn get_node_mut(&mut self, uuid: &UUID) -> Option<&mut ElementNode<'surface>> {
        self.nodes.get_mut(*self.index.get(uuid)?)
    }
    
    pub fn get_roots(&self) -> impl Iterator<Item = &ElementNode<'surface>> {
        self.roots.iter().filter_map(move |node_index| self.nodes.get(*node_index))
    }
    
    /// TODO
    pub fn get_children_of(&'surface self, parent_uuid: &UUID) -> Option<impl Iterator<Item = &'surface ElementNode<'surface>> + 'surface> {
        let parent_index = self.index.get(parent_uuid)?;
        let children = self.edges.get(parent_index)?;
        Some(children.iter().filter_map(|child_index| self.nodes.get(*child_index)))
    }
}

impl<'surface> Surface<'surface> {
    /// TODO
    pub fn draw<F>(&mut self, draw_fn: F) -> Result<DrawResult, ScaffoldError>
    where
        F: FnOnce(&mut Scaffold<'surface>) -> Result<(), ScaffoldError>
    {
        #[cfg(feature = "debug")]
        tracing::trace!("Drawing on Surface({:?}) ..", self.uuid);
        
        // The scaffold is a temporary structure used to build the current
        // pass of the surface, which is used to apply collected updates.
        let mut scaffold = Scaffold::new();
        draw_fn(&mut scaffold)?;
        
        // As we traverse the tree and apply changes, updates are collected
        // to be used by event updates and returned to the caller.
        let mut updates = Vec::new();
        
        // When the number of roots changes, we force a full update of the
        //  surface to ensure all indexes are updated.
        // TODO: Move this to the apply_scaffold method.
        let force = self.roots.len() != scaffold.children().len();
        
        for (cursor, mut root) in scaffold.children_mut().into_iter().enumerate() {
            self.apply_scaffold(&mut root, None, cursor, force, &mut updates)?;
        }
        
        if updates.len() > 0 {
            Ok(DrawResult::Success(updates))
        } else {
            Ok(DrawResult::Noop)
        }
    }
    
    /// TODO
    fn apply_scaffold(&mut self, scaffold: &mut Scaffold<'surface>, parent_index: Option<usize>, cursor: usize, force: bool, updates: &mut Vec<SurfaceUpdate>) -> Result<usize, ScaffoldError> {
        // Get the existing UUID at the current cursor position.
        let existing_element_index = match parent_index {
            // Parent provided; Attempt to get existing child UUID at `cursor`.
            Some(parent_index) => {
                // .. and use it to get the index for the item at `cursor`.
                // Evaluates to Some(UUID) when the parent exists and has
                // a child at `index`.
                self.edges.get(&parent_index).and_then(|edges| edges.get(cursor).map(|uuid| *uuid))
            }
            
            // No parent provided; Check the root index.
            None => {
                // Evaluates to Some(UUID) when a root exists at `cursor`.
                self.roots.get(cursor).map(|node_index| *node_index)
            }
        };
        
        // Get the concrete UUID for the current node.
        let element_index = match existing_element_index {
            // An existing UUID was found. Attempt to update the node.
            Some(element_index) => {
                let element_node = self.nodes.get_mut(element_index).ok_or(ScaffoldError::CursorOutOfBounds)?;
                
                let should_update = force || scaffold.has_changes(element_node)?;
                
                #[cfg(feature = "verbose")]
                tracing::debug!("Element({:?}) should update? ({:?})", element_node.uuid(), should_update);
                
                if should_update {
                    let element_uuid = element_node.uuid();
                    
                    element_node.set_element(scaffold.take_element());
                    element_node.set_hash(scaffold.hash().ok_or(ScaffoldError::HashMissing)?);
                    
                    // TODO: Overwrite the stylesheet and event handlers.
                    
                    updates.push(SurfaceUpdate::Update(element_uuid));
                    tracing::debug!("Updated Element({:?})!", element_uuid);
                }
                
                element_index
            }
            
            // No existing UUID found. Create a new node.
            None => {
                let mut element_node = ElementNode::new(scaffold.take_element());
                #[cfg(feature = "verbose")]
                tracing::debug!("Created Element({:?})", element_node.uuid());
                
                // TODO: This should probably be done in the ElementNode::from(Scaffold) impl.
                element_node.set_hash(scaffold.hash().ok_or(ScaffoldError::HashMissing)?);
                element_node.stylesheet_mut().append(scaffold.stylesheet_mut());
                
                let element_uuid = element_node.uuid();
                let element_index = self.add_node(element_node)?;
                
                updates.push(SurfaceUpdate::Add(element_uuid));
                tracing::debug!("Added Element({:?})!", element_uuid);
                #[cfg(feature = "inspect")]
                tracing::debug!("Element:\n{:#?}", self.get_node(&element_uuid));
                
                element_index
            }
        };
        
        if parent_index == None && !self.roots.contains(&element_index) {
            self.roots.push(element_index);
        }
        
        for (cursor, mut child) in scaffold.children_mut().into_iter().enumerate() {
            let child_uuid = self.apply_scaffold(&mut child, Some(element_index), cursor, false, updates)?;
            self.edges.entry(element_index).or_insert_with(Vec::new).push(child_uuid);
        }
        
        Ok(element_index)
    }
    
    /// TODO
    fn add_node(&mut self, element_node: ElementNode<'surface>) -> Result<usize, ScaffoldError> {
        let node_uuid = element_node.uuid();
        let node_index = self.nodes.len();
        
        self.nodes.push(element_node);
        
        if let Some(old_node) = self.index.insert(node_uuid, node_index) {
            tracing::warn!("Updated node {:?}! Now what? Probably clean it up ..", node_uuid);
            tracing::debug!("Old node: {:?}", old_node);
        }
        
        Ok(node_index)
    }
}

//---
/// TODO
#[derive(Debug, Copy, Clone)]
pub enum SurfaceUpdate {
    /// TODO
    Add(UUID),
    
    /// TODO
    Update(UUID),
    
    /// TODO
    Remove(UUID),
}

//---
/// TODO
#[derive(Debug)]
pub enum SurfaceError {
    /// TODO
    CursorOutOfBounds,
    
    /// TODO
    ScaffoldError(ScaffoldError),
    
    /// TODO
    Unknown(&'static str),
}

impl From<ScaffoldError> for SurfaceError {
    /// TODO
    fn from(error: ScaffoldError) -> Self {
        SurfaceError::ScaffoldError(error)
    }
}

impl From<&'static str> for SurfaceError {
    /// TODO
    fn from(error: &'static str) -> Self {
        SurfaceError::Unknown(error)
    }
}

// TODO: Remove this when we bring back oops.
#[automatically_derived]
impl core::error::Error for SurfaceError {}

// TODO: Remove this when we bring back oops.
#[automatically_derived]
impl core::fmt::Display for SurfaceError {
    /// TODO
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SurfaceError::CursorOutOfBounds => write!(f, "TerminalError::CursorOutOfBounds"),
            SurfaceError::ScaffoldError(error) => write!(f, "TerminalError::ScaffoldError: {:}", error),
            SurfaceError::Unknown(error) => write!(f, "TerminalError::Unknown: {:}", error),
        }
    }
}
