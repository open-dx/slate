
pub trait Event {
    fn kind(self) -> EventKind;
}

#[derive(Debug)]
pub enum EventKind {
    /// TODO
    Click,
}

pub type EventHandlerFn<E> = fn(event: &E);

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
