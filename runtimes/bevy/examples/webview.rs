#![allow(unused)]

use std::collections::HashMap;
use std::cell::OnceCell;
use std::sync::Arc;

use bevy::ui::Style;
use dpi::LogicalPosition;
use uuid::Uuid as UUID;

use crossbeam_channel::Sender;
use crossbeam_channel::Receiver;
use crossbeam_channel::bounded;

use webview2::Controller;
use webview2::EnvironmentBuilder;

use winapi::um::winuser::GetClientRect;
use winapi::shared::windef::RECT;

use raw_window_handle::RawWindowHandle;

use bevy::app::App;
use bevy::app::Plugin;
use bevy::app::Startup;
use bevy::app::PreUpdate;
use bevy::app::Update;
use bevy::app::Last;
use bevy::prelude::GlobalTransform;
use bevy::prelude::Transform;
use bevy::ui::Node;
// use bevy::ui::Node;
use bevy::ui::UiRect;
use bevy::ecs::prelude::Entity;
use bevy::ecs::query::Added;
use bevy::ecs::system::NonSend;
use bevy::ecs::prelude::Event;
use bevy::ecs::prelude::EventReader;
use bevy::ecs::prelude::EventWriter;
use bevy::ecs::component::Component;
use bevy::ecs::system::Commands;
use bevy::ecs::system::Query;
use bevy::ecs::system::NonSendMut;
use bevy::ecs::query::With;
use bevy::window::PrimaryWindow;
use bevy::window::WindowResized;
use bevy::winit::WinitWindows;

use bevy_slate::BevySlatePlugin;
use bevy_slate::config::BevySlateConfig;
use bevy_slate::window::Window;
use bevy_slate::window::WindowKind;

pub const LOG_FILTER: &str = "error,webview=trace,bevy_slate_basic=debug,bevy_slate=debug,slate=debug,wgpu_core=warn,wgpu_hal=warn";

const WEBVIEW_INSET: i32 = 50;

//--
fn main() {
    slate::log::init(LOG_FILTER);
    
    App::new()
        .add_plugins(BevySlatePlugin::default())
        .add_plugins(WebViewPlugin::default())
        .run();
}

#[derive(oops::Error)]
pub enum WebViewError {
    #[msg("missing expected webview for entity {:0}")]
    Missing(Entity),
}

#[derive(Default)]
struct WebViewPlugin;

impl Plugin for WebViewPlugin {
    // TODO
    fn build(&self, app: &mut App) {
        let (event_tx, event_rx) = bounded(100);
        let (command_tx, command_rx) = bounded(100);
        
        app.insert_non_send_resource(WebViewProvider::new(command_tx, event_rx));
        
        app.add_event::<WebViewEvent>();
        
        app.add_systems(Startup, setup_webview_provider);
        app.add_systems(Startup, setup_webview_elements);
        
        app.add_systems(Last, spawn_webview_controllers);
        app.add_systems(Last, sync_window_resize);
        app.add_systems(Last, handle_webview_events);
    }
}

/// TODO
pub struct WebViewProvider<'provider> {
    /// TODO
    env_builder: EnvironmentBuilder<'provider>,
    
    /// TODO
    controllers: HashMap<UUID, Arc<OnceCell<Controller>>>,
    
    /// TODO
    command_tx: Sender<WebViewCommand>,
    
    /// TODO
    events_rx: Receiver<WebViewEvent>,
}

impl WebViewProvider<'_> {
    /// TODO
    fn new(command_tx: Sender<WebViewCommand>, events_rx: Receiver<WebViewEvent>) -> Self {
        WebViewProvider {
            env_builder: EnvironmentBuilder::new(),
            controllers: HashMap::new(),
            command_tx,
            events_rx,
        }
    }
}

/// TODO
pub fn setup_webview_provider(
    mut webview_provider: NonSendMut<WebViewProvider>,
    windows: Query<(Entity, &Window), With<PrimaryWindow>>,
    raw_windows: NonSend<WinitWindows>,
    mut commands: Commands,
) {
    commands.spawn(bevy::prelude::Camera2d);
}

//---
/// TODO
#[derive(Component, Default, Debug)]
struct WebViewElement {
    /// TODO
    uuid: UUID,
}

impl WebViewElement {
    /// TODO
    pub fn new() -> Self {
        WebViewElement {
            uuid: UUID::new_v4(),
        }
    }
}

impl WebViewElement {
    /// TODO
    pub fn put_bounds(&mut self, rect: RECT) -> Result<(), ()> {
        Ok(()) // TODO
    }
    /// TODO
    pub fn notify_parent_window_position_changed(&mut self) -> () {
        // tracing::debug!("Notify webview window position change ..");
    }
}

/// TODO
fn setup_webview_elements(
    mut commands: Commands
) {
    for i in 0..2 {
        tracing::trace!("Adding webview #{} ..", i);
        
        let node_bundle = if i == 0 {
            bevy::prelude::NodeBundle {
                style: Style {
                    width: bevy::ui::Val::Px(400.),
                    height: bevy::ui::Val::Px(400.),
                    top: bevy::ui::Val::Px(50.),
                    left: bevy::ui::Val::Px(50.),
                    ..Default::default()
                },
                background_color: bevy::ui::BackgroundColor(bevy::color::Color::srgba(255., 255., 255., 0.2)),
                ..Default::default()
            }
        } else {
            bevy::prelude::NodeBundle {
                style: Style {
                    width: bevy::ui::Val::Px(400.),
                    height: bevy::ui::Val::Px(400.),
                    top: bevy::ui::Val::Px(50.),
                    left: bevy::ui::Val::Px(500.),
                    ..Default::default()
                },
                background_color: bevy::ui::BackgroundColor(bevy::color::Color::srgba(255., 255., 255., 0.2)),
                ..Default::default()
            }
        };
        
        commands.spawn((node_bundle, WebViewElement::new()));
    }
}

// System to spawn controllers for new WebViewElements
fn spawn_webview_controllers(
    windows: Query<(Entity, &Window), With<PrimaryWindow>>,
    winit_windows: NonSend<WinitWindows>,
    mut webview_provider: NonSendMut<WebViewProvider>,
    mut webviews: Query<(Entity, &Node, &GlobalTransform, &Transform, &WebViewElement), Added<WebViewElement>>,
    mut commands: Commands,
) {
    use raw_window_handle::RawWindowHandle;
    use raw_window_handle::HasWindowHandle;
    
    for (entity, node, global, local, webview) in webviews.iter_mut() {
        tracing::debug!("Handling new webview (entity#{:})", entity);
        tracing::trace!("Local Transform: {:#?}", local);
        
        // Create a new controller using the provider's builder
        if let Ok((entity, window)) = windows.get_single() {
            let controller = Arc::new(OnceCell::new());
            
            use winapi::shared::windef::HWND;
            
            // TODO: Get the HWND for the window.
            let winit_window = winit_windows.get_window(entity).expect("winit window");
            let window_handle = winit_window.window_handle().expect("window handle").as_raw();
            
            let RawWindowHandle::Win32(win32_window_handle) = window_handle else {
                return tracing::error!("Failed to get raw window handle!");
            };
            
            let raw_window_handle = win32_window_handle.hwnd.get() as HWND;
            tracing::debug!("Raw window handle: {:?}", raw_window_handle);
            
            if raw_window_handle.is_null() {
                return tracing::error!("Invalid HWND for WebView2 controller");
            }
            
            let controller_clone = controller.clone();
            
            let pos = global.translation();
            let size = node.size();
        
            tracing::debug!("Setting webview position to: {:?}", (pos, size));
        
            if let Err(error) = webview_provider.env_builder.build(move |env| {
                env?.create_controller(raw_window_handle, move |controller| {
                    let controller = controller?;
                    let webview = controller.get_webview()?;
                    
                    let settings = webview.get_settings()?;
                    
                    settings.put_is_script_enabled(true)?;
                    settings.put_are_default_context_menus_enabled(true)?;
                    settings.put_is_status_bar_enabled(false)?;
                    settings.put_is_zoom_control_enabled(false)?;
                    
                    let bounds = unsafe {
                        let mut rect = core::mem::zeroed();
                        GetClientRect(raw_window_handle, &mut rect);
                        rect
                    };
                    
                    controller.put_bounds(bounds);
                    // controller.put_bounds(position_to_rect((), pos, size))?;
                    webview.navigate("https://google.com")?;
                    
                    tracing::trace!("Added webview: {:?}", controller);
                    controller_clone.set(controller).expect("set controller");
                    
                    Ok(()) // <3
                })
            }) {
                // Handle errors (e.g., log an error message)
                tracing::error!("Couldn't create WebView2 controller for {:?}: {:?}", entity, error);
            }
            
            // Store the controller in the provider's HashMap
            webview_provider.controllers.insert(webview.uuid, controller);
        }
    }
}

fn sync_window_resize(
    windows: Query<(Entity, &Window), With<PrimaryWindow>>,
    winit_windows: NonSend<WinitWindows>,
    mut webview_provider: NonSendMut<WebViewProvider>,
    mut webviews: Query<(Entity, &GlobalTransform, &Node, &mut WebViewElement)>,
    mut resize_evtr: EventReader<WindowResized>,
    mut commands: Commands,
) {
    if resize_evtr.len() > 0 {
        resize_evtr.clear();
        
        use raw_window_handle::RawWindowHandle;
        use raw_window_handle::HasWindowHandle;
        
        for (entity, transform, node, mut webview) in webviews.iter_mut() {
            if let Ok((entity, window)) = windows.get_single() {
                use winapi::shared::windef::HWND;
            
                // TODO: Get the HWND for the window.
                let winit_window = winit_windows.get_window(entity).expect("winit window");
                let window_handle = winit_window.window_handle().expect("window handle").as_raw();
                
                let RawWindowHandle::Win32(win32_window_handle) = window_handle else {
                    return tracing::error!("Failed to get raw window handle!");
                };
                
                let raw_window_handle = win32_window_handle.hwnd.get() as HWND;
                tracing::debug!("Raw window handle: {:?}", raw_window_handle);
                
                if raw_window_handle.is_null() {
                    return tracing::error!("Invalid HWND for WebView2 controller");
                }
                
                if let Some(webview_controller) = webview_provider.controllers.get(&webview.uuid) {
                    let pos = transform.translation();
                    let size = node.size();
                    
                    if let Some(controller_lock) = webview_controller.get() {
                        let bounds = unsafe {
                            let mut rect = core::mem::zeroed();
                            GetClientRect(raw_window_handle, &mut rect);
                            rect
                        };
                        // if let Err(error) = controller_lock.put_bounds(position_to_rect((), pos, size)) {
                        if let Err(error) = controller_lock.put_bounds(bounds) {
                                // TODO: Better error recovery here.
                            tracing::error!("Failed to resize webview: {:?}", error)
                        }
                    }
                }
            }
        }
    }
}

fn position_to_rect(window: (), pos: bevy::prelude::Vec3, size: bevy::prelude::Vec2) -> RECT {
    let bounds = (0, 300, 300, 0);
    
    // Extract window dimensions from the RECT
    let window_width = (bounds.1 - bounds.3) as f32;
    let window_height = (bounds.2 - bounds.0) as f32;
    
    // Calculate RECT coordinates in screen space
    let left = pos.x - (size.x / 2.0);
    let top = pos.y + size.y / 2.0;
    let right = pos.x + (size.x / 2.0);
    let bottom = pos.y - size.y / 2.0;
    
    // Convert to WebView2-compatible RECT with top-left origin
    // RECT {
    //     left: (left + window_width / 2.0) as i32 + bounds.3,
    //     top: (window_height / 2.0 - top) as i32 + bounds.0,
    //     right: (right + window_width / 2.0) as i32 + bounds.3,
    //     bottom: (window_height / 2.0 - bottom) as i32 + bounds.0,
    // }
    RECT {
        left: 0,
        top: 0,
        right: 600,
        bottom: 600,
    }
}

/// TODO
fn handle_webview_events(
    webview_provider: NonSend<WebViewProvider>,
    mut webview_evt: EventWriter<WebViewEvent>,
) {
    // tracing::trace!("Watching for webview events ..");
    
    while let Ok(message) = webview_provider.events_rx.recv() {
        tracing::debug!("Got message from WebView Channel: {:#?}", message);
        match message {
            // TODO
            _ => {
                webview_evt.send(message);
            },
        }
    }
}

//---
/// TODO
enum WebViewCommand {
    /// TODO
    CreateWebView,
    
    /// TODO
    UpdateBounds,
}

//---
// Define a custom Bevy event type (optional)
#[derive(Event, Debug)]
enum WebViewEvent {
    /// TODO
    PageNavigated(String),
}
