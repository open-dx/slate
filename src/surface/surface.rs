use core::fmt::Debug;

use alloc::boxed::Box;
use alloc::vec::Vec;

use bumpalo::Bump;
#[cfg(feature = "bump")]
use bumpalo_herd::Herd;

use crate::scaffold::Scaffold;
use crate::scaffold::ScaffoldError;
// use crate::scaffold::DebugHerd;
// use crate::element::Element;
use crate::element::ElementNode;
use crate::element::DrawReport;
use crate::element::UUID;
use crate::collections::HashMap;
// use crate::xtra::Entry;

//---
/// A `Surface` is a managed, reactive composition target for drawing visual
/// elements to various output renderers. It's primarily responsible for
/// managing the lifecycle and hierarchy of elements it holds, and reporting
/// updates to a rendering front-end.
/// 
/// It can represent any kind of visual output, including a DOM, a scene graph,
/// a terminal, etc. It's designed to be renderer-agnostic and can be used
/// with any renderer that can consume the `DrawReport` it produces.
/// 
/// It also provides several quality-of-life features for improved ergonomics
/// during development, debugging, monitoring, etc.
/// 
/// ## Guide: Managed Updates
/// 1. Create a new Surface:
/// ```rust
/// let mut surface = Surface::new(UUID::new_v4());
/// // Configure, bootstrap, etc.
/// ```
/// 
/// 2. Add elements to the surface:
/// ```rust
/// // Elements are added to a surface by passing a `DrawFn` to the
/// // `surface.draw(..)` method. The `DrawFn` is used to build a `Scaffold`,
/// // compare it to the current state of the surface, and report changes.
/// // 
/// // The resulting `DrawReport` is used by the renderer to sync the surface
/// // with the scene/dom/whatever.
/// let draw_report = surface.draw(chizel::uix! {
///     <Container>
///         <Paragraph text="TODO" />
///     </Container>
/// });
/// ```
/// 
/// 3. Pass updates to a renderer:
/// ```rust
/// // The draw report is designed to be consumed by a renderer.
/// match draw_report {
///     // A successful draw result contains a list of updates. The renderer
///     // should apply these updates to the computed UI, scene, etc.
///     Ok(DrawResult::Success(mut updates)) => {
///         self.consume_updates(&mut updates);
///     }
///     // A no-op draw result indicates that no updates were made, which
///     // means all operations involved in the draw were successful but no
///     // differences were found between the scaffold and the previous state
///     // of the surface.
///     // 
///     // Generally, the renderer can safely advance.
///     Ok(DrawResult::Noop) => {
///         tracing::trace!("No-op! <3");
///     }
///     // An error here means something didn't go as planned. The renderer
///     // should 1) log the error, 2) (optionally) display an error message
///     // to the user, and 3) decide what to do next (re-draw attempt, etc.).
///     Err(error) => {
///         tracing::error!("Failed to draw Surface: {}", error);
///     }
/// }
/// ```
pub struct Surface<'surface, Index=UUID> {
    /// The UUID of the surface.
    #[cfg(feature = "debug")]
    uuid: UUID,
    
    /// The nodes of the surface.
    nodes: Vec<ElementNode<'surface>>,
    
    /// An index to the element nodes of the scaffold.
    index: HashMap<Index, usize>,
    
    /// An index to the root nodes of the scaffold.
    roots: Vec<usize>,
    
    /// An index to the root nodes of the scaffold.
    edges: HashMap<usize, Vec<usize>>,
}

impl<'surface> Surface<'surface> {
    /// Creates a new Surface using the Global allocator.
    pub fn new() -> Self {
        Surface {
            #[cfg(feature = "debug")]
            uuid: UUID::new_v4(),
            nodes: Vec::new(),
            index: HashMap::new(),
            roots: Vec::new(),
            edges: HashMap::new(),
        }
    }
}

impl<'surface> Surface<'surface> {
    /// Get a node from the surface.
    /// 
    /// ```rust
    /// let node: ElementNode<'surface> = surface.get_node(&uuid);
    /// ```
    pub fn get_node(&self, uuid: &UUID) -> Option<&ElementNode<'surface>> {
        self.index.get(uuid).and_then(|i| self.nodes.get(*i))
    }
    
    /// Get a mutable reference to a node from the surface.
    /// 
    /// ```rust
    /// let node: &mut ElementNode<'surface> = surface.get_node_mut(&uuid);
    /// ```
    pub fn get_node_mut(&mut self, uuid: &UUID) -> Option<&mut ElementNode<'surface>> {
        self.nodes.get_mut(*self.index.get(uuid)?)
    }
    
    /// Get the roots of the surface.
    /// 
    /// ```rust
    /// let roots: Vec<&ElementNode<'surface>> = surface.get_roots().collect();
    /// ```
    pub fn get_roots(&self) -> impl Iterator<Item = &ElementNode<'surface>> {
        self.roots.iter().filter_map(move |node_index| {
            self.nodes.get(*node_index)
        })
    }
    
    /// Get the children of a node from the surface.
    /// 
    /// ```rust
    /// let children: Vec<&ElementNode<'surface>> = surface.get_children_of(&uuid).collect();
    /// ```
    pub fn get_children_of(&'surface self, parent_uuid: &UUID) -> Option<impl Iterator<Item = &'surface ElementNode<'surface>> + 'surface> {
        let parent_index = self.index.get(parent_uuid)?;
        let children = self.edges.get(parent_index)?;
        Some(children.iter().filter_map(|child_index| {
            self.nodes.get(*child_index)
        }))
    }
}

#[derive(Copy, Clone)]
pub struct Context<'ctx> {
    arena: &'ctx Bump,
}

impl<'ctx> Context<'ctx> {
    pub fn new_in(arena: &'ctx Bump) -> Self {
        Context {
            arena
        }
    }
    
    pub fn arena(&self) -> &Bump {
        self.arena
    }
}

impl<'surface> Surface<'surface> {
    /// Draws the given `DrawFn` to the surface.
    /// 
    /// ```rust
    /// let draw_report = surface.draw(chizel::uix! {
    ///     <Container>
    ///         <Paragraph text="TODO" />
    ///     </Container>
    /// })?;
    pub fn draw<F>(&mut self, draw_fn: F) -> Result<DrawReport, ScaffoldError>
    where
        F: FnOnce(&mut Scaffold) -> Result<(), ScaffoldError>
    {
        #[cfg(feature = "debug")]
        tracing::trace!("Drawing on Surface({:?}) ..", self.uuid);
        
        // Get a new bump for the scaffold to build it's internal tree.
        // let alloc = bumpalo::Bump::new();
        #[cfg(feature = "bump")]
        let alloc = crate::arena::get();
        
        #[cfg(feature = "bump")]
        let ctx = Context::new_in(alloc.as_bump());
        
        // A scaffold is a temporary structure used to build the current
        // pass of the surface, which is used to apply collected updates.
        // TODO: Get the bump from a herd on the surface.
        #[cfg(feature = "bump")]
        let mut scaffold = Scaffold::new_in(ctx.arena());
        
        #[cfg(not(feature = "bump"))]
        let mut scaffold = Scaffold::new();
        
        draw_fn(&mut scaffold)?;
        
        // As we traverse the tree and apply changes, updates are collected
        // to be used by event updates and returned to the caller.
        let mut updates = Vec::new();
        
        // When the number of roots changes, we force a full update of the
        // surface to ensure all indexes are updated.
        let force_updates = self.roots.len() != scaffold.children().len();
        
        // Enumerate each root-level element within the scaffolding tree
        // and apply the scaffold at the given cursor/index.
        for (index, mut root) in scaffold.children_mut().into_iter().enumerate() {
            self.apply_scaffold(root, None, index, force_updates, &mut updates)?;
        }
        
        if updates.len() == 0 {
            Ok(DrawReport::Noop)
        } else {
            Ok(DrawReport::Success(updates))
        }
    }
    
    /// Applies a given scaffold to the surface. Designed for recursive use.
    fn apply_scaffold(&mut self, scaffold: &mut Scaffold, parent_index: Option<usize>, current_index: usize, force_updates: bool, updates: &mut Vec<SurfaceUpdate>) -> Result<usize, ScaffoldError> {
        // Get the existing UUID at the current cursor position.
        let prev_element_index = match parent_index {
            // Parent provided; Attempt to get existing child index at `cursor`.
            Some(parent_index) => {
                // .. and use it to get the child index for the item at `current_index`.
                // Evaluates to Some(index) when the parent exists and has
                // a child at `current_index`.
                self.edges.get(&parent_index).and_then(|edges| {
                    edges.get(current_index).map(|index| *index)
                })
            }
            
            // No parent provided; Check the root index.
            None => {
                // Evaluates to Some(index) when a root exists at `cursor`.
                self.roots.get(current_index).map(|node_index| *node_index)
            }
        };
        
        // Get the concrete index for the current node.
        let next_element_index = match prev_element_index {
            Some(element_index) => {
                // An existing index was found. Attempt to update the node.
                let element_node = self.nodes.get_mut(element_index).ok_or(ScaffoldError::IndexOutOfBounds(element_index))?;
                let should_update = force_updates || scaffold.has_changes(element_node)?;
                
                #[cfg(feature = "verbose")]
                tracing::debug!("Element({:?}) should update? ({:?})", element_node.uuid(), should_update);
                
                if should_update {
                    let element_uuid = element_node.uuid();
                    
                    element_node.set_element(scaffold.take_element_boxed());
                    element_node.set_hash(scaffold.hash().ok_or(ScaffoldError::HashMissing)?);
                    
                    // TODO: Overwrite the stylesheet and event handlers.
                    
                    updates.push(SurfaceUpdate::Update(element_uuid));
                    tracing::debug!("Updated Element({:?})!", element_uuid);
                }
                
                element_index
            }
            None => {
                // No existing UUID found. Create a new node.
                let mut element_node = ElementNode::<'surface>::new(scaffold.take_element_boxed());
                
                #[cfg(feature="verbose")]
                tracing::debug!("Created new ElementNode#{:?}", element_node.uuid());
                
                // TODO: This should probably be done in the ElementNode::from(Scaffold) impl.
                element_node.set_hash(scaffold.hash().ok_or(ScaffoldError::HashMissing)?);
                
                element_node.stylesheet_mut().extend(scaffold.stylesheet_mut());
                element_node.take_events(scaffold.events_mut());
                
                let element_uuid = element_node.uuid();
                let element_index = self.add_node(element_node)?;
                
                updates.push(SurfaceUpdate::Add(element_uuid));
                
                #[cfg(all(feature="verbose", not(feature="inspect")))]
                tracing::debug!("Added ElementNode#{:?} to Surface!", element_uuid);
                
                #[cfg(all(feature="verbose", feature="inspect"))]
                tracing::debug!("Added ElementNode:\n{:#?}", self.get_node(&element_uuid));
                
                element_index
            }
        };
        
        if parent_index == None && !self.roots.contains(&next_element_index) {
            self.roots.push(next_element_index);
        }
        
        for (cursor, mut child) in scaffold.children_mut().into_iter().enumerate() {
            let child_uuid = self.apply_scaffold(&mut child, Some(next_element_index), cursor, false, updates)?;
            self.edges.entry(next_element_index).or_insert_with(Vec::new).push(child_uuid);
        }
        
        Ok(next_element_index)
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
