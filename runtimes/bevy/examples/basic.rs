#![feature(allocator_api)]

// TODO: Remove this ..
#![allow(unused)]

use core::time::Duration;

use std::process::ExitCode;

use anyhow::Result;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use bevy_slate::config::BevySlateConfig;
use bevy_slate::BevySlatePlugin;
use bevy_slate::provider::SurfaceProvider;
use bevy_slate::window::WindowKind;
use bevy_slate::time::DrawTimer;
use bevy_slate::time::DrawTimerFinishedEvent;

use slate::component::x::layout::Div;
use slate::element::UUID;
use slate::event::ClickEvent;
use slate::component::x::layout::Container;
use slate::component::x::layout::Header;
use slate::component::x::layout::Footer;
use slate::component::x::layout::Sidebar;
use slate::component::x::content::TextBlock;
use slate::component::x::input::Label;
use slate::component::x::input::TextInput;
use slate::component::x::input::Button;

use slate::element::tests::ElementTestImpl as Banner;

const WINDOW_CLEAR_COLOR: Color = Color::hsla(220.0, 0.11, 0.11, 1.0);

//---
/// TODO
fn main() -> Result<ExitCode> {
    slate::log::init(bevy_slate::log::DEFAULT_LOG_FILTER);
    
    App::new()
        // Add the timer event ..
        .add_event::<DrawTimerFinishedEvent>()
        // Load the Slate plugin, which provides the `SurfaceProvider` resource
        // and systems to sync the Slate Surface with Bevy's scenes.
        .add_plugins(BevySlatePlugin::new({
            BevySlateConfig::default()
                .with_clear_color(WINDOW_CLEAR_COLOR)
        }))
        .add_systems(PreStartup, spawn_ui_surface)
        .add_systems(PostStartup, show_primary_window)
        .add_systems(PreUpdate, bevy_slate::time::sync_draw_timer)
        .add_systems(Update, draw_basic_surface)
        .run();
    
    //--
    Ok(ExitCode::SUCCESS)
}

/// TODO
pub(crate) fn show_primary_window(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = window_query.single_mut();
    window.position = WindowPosition::Centered(MonitorSelection::Current);
    window.visible = true;
}

//---
/// TODO
pub(crate) fn spawn_ui_surface(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SurfaceProvider::new());
    commands.spawn(DrawTimer::new(Duration::from_secs(2)));
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
        
        let update_time = timer_event.duration().as_secs().to_string();
        #[cfg(feature = "verbose")]
        tracing::trace!("Forced synthetic update at {:} seconds ..", update_time);
        
        // let thickums = Some(100.);
        let thickums = None;
        
        let on_click_fn = |evt: &ClickEvent| {
            tracing::debug!("Clicked:\n{0:#?}", evt);
        };
        
        // The `chizel::styles!` macro offers a shorthand for the following:
        // ```rust
        // let some_shared_styles = |styles: &mut StyleSheet| {
        //     styles.push(BackgroundColor::hex("#886666"));
        //     
        //     #[cfg(feature = "verbose")]
        //     {
        //         tracing::debug!("Styles:\n{0:#?}", styles);
        //     }
        // };
        // ```
        chizel::styles! {
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
                FlexGrow::new(0u8),
                FlexShrink::new(0u8),
                Margin::new(thickums.unwrap_or(0.)),
                Padding::all(0., 0., 0., 0.),
                BackgroundColor::hex("#2A2A2A"),
            }
            
            // TODO
            .body {
                FlexDirection::Column,
                FlexGrow::new(1.),
                Gap::new(10.),
                Padding::all(10., 10., 10., 10.),
            }
            
            .content {
                FlexDirection::Row,
                FlexGrow::new(1.),
                FlexShrink::new(1.),
                Gap::new(10.),
                Width::new(Full),
            }
            
            .navbar {
                FlexGrow::new(1.),
                BoxSize::xy(Percent(30.), Full),
                MinWidth::new(200.),
                MaxWidth::new(400.),
                MaxHeight::new(100.),
                BackgroundColor::hex("#668866"),
                BorderWeight::all(0., 0., 20.0, 50.),
                BorderColor::hex("#88AA88"),
            }
        }
        
        surface.draw(&mut commands, chizel::uix! {
            .container {
                FlexGrow::new(0.),
                FlexShrink::new(1.),
                FlexDirection::Column,
                BoxSize::xy(Percent(100.), Percent(100.)),
                MinHeight::new(Percent(100.)),
                MinWidth::new(Percent(100.)),
            }
            
            // Styles can also be defined in any UIx block. The resulting
            // variables are the same kind as is created by `chizel::style!`
            // blocks directly.
            .banner {
                FlexGrow::new(1.),
                FlexShrink::new(1.),
                Padding::all(10., 10., 10., 10.),
                BoxSize::xy(Full, Auto),
                MinHeight::new(30.),
                MaxHeight::new(60.),
                BackgroundColor::hex("#886666"),
                BorderRadius::new(6.),
                // :hover, :focus {
                //     BackgroundColor::hex("#886666"),
                // }
            }
            
            .messages {
                FlexGrow::new(1.),
                BoxSize::xy(Full, Full),
                MaxHeight::new(300.),
                BackgroundColor::hex("#668866"),
            }
            
            #[class(container)]
            <Div>
                /// Elements are expressed with a JSX-like DSL.
                /// - Each element is a type which implements `Element`.
                /// - Props are expanded to a call, `.with_[key](value)`, which
                ///   takes an argument `Into<T>` for convenience.
                #[style(BoxSize::xy(Full, Px(80.)))]
                #[class(header)]
                <Header alt=root01_name />
                
                /// Documentation comments (like this one) are attached to the 
                /// element defined here and can be used by debuggers, editors,
                /// etc to display them for dev-use.
                #[uuid(root01_uuid)]
                #[class(body)]
                <Container alt=update_time>
                    #[when(root01_name == "First Root")]
                    #[on(Click, on_click_fn)]
                    #[class(banner)]
                    <Banner name="Second Root" number=0usize>
                        <TextInput value="First Nested Child of Second Root" />
                    </Banner>
                    
                    #[uuid("b2b2b2b2-b2b2-b2b2-b2b2-b2b2b2b2b2b2")]
                    #[class(content)]
                    <Container alt="Third Root">
                        #[class(navbar)]
                        <Sidebar alt="First Child of Third Root" />
                        
                        #[each(n in 0..4)]
                        #[class(messages)]
                        <TextBlock text="Second Child of Third Root" />
                    </Container>
                </Container>
                
                #[style(BoxSize::xy(Full, Px(50.)))]
                #[class(footer)]
                <Footer>
                    <Sidebar />
                    
                    #[when(n == 1)]
                    #[style(FlexGrow::new(1.))]
                    <TextInput value="Second Child of Fourth Root" />
                    
                    #[style(Padding::new(10.))]
                    <Container>
                        #[style(Padding::new(5.))]
                        #[style(Width::new(80.))]
                        #[style(BackgroundColor::hex("#666688"))]
                        <Button value="Continue" />
                    </Container>
                </Footer>
            </Div>
        });
    }
}
