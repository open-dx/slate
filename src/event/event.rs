use bumpalo::Bump;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub trait Event {
    fn kind(self) -> EventKind;
}

#[derive(Debug)]
pub enum EventPin {
    Click(EventHandlerFn<ClickEvent>),
    Hover(EventHandlerFn<HoverEvent>),
}

impl From<EventHandlerFn<ClickEvent>> for EventPin {
    fn from(handler: EventHandlerFn<ClickEvent>) -> Self {
        EventPin::Click(handler)
    }
}

impl From<EventHandlerFn<HoverEvent>> for EventPin {
    fn from(handler: EventHandlerFn<HoverEvent>) -> Self {
        EventPin::Hover(handler)
    }
}

pub type EventHandlerFn<E> = fn(event: &E);

#[derive(Debug)]
pub struct EventStack<'arena> {
    /// TODO
    #[cfg(feature="bump")]
    events: Vec<Box<EventPin, &'arena Bump>, &'arena Bump>,
    
    /// TODO
    #[cfg(not(feature="bump"))]
    events: Vec<Box<EventPin>>,
}

#[cfg(feature="bump")]
impl<'arena> EventStack<'arena> {
    pub fn new_in(arena: &'arena Bump) -> Self {
        EventStack {
            events: Vec::new_in(arena),
        }
    }
    
    pub fn as_ref(&self) -> &Vec<Box<EventPin, &'arena Bump>, &'arena Bump> {
        self.events.as_ref()
    }
    
    pub fn as_mut(&mut self) -> &mut Vec<Box<EventPin, &'arena Bump>, &'arena Bump> {
        self.events.as_mut()
    }
    
    pub fn push(&mut self, event: Box<EventPin, &'arena Bump>) {
        self.events.push(event);
    }
}

#[cfg(not(feature="bump"))]
impl EventStack {
    pub fn new() -> Self {
        EventStack {
            events: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum EventKind {
    /// TODO
    Click,
    Hover,
}

// TODO: Remove this!
// pub struct EventHandlerFn<F>
// where
//     F: Fn(),
// {
//     function: F,
// }
// impl<F> EfficientStruct<F>
// where
//     F: Fn(),
// {
//     fn call_function(&self) {
//         (self.function)();
//     }
// }

//---
/// TODO: Get this from slate::events::*.
#[derive(Debug)]
pub struct ClickEvent;

impl Event for ClickEvent {
    /// TODO
    fn kind(self) -> EventKind {
        EventKind::Click
    }
}

/// TODO: Get this from slate::events::*.
#[derive(Debug)]
pub struct HoverEvent;

impl Event for HoverEvent {
    /// TODO
    fn kind(self) -> EventKind {
        EventKind::Hover
    }
}
