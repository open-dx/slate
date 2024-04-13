use core::ops::Deref;
use core::ops::DerefMut;

use bevy::ecs::prelude::*;
use bevy::ecs::entity::Entity;
use bevy::ecs::component::Component;
use bevy::ecs::system::Commands;
use bevy::ecs::system::Query;
use bevy::hierarchy::BuildChildren;
use bevy::hierarchy::ChildBuilder;
// use bevy::hierarchy::DespawnRecursiveExt;
use bevy::ui::prelude::*;
use bevy::ui::node_bundles::NodeBundle;

use slate::surface::Surface;
// use slate::surface::builder::SurfaceBuilder;
// use slate::surface::SurfaceError;
use slate::surface::SurfaceUpdate;
use slate::scaffold::Scaffold;
use slate::scaffold::ScaffoldError;
// use slate::element::Element;
use slate::element::ElementNode;
use slate::element::DrawReport;
use slate::element::UUID;
use slate::style::StyleValueRef;
use slate::style::primitive::Unit;
// use slate::style::properties::BoxSize;
use slate::x::HashMap;

/// Convenience type for unpacking a 2D size.
struct Size2d(Val, Val);

//---
/// TODO
#[derive(Component)]
pub struct SurfaceProvider {
    /// The surface that provides the scene/ui/whatever
    surface: Surface<'static>,
    
    /// The entity that represents the surface in the Bevy hierarchy.
    surface_entity: Option<Entity>,
    
    /// TODO
    entity_index: HashMap<UUID, Entity>,
}

/// TODO
#[derive(Component)]
pub struct EntityMap(HashMap<UUID, Entity>);

impl SurfaceProvider {
    /// TODO
    pub fn new() -> Self {
        SurfaceProvider {
            surface: Surface::<'static>::new(),
            surface_entity: None,
            entity_index: HashMap::new(),
        }
    }
    
    /// TODO
    pub fn from_surface(surface: Surface<'static>) -> Self {
        SurfaceProvider {
            surface,
            surface_entity: None,
            entity_index: HashMap::new(),
        }
    }
}

impl SurfaceProvider {
    /// TODO
    pub fn surface(&self) -> &Surface<'static> {
        &self.surface
    }
    
    /// TODO
    pub fn surface_mut(&mut self) -> &mut Surface<'static> {
        &mut self.surface
    }
}

impl Deref for SurfaceProvider {
    type Target = Surface<'static>;
    
    /// TODO
    fn deref(&self) -> &Self::Target {
        &self.surface
    }
}

impl DerefMut for SurfaceProvider {
    /// TODO
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.surface
    }
}

impl SurfaceProvider {
    /// TODO
    pub fn draw<F>(&mut self, commands: &mut Commands, draw_fn: F)
    where
        F: FnOnce(&mut Scaffold) -> Result<(), ScaffoldError>
    {
        match self.surface.draw(draw_fn) {
            Ok(DrawReport::Success(mut updates)) => {
                self.consume_updates(commands, &mut updates);
            }
            Ok(DrawReport::Noop) => {
                tracing::trace!("No-op! <3");
            }
            Err(error) => {
                tracing::error!("Failed to draw Surface: {:}", error);
            }
        }
    }
    
    /// Consume updates from the surface and apply them to the Bevy hierarchy.
    fn consume_updates(&mut self, commands: &mut Commands, surface_updates: &mut Vec<SurfaceUpdate>) {
        // Get a copy of the surface Entity so we can operate on it.
        // TODO: In some cases, we'll want to operate on an entity determined
        //  by which node is being updated (e.g. when updating a node's style).
        let Some(surface_entity) = self.surface_entity else {
            return tracing::warn!("SurfaceProvider has no entity.");
        };
        
        // Stores a list of updates that have been consumed by the system.
        // 
        // Since we update recursively, we need to keep track of which
        // updates have already made it into their destimation and which
        // ones haven't so we don't duplicate render operations.
        // 
        // At the end of the update pass, this is drained into the entity
        // index for use in future update calls, indexing, etc.
        let mut consumed_updates = HashMap::with_capacity(surface_updates.len());
        
        #[cfg(feature = "verbose")]
        tracing::trace!("Found {:?} updates ..", consumed_updates.len());
        
        // TODO: Evaluate: Sort updates by kind?
        //  1. Add
        //  2. Update
        //  3. Remove

        // TODO: Iterate over the vec of updates
        for update in surface_updates.drain(..).filter(|_| true) {
            match update {
                SurfaceUpdate::Add(element_uuid) => {
                    // Ensure we're not adding a node that's already been
                    // added by a different recursive update operation.
                    // 
                    // Note: Direct node updates (via tools/editor, for
                    //  example) are currently unsupported. See the TODO below.
                    // 
                    // TODO: An update may represent a node that was added
                    //  as a child of another node as a direct update. In those
                    //  cases, we'll need to lookup the parent of the current
                    //  node and add the child to it.
                    // 
                    // TODO: This is a naive implementation. Can we do better
                    //  via more direct updates?
                    if !consumed_updates.contains_key(&element_uuid) {
                        commands
                            .entity(surface_entity)
                            .with_children(|child_builder| {
                                self.spawn_element_node(child_builder, &element_uuid, &mut consumed_updates);
                            });
                    }
                }
                SurfaceUpdate::Update(element_uuid) => {
                    // TODO: Update the node's style, children, etc.
                    match self.entity_index.get(&element_uuid) {
                        Some(element_entity) => {
                            // We have a target entity for the update, so we:
                            // TODO.
                            if let Some(element_node) = self.surface().get_node(&element_uuid) {
                                let mut element_cmd = commands.entity(*element_entity);
                                
                                tracing::warn!("TODO: Update Bevy Node for {:?} at Node {:?}", element_uuid, element_entity);
                                // TODO: Recursively update element nodes in the tree.
                            }
                        }
                        None => {
                            tracing::warn!("Attempting to update untracked node {:?}; Skipping ..", element_uuid);
                            continue; // TODO: Pass the error off to a diagnostic report.
                        }
                    }
                }
                SurfaceUpdate::Remove(element_uuid) => {
                    tracing::warn!("TODO: Remove Node for {:?}", element_uuid);
                }
            }
        }
        
        tracing::trace!("Consumed {:?} updates ..", consumed_updates.len());
        
        for (element_uuid, element_entity) in consumed_updates.drain() {
            self.entity_index.insert(element_uuid, element_entity);
        }
        
        if !surface_updates.is_empty() {
            // Any updates left in the updates vec are orphaned.
            // TODO: Handle these.
            tracing::warn!("Unhandled updates: {:?}", surface_updates);
        }
    }
    
    /// TODO
    fn spawn_element_node(&self, builder: &mut ChildBuilder, element_uuid: &UUID, consumed_updates: &mut HashMap<UUID, Entity>) {
        // Get a copy of the surface Entity so we can operate on it.
        let Some(surface_entity) = self.surface_entity else {
            return tracing::warn!("SurfaceProvider has no entity.");
        };
        
        match self.surface.get_node(element_uuid) {
            Some(element_node) => {
                // TODO: Get the element's style from the surface.
                let mut element_entity = builder.spawn(ElementNodeBundle::from(element_node));
                
                if let Some(element_children) = self.surface.get_children_of(&element_uuid) {
                    element_entity
                        .with_children(|child_builder| {
                            for child_element_node in element_children.into_iter() {
                                self.spawn_element_node(child_builder, &child_element_node.uuid(), consumed_updates);
                            }
                        });
                }
                
                consumed_updates.insert(*element_uuid, element_entity.id());
                
                #[cfg(feature = "inspect")]
                tracing::trace!("Spawned Element#{:?} at Node {:?}", element_uuid, element_entity.id());
            }
            None => {
                // TODO: Handle this error visually in the tree.
                tracing::warn!("Can't find Element {:?} for rendering.", element_uuid);
                return;
            }
        }
    }
    
    fn update_element_node(&self, commands: &mut Commands, element_uuid: &UUID, element_entity: Entity) {
        match self.surface().get_node(element_uuid) {
            Some(element_node) => {
                let mut element_cmd = commands.entity(element_entity);
                
                tracing::warn!("TODO: Update Bevy Node for {:?} at Node {:?}", element_uuid, element_entity);
            }
            
            None => {
                tracing::warn!("Attempting to update untracked node {:?}; Skipping ..", element_uuid);
                return; // TODO: Pass the error off to a diagnostic report.
            }
        }
    }
}

/// TODO
#[derive(Bundle)]
pub struct SurfaceNodeBundle {
    /// TODO
    node_bundle: NodeBundle,
}

impl From<&Surface<'_>> for SurfaceNodeBundle {
    /// TODO
    fn from(_surface_ref: &Surface<'_>) -> Self {
        SurfaceNodeBundle {
            node_bundle: NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    // flex_grow: 0.,
                    // flex_shrink: 1.,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

/// TODO
pub(crate) fn setup_new_surface(
    mut surface_query: Query<(Entity, &mut SurfaceProvider), Added<SurfaceProvider>>,
    mut commands: Commands,
) {
    for (surface_entity, mut surface_provider) in surface_query.iter_mut() {
        let mut surface_cmd = commands.entity(surface_entity);
        
        surface_cmd.insert(SurfaceNodeBundle::from(surface_provider.surface()));
        surface_provider.surface_entity = Some(surface_entity);
        
        tracing::trace!("Surface `{:?}` changed ..", surface_entity);
    }
}

//---
#[derive(Component)]
pub struct ElementNodeHandle(UUID);

impl ElementNodeHandle {
    /// TODO
    pub fn uuid(&self) -> &UUID {
        &self.0
    }
}

impl core::fmt::Display for ElementNodeHandle {
    /// TODO
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:} (Handle)", self.0)
    }
}

impl core::fmt::Debug for ElementNodeHandle {
    /// TODO
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ElementNodeHandle")
            .field(&self.0)
            .finish()
    }
}

#[derive(Bundle)]
pub struct ElementNodeBundle {
    element_node_handle: ElementNodeHandle,
    node_bundle: NodeBundle,
}

impl ElementNodeBundle {
    pub fn new(uuid: UUID) -> Self {
        ElementNodeBundle {
            element_node_handle: ElementNodeHandle(uuid),
            // node_bundle: NodeBundle::default(),
            // TODO: Remove this default when we implement the StyleGuide.
            node_bundle: NodeBundle {
                style: Style {
                    // flex_grow: 0.,
                    // flex_shrink: 1.,
                    // border: UiRect {
                    //     top: Val::Px(2.),
                    //     right: Val::Px(2.),
                    //     bottom: Val::Px(2.),
                    //     left: Val::Px(2.),
                    // },
                    ..Default::default()
                },
                // border_color: bevy::ui::BorderColor(bevy::render::color::Color::GRAY),
                ..Default::default()
            },
        }
    }
}

impl From<&ElementNode<'_>> for ElementNodeBundle {
    fn from(element_node: &ElementNode<'_>) -> Self {
        let mut bundle = ElementNodeBundle::new(element_node.uuid());
        
        for (_style_type_id, styles) in element_node.stylesheet().styles() {
            #[cfg(feature = "inspect")]
            tracing::trace!("Style TypeId: {:?}", _style_type_id);
            
            for style in styles.iter() {
                #[cfg(feature = "inspect")]
                tracing::trace!("Processing Style: {:?}", style);
                
                bundle.apply_style(style);
            }
        }
        
        bundle
    }
}

impl ElementNodeBundle {
    /// Apply a style to the ElementNodeBundle.
    #[inline(always)]
    fn apply_style(&mut self, style_value: &StyleValueRef) {
        #[allow(unreachable_patterns)]
        match style_value {
            StyleValueRef::Flex(flex) => self.apply_flex(flex),
            StyleValueRef::FlexDirection(flex_direction) => self.apply_flex_direction(flex_direction),
            StyleValueRef::FlexBasis(flex_basis) => Self::apply_unit(flex_basis, &mut self.node_bundle.style.flex_basis),
            StyleValueRef::FlexGrow(flex_grow) => Self::apply_weight(flex_grow, &mut self.node_bundle.style.flex_grow),
            StyleValueRef::FlexShrink(flex_shrink) => Self::apply_weight(flex_shrink, &mut self.node_bundle.style.flex_shrink),
            StyleValueRef::Gap(gap) => Self::apply_gap(gap, &mut self.node_bundle.style),
            StyleValueRef::BackgroundColor(color) => Self::apply_background_color(color, &mut self.node_bundle.background_color),
            StyleValueRef::Margin(margin) => Self::apply_rect(margin, &mut self.node_bundle.style.margin),
            StyleValueRef::Padding(padding) => Self::apply_rect(padding, &mut self.node_bundle.style.padding),
            StyleValueRef::BoxSize(box_size) => Self::apply_box_size(box_size, &mut self.node_bundle.style),
            StyleValueRef::Width(width) => Self::apply_unit(width, &mut self.node_bundle.style.width),
            StyleValueRef::Height(height) => Self::apply_unit(height, &mut self.node_bundle.style.height),
            StyleValueRef::MinWidth(min_width) => Self::apply_unit(min_width, &mut self.node_bundle.style.min_width),
            StyleValueRef::MinHeight(min_height) => Self::apply_unit(min_height, &mut self.node_bundle.style.min_height),
            StyleValueRef::MaxWidth(max_width) => Self::apply_unit(max_width, &mut self.node_bundle.style.max_width),
            StyleValueRef::MaxHeight(max_height) => Self::apply_unit(max_height, &mut self.node_bundle.style.max_height),
            StyleValueRef::BorderWeight(weight) => self.apply_border_weight(weight),
            StyleValueRef::BorderColor(color) => Self::apply_border_color(color, &mut self.node_bundle.border_color),
            _ => {
                tracing::warn!("Skipping unsupported style: {:?}", style_value);
            }
        }
    }
    
    //--
    /// Apply a weight value to a float.
    #[inline(always)]
    fn apply_weight(weight: &f32, val: &mut f32) {
        *val = *weight;
    }
    
    //--
    /// Apply a FlexDirection style to the ElementNodeBundle.
    #[inline(always)]
    fn apply_unit(unit: &slate::style::primitive::Unit<f32>, val: &mut Val) {
        *val = unpack_unit(unit);
    }
    
    /// TODO
    #[inline(always)]
    fn apply_rect(rect: &slate::style::primitive::Rect<f32>, val: &mut UiRect) {
        *val = unpack_rect(rect);
    }
    
    //--
    /// Apply a Flex style to the ElementNodeBundle.
    #[inline(always)]
    fn apply_flex(&mut self, flex: &slate::style::property::Flex) {
        tracing::warn!("Skipping unsupported style: Flex({:?})", flex);
    }
    
    /// Apply a FlexDirection style to the ElementNodeBundle.
    #[inline(always)]
    fn apply_flex_direction(&mut self, flex_direction: &slate::style::property::FlexDirection) {
        self.node_bundle.style.flex_direction = match flex_direction {
            slate::style::property::FlexDirection::Row => FlexDirection::Row,
            slate::style::property::FlexDirection::Column => FlexDirection::Column,
        };
    }
    
    fn apply_gap(gap: &slate::style::property::Gap, style: &mut bevy::ui::Style) {
        style.row_gap = unpack_unit(gap.unit());
        style.column_gap = unpack_unit(gap.unit());
    }
    
    /// Apply a BoxSize style to the ElementNodeBundle.
    #[inline(always)]
    fn apply_box_size(box_size: &slate::style::property::BoxSize, style: &mut bevy::ui::Style) {
        let Size2d(width, height) = unpack_size_2d(box_size.get_size_2d());
        style.width = width;
        style.height = height;
    }
    
    /// Apply a BackgroundColor style to the ElementNodeBundle.
    #[inline(always)]
    fn apply_background_color(background_color: &slate::style::property::BackgroundColor, color: &mut bevy::ui::BackgroundColor) {
        *color = bevy::ui::BackgroundColor(unpack_color(background_color.color()));
    }
    
    /// Apply a BorderWeight style to the ElementNodeBundle.
    #[inline(always)]
    fn apply_border_weight(&mut self, weight: &slate::style::property::BorderWeight) {
        self.node_bundle.style.border = unpack_rect(weight.rect());
    }
    
    /// Apply a BorderColor style to the ElementNodeBundle.
    #[inline(always)]
    fn apply_border_color(border_color: &slate::style::property::BorderColor, color: &mut bevy::ui::BorderColor) {
        *color = bevy::ui::BorderColor(unpack_color(border_color.color()));
    }
}

/// Unpack a Slate Size2d into a Bevy Size2d.
#[inline(always)]
fn unpack_size_2d(rect: &slate::style::primitive::Size2d<f32>) -> Size2d {
    Size2d(
        unpack_unit(rect.x()),
        unpack_unit(rect.y()),
    )
}

/// Unpack a Slate Rect into a Bevy UiRect.
#[inline(always)]
fn unpack_rect(rect: &slate::style::primitive::Rect<f32>) -> bevy::ui::UiRect {
    UiRect::new(
        unpack_unit(rect.left()),
        unpack_unit(rect.right()),
        unpack_unit(rect.top()),
        unpack_unit(rect.bottom()),
    )
}

/// Unpack a Slate Unit into a Bevy Val.
#[inline(always)]
fn unpack_unit(unit: &slate::style::primitive::Unit<f32>) -> bevy::ui::Val {
    match unit {
        Unit::Px(value) => Val::Px(*value),
        Unit::Percent(value) => Val::Percent(*value),
        Unit::Full => Val::Percent(100.),
        Unit::Auto => Val::Auto,
        Unit::None => Val::Auto,
    }
}

/// Unpack a Slate Color into a Bevy Color.
#[inline(always)]
fn unpack_color(color: &slate::style::primitive::Color) -> bevy::render::color::Color {
    match color {
        slate::style::primitive::Color::Rgba(r, g, b, a) => bevy::render::color::Color::rgba_u8(*r, *g, *b, *a),
        slate::style::primitive::Color::Hsla(h, s, l, a) => bevy::render::color::Color::hsla(*h * 360.0, *s, *l, *a),
        slate::style::primitive::Color::Transparent => bevy::render::color::Color::rgba_u8(0, 0, 0, 0),
    }
}
