use std::io::BufWriter;
use std::io::Write;
use std::vec;

use crossterm::cursor::MoveTo as MoveCursorTo;
use crossterm::style::ResetColor;
use crossterm::style::Print as PrintContent;

use slate::element::UUID;

use crate::terminal::TerminalError;

//---
/// TODO
pub struct ElementBlock<C: Clone = String> {
    /// TODO
    uuid: Option<UUID>,
    
    /// TODO
    bounds: Bounds<u16>,
    
    /// TODO
    padding: Padding<u16>,
    
    /// TODO: Move this off the heap. Use SmallString or something.
    alt: Option<String>,
    
    /// TODO: These should probably be a ContentBlock type.
    content: Vec<C>,
}

impl<C: Clone> ElementBlock<C> {
    /// TODO
    pub fn new() -> Self {
        ElementBlock {
            uuid: None,
            alt: None,
            content: vec![],
            bounds: Bounds::new(0, 0, 0, 0),
            padding: Padding::new(0, 0, 0, 0)
        }
    }
    
    /// TODO
    pub fn with_uuid(mut self, uuid: UUID) -> Self {
        self.uuid = Some(uuid);
        self // etc..
    }
    
    /// TODO
    pub fn with_bounds(mut self, bounds: Bounds<u16>) -> Self {
        self.bounds = bounds;
        self // etc..
    }
    
    /// TODO
    pub fn set_bounds(&mut self, bounds: Bounds<u16>) {
        self.bounds = bounds;
    }
    
    /// TODO
    pub fn with_alt(mut self, alt: Option<String>) -> Self {
        self.alt = alt;
        self // etc..
    }
    
    /// TODO: Content blocks should be complex content types.
    pub fn with_content(mut self, content: &[C]) -> Self {
        self.content = content.to_vec();
        self // etc..
    }
    
    /// TODO
    pub fn with_padding(mut self, padding: Padding<u16>) -> Self {
        self.padding = padding;
        self // etc..
    }
}

impl ElementBlock {
    /// TODO
    pub fn bounds(&self) -> Bounds<u16> {
        self.bounds
    }
    
    /// TODO
    pub fn size(&self) -> Size<u16> {
        Size(
            self.bounds.2 - self.bounds.1 + 1,
            self.bounds.3 - self.bounds.0 + 1,
        )
    }
    
    /// TODO
    pub fn inner_bounds(&self) -> Bounds<u16> {
        Rect(
            self.bounds.0 + self.padding.0 + 1,
            self.bounds.1 + self.padding.1 + 1,
            self.bounds.2 - self.padding.2 - 1,
            self.bounds.3 - self.padding.3 - 1,
        )
    }
    
    /// TODO
    pub fn inner_size(&self) -> Size<u16> {
        Size(
            // Outer width minus the padding and border.
            (self.bounds.2 - self.bounds.1 + 1) - (self.padding.1 + self.padding.2) - 2,
            (self.bounds.3 - self.bounds.0 + 1) - (self.padding.0 + self.padding.3) - 2,
        )
    }
}

impl ElementBlock {
    /// TODO
    pub fn draw_to<W: Write>(&self, draw_buf: &mut BufWriter<W>) -> Result<&Self, TerminalError> {
        self.draw_border(draw_buf)?;
        self.draw_alt(draw_buf)?;
        self.draw_content(draw_buf)?;
        Ok(self) // etc..
    }
    
    /// TODO
    pub fn draw_background<W: Write>(&self, draw_buf: &mut BufWriter<W>) -> Result<&Self, TerminalError> {
        if false {
            let Rect(top, left, right, bottom) = self.inner_bounds();
            let (width, height) = (right - left, bottom - top);
            
            for row in top..=height {
                for col in left..=width {
                    crossterm::queue!(
                        draw_buf,
                        MoveCursorTo(col, row),
                        ResetColor,
                        // SetForegroundColor(Color::DarkGrey),
                        // SetBackgroundColor(Color::Black),
                        PrintContent(" ")
                    )?;
                }
            }
        }
        
        Ok(self) // etc..
    }
    
    /// TODO
    pub fn draw_border<W: Write>(&self, draw_buf: &mut BufWriter<W>) -> Result<&Self, TerminalError> {
        let Rect(top, left, right, bottom) = self.bounds();
        let Size(width, height) = self.size();
        
        // Set the border color(s).
        crossterm::queue!(draw_buf, ResetColor)?;
        
        // TODO: Set the border style.
        // crossterm::queue!(draw_buf, ..)?;
        
        // TODO: Get the border glyphs from the border style.
        for col in left..=right {
            // Draw the top border (╭, ┌, ─, ┐, ╮).
            crossterm::queue!(draw_buf, MoveCursorTo(col, top), PrintContent({
                if col == left {
                    "┌" // (╭, ┌)
                } else if col == right {
                    "┐" // (╮, ┐)
                } else {
                    "─"
                }
            }))?;
            
            // Draw the bottom border (╰, └, ─, ┘, ╯).
            crossterm::queue!(draw_buf, MoveCursorTo(col, bottom), PrintContent({
                if col == left {
                    "└" // (╰, └)
                } else if col == right {
                    "┘" // (╯, ┘)
                } else {
                    "─"
                }
            }))?;
        }
        
        for row in (top + 1)..=(bottom - 1) {
            // Draw the left border ..
            crossterm::queue!(draw_buf, MoveCursorTo(left, row), PrintContent("│"))?;
            
            // .. and the right border.
            crossterm::queue!(draw_buf, MoveCursorTo(right, row), PrintContent("│"))?;
        }
        
        // Debug the width and height of the element.
        // TODO: Move this to a better debug facility.
        {
            let debug_pos = self.bounds();
            let debug_content = &format!("┄{}x{}┄", width, height);
            let debug_len = debug_content.chars().count() as u16;
            
            crossterm::queue!(
                draw_buf,
                MoveCursorTo(debug_pos.2 - 1 - debug_len, bottom),
                PrintContent(debug_content),
            )?;
        }
        
        Ok(self) // etc..
    }
    
    /// TODO
    pub fn draw_alt<W: Write>(&self, draw_buf: &mut BufWriter<W>) -> Result<&Self, TerminalError> {
        let Rect(top, left, _, _) = self.bounds();
        let Size(width, _) = self.size();
        
        if let Some(alt) = &self.alt {
            // Truncate the alt text to fit the element's label.
            // TODO: Do the actual truncation work elsewhere.
            let alt_str = alt.as_str();
            let max_len = (width - 6) as usize;
            let alt_end = alt_str.char_indices().nth(max_len).map_or(alt_str.len(), |(i, _)| i);
            let alt_str = &alt_str[..alt_end as usize];
            
            crossterm::queue!(
                draw_buf,
                MoveCursorTo(left + 2, top),
                // ResetColor,
                // SetForegroundColor(Color::Black),
                // SetBackgroundColor(Color::White),
                // SetAttribute(Attribute::Bold),
                PrintContent(format!("┄{:}┄", &alt_str))
            )?;
        }
        
        Ok(self) // etc..
    }
    
    /// TODO
    pub fn draw_content<W: Write>(&self, draw_buf: &mut BufWriter<W>) -> Result<&Self, TerminalError> {
        let Rect(top, left, _, _) = self.inner_bounds();
        
        let mut line_num = 0u16;
        
        while let Some(line_content) = self.content.get(line_num as usize) {
            crossterm::queue!(
                draw_buf,
                MoveCursorTo(left, top + line_num),
                // SetForegroundColor(Color::Black),
                // SetBackgroundColor(Color::White),
                // SetAttribute(Attribute::Bold),
                // PrintContent(format!("{:?}", inner_bounds))
                PrintContent(line_content)
            )?;
            
            line_num += 1;
        }
        
        Ok(self) // etc..
    }
}

//---
/// Represents the sides of a rectangle.
#[derive(Copy, Clone, Default, Debug)]
pub struct Rect<W>(pub W, pub W, pub W, pub W);

impl<W> Rect<W> {
    /// TODO
    pub fn new(top: W, left: W, right: W, bottom: W) -> Self {
        Rect(top, left, right, bottom)
    }
}

impl<W: std::ops::Sub<Output = W> + Copy> Rect<W> {
    /// TODO
    pub fn width(&self) -> W {
        self.2 - self.1
    }
    
    /// TODO
    pub fn height(&self) -> W {
        self.3 - self.0
    }
}

/// TODO
pub type Bounds<W> = Rect<W>;

/// TODO
pub type Border<W> = Rect<W>;

/// TODO
pub type Margin<W> = Rect<W>;

/// TODO
pub type Padding<W> = Rect<W>;

/// TODO
pub struct Size<W>(pub W, pub W);

impl<W> Size<W> {
    /// TODO
    pub fn new(width: W, height: W) -> Self {
        Size(width, height)
    }
}
