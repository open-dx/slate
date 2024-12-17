extern crate alloc;

use core::any::{Any, TypeId};
use alloc::collections::BTreeMap;

use bumpalo::Bump;

pub trait Event: Any {
    fn event_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> Event for T where T: Any {
    fn event_name(&self) -> &'static str {
        core::any::type_name::<T>()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

type Callback<'a> = &'a dyn Fn(&dyn Event);

pub struct EventStack<'a> {
    handlers: BTreeMap<TypeId, Vec<Callback<'a>>>,
    allocator: &'a Bump,
}

impl<'a> EventStack<'a> {
    pub fn new(allocator: &'a Bump) -> Self {
        Self {
            handlers: BTreeMap::new(),
            allocator,
        }
    }

    pub fn register<E: Event>(&mut self, handler: impl Fn(&E) + 'a) {
        let type_id = TypeId::of::<E>();
        let callback: Callback<'a> = self.allocator.alloc(move |event: &dyn Event| {
            if let Some(event) = event.as_any().downcast_ref::<E>() {
                handler(event);
            }
        });
        self.handlers.entry(type_id).or_default().push(callback);
    }

    pub fn trigger<E: Event>(&self, event: &E) {
        if let Some(handlers) = self.handlers.get(&TypeId::of::<E>()) {
            for handler in handlers {
                handler(event);
            }
        }
    }
}

// Example usage
struct ClickEvent {
    pub x: i32,
    pub y: i32,
}

fn main() {
    let bump = Bump::new();
    let mut event_stack = EventStack::new(&bump);

    event_stack.register(|e: &ClickEvent| {
        println!("Click at ({}, {})", e.x, e.y);
    });

    let click_event = ClickEvent { x: 100, y: 200 };
    event_stack.trigger(&click_event);
}
