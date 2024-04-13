use std::alloc::Global;
use std::collections::VecDeque;
use std::process::ExitCode;
use std::sync::Arc;
use std::sync::Mutex;
use std::io::Stdout;
use std::io::Write;
use std::io::BufWriter;
use std::time::Duration;

// use smallvec::SmallVec;

// use tracing::log::LevelFilter;
// use tracing::log;

use crossterm::event::MouseEvent;
// use crossterm::ExecutableCommand;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
#[cfg(not(feature = "raw"))]
use crossterm::{
    terminal::EnterAlternateScreen,
    terminal::LeaveAlternateScreen,
    cursor::Show as ShowCursor,
    cursor::Hide as HideCursor,
};
// use crossterm::style::Attribute;
// use crossterm::style::SetAttribute;
// use crossterm::style::Color;
// use crossterm::style::SetForegroundColor;
// use crossterm::style::SetBackgroundColor;
use crossterm::event::Event as CrosstermEvent;
use crossterm::event::EnableMouseCapture;
use crossterm::event::DisableMouseCapture;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use crossterm::event::KeyModifiers;

use slate::surface::Surface;
// use slate::surface::SurfaceError;
use slate::scaffold::Scaffold;
use slate::scaffold::ScaffoldError;
use slate::element::DrawReport;
use slate::element::UUID;
use slate::x::HashMap;

use crate::element::ElementBlock;
use crate::element::Padding;
use crate::element::Rect;

//--
/// TODO
// #[derive(Debug)] // TODO
pub struct TerminalSurface<'surface, W: Write = Stdout> {
    /// TODO
    refresh_rate: Duration,
    
    //--
    /// TODO
    surface: Surface<'surface>,
    
    // TODO: Abstract this out to TerminalHistory (or something) ..
    #[allow(dead_code)]
    history: VecDeque<Arc<Mutex<TerminalPage>>>,
    
    //--
    /// TODO
    writer: W,
}

impl<'surface> TerminalSurface<'surface, Stdout> {
    /// TODO
    const DEFAULT_REFRESH_RATE: Duration = Duration::from_millis(100000);
    
    /// TODO
    pub fn new() -> Self {
        TerminalSurface::new_for(std::io::stdout())
    }
    
    /// TODO
    pub fn new_on(surface: Surface<'surface>) -> Self {
        TerminalSurface::new_for_on(std::io::stdout(), surface)
    }
}

impl<'surface, W: Write> TerminalSurface<'surface, W> {
    /// TODO
    pub fn new_for(writer: W) -> Self {
        TerminalSurface::new_for_on(writer, Surface::new())
    }
    
    /// TODO
    pub fn new_for_on(writer: W, surface: Surface<'surface>) -> Self {
        let refresh_rate = TerminalSurface::DEFAULT_REFRESH_RATE;
        let history = VecDeque::new(); // TODO: Use a ring buffer?
        // let arena = Global; // Because the surface provided here is Global ..
        TerminalSurface {
            surface,
            refresh_rate,
            history,
            writer,
            // arena,
        }
    }
}

impl<'surface> Default for TerminalSurface<'surface, Stdout> {
    /// TODO
    fn default() -> Self {
        TerminalSurface::new()
    }
}

impl<'surface, W: Write> TerminalSurface<'surface, W> {
    /// TODO
    pub fn setup(&mut self) -> Result<ExitCode, TerminalError> {
        #[cfg(not(feature = "raw"))]
        {
            if crossterm::terminal::is_raw_mode_enabled()? == false {
                crossterm::terminal::enable_raw_mode()?;
            }
            
            crossterm::execute!(self.writer, EnterAlternateScreen)?;
            crossterm::execute!(self.writer, HideCursor)?;
        }
        
        crossterm::execute!(self.writer, EnableMouseCapture)?;
        
        Ok(ExitCode::SUCCESS)
    }
}

impl<'surface, W: Write> TerminalSurface<'surface, W> {
    /// TODO: Move this to Drop impl.
    pub fn drop(&mut self) -> Result<ExitCode, TerminalError> {
        #[cfg(not(feature = "raw"))]
        {
            if crossterm::terminal::is_raw_mode_enabled()? == true {
                crossterm::terminal::disable_raw_mode()?;
            }
            
            crossterm::execute!(self.writer, LeaveAlternateScreen)?;
            crossterm::execute!(self.writer, ShowCursor)?;
        }
        
        crossterm::execute!(self.writer, DisableMouseCapture)?;
        
        Ok(ExitCode::SUCCESS)
    }
}

#[allow(dead_code, unused_variables)]
impl<'surface, W: Write> TerminalSurface<'surface, W> {
    /// TODO
    #[allow(unreachable_code)]
    pub fn start(&mut self) -> Result<ExitCode, TerminalError> {
        #[cfg(feature = "raw")]
        {
            tracing::trace!("Starting TerminalSurface in raw-mode ..");
        }
        
        // Try to setup the terminal environment.
        if let Err(error) = self.setup() {
            // TODO: Clean up more smart.
            self.drop()?;
            return Err(error);
        }
        
        // let mut backend = &self.backend;
        let mut should_stop = false;
        while !should_stop {
            if crossterm::event::poll(self.refresh_rate)? {
                let mut terminal_cmd: Option<TerminalCmd> = None;
                let mut draw_buf = BufWriter::new(std::io::stdout());
                
                // Handle terminal events.
                match crossterm::event::read()? {
                    // TODO
                    CrosstermEvent::FocusGained => {
                        // tracing::trace!("Focus Gained! <3");
                        terminal_cmd = Some(TerminalCmd::Print(true));
                    }
                    
                    // TODO
                    CrosstermEvent::FocusLost => {
                        #[cfg(feature = "verbose")]
                        tracing::trace!("Window lost focus ..");
                    }
                    
                    // TODO
                    CrosstermEvent::Resize(_, _) => {
                        // tracing::trace!("Resized! <3");
                        terminal_cmd = Some(TerminalCmd::Print(true));
                    }
                    
                    // TODO
                    CrosstermEvent::Key(key_evt) => {
                        let ctrl = key_evt.modifiers.contains(KeyModifiers::CONTROL);
                        
                        // TODO: Move this to a KeyCmd enum.
                        match key_evt.code {
                            // TODO
                            KeyCode::Char('q') if KeyEventKind::Press == key_evt.kind => {
                                terminal_cmd = Some(TerminalCmd::Quit);
                            }
                            
                            // TODO
                            KeyCode::Char('c') if ctrl && KeyEventKind::Press == key_evt.kind => {
                                terminal_cmd = Some(TerminalCmd::Quit);
                            }
                            
                            // TODO
                            KeyCode::Char('r') if ctrl && KeyEventKind::Press == key_evt.kind => {
                                terminal_cmd = Some(TerminalCmd::Print(true));
                            }
                            
                            // TODO
                            _ => {
                                //..
                            }
                        }
                    }
                    
                    // TODO
                    CrosstermEvent::Mouse(mouse_evt) => {
                        let MouseEvent { kind, row, column, modifiers } = mouse_evt;
                        tracing::trace!("Mouse {:?} at row {:}, column {:}.", kind, row, column);
                        terminal_cmd = Some(TerminalCmd::Print(false));
                    }
                    
                    // TODO
                    CrosstermEvent::Paste(paste_evt) => {
                        // tracing::trace!("Pasted: {:?}", paste_evt);
                        terminal_cmd = Some(TerminalCmd::Print(false));
                    }
                }
                
                // Handle terminal commands (if any).
                if let Some(terminal_cmd) = terminal_cmd {
                    match terminal_cmd {
                        // TODO
                        TerminalCmd::Print(should_clear) => {
                            #[cfg(feature = "verbose")]
                            tracing::trace!("Attempting to Draw Terminal");
                            
                            if let Err(error) = self.print(&mut draw_buf, should_clear) {
                                eprintln!("Failed to print terminal: {}", error);
                            }
                        }
                        
                        // TODO
                        TerminalCmd::Quit => {
                            should_stop = true;
                        }
                    }
                }
                
                draw_buf.flush()?;
            }
        }
        
        self.drop()?;
        
        Ok(ExitCode::SUCCESS)
    }
}

impl<'surface, W: Write> TerminalSurface<'surface, W> {
    /// TODO
    pub fn draw<F>(&mut self, draw_fn: F) -> Result<DrawReport, ScaffoldError>
    where
        F: FnOnce(&mut Scaffold) -> Result<(), ScaffoldError>
    {
        self.surface.draw(draw_fn)
    }
}

impl<'surface, W: Write> TerminalSurface<'surface, W> {
    /// TODO: Use the parameterized writer for the buffer.
    fn print(&mut self, draw_buf: &mut BufWriter<Stdout>, should_clear: bool) -> Result<(), TerminalError> {
        let terminal_size = crossterm::terminal::size()?;
        let mut print_stack = HashMap::<UUID, ElementBlock, Global>::new();
        
        // Calculate the draw size for each element block.
        let terminal_width = terminal_size.0;
        let terminal_height = terminal_size.1;
        
        tracing::trace!("Terminal Size: {}x{}", terminal_width, terminal_height);
        
        for (_, root) in self.surface.get_roots().enumerate() {
            print_stack.insert(root.uuid(), {
                ElementBlock::new()
                    .with_uuid(root.uuid())
                    // .with_bounds(bounds)
                    .with_padding(Padding::new(0, 1, 1, 0))
                    // .with_alt(curr_node.alt())
            });
        }
        
        // Walk each element block and calculate its bounds in the dumbest way possible.
        let mut curr_bounds = Rect(0, 0, terminal_width - 1, terminal_height - 1);
        for (_, element_block) in print_stack.iter_mut() {
            element_block.set_bounds(curr_bounds);
            curr_bounds = element_block.inner_bounds();
        }

        if should_clear {
            crossterm::queue!(draw_buf, Clear(ClearType::All))?;
        }
        
        #[cfg(not(feature = "raw"))]
        for (_, element_block) in print_stack.iter() {
            element_block.draw_to(draw_buf)?;
        }

        Ok(())
    }
}

//---
/// TODO
#[derive(Debug)]
pub struct TerminalPage {
    // TODO
    // #[allow(dead_code)]
    // frame: Option<CompletedFrame<'static>>,
    
    //..
}

// impl TerminalPage {
//     // TODO
//     pub fn new(frame: CompletedFrame<'static>) -> Self {
//         let frame = Some(frame);
//         TerminalPage {
//             frame,
//         }
//     }
// }

/// TODO
pub struct DrawStack<A: std::alloc::Allocator>(Option<usize>, Vec<usize, A>);

//---
/// TODO
pub enum TerminalCmd {
    /// TODO
    Print(bool),
    
    /// TODO
    Quit,
}

//---
/// TODO
#[derive(Debug)]
pub enum TerminalError {
    /// TODO
    IoError(std::io::Error),
    
    /// TODO
    CrosstermError(std::io::Error),
}

impl From<std::io::Error> for TerminalError {
    /// TODO: Do this implicitly in oops using proc-macro attributes.
    fn from(error: std::io::Error) -> Self {
        TerminalError::IoError(error)
    }
}

// TODO: Remove this when we bring back oops.
#[automatically_derived]
impl std::error::Error for TerminalError {}

// TODO: Remove this when we bring back oops.
#[automatically_derived]
impl std::fmt::Display for TerminalError {
    /// TODO
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TerminalError::IoError(error) => write!(f, "TerminalError::IoError: {}", error),
            TerminalError::CrosstermError(error) => write!(f, "TerminalError::CrosstermError: {}", error),
        }
    }
}
