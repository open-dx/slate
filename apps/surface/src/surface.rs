use std::num::NonZeroU8;
use std::process::ExitCode;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use slate::event::ClickEvent;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

use context::ContextMenu;
use context::ContextMenuEvent;

use bevy::prelude::ChildBuild;
use bevy::prelude::BuildChildren;
use bevy::prelude::DespawnRecursiveExt;
use bevy::prelude::Parent;
use bevy::app::AppExit;
use bevy::app::prelude::*;
use bevy::ecs::prelude::*;
use bevy::ui::FocusPolicy;
use bevy::ui::prelude::*;
use bevy::color::Color;
use bevy::color::Alpha;
use bevy::input::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::math::prelude::*;
use bevy::core_pipeline::core_2d::Camera2d;
use bevy::gizmos::config::GizmoConfigStore;
use bevy::gizmos::config::DefaultGizmoConfigGroup;
use bevy::tasks::AsyncComputeTaskPool;

use bevy_slate::BevySlatePlugin;
use bevy_slate::config::BevySlateConfig;
use bevy_slate::provider::NodeSurface;
use bevy_slate::reticle::ReticleShape;
use bevy_slate::reticle::ReticlePosition;
use bevy_slate::window::Window;
use bevy_slate::window::WindowEvent;
use bevy_slate::window::WindowLevel;

use slate::style::StyleSheet;
use slate::component::x::layout::Div;
use slate::component::x::layout::Section;
use slate::component::x::layout::Sidebar;
use slate::component::x::content::TextBlock;
use slate::component::x::content::WebView;
use slate::component::x::input::Button;

#[allow(unused)]
pub struct SignalChannel(Sender<Signal>, Receiver<Signal>);

impl SignalChannel {
    pub fn new(size: usize) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(size);
        SignalChannel(tx, rx)
    }
}

#[allow(unused)]
impl SignalChannel {
    pub fn sender(&mut self) -> Sender<Signal> {
        self.0.clone()
    }
    
    pub fn receiver(&self) -> &Receiver<Signal> {
        &self.1
    }
}

/// TODO
#[allow(unused)]
pub enum Signal {
    /// TODO
    Terminate,
}

//---
/// TODO
pub struct Surface {
    /// TODO
    app: App,
    
    /// TODO
    _signals: SignalChannel,
}

impl Surface {
    /// TODO
    pub fn new() -> Self {
        Surface {
            app: App::new(),
            _signals: SignalChannel::new(100),
        }
    }
}

impl Surface {
    /// The underlying window color for transparent windows.
    /// TODO: Set this to Color::None when in release mode.
    /// TODO: Set this to Color::Pink (or whatever it is) in debug mode.
    pub const WINDOW_CLEAR_COLOR: Color = Color::hsla(220.0, 0.11, 0.11, 1.0);
    
    /// TODO
    pub const _DEFAULT_WINDOW_SIZE: [i32; 2] = [800, 400];
    
    /// TODO
    pub fn build(mut self) -> Result<Self, SurfaceError> {
        self.app.add_plugins(BevySlatePlugin::new(self.create_bevy_slate_config()));
        
        self.app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin);
        self.app.add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin);
        self.app.add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin);
        
        self.app.add_plugins(iyes_perf_ui::PerfUiPlugin);
        
        self.app.insert_non_send_resource(ContextMenu::new());
        self.app.add_event::<ContextMenuEvent>();
        
        self.app.add_event::<WindowEvent>();
        
        self.app.add_systems(PreStartup, setup_service);
        
        self.app.add_systems(PreStartup, setup_gizmos);
        self.app.add_systems(PreStartup, setup_cameras);
        self.app.add_systems(PreStartup, setup_tools);
        self.app.add_systems(PreStartup, crate::surface::context::setup_menu);
        
        self.app.add_systems(Startup, setup_artboards);
        
        self.app.add_systems(PostStartup, draw_toolbar);
        // self.app.add_systems(PostStartup, setup_performance_ui);
        
        self.app.add_systems(PreUpdate, setup_primary_window);
        self.app.add_systems(PreUpdate, route_keybinds);
        
        self.app.add_systems(Update, bevy_slate::window::route_window_events);
        self.app.add_systems(Update, bevy_slate::window::toggle_fullscreen);
        self.app.add_systems(Update, bevy_slate::window::toggle_decorations);
        
        // TODO: Move this to the Terminal plugin.
        #[cfg(feature = "terminal")]
        self.app.add_systems(Update, TerminalProvider::render);
        
        // TODO: Move this to the SelectionTool plugin.
        self.app.add_systems(Update, show_selection_marquee);
        
        self.app.add_systems(Update, crate::surface::context::handle_menu_events);
        
        Ok(self)
    }
    
    /// TODO
    pub fn create_bevy_slate_config(&self) -> BevySlateConfig {
        BevySlateConfig::default()
            .with_log_filter(crate::log::DEFAULT_LOG_FILTER)
            .with_asset_dir("./assets")
            .with_clear_color(Surface::WINDOW_CLEAR_COLOR)
    }
    
    /// TODO
    pub fn run(&mut self) -> Result<ExitCode, SurfaceError> {
        match self.app.run() {
            AppExit::Success => Ok(ExitCode::SUCCESS),
            AppExit::Error(error) => Err(SurfaceError::RunFailed(error)),
        }
    }
}

fn setup_service() {
    use tokio::time::Duration;
    use tokio::runtime::Runtime;
    
    const SLEEP_DUR: Duration = Duration::from_secs(60*60);
    
    let thread = async move {
        loop {
            tracing::trace!("Running within nested Tokio runtime");
            
            tokio::select! {
                _ = tokio::time::sleep(SLEEP_DUR) => {
                    continue;
                }
            }
        }
    };
    
    let task = async move {
        // Create a Tokio runtime inside the async task
        let rt = Runtime::new().unwrap();
        rt.spawn(thread).await.unwrap();
    };

    AsyncComputeTaskPool::get()
        .spawn(task)
        .detach();
}

//---
/// TODO
fn setup_gizmos(
    mut gizmo_cfg: ResMut<GizmoConfigStore>,
) {
    let (gizmo_cfg, _) = gizmo_cfg.config_mut::<DefaultGizmoConfigGroup>();
    
    gizmo_cfg.enabled = true;
    gizmo_cfg.line_width = 0.5;
}

/// TODO
fn setup_cameras(
    windows: Query<&Window>,
    mut commands: Commands,
) {
    if let Ok(_) = windows.get_single() {
        commands.spawn((MainCamera, Camera2d));
    }
}

//--
/// Route app-level events.
/// TODO: Move key-bind management to a "KeyBinds" plugin.
fn route_keybinds(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut keyboard_evt: EventReader<KeyboardInput>,
    mut app_exit_evt: EventWriter<AppExit>,
    mut ui_scale: ResMut<UiScale>,
    mut window_evt: EventWriter<WindowEvent>,
) {
    use bevy::input::keyboard::KeyCode::*;
    use bevy::input::ButtonState::*;
    
    const MODIFIER_KEYS: [[KeyCode; 2]; 3] = [
        [AltLeft, AltRight],
        [ControlLeft, ControlRight],
        [ShiftLeft, ShiftRight],
    ];
    
    let alt = keyboard.any_pressed(MODIFIER_KEYS[0]);
    let ctrl = keyboard.any_pressed(MODIFIER_KEYS[1]);
    let shift = keyboard.any_pressed(MODIFIER_KEYS[2]);
    
    for event in keyboard_evt.read() {
        match event.key_code {
            // Alt+Q to exit the app.
            KeyQ if alt => {
                app_exit_evt.send(AppExit::Success);
            }
            
            // Alt+PgUp to "pin" the current on top of other windows.
            PageUp if ctrl && alt => if event.state == Pressed {
                window_evt.send(WindowEvent::SetWindowLevel(WindowLevel::AlwaysOnTop));
            }
            
            // Alt+PgDown to "un-pin" the current on top of other windows.
            PageDown if ctrl && alt => if event.state == Pressed {
                window_evt.send(WindowEvent::SetWindowLevel(WindowLevel::Normal));
            }
            
            // TODO
            Digit0 | Numpad0 if !alt && ctrl && !shift => if event.state == Pressed {
                ui_scale.0 = 1.0;
            }
            
            // TODO
            Minus | NumpadSubtract if !alt && ctrl && !shift => if event.state == Pressed {
                if ui_scale.0 > 0.5 {
                    ui_scale.0 -= 0.1;
                }
            }
            
            // TODO
            Equal | NumpadAdd if !alt && ctrl && !shift => if event.state == Pressed {
                if ui_scale.0 < 2.0 {
                    ui_scale.0 += 0.1;
                }
            }
            
            // TODO
            ref key => if !MODIFIER_KEYS.iter().any(|&m| m.contains(key)) && event.state == Pressed {
                #[cfg(feature = "verbose")]
                tracing::debug!("Untracked Key Press: {:?} (alt={}; ctrl={}; shift={})", event.key_code, alt, ctrl, shift);
            }
        }
    }
}

/// TODO
fn setup_tools(
    mut commands: Commands,
) {
    commands.spawn(NodeSurface::new());
}

pub mod fa {
    #[allow(unused)]
    pub mod icons {
        pub const BARS: &str = "\u{f0c9}";
        pub const GEAR: &str = "\u{f013}";
        pub const BUG: &str = "\u{f188}";
        pub const PLAY: &str = "\u{f04b}";
        pub const BACK: &str = "\u{f104}";
        pub const FORWARD: &str = "\u{f105}";
        pub const MINUS: &str = "\u{f068}";
        pub const PLUS: &str = "\u{2b}";
        
    }
}

#[allow(dead_code)]
pub type ConstStyleFn = fn(&mut StyleSheet);

#[allow(dead_code)]
pub const DANG_01: ConstStyleFn = |stylesheet: &mut StyleSheet| {
    use slate::style::property::BackgroundColor;
    
    stylesheet
        .push(BackgroundColor::hex("#FF000008"));
};

/// TODO
fn draw_toolbar(
    mut surface_qry: Query<&mut NodeSurface>,
    selected_tab: Local<AtomicUsize>,
    mut commands: Commands,
) {
    let mut surface = surface_qry.single_mut();
    
    const ICON_SIZE: f32 = 14.0;
    
    const TABS: [(&str, [(&str, &str); 2]); 2] = [
        (
            "Design",
            [
                ("Penpot", "https://design.penpot.app"),
                ("Excalidraw", "https://www.excalidraw.com"),
            ]
        ),
        (
            "Development",
            [
                ("Visual Studio Code", "https://vscode.dev"),
                ("Google", "https://www.google.com"),
            ]
        ),
    ];
    
    let selected_tab_idx = {
        selected_tab.store(1, Ordering::SeqCst);
        selected_tab.load(Ordering::SeqCst)
    };
    
    let report = |event: &ClickEvent| {
        tracing::debug!("Event: {:?}", event);
        // selected_tab.store(1, Ordering::SeqCst);
    };
    
    surface.draw(&mut commands, chizel::uix! {
        .toolbar {
            FlexDirection::Column,
            FlexGrow::new(0.),
            FlexShrink::new(0.),
            BackgroundColor::hex("#333333"),
            Padding::new(2.),
            BorderWeight::new(1.0),
            BorderColor::hex("#00000010"),
            BorderRadius::new(4.0),
        }
        .toolbar_section {
            FlexDirection::Column,
            FlexGrow::new(1.),
            FlexShrink::new(0.),
            Padding::new(2.),
        }
        .toolbar_button {
            // BackgroundColor::hex("#444444"),
            Margin::new(2.),
            Padding::new(2.),
            BoxSize::both(30., 30.),
            AlignItems::Center,
            JustifyContent::Center,
            // BorderColor::hex("#000000"),
            BorderWeight::new(1.0),
            BorderRadius::new(3.0),
        }
        ._icon {
            FlexGrow::new(1.),
            BackgroundColor::hex("#AAAAAA"),
            BoxSize::xy(20., 20.),
        }
        
        .workspace {
            FlexDirection::Row,
            FlexGrow::new(1.),
        }
        
        .resource_viewer {
            FlexDirection::Row,
            FlexGrow::new(1.),
            BoxSize::new(Auto, Percent(100.)),
        }
        
        #[class(workspace)]
        <Div>
            #[style(FlexGrow::new(0.))]
            #[style(FlexDirection::Row)]
            #[style(AlignItems::Center)]
            #[style(Padding::new(10.))]
            #[style(BackgroundColor::hex("#191B1FEE"))]
            <Sidebar>
                #[class(toolbar)]
                <Div>
                    #[class(toolbar_section)]
                    <Section>
                        #[class(toolbar_button)]
                        <Button value="TODO">
                            #[style(FontFamily::new("fa-solid-900"))]
                            #[style(FontSize::new(ICON_SIZE))]
                            #[style(ContentColor::hex("#777777"))]
                            <TextBlock text=fa::icons::BARS />
                        </Button>
                        
                        #[class(toolbar_button)]
                        #[style(BackgroundColor::hex("#444444"))]
                        <Button value="TODO">
                            #[style(FontFamily::new("fa-solid-900"))]
                            #[style(FontSize::new(ICON_SIZE))]
                            #[style(ContentColor::hex("#DDDDDD"))]
                            <TextBlock text=fa::icons::PLAY />
                        </Button>
                    </Section>
                    
                    #[class(toolbar_section)]
                    <Section>
                        #[class(toolbar_button)]
                        <Button value="TODO">
                            #[style(FontFamily::new("fa-solid-900"))]
                            #[style(FontSize::new(ICON_SIZE))]
                            #[style(ContentColor::hex("#777777"))]
                            <TextBlock text=fa::icons::BUG />
                        </Button>
                        
                        /// TODO
                        #[class(toolbar_button)]
                        <Button value="TODO">
                            #[style(FontFamily::new("fa-solid-900"))]
                            #[style(FontSize::new(ICON_SIZE))]
                            #[style(ContentColor::hex("#777777"))]
                            <TextBlock text=fa::icons::GEAR />
                        </Button>
                    </Section>
                </Div>
            </Sidebar>
            
            /// WebView Number One!
            #[class(resource_viewer)]
            <Div>
                #[style(FlexGrow::new(1.))]
                #[style(FlexDirection::Column)]
                <Div>
                    /// Tab Toolbar
                    #[style(FlexDirection::Row)]
                    #[style(Gap::new(10.))]
                    #[style(BackgroundColor::hex("#191B1FEE"))]
                    #[style(Padding::xy(15., 10.))]
                    <Div>
                        /// Tab Name
                        <Div>
                            #[style(FontFamily::new("Montserrat-Medium"))]
                            #[style(FontSize::new(12.))]
                            #[style(ContentColor::hex("#AAAAAA"))]
                            <TextBlock text="Designer" />
                        </Div>
                        
                        /// Address Bar
                        <Div>
                            #[style(FontFamily::new("Montserrat-Medium"))]
                            #[style(FontSize::new(12.))]
                            #[style(ContentColor::hex("#888888"))]
                            <TextBlock text="Some Tool" />
                        </Div>
                    </Div>
                    
                    /// WebView Number One!
                    #[style(FlexGrow::new(1.))]
                    <Div />
                </Div>
                
                /// WebView Number One!
                #[style(FlexGrow::new(1.))]
                #[style(BackgroundColor::hex("#191B1FEE"))]
                #[style(FlexDirection::Column)]
                <Div>
                    /// WebView Number One!
                    #[style(FlexDirection::Row)]
                    #[style(Gap::new(Px(2.)))]
                    #[style(Padding::all(5., 5., 0., 5.))]
                    <Div>
                        /// This is the first tab!
                        #[when(dogs.len() > 0)]
                        #[each(dog in dogs.iter())]
                        #[style(BackgroundColor::hex("#FFFFFF04"))]
                        #[style(Padding::xy(10., 5.))]
                        #[style(BorderRadius::new(Percent(50.)))]
                        #[on(Click(report))]
                        <Div>
                            #[style(FontFamily::new("Montserrat-Regular"))]
                            #[style(FontSize::new(12.))]
                            #[style(ContentColor::hex("#AAAAAA"))]
                            <TextBlock text={TABS[0].0} />
                        </Div>
                        
                        /// This is the second tab!
                        #[style(BackgroundColor::hex("#FFFFFF08"))]
                        #[style(Padding::xy(10., 5.))]
                        #[style(BorderRadius::new(Percent(50.)))]
                        #[on(Click(report))]
                        <Div>
                            #[style(FontFamily::new("Montserrat-Medium"))]
                            #[style(FontSize::new(12.))]
                            #[style(ContentColor::hex("#DDDDDD"))]
                            <TextBlock text={TABS[1].0} />
                        </Div>
                                
                        #[style(BackgroundColor::hex("#FFFFFF04"))]
                        #[style(Padding::xy(10., 5.))]
                        #[style(BorderRadius::new(Percent(50.)))]
                        #[on(Click(report))]
                        <Div>
                            /// Tab Name
                            #[style(FontFamily::new("Montserrat-Medium"))]
                            #[style(FontSize::new(12.))]
                            #[style(ContentColor::hex("#AAAAAA"))]
                            <TextBlock text=fa::icons::PLUS />
                        </Div>
                    </Div>
                        
                    #[style(FlexGrow::new(1.))]
                    #[style(FlexDirection::Row)]
                    <Div>
                        #[style(FlexGrow::new(1.))]
                        #[style(FlexDirection::Column)]
                        <Div>
                            /// Tab Toolbar
                            #[style(FlexDirection::Row)]
                            #[style(Gap::new(10.))]
                            #[style(Padding::xy(15., 10.))]
                            <Div>
                                #[style(FlexDirection::Row)]
                                #[style(Gap::new(5.))]
                                <Div>
                                    #[style(FontFamily::new("fa-solid-900"))]
                                    #[style(FontSize::new(12.))]
                                    #[style(ContentColor::hex("#444444"))]
                                    #[on(Click(report))]
                                    <TextBlock text=fa::icons::BACK />
                                    
                                    #[style(FontFamily::new("fa-solid-900"))]
                                    #[style(FontSize::new(12.))]
                                    #[style(ContentColor::hex("#444444"))]
                                    #[on(Click(report))]
                                    <TextBlock text=fa::icons::FORWARD />
                                </Div>
                                
                                #[style(FlexDirection::Row)]
                                #[style(Gap::new(5.))]
                                <Div>
                                    #[style(FontFamily::new("fa-solid-900"))]
                                    #[style(FontSize::new(12.))]
                                    #[style(ContentColor::hex("#444444"))]
                                    #[on(Click(report))]
                                    <TextBlock text=fa::icons::BUG />
                                </Div>
                                
                                <Div>
                                    /// Tab Name
                                    #[style(FontFamily::new("Montserrat-Medium"))]
                                    #[style(FontSize::new(12.))]
                                    #[style(ContentColor::hex("#AAAAAA"))]
                                    <TextBlock text={TABS[selected_tab_idx].1[0].0} />
                                </Div>
                                
                                <Div>
                                    /// Address Bar
                                    #[style(FontFamily::new("Montserrat-Medium"))]
                                    #[style(FontSize::new(12.))]
                                    #[style(ContentColor::hex("#888888"))]
                                    <TextBlock text={TABS[selected_tab_idx].1[0].1} />
                                </Div>
                            </Div>
                            
                            /// WebView Number One!
                            #[style(FlexGrow::new(1.))]
                            <WebView address={TABS[selected_tab_idx].1[0].1} />
                        </Div>
                        
                        #[style(FlexGrow::new(1.))]
                        #[style(FlexDirection::Column)]
                        <Div>
                            /// Tab Toolbar
                            #[style(FlexDirection::Row)]
                            #[style(Gap::new(10.))]
                            #[style(Padding::xy(15., 10.))]
                            <Div>
                                #[style(FlexDirection::Row)]
                                #[style(Gap::new(5.))]
                                <Div>
                                    #[style(FontFamily::new("fa-solid-900"))]
                                    #[style(FontSize::new(12.))]
                                    #[style(ContentColor::hex("#444444"))]
                                    #[on(Click(report))]
                                    <TextBlock text=fa::icons::BACK />
                                    
                                    #[style(FontFamily::new("fa-solid-900"))]
                                    #[style(FontSize::new(12.))]
                                    #[style(ContentColor::hex("#444444"))]
                                    #[on(Click(report))]
                                    <TextBlock text=fa::icons::FORWARD />
                                </Div>
                                
                                #[style(FlexDirection::Row)]
                                #[style(Gap::new(5.))]
                                <Div>
                                    #[style(FontFamily::new("fa-solid-900"))]
                                    #[style(FontSize::new(12.))]
                                    #[style(ContentColor::hex("#444444"))]
                                    #[on(Click(report))]
                                    <TextBlock text=fa::icons::BUG />
                                </Div>
                                
                                <Div>
                                    /// Tab Name
                                    #[style(FontFamily::new("Montserrat-Medium"))]
                                    #[style(FontSize::new(12.))]
                                    #[style(ContentColor::hex("#AAAAAA"))]
                                    <TextBlock text={TABS[selected_tab_idx].1[1].0} />
                                </Div>
                                
                                <Div>
                                    /// Address Bar
                                    #[style(FontFamily::new("Montserrat-Medium"))]
                                    #[style(FontSize::new(12.))]
                                    #[style(ContentColor::hex("#888888"))]
                                    <TextBlock text={TABS[selected_tab_idx].1[1].1} />
                                </Div>
                            </Div>
                            
                            /// WebView Number Two!
                            #[style(FlexGrow::new(1.))]
                            <WebView address={TABS[selected_tab_idx].1[1].1} />
                        </Div>
                    </Div>
                </Div>
            </Div>
        </Div>
    });
}

//---
// fn setup_performance_ui(
//     mut commands: Commands,
// ) {
//     commands.spawn(iyes_perf_ui::prelude::PerfUiAllEntries::default());
// }

//---
fn setup_primary_window(
    // mut window: Single<&mut bevy::window::Window, Added<bevy::window::PrimaryWindow>>,
) {
    // window.set_maximized(true);
}

/// TODO
fn setup_artboards(
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>,
    // mut commands: Commands,
) {
    // ..
}

//---
/// TODO
#[derive(Default)]
pub struct SelectionState {
    /// TODO
    pub bounds: Option<Entity>,
    
    /// TODO
    pub start_pos: Option<Vec2>,
}

//---
/// TODO
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct SelectionMarquee {
    /// TODO
    pub _color: Color,
}

impl SelectionMarquee {
    /// TODO
    pub fn new(color: Color) -> Self {
        SelectionMarquee {
            _color: color,
        }
    }
}

/// TODO
#[derive(Bundle, Default, Debug, Clone)]
pub struct SelectionBoundaryBundle {
    /// TODO
    pub node: Node,
    pub z_index: ZIndex,
}

impl SelectionBoundaryBundle {
    /// TODO
    pub fn new() -> Self {
        SelectionBoundaryBundle {
            node: Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                bottom: Val::Px(0.0),
                ..Default::default()
            },
            z_index: ZIndex(-1),
        }
    }
}

/// TODO
#[derive(Bundle, Default, Debug, Clone)]
pub struct SelectionMarqueeBundle {
    /// TODO
    pub marquee: SelectionMarquee,
    
    /// TODO
    pub node: Node,
    
    pub background_color: BackgroundColor,
    
    pub border_color: BorderColor,
    
    pub focus_policy: FocusPolicy,
    
    pub z_index: ZIndex,
}

impl SelectionMarqueeBundle {
    /// TODO
    pub fn new(color: Color, rect: Rect) -> Self {
        SelectionMarqueeBundle {
            marquee: SelectionMarquee::new(color),
            node: Node {
                position_type: PositionType::Absolute,
                top: Val::Px(rect.min.y),
                left: Val::Px(rect.min.x),
                width: Val::Px(rect.max.x - rect.min.x),
                height: Val::Px(rect.max.y - rect.min.y),
                right: Val::Auto,
                bottom: Val::Auto,
                border:UiRect::all(Val::Px(1.5)),
                ..Default::default()
            },
            background_color: color.with_alpha(0.2).into(),
            border_color: color.with_alpha(0.25).into(),
            focus_policy: FocusPolicy::Pass,
            z_index: ZIndex(i32::MIN),
            ..Default::default()
        }
    }
}

/// TODO
fn show_selection_marquee(
    mouse_btn: Res<ButtonInput<MouseButton>>,
    reticles: Query<&ReticlePosition, With<ReticleShape>>,
    mut selection_marquee: Query<(Entity, &Parent, &mut Node), With<SelectionMarquee>>,
    mut active_selection: Local<SelectionState>,
    mut commands: Commands,
) {
    if mouse_btn.just_pressed(MouseButton::Left) {
        // TODO: Get only the current user's reticle.
        if let Ok(reticle_pos) = reticles.get_single() {
            commands.spawn(SelectionBoundaryBundle::new()).with_children(|bounds| {
                active_selection.bounds = Some(bounds.parent_entity());
                active_selection.start_pos = Some(reticle_pos.screen);
                
                bounds.spawn(SelectionMarqueeBundle::new(
                    Color::from(bevy::color::palettes::css::PINK),
                    Rect::new(
                        reticle_pos.screen.x,
                        reticle_pos.screen.y,
                        reticle_pos.screen.x,
                        reticle_pos.screen.y,
                    )
                ));
            });
        }
    } else if mouse_btn.pressed(MouseButton::Left) {
        // TODO: Modify the position of the show_selection_marquee marquee node.
        if let Ok((_, parent, mut node)) = selection_marquee.get_single_mut() {
            // Are we working with the correct parent for the current show_selection_marquee marquee?
            if active_selection.bounds == Some(parent.get()) {
                if let Ok(reticle_pos) = reticles.get_single() {
                    let start_pos = active_selection.start_pos.unwrap_or_default();
                    
                    // Set the top and left positions based on the minimum values
                    node.top = Val::Px(start_pos.y.min(reticle_pos.screen.y));
                    node.left = Val::Px(start_pos.x.min(reticle_pos.screen.x));
                    
                    // Calculate the new width and height based on the current mouse position
                    node.width = Val::Px((reticle_pos.screen.x - start_pos.x).abs());
                    node.height = Val::Px((reticle_pos.screen.y - start_pos.y).abs()); // Fixed this line
                }
            }
        }
    } else if mouse_btn.just_released(MouseButton::Left) {
        // TOOD: Destroy the spawned show_selection_marquee marquee node.
        if let Some(bounds) = active_selection.bounds {
            commands.entity(bounds).despawn_recursive();
            active_selection.bounds = None;
        }
    }
}

//---
/// TODO
#[derive(oops::Error)]
#[allow(unused)]
pub enum SurfaceError {
    /// TODO
    #[msg("failed to boot surface")]
    BootFailed,
    
    /// TODO
    #[msg("failed to run surface (exit code {})")]
    RunFailed(NonZeroU8),
}

//---
/// TODO
#[derive(Component)]
struct MainCamera;

//---
/// TODO
#[derive(Component, Default, Debug, Clone)]
pub struct WindowMoveHandle {
    /// The Window that this handle is attached to.
    pub _window: Option<Entity>,
}

impl WindowMoveHandle {
    /// TODO
    pub fn _new(window: Option<Entity>) -> Self {
        WindowMoveHandle {
            _window: window,
        }
    }
}

//--
// TODO: Move this to a context module ..
pub mod context {
    use muda::{Menu, MenuItem, Submenu, PredefinedMenuItem};
    use muda::accelerator::Accelerator;
    use muda::accelerator::Modifiers;
    use muda::accelerator::Code;
    
    use bevy::prelude::{Event, EventReader};
    use bevy::prelude::{NonSend, NonSendMut};

    pub struct ContextMenu(Option<Menu>);

    impl ContextMenu {
        pub fn new() -> Self {
            ContextMenu(None)
        }
    }

    pub fn setup_menu(
        mut context_menu: NonSendMut<ContextMenu>,
    ) {
        // Create a new menu using muda
        let menu = Menu::new();
        let menu_item2 = MenuItem::new("Menu item #2", false, None);
        let _submenu = Submenu::with_items("Submenu Outer", true, &[
            &MenuItem::new("Menu item #1", true, Some(Accelerator::new(Some(Modifiers::ALT), Code::KeyD))),
            &PredefinedMenuItem::separator(),
            &menu_item2,
            &MenuItem::new("Menu item #3", true, None),
            &PredefinedMenuItem::separator(),
            &Submenu::with_items("Submenu Inner", true, &[
                &MenuItem::new("Submenu item #1", true, None),
                &PredefinedMenuItem::separator(),
                &menu_item2,
            ]).expect("Submenu Inner")
        ]).expect("Submenu Outer");
        
        context_menu.0 = Some(menu);
    }

    #[derive(Event, Debug)]
    #[allow(unused)]
    pub enum ContextMenuEvent {
        Show,
        Hide,
    }

    #[allow(unused)]
    pub fn handle_menu_events(
        menu: NonSend<ContextMenu>,
        mut events: EventReader<ContextMenuEvent>,
    ) {
        for event in events.read() {
            match event {
                ContextMenuEvent::Show => tracing::debug!("Show Context Menu"),
                ContextMenuEvent::Show => tracing::debug!("Hide Context Menu"),
                _ => tracing::debug!("Unknown '{:?}'", event),
            }
        }
    }
}
