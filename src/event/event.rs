
pub trait Event {
    fn kind(self) -> EventKind;
}

#[derive(Debug)]
pub enum EventKind {
    /// TODO
    Click,
}

pub type EventHandlerFn<E> = fn(event: &E);

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
