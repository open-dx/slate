use std::collections::VecDeque;
use std::io::Stdout;
use std::process::ExitCode;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

// use tracing::log::LevelFilter;
use tracing::log;

use smallvec::SmallVec;

use crossterm::event::KeyModifiers;
use crossterm::event::Event as BackendEvent;
use crossterm::event::KeyCode as TerminalKeyCode;

use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;

use crossterm::event::EnableMouseCapture;
use crossterm::event::DisableMouseCapture;

use crossterm::execute;

//--
use ratatui::Terminal;

use ratatui::Frame;
use ratatui::CompletedFrame;

use ratatui::backend::CrosstermBackend;

use ratatui::layout::Layout;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Rect;
// use ratatui::layout::Alignment::*;

// use ratatui::style::Style as TerminalStyle;
use ratatui::style::Color::*;

use ratatui::widgets::Widget;
use ratatui::widgets::Block;
use ratatui::widgets::Padding;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Borders;
use ratatui::widgets::Wrap;
use ratatui::widgets::Tabs;
use ratatui::widgets::BorderType::*;
// use ratatui::widgets::Widget;

use ratatui::symbols::DOT;

use ratatui::text::Line;
// use ratatui::text::Text;

// use tui_textarea::TextArea;
// use tui_textarea::Input as TextAreaInput;
// use tui_textarea::Key as TextAreaKey;

use bevy::app::prelude::*;
use bevy::app::AppExit;

use bevy::ecs::prelude::*;

use bevy::input::prelude::*;
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;

use bevy::window::prelude::*;

//---
/// TODO
#[derive(Default, Debug)]
pub struct TerminalPlugin {
    //..
}

impl TerminalPlugin {
    // TODO
    pub fn new() -> Self {
        TerminalPlugin {
            //..
        }
    }
}

impl Plugin for TerminalPlugin {
    /// TODO
    fn build(&self, app: &mut App) {
        app.add_event::<TerminalEvent>();
        app.add_event::<TerminalError>();
        
        app.insert_resource(TerminalProvider::new().expect("Terminal Provider"));
        
        app.add_systems(Startup, TerminalProvider::startup);
        app.add_systems(PreUpdate, TerminalProvider::forward_events);
        app.add_systems(PreUpdate, TerminalProvider::dispatch_events);
        app.add_systems(Last, TerminalProvider::exit_keys);
        app.add_systems(Last, TerminalProvider::exit);
    }
}

//--
/// TODO
pub struct TerminalPage {
    // TODO
    _content: String,
    
    //..
}

// impl TerminalPage {
//     // TODO
//     pub fn new() -> Self {
//         TerminalPage {
//             _content: String::from("dang"),
//         }
//     }
// }

impl From<String> for TerminalPage {
    // TODO
    fn from(_content: String) -> Self {
        TerminalPage { _content }
    }
}

//--
/// TODO
#[derive(Resource)]
pub struct TerminalProvider {
    /// TODO
    terminal: Terminal<CrosstermBackend<Stdout>>,
    
    // TODO: Abstract this out to TerminalHistory (or something) ..
    _history: VecDeque<Arc<Mutex<TerminalPage>>>,
    
    /// TODO
    pub surface_ent: Option<Entity>,
}

impl TerminalProvider {
    // TODO
    pub fn new() -> Result<Self, TerminalError> {
        Ok(TerminalProvider {
            terminal: Terminal::new(CrosstermBackend::new(std::io::stdout()))?,
            _history: VecDeque::new(),
            surface_ent: None,
        })
    }
}

impl TerminalProvider {
    /// TODO
    pub fn setup(&mut self) -> Result<ExitCode, TerminalError> {
        crossterm::terminal::enable_raw_mode()?;
        
        match execute!(self.terminal.backend_mut(), EnterAlternateScreen, EnableMouseCapture) {
            // Ate hamburders good; Gtg! <3
            Ok(_) => {
                Ok(ExitCode::SUCCESS)
            }
            
            // TODO: Don't drop the exec_error.
            Err(exec_error) => match self.drop() {
                // Successful surrender!
                Ok(_) => Err(TerminalError::from(exec_error)),
                
                // I said kindly..
                Err(error) => Err(TerminalError::from(error)),
            }
        }
    }
}

impl TerminalProvider {
    /// TODO: Move this to Drop impl.
    pub fn drop(&mut self) -> Result<ExitCode, TerminalError> {
        if crossterm::terminal::is_raw_mode_enabled()? {
            if let Err(_) = crossterm::terminal::disable_raw_mode() {
                // TODO
            }
        }
        
        match execute!(self.terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture) {
            Ok(_) => {
                // Successfully death. :D
                self.terminal.show_cursor()?;
                Ok(ExitCode::SUCCESS)
            }
            Err(error) => {
                // TODO
                Err(TerminalError::from(error))
            }
        }
    }
}

// impl TerminalProvider {
//     /// TODO
//     pub fn push_history(&mut self, _: String) {
//         self._history.push_back(Arc::new(Mutex::new(TerminalPage::new())))
//     }
// }

impl TerminalProvider {
    /// TODO
    pub fn draw<F>(&mut self, framer_fn: F) -> Result<CompletedFrame, TerminalError>
    where
        F: FnOnce(&mut Frame),
    {
        Ok(self.terminal.draw(framer_fn)?)
    }
    
    /// TODO
    pub fn print<W: Widget + Clone>(&mut self, widget: W) -> bool {
        match self.draw(|frame| {
            frame.render_widget(widget, frame.area());
        }) {
            Ok(_) => true,
            Err(error) => {
                let message = Paragraph::new(format!("Failed to print to terminal: {:}", error));
                
                match self.draw(move |frame| {
                    frame.render_widget(message, frame.area());
                }) {
                    Ok(_) => true,
                    Err(error) => panic!("Failed to print to terminal: {:}", error),
                }
            }
        }
    }
}

impl TerminalProvider {
    /// TODO
    fn startup(
        mut terminal: ResMut<TerminalProvider>,
        mut terminal_evt: EventWriter<TerminalEvent>,
    ) {
        // Attempt to Crossterm's "Raw Mode" so we can draw ui in systems with ratatui.
        // TODO: Call `disable_raw_` the terminal when we shut down.
        match terminal.setup() {
            Ok(_) => {
                // TOOD
                log::trace!("Terminal is ready!");
                terminal_evt.send(TerminalEvent::Ready);
            }
            Err(error) => {
                // TOOD
                log::error!("Couldn't start terminal: {:}", error);
                log::debug!("Falling back to default stdout.");
                terminal_evt.send(TerminalEvent::StartupFailed(error));
            }
        }
    }
     
    /// TODO: Move this to a seperate thread to avoid blocking the update loop.
    fn forward_events(
        mut terminal_evt: EventWriter<TerminalEvent>,
        mut error_evt: EventWriter<TerminalError>,
    ) {
        if crossterm::event::poll(Duration::from_millis(1)).unwrap_or(false) {
            match crossterm::event::read() {
                Ok(event) => {
                    terminal_evt.send(TerminalEvent::from(event));
                },
                Err(error) => {
                    error_evt.send(TerminalError::IoError(error));
                },
            }
        }
    }
    
    /// TODO
    fn dispatch_events(
        mut terminal: ResMut<TerminalProvider>,
        mut terminal_evt: EventReader<TerminalEvent>,
        mut exit_evt: EventWriter<AppExit>,
    ) {
        for event in terminal_evt.read() {
            match event {
                // TODO
                TerminalEvent::Ready => {
                    let message = Paragraph::new("Terminal is ready!");
                    
                    if terminal.print(message) {
                        //..
                    }
                }
                
                // TODO
                TerminalEvent::Backend(BackendEvent::Resize(width, height)) => {
                    let message = Paragraph::new(format!("Resized to {:}:{:}", width, height));
                    
                    if terminal.print(message) {
                        //..
                    }
                }
                
                TerminalEvent::Backend(BackendEvent::Key(key)) => {
                    if key.code == TerminalKeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                        exit_evt.send(AppExit::Success);
                    }
                }
                
                // TODO
                _ => {
                    //..
                }
            }
        }
    }
    
    /// TODO
    pub fn render(
        windows: Query<&Window>,
        mut terminal: ResMut<TerminalProvider>,
    ) {
        // Draw the thing to the screen.
        if let Err(_) = terminal.draw(|frame| {
            let layout = Layout::new(Direction::Horizontal, [
                Constraint::Min(50),
                Constraint::Length(40),
            ]);
            
            let columns = layout.split(frame.area());
            
            // Render the left-hand column for the log-stream.
            if let Some(col) = columns.get(0) {
                let filter = LogFilter {
                    prompt: Line::from(format!("$")),
                };
                
                let block = Block::new().borders(Borders::ALL).padding(Padding::horizontal(1)).border_type(Rounded);
                let input = Paragraph::new(filter.prompt).block(block);
                frame.render_widget(input, Rect::new(col.x, col.y, col.width, 3));
                
                // let stream = Text::from("dasdf\nasdfasdf");
                let tab_labels = ["Error\u{00B9}", "Warning\u{00B2}", "Info\u{00B3}", "Debug\u{2074}", "Trace\u{2075}"];
                
                let streams = Tabs::new(tab_labels.iter().cloned().map(Line::from).collect::<Vec<_>>()).select(2)
                    .highlight_style(ratatui::style::Style::default().fg(Cyan))
                    .divider(DOT);
                
                frame.render_widget(streams, Rect::new(col.x, col.y + 3, col.width, col.height - 3));
                
                // let stream = Text::from("dasdf\nasdfasdf");
                let body = Line::from("Some stream of data ..");
                let streams = Paragraph::new(body).block(Block::new().padding(Padding::horizontal(1)));
                frame.render_widget(streams, Rect::new(col.x, col.y + 5, col.width, col.height - 5));
            }
            
            // Render the left-hand column for the log-stream.
            if let Some(col) = columns.get(1) {
                match windows.get_single() {
                    Ok(window) => {
                        let window_dbg = DebugPanel::new(format!("Window ({:})", window.title), false, vec![
                            Line::from(format!("Mode:    {:?}", window.mode)),
                            Line::from(format!("Focus:   {:?}", window.focused)),
                            Line::from(format!("Scale:   {:?}", window.resolution.base_scale_factor())),
                            Line::from(format!("Alpha:   {:?}", window.composite_alpha_mode)),
                            Line::from(format!("Pos:     {:}", match window.position {
                                WindowPosition::Automatic => String::from("Automatic"),
                                WindowPosition::Centered(monitor) => format!("Centered({:?})", monitor),
                                WindowPosition::At(pos) => format!("{:}", pos),
                            })),
                            Line::from(format!("Size:    {:?}", [
                                window.resolution.width(),
                                window.resolution.height()
                            ])),
                            Line::from(format!("Cursor:  {:}", window.cursor_position().unwrap_or_default())),
                        ]);
                    
                        let mut top = 0u16;
                        for panel in [window_dbg] {
                            let height = 2 + panel.lines.len() as u16;
                            let wrap = Wrap { trim: !panel.wrap };
                            
                            let block = Block::new()
                                .title(panel.title)
                                .padding(Padding::horizontal(1))
                                .borders(Borders::ALL)
                                .border_type(Rounded);
                            
                            let content = Paragraph::new(panel.lines).wrap(wrap).block(block);
                            frame.render_widget(content, Rect::new(col.x, top, col.width, height));
                            
                            top += height;
                        }
                    }
                    Err(error) => {
                        let error_dbg = DebugPanel::new(String::from("Window (Err)"), true, vec![
                            Line::from(error.to_string()),
                        ]);
                            
                        let block = Block::new()
                            .title(error_dbg.title.clone())
                            .padding(Padding::horizontal(1))
                            .borders(Borders::ALL)
                            .border_type(Rounded);
                        
                        let content = Paragraph::new(error_dbg.lines.clone()).wrap(error_dbg.wrap()).block(block);
                        frame.render_widget(content, Rect::new(col.x, col.y, col.width, 2u16 + error_dbg.lines.len() as u16));
                    }
                }
            }
        }) {
            //..
        }
    }
    
    /// TODO
    fn exit_keys(
        keys: Res<ButtonInput<KeyCode>>,
        mut key_evt: EventReader<KeyboardInput>,
        mut exit_evt: EventWriter<AppExit>,
    ) {
        use KeyCode::*;
        use ButtonState::*;
        
        let ctrl = keys.any_pressed([ControlLeft, ControlRight]);
        
        for event in key_evt.read() {
            if ctrl && event.state == Pressed {
                // Forward the exit message.
                // Goodnight <3
                exit_evt.send(AppExit::Success);
            }
        }
    }
    
    /// Route app-level events.
    fn exit(
        mut terminal: ResMut<TerminalProvider>,
        mut exit_evt: EventReader<AppExit>,
    ) {
        for _ in exit_evt.read() {
            terminal.print(Paragraph::new("Exiting. Goodbye! <3"));
            if let Err(error) = terminal.drop() {
                eprintln!("Failed to drop terminal: {:}", error);
            }
        }
    }
}

//--
/// TODO
#[derive(Event, Debug)]
pub enum TerminalEvent {
    /// TODO
    StartupFailed(TerminalError),
    
    /// TODO
    Ready,
    
    /// TODO
    Backend(BackendEvent),
}

impl From<BackendEvent> for TerminalEvent {
    /// TODO: Do this implicitly in oops using proc-macro attributes.
    fn from(event: BackendEvent) -> Self {
        TerminalEvent::Backend(event)
    }
}

//--
/// TODO
#[derive(oops::Error, Event)]
pub enum TerminalError {
    /// TODO
    #[msg("Std I/O Error: {:}")]
    IoError(std::io::Error),
}

impl From<std::io::Error> for TerminalError {
    /// TODO: Do this implicitly in oops using proc-macro attributes.
    fn from(error: std::io::Error) -> Self {
        TerminalError::IoError(error)
    }
}

//---
/// TODO
#[derive(Resource)]
struct LogFilter<'panel> {
    /// TODO
    prompt: Line<'panel>,
}

/// TODO
#[derive(Resource)]
struct LogStream<'panel> {
    /// TODO
    _prompt: Line<'panel>,
}

/// TODO
#[derive(Resource)]
struct DebugPanels<'panels> {
    /// TODO
    _list: SmallVec<[DebugPanel<'panels>; 10]>,
}

/// TODO
#[derive(Resource)]
struct DebugPanel<'panel> {
    /// TODO
    title: String,
    
    /// TODO
    lines: Vec<Line<'panel>>,
    
    /// TODO
    wrap: bool,
}

impl<'panel> DebugPanel<'panel> {
    /// TODO
    pub fn wrap(&self) -> Wrap {
        Wrap {
            trim: !self.wrap,
        }
    }
}

impl<'panel> DebugPanel<'panel> {
    /// TODO
    pub fn new(title: String, wrap: bool, lines: Vec<Line<'panel>>) -> Self {
        DebugPanel {
            title,
            lines,
            wrap,
        }
    }
}

impl<'panel> From<(String, Line<'panel>)> for DebugPanel<'panel> {
    /// TODO
    fn from(value: (String, Line<'panel>)) -> Self {
        DebugPanel {
            title: value.0,
            lines: vec![value.1],
            wrap: true,
        }
    }
}