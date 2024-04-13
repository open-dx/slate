use std::process::ExitCode;

use core::ops::Deref;
use core::ops::DerefMut;
use core::time::Duration;

use anyhow::Result;

use bevy::prelude::*;
use bevy::log::LogPlugin;
use bevy::render::RenderPlugin;
use bevy::render::settings::RenderCreation;
use bevy::render::settings::WgpuSettings;
use bevy::render::settings::Backends;
use bevy::window::PrimaryWindow;
use bevy::window::WindowLevel;
// use bevy::window::PresentMode;
use bevy::window::WindowResolution;
// use bevy::window::ExitCondition;
// use bevy::window::PrimaryWindow;
use bevy::winit::WinitSettings;
use bevy::time::Timer;
use bevy::time::TimerMode;

//--
// use slate::surface::builder::SurfaceBuilder;
use slate::element::UUID;
use slate::element::tests::ElementTestImpl as Label;
use slate::element::tests::ElementTestImpl as TextInput;
use slate::element::tests::ElementTestImpl as Container;
use slate::element::tests::ElementTestImpl as Header;
use slate::element::tests::ElementTestImpl as Footer;
use slate::element::tests::ElementTestImpl as Sidebar;
use slate::element::tests::ElementTestImpl as Banner;
use slate::element::tests::ElementTestImpl as Content;
// use slate::style::Unit;
// use slate::event::EventKind;
// use slate::event::EventHandlerFn;
use slate::event::ClickEvent;

use bevy_slate::BevySlatePlugin;
use bevy_slate::provider::SurfaceProvider;

use slate::style::StyleSheet;
#[cfg(feature = "profiling")]
use tracy_client::Client as TracyClient;

//---
/// TODO
#[cfg(feature = "debug")]
#[cfg(not(feature = "verbose"))]
const LOG_FILTER: &str = "info,bevy_slate_basic=trace,bevy_slate=debug,slate=debug,wgpu_core=error,wgpu_hal=error";

/// TODO
#[cfg(not(feature = "debug"))]
#[cfg(not(feature = "verbose"))]
const LOG_FILTER: &str = "info,bevy_slate_basic=trace,bevy_slate=info,slate=info,wgpu_core=error,wgpu_hal=error";

/// TODO
#[cfg(feature = "verbose")]
const LOG_FILTER: &str = "info,bevy_slate_basic=trace,bevy_slate=trace,slate=trace,wgpu_core=error,wgpu_hal=error";

//---
/// TODO
fn main() -> Result<ExitCode> {
    #[cfg(feature = "profiling")]
    let _tracy_client = TracyClient::start();
    
    slate::log::init(LOG_FILTER);
    
    App::new()
        // Sets the "clear color" of the window to a dark grey.
        // Note: This doesn't prevent the white popping on window creation.
        // TODO: Figure out how to prevent the white popping on window creation.
        // .insert_resource(ClearColor(Color::rgb(0.12, 0.12, 0.12)))
        .insert_resource(ClearColor(Color::hex("#202020").expect("Bad hex color.")))
        // Reduce update operations.
        // Improves power consumption in a "desktop app" context.
        // TODO: This should be configurable by cli args.
        .insert_resource(WinitSettings::desktop_app())
        // Add the timer event ..
        .add_event::<DrawTimerFinishedEvent>()
        // Note: We should probably offer a default set w/ configs for  the
        //  most common render targets (app, game, etc.).
        // TODO: Replace this with a custom plugin group built by Slate.
        .add_plugins({
            DefaultPlugins
                // TODO: Format log entries per environment.
                .set(LogPlugin {
                    filter: String::from(LOG_FILTER),
                    ..Default::default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Basic"),
                        // present_mode: PresentMode::AutoVsync,
                        resolution: WindowResolution::new(640.0, 480.0),
                        window_level: WindowLevel::AlwaysOnTop,
                        visible: false,
                        ..default()
                    }),
                    // exit_condition: ExitCondition::DontExit,
                    ..default()
                })
                // TODO: Load the correct render backend per environment.
                .set(RenderPlugin {
                    // WGPU on windows sometimes has performance issues.
                    // Setting the backend to DX12 seems to fix this.
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::DX12),
                        ..default()
                    }),
                    ..default()
                })
        })
        // Load the Slate plugin, which provides the `SurfaceProvider` resource
        // and systems to sync the Slate Surface with Bevy's scenes.
        .add_plugins(BevySlatePlugin)
        
        .add_systems(PostStartup, show_primary_window)
        
        .add_systems(PreStartup, setup_draw_timer)
        .add_systems(PreUpdate, sync_draw_timer)
        
        .add_systems(PreStartup, spawn_ui_surface)
        .add_systems(Update, draw_basic_surface)
        
        .run();
    
    //--
    Ok(ExitCode::SUCCESS)
}

//---
/// TODO
pub(crate) fn spawn_ui_surface(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SurfaceProvider::new());
}

/// TODO
pub(crate) fn show_primary_window(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = window_query.single_mut();
    window.position = WindowPosition::Centered(MonitorSelection::Current);
    window.visible = true;
}

/// TODO
fn draw_basic_surface(
    mut surface_qry: Query<&mut SurfaceProvider>,
    mut timer_finished_evtr: EventReader<DrawTimerFinishedEvent>,
    mut commands: Commands,
) {
    let mut surface = surface_qry.single_mut();
    
    for timer_event in timer_finished_evtr.read() {
        let root01_uuid = UUID::new_v4();
        let root01_name = format!("First Root");
        
        let update_time = timer_event.0.as_secs() as usize;
        #[cfg(feature = "verbose")]
        tracing::trace!("Forced synthetic update at {:} seconds ..", update_time);
        
        // let thickums = Some(100.);
        let thickums = None;
        
        let on_click_fn = |evt: &ClickEvent| {
            tracing::debug!("Clicked:\n{0:#?}", evt);
        };
        
        let some_shared_styles = |styles: &mut StyleSheet| {
            tracing::debug!("Styles:\n{0:#?}", styles);
        };
        
        // use slate::element::
        
        surface.draw(&mut commands, chizel::uix! {
            // Styles can be defined in a style block.
            // 
            // Class blocks are lowered into (something like):
            // ```rust
            // let [class_name] = move |stylesheet: &StyleSheet, ..| {
            //     stylesheet.push(BackgroundColor::new("#ff000");
            //     stylesheet.push(MinWidth::new(0.));
            //     // etc ..
            // };
            // ```
            // 
            // In the above, stylesheet is an isolated style container and the
            // closure is used to build elements in the style sheet.
            // 
            // The resulting closure is then passed to elements with `#[class(..)]`.
            // 
            // Note: Style-block closures can also be built manually or passed
            // from other functions.
            .header, .footer {
                FlexDirection::Row,
                FlexGrow::new(0.),
                FlexShrink::new(0.),
                Margin::new(thickums.unwrap_or(0.)),
                Padding::all(0., 0., 0., 0.),
                BackgroundColor::hex("#2A2A2A"),
            }
            
            /// Elements are expressed with a JSX-like DSL.
            /// - Each element is a type which implements `Element`.
            /// - Props are expanded to a call, `.with_[key](value)`, which
            ///   takes an argument `Into<T>` for convenience.
            #[style(BoxSize::xy(Full, Px(80.)))]
            #[class(header)]
            <Header
                uuid=root01_uuid
                name=root01_name />
            
            /// Documentation comments (like this one) are attached to the 
            /// element defined here and can be used by debuggers, editors, etc
            /// to display them for dev-use.
            #[style(FlexDirection::Column)]
            #[style(FlexGrow::new(1.))]
            #[style(Gap::new(10.))]
            #[style(Padding::all(10., 10., 10., 10.))]
            #[class(some_shared_styles)]
            <Container number=update_time>
                #[when(root01_name == "First Root")]
                #[on(Click, on_click_fn)]
                #[style(FlexGrow::new(1.))]
                #[style(FlexShrink::new(1.))]
                #[style(Padding::all(10., 10., 10., 10.))]
                #[style(BoxSize::xy(Full, Auto))]
                #[style(MinHeight::new(30.))]
                #[style(MaxHeight::new(60.))]
                #[style(BackgroundColor::hex("#886666"))]
                <Banner name="Second Root" number=0usize>
                    <TextInput name="First Nested Child of Second Root" number=10 />
                </Banner>
                
                #[uuid("b2b2b2b2-b2b2-b2b2-b2b2-b2b2b2b2b2b2")]
                #[style(FlexDirection::Row)]
                #[style(FlexGrow::new(1.))]
                #[style(FlexShrink::new(1.))]
                #[style(Gap::new(10.))]
                #[style(Width::new(Full))]
                <Container name="Third Root">
                    #[style(FlexGrow::new(1.))]
                    #[style(BoxSize::xy(Percent(30.), Full))]
                    #[style(MinWidth::new(200.))]
                    #[style(MaxWidth::new(400.))]
                    #[style(MaxHeight::new(100.))]
                    #[style(BackgroundColor::hex("#668866"))]
                    #[style(BorderWeight::all(0., 0., 20.0, 50.))]
                    #[style(BorderColor::hex("#88AA88"))]
                    <Sidebar name="First Child of Third Root" />
                    
                    #[style(FlexGrow::new(1.))]
                    #[style(BoxSize::xy(Full, Full))]
                    #[style(MaxHeight::new(300.))]
                    #[style(BackgroundColor::hex("#668866"))]
                    <Content name="Second Child of Third Root" number=31 />
                </Container>
            </Container>
            
            #[each(n in 0..4)]
            #[style(BoxSize::xy(Full, Px(50.)))]
            #[class(footer)]
            <Footer name="Fourth Root">
                <Sidebar name="First Child of Fourth Root" />
                
                #[when(n == 1)]
                #[style(FlexGrow::new(1.))]
                <TextInput name="Second Child of Fourth Root" />
                
                #[style(Padding::new(10.))]
                <Container name="Third Child of Fourth Root">
                    #[style(Padding::new(5.))]
                    #[style(Width::new(80.))]
                    #[style(BackgroundColor::hex("#666688"))]
                    <Label name="First Nested Child of Third Child of Fourth Root" number=6 />
                </Container>
            </Footer>
        });
    }
}

//--
/// The duration between draw operations.
const TIMER_DURATION: Duration = Duration::from_secs(2);

#[derive(Component)]
struct DrawTimer(Timer);

impl DrawTimer {
    /// TODO
    pub fn new(duration: Duration) -> Self {
        let mut timer = Timer::new(duration, TimerMode::Repeating);
        timer.set_elapsed(duration);
        // timer.tick(Duration::from_nanos(0));
        DrawTimer(timer)
    }
}

impl Deref for DrawTimer {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DrawTimer {
    /// TODO
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Represents a timer event. The inner `Duration` represents the time elapsed
/// since the timer was last reset.
#[derive(Event, Default, Debug)]
struct DrawTimerFinishedEvent(Duration);

/// TODO
pub(crate) fn setup_draw_timer(
    mut commands: Commands,
) {
    commands.spawn(DrawTimer::new(TIMER_DURATION));
}

/// TODO
pub(crate) fn sync_draw_timer(
    mut timer_query: Query<&mut DrawTimer>,
    mut timer_finished_evtw: EventWriter<DrawTimerFinishedEvent>,
    time: Res<Time>,
) {
    for mut timer in timer_query.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            // timer.set_elapsed(TIMER_DURATION);
            timer_finished_evtw.send(DrawTimerFinishedEvent(time.elapsed()));
        }
    }
}
