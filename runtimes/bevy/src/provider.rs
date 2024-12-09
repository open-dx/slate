use core::ops::Deref;
use core::ops::DerefMut;

use bevy::color::Color;
use bevy::asset::AssetServer;
use bevy::asset::Handle;
use bevy::ecs::prelude::*;
use bevy::ecs::entity::Entity;
use bevy::ecs::component::Component;
use bevy::ecs::system::Commands;
use bevy::ecs::system::Query;
use bevy::hierarchy::BuildChildren;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::Image;
use bevy::ui::prelude::*;
use bevy::ui::FocusPolicy;
use bevy::text::Font;
use bevy::text::Text;
use bevy::text::TextSection;
use bevy::text::TextStyle;

use slate::element::Content;
use slate::event::EventPin;
use slate::event::ClickEvent;
use slate::style::property::FontFamily;
use slate::style::property::FontSize;
use slate::surface::Surface;
use slate::surface::SurfaceUpdate;
use slate::scaffold::Scaffold;
use slate::scaffold::ScaffoldError;
use slate::element::ElementNode;
use slate::element::DrawReport;
use slate::element::UUID;
use slate::style::StyleValue;
use slate::style::primitive::Unit;
use slate::style::property::ContentColor;
use slate::collections::HashMap;

use crate::webview::WebViewDisplay;

/// Convenience type for unpacking a 2D size.
struct Size2d(Val, Val);

pub struct FontRegistry {
    assets: Vec<Handle<Font>>,
    index: HashMap<String, usize>,
}

impl FontRegistry {
    pub fn new() -> Self {
        FontRegistry {
            assets: Vec::new(),
            index: HashMap::new(),
        }
    }
    
    pub fn with_capacity(cap: usize) -> Self {
        FontRegistry {
            assets: Vec::with_capacity(cap),
            index: HashMap::with_capacity(cap),
        }
    }
}

impl FontRegistry {
    pub fn add(&mut self, family: &str, asset: Handle<Font>) -> &Self {
        let idx = self.assets.len();
        self.assets.push(asset);
        self.index.insert(family.to_string(), idx);
        self // etc..
    }
}

impl FontRegistry {
    pub fn get(&self, family: &str) -> Option<Handle<Font>> {
        self.index.get(family).map(|f| self.assets[*f].clone_weak())
    }
}

//---
/// TODO
#[derive(Component)]
pub struct WindowSurface {
    /// The surface that provides the scene/ui/whatever
    surface: Surface<'static>,
    
    /// The entity that represents the surface in the Bevy hierarchy.
    root_entity: Option<Entity>,
    
    /// TODO
    entity_index: HashMap<UUID, Entity>,
    
    /// TODO: Make this a font asset manager.
    font_assets: FontRegistry,
}

impl WindowSurface {
    const MIN_ENTITIES: usize = 1000;
    
    const MIN_FONTS: usize = 60;
    
    /// TODO
    pub fn new() -> Self {
        WindowSurface {
            surface: Surface::<'static>::new(),
            root_entity: None,
            entity_index: HashMap::with_capacity(Self::MIN_ENTITIES),
            font_assets: FontRegistry::with_capacity(Self::MIN_FONTS),
        }
    }
    
    /// TODO
    pub fn with_root(mut self, root: Entity) -> Self {
        self.root_entity = Some(root);
        self // etc..
    }
    
    /// TODO
    pub fn with_font(mut self, family: &str, font: Handle<Font>) -> Self {
        self.font_assets.add(family, font);
        self // etc..
    }
}

impl WindowSurface {
    /// TODO
    pub fn from_surface(surface: Surface<'static>) -> Self {
        WindowSurface {
            surface,
            root_entity: None,
            entity_index: HashMap::with_capacity(Self::MIN_ENTITIES),
            font_assets: FontRegistry::with_capacity(Self::MIN_FONTS),
        }
    }
}

impl WindowSurface {
    /// TODO
    pub fn surface(&self) -> &Surface<'static> {
        &self.surface
    }
    
    /// TODO
    pub fn surface_mut(&mut self) -> &mut Surface<'static> {
        &mut self.surface
    }
}

impl Deref for WindowSurface {
    type Target = Surface<'static>;
    
    /// TODO
    fn deref(&self) -> &Self::Target {
        &self.surface
    }
}

impl DerefMut for WindowSurface {
    /// TODO
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.surface
    }
}

impl WindowSurface {
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
        #[cfg(feature="verbose")]
        tracing::trace!("Found {:?} updates ..", surface_updates.len());
        
        // Stores a list of updates that have been consumed by the system.
        // 
        // Since we update recursively, we need to keep track of which
        // updates have already made it into their destimation and which
        // ones haven't so we don't duplicate render operations.
        // 
        // At the end of the update pass, this is drained into the entity
        // index for use in future update calls, indexing, etc.
        let mut consumed_updates = HashMap::with_capacity(surface_updates.len());
        
        // TODO: Evaluate: Sort updates by kind?
        //  1. Add
        //  2. Update
        //  3. Remove
        
        // Get a copy of the surface Entity so we can operate on it.
        // TODO: In some cases, we'll want to operate on an entity determined
        //  by which node is being updated (e.g. when updating a node's style).
        let Some(root_entity) = self.root_entity else {
            return tracing::warn!("SurfaceProvider has no entity.");
        };
        
        let filter: fn(&SurfaceUpdate) -> bool = |_| {
            true // TODO
        };
        
        // TODO: Iterate over the vec of updates
        for surface_update in surface_updates.drain(..).filter(filter) {
            match surface_update {
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
                            .entity(root_entity)
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
        let Some(root_entity) = self.root_entity else {
            return tracing::warn!("SurfaceProvider has no entity.");
        };
        
        let Some(element) = self.surface.get_node(element_uuid) else {
            // TODO: Handle this error visually in the tree.
            return tracing::warn!("Can't find Element {:?} for rendering.", element_uuid);
        };
        
        for event in element.events() {
            match **event {
                EventPin::Click(ev) => {
                    ev(&ClickEvent);
                }
            }
        }
        
        // TODO: Get the element's style from the surface.
        let mut element_node = ElementNodeBundle::from(element);
        
        let text = element_node.content.text.take().and_then(|mut text| {
            if let Some(font_asset) = self.font_assets.get(&element_node.font_style.family) {
                for section in &mut text.sections {
                    section.style.font = font_asset.clone_weak();
                    section.style.font_size = element_node.font_style.size;
                }
            }
            Some(text)
        });
        
        let webview_display = element_node.content.webview.take();
        
        #[cfg(all(feature="debug", feature="verbose", feature="inspect"))]
        tracing::debug!("ElementNodeBundle:\n{:#?}", element_node);
        
        let mut element_node_entity = builder.spawn(element_node);
        if let Some(text) = text {
            element_node_entity
                .with_children(|child_builder| {
                    child_builder.spawn((
                        Interaction::default(),
                        TextBundle {
                            text,
                            ..Default::default()
                        }
                    ));
                });
        }
        
        if let Some(webview_display) = webview_display {
            element_node_entity
                .insert(webview_display);
        }
        
        if let Some(element_children) = self.surface.get_children_of(&element_uuid) {
            element_node_entity
                .with_children(|child_builder| {
                    for child_element_node in element_children.into_iter() {
                        self.spawn_element_node(child_builder, &child_element_node.uuid(), consumed_updates);
                    }
                });
        }
        
        consumed_updates.insert(*element_uuid, element_node_entity.id());
        
        #[cfg(feature = "inspect")]
        tracing::trace!("Spawned Element#{:?} at Node {:?}", element_uuid, element_entity.id());
    }
    
    fn update_element_node(&self, commands: &mut Commands, element_uuid: &UUID, element_entity: Entity) {
        match self.surface().get_node(element_uuid) {
            Some(element_node) => {
                let mut element_cmd = commands.entity(element_entity);
                
                tracing::warn!("TODO: Update Bevy Node for {:?} at Node {:?}", element_uuid, element_entity);
            }
            
            None => {
                return tracing::warn!("Attempting to update untracked node {:?}; Skipping ..", element_uuid);
                // TODO: Pass the error off to a diagnostic report.
            }
        }
    }
}

/// TODO
#[derive(Bundle)]
pub struct SurfaceNodeBundle {
    /// TODO
    node: NodeBundle,
}

impl From<&Surface<'_>> for SurfaceNodeBundle {
    /// TODO
    fn from(_surface_ref: &Surface<'_>) -> Self {
        SurfaceNodeBundle {
            node: NodeBundle {
                style: Style {
                    // display: Display::None,
                    flex_direction: FlexDirection::Column,
                    flex_grow: 0.,
                    flex_shrink: 1.,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

//---
/// TODO
pub(crate) fn setup_new_surface(
    mut surface_query: Query<(Entity, &mut WindowSurface), Added<WindowSurface>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for (surface_entity, mut surface_provider) in surface_query.iter_mut() {
        let mut surface_cmds = commands.entity(surface_entity);
        
        surface_cmds.insert(SurfaceNodeBundle::from(surface_provider.surface()));
        surface_provider.root_entity = Some(surface_entity);
        surface_provider.font_assets.add("FiraMono-Medium", asset_server.load("fonts/FiraMono-Medium.ttf"));
        surface_provider.font_assets.add("FiraSans-Bold", asset_server.load("fonts/FiraSans-Bold.ttf"));
        surface_provider.font_assets.add("fa-regular-400", asset_server.load("fonts/fa-regular-400.ttf"));
        surface_provider.font_assets.add("fa-solid-900", asset_server.load("fonts/fa-solid-900.ttf"));
        surface_provider.font_assets.add("Montserrat-Medium", asset_server.load("fonts/Montserrat/Montserrat-Medium.ttf"));
        surface_provider.font_assets.add("Montserrat-Regular", asset_server.load("fonts/Montserrat/Montserrat-Regular.ttf"));
        surface_provider.font_assets.add("Montserrat-Light", asset_server.load("fonts/Montserrat/Montserrat-Light.ttf"));
        surface_provider.font_assets.add("Montserrat-SemiBold", asset_server.load("fonts/Montserrat/Montserrat-SemiBold.ttf"));
        
        tracing::trace!("Surface `{:?}` changed ..", surface_entity);
    }
}

//---
#[derive(Component, Default)]
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

#[derive(Component, Default, Debug)]
pub struct ElementContent {
    text: Option<Text>,
    image: Option<Image>,
    video: Option<[Image; 32]>, // lol ..
    webview: Option<WebViewDisplay>,
}

#[derive(Component, Default, Debug)]
pub struct ElementFontStyle {
    pub family: String,
    pub size: f32,
    pub color: Color,
}

#[derive(Bundle, Default, Debug)]
pub struct ElementNodeBundle {
    node: NodeBundle,
    content: ElementContent,
    font_style: ElementFontStyle,
    handle: ElementNodeHandle,
    // background_color: BackgroundColor,
    // border_color: BorderColor,
    // border_radius: BorderRadius,
}

impl ElementNodeBundle {
    pub fn new(uuid: UUID) -> Self {
        ElementNodeBundle {
            handle: ElementNodeHandle(uuid),
            ..Default::default()
        }
    }
}

impl From<&ElementNode<'_>> for ElementNodeBundle {
    fn from(element: &ElementNode<'_>) -> Self {
        let mut bundle = ElementNodeBundle::new(element.uuid());
        
        for (_style_type_id, styles) in element.stylesheet().styles() {
            #[cfg(feature = "inspect")]
            tracing::trace!("Style TypeId: {:?}", _style_type_id);
            
            for style in styles.iter() {
                #[cfg(feature = "inspect")]
                tracing::debug!("Processing Style: {:?}", style);
                
                bundle.apply_style(style);
            }
        }
        
        bundle.apply_content(element);
        
        bundle
    }
}

impl ElementNodeBundle {
    pub fn apply_content(&mut self, element: &ElementNode<'_>) {
        match element.content() {
            Some(Content::Text(content)) => {
                self.content.text = Some(Text {
                    sections: vec![
                        TextSection {
                            value: String::from(content),
                            style: TextStyle {
                                font: Handle::default(),
                                font_size: self.font_style.size,
                                color: self.font_style.color,
                            },
                        },
                    ],
                    justify: bevy::text::JustifyText::Left,
                    linebreak_behavior: bevy::text::BreakLineOn::AnyCharacter,
                });
            },
            Some(Content::WebView(address)) => {
                tracing::debug!("Found Webview with address {:?} ..", address);
                if let Ok(display) = WebViewDisplay::new().with_address(address) {
                    self.content.webview = Some(display);
                }
            },
            Some(Content::Image(_)) => {
                tracing::debug!("TODO: Image content for ElementNodeBundle");
            },
            None => {
                // Zzzz ..
            },
        };
    }
}

impl ElementNodeBundle {
    /// Apply a style to the ElementNodeBundle.
    #[inline(always)]
    fn apply_style(&mut self, style_value: &StyleValue) {
        #[allow(unreachable_patterns)]
        match style_value {
            StyleValue::Flex(flex) => Self::apply_flex(flex),
            StyleValue::FlexDirection(flex_direction) => Self::apply_flex_direction(flex_direction, &mut self.node.style.flex_direction),
            StyleValue::FlexBasis(flex_basis) => Self::apply_unit(flex_basis, &mut self.node.style.flex_basis),
            StyleValue::FlexGrow(flex_grow) => Self::apply_weight(flex_grow, &mut self.node.style.flex_grow),
            StyleValue::FlexShrink(flex_shrink) => Self::apply_weight(flex_shrink, &mut self.node.style.flex_shrink),
            StyleValue::AlignItems(align_items) => self.node.style.align_items = match align_items {
                slate::style::property::AlignItems::Center => bevy::ui::AlignItems::Center,
                _ => bevy::ui::AlignItems::Default
            },
            StyleValue::JustifyContent(justify_content) => self.node.style.justify_content = match justify_content {
                slate::style::property::JustifyContent::Center => bevy::ui::JustifyContent::Center,
                slate::style::property::JustifyContent::Start => bevy::ui::JustifyContent::Start,
                _ => bevy::ui::JustifyContent::Default
            },
            StyleValue::Gap(gap) => Self::apply_gap(gap, &mut self.node.style),
            StyleValue::BackgroundColor(color) => Self::apply_background_color(color, &mut self.node.background_color),
            StyleValue::Margin(margin) => Self::apply_rect(margin, &mut self.node.style.margin),
            StyleValue::Padding(padding) => Self::apply_rect(padding, &mut self.node.style.padding),
            StyleValue::BoxSize(box_size) => Self::apply_box_size(box_size, &mut self.node.style),
            StyleValue::Width(width) => Self::apply_unit(width, &mut self.node.style.width),
            StyleValue::Height(height) => Self::apply_unit(height, &mut self.node.style.height),
            StyleValue::MinWidth(min_width) => Self::apply_unit(min_width, &mut self.node.style.min_width),
            StyleValue::MinHeight(min_height) => Self::apply_unit(min_height, &mut self.node.style.min_height),
            StyleValue::MaxWidth(max_width) => Self::apply_unit(max_width, &mut self.node.style.max_width),
            StyleValue::MaxHeight(max_height) => Self::apply_unit(max_height, &mut self.node.style.max_height),
            StyleValue::FontFamily(family) => self.apply_font_family(family),
            StyleValue::FontSize(size) => self.apply_font_size(size),
            StyleValue::ContentColor(color) => self.apply_text_color(color),
            StyleValue::BorderWeight(weight) => self.apply_border_weight(weight),
            StyleValue::BorderRadius(radius) => Self::apply_border_radius(radius, &mut self.node.border_radius),
            StyleValue::BorderColor(color) => Self::apply_border_color(color, &mut self.node.border_color),
            #[cfg(feature = "dev")]
            _ => {
                tracing::warn!("Skipping unsupported style: {:?}", style_value);
            }
        }
    }
    
    fn apply_font_family(&mut self, font_family: &FontFamily) {
        self.font_style.family = font_family.name().to_owned();
    }
    
    fn apply_font_size(&mut self, size: &FontSize) {
        self.font_style.size = match size.unit() {
            Unit::Px(pixels) => *pixels,
            _ => {
                tracing::warn!("unsupported font size '{:?}'", size);
                14.0 // TODO: Get a default size from the styleguide ..
            },
        };
    }
    
    fn apply_text_color(&mut self, color: &ContentColor) {
        self.font_style.color = unpack_color(&color);
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
    
    /// TODO
    #[inline(always)]
    fn apply_border_radius(rect: &slate::style::primitive::Rect<f32>, val: &mut BorderRadius) {
        *val = BorderRadius {
            top_left: unpack_unit(rect.left()),
            top_right: unpack_unit(rect.left()),
            bottom_left: unpack_unit(rect.left()),
            bottom_right: unpack_unit(rect.left()),
        };
    }
    
    //--
    /// Apply a Flex style to the ElementNodeBundle.
    #[inline(always)]
    fn apply_flex(flex: &slate::style::property::Flex) {
        tracing::warn!("Skipping unsupported style: Flex({:?})", flex);
    }
    
    /// Apply a FlexDirection style to the ElementNodeBundle.
    #[inline(always)]
    fn apply_flex_direction(source: &slate::style::property::FlexDirection, target: &mut bevy::ui::FlexDirection) {
        *target = match source {
            slate::style::property::FlexDirection::Row => bevy::ui::FlexDirection::Row,
            slate::style::property::FlexDirection::Column => bevy::ui::FlexDirection::Column,
        };
    }
    
    #[inline(always)]
    fn apply_gap(source: &slate::style::property::Gap, target: &mut bevy::ui::Style) {
        target.row_gap = unpack_unit(source.unit());
        target.column_gap = unpack_unit(source.unit());
    }
    
    /// Apply a BoxSize style to the ElementNodeBundle.
    #[inline(always)]
    fn apply_box_size(box_size: &slate::style::property::BoxSize, target: &mut bevy::ui::Style) {
        let Size2d(width, height) = unpack_size_2d(box_size.get_size_2d());
        target.width = width;
        target.height = height;
    }
    
    /// Apply a BackgroundColor style to the ElementNodeBundle.
    #[inline(always)]
    fn apply_background_color(background_color: &slate::style::property::BackgroundColor, color: &mut bevy::ui::BackgroundColor) {
        *color = bevy::ui::BackgroundColor(unpack_color(background_color.color()));
    }
    
    /// Apply a BorderWeight style to the ElementNodeBundle.
    #[inline(always)]
    fn apply_border_weight(&mut self, weight: &slate::style::property::BorderWeight) {
        self.node.style.border = unpack_rect(weight.rect());
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
fn unpack_color(color: &slate::style::primitive::Color) -> bevy::color::Color {
    match color {
        slate::style::primitive::Color::Rgba(r, g, b, a) => bevy::color::Color::srgba_u8(*r, *g, *b, *a),
        slate::style::primitive::Color::Hsla(h, s, l, a) => bevy::color::Color::hsla(*h * 360.0, *s, *l, *a),
        slate::style::primitive::Color::Transparent => bevy::color::Color::srgba_u8(0, 0, 0, 0),
    }
}
