#![feature(coerce_unsized)]

#![allow(unused)]

extern crate slate;
extern crate tracing;

use std::process::ExitCode;

use anyhow::Result;

use once_cell::sync::OnceCell;

use bumpalo::Bump;
use bumpalo::boxed::Box;
use bumpalo::collections::Vec;
use bumpalo_herd::Herd;

use slate::surface::SurfaceError;
use slate::scaffold::ScaffoldError;

pub type DrawFn = fn(&mut Scaffold) -> Result<(), ScaffoldError>;

pub struct Surface {
    alloc: bumpalo_herd::Herd,
}

impl Surface {
    pub fn new() -> Self {
        Surface {
            alloc: Herd::new(),
        }
    }
}

impl Surface {
    pub fn draw(&mut self, draw_fn: DrawFn) -> Result<(), SurfaceError> {
        let member = self.alloc.get();
        let mut scaffold = Scaffold::new_in(member.as_bump());
        
        draw_fn(&mut scaffold)?;
        
        Ok(())
    }
}

pub struct Scaffold<'scaffold> {
    element: Option<&'scaffold dyn Element>,
    styles: Vec<'scaffold, ()>,
    children: Vec<'scaffold, Scaffold<'scaffold>>,
    arena: &'scaffold Bump,
}

impl<'scaffold> Scaffold<'scaffold> {
    pub fn new_in(arena: &'scaffold Bump) -> Self {
        Scaffold {
            element: None,
            styles: Vec::new_in(arena),
            children: Vec::new_in(arena),
            arena,
        }
    }
    
    pub fn arena(&self) -> &Bump {
        self.arena
    }
    
    pub fn add<E>(&mut self, element: E) -> Result<&mut Self, ScaffoldError>
    where
        E: Element + 'scaffold,
    {
        let child_idx = self.children.len() + 1;
        let mut child_scaffold = Scaffold::new_in(self.arena);
        
        // element.hash(&mut new_scaffold.hasher);
        
        // let element_ref: &'scaffold E = self.arena.alloc(element);
        // let element_dyn_ref: &'scaffold dyn Element = element_ref;
        child_scaffold.element = Some(self.arena.alloc(element));
        
        self.children.push(child_scaffold);
        self.children.last_mut().ok_or(ScaffoldError::IndexOutOfBounds(child_idx))
    }
    
    /// TODO
    pub fn with_children<F>(&mut self, child_builder_fn: F) -> Result<&mut Self, ScaffoldError>
    where
        F: FnOnce(&mut Scaffold) -> Result<(), ScaffoldError>
    {
        child_builder_fn(self)?;
        Ok(self)
    }
    
    pub fn build(&mut self) -> Result<&mut Self, ScaffoldError> {
        if let Some(ref element) = self.element {
            // Call the DrawFn returned by the element's render method.
            tracing::info!("Building defaults for render boi!");
            element.draw()(self)?;
        }
        
        // let final_hash = self.hasher.finish();
        // self.hash = Some(final_hash);
        
        // #[cfg(feature = "verbose")]
        // tracing::debug!("Built Scaffold with Hash({:?})", self.hash);
        
        Ok(self)
    }
}

pub trait Element: Draw {
}

pub trait Draw {
    fn draw(&self) -> DrawFn {
        |_| Ok(())
    }
}

#[derive(Default)]
pub struct Container {
    // ..
}

impl Element for Container {}

impl Draw for Container {}

#[derive(Default)]
pub struct Section {
    // ..
}

impl Element for Section {
    // ..
}

impl Draw for Section {
    fn draw(&self) -> DrawFn {
        // ..
        slate::chizel::uix! {
            //..
        }
    }
}

const DEFAULT_LOG_FILTER: &str = "error,slate-alloc=trace,slate=info";

pub fn main() -> Result<ExitCode> {
    slate::log::init(DEFAULT_LOG_FILTER);
    
    let mut surface = Surface::new();
    
    surface.draw(chizel::uix! {
        <Container>
            <Container>
                <Container>
                    <Container>
                        <Container>
                            <Container>
                                <Container>
                                    <Section />
                                </Container>
                            </Container>
                        </Container>
                    </Container>
                </Container>
            </Container>
        </Container>
    })?;
        
    Ok(ExitCode::SUCCESS)
}
