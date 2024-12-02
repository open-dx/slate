use core::ops::Deref;
use core::ops::DerefMut;
use core::time::Duration;

use bevy::prelude::EventWriter;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::time::Time;
use bevy::time::Timer;
use bevy::time::TimerMode;
use bevy::ecs::component::Component;
use bevy::ecs::event::Event;

#[cfg(feature = "profiling")]
use tracy_client::Client as TracyClient;

#[derive(Component)]
pub struct DrawTimer(Timer);

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
pub struct DrawTimerFinishedEvent(Duration);

impl DrawTimerFinishedEvent {
    pub fn new(duration: Duration) -> Self {
        DrawTimerFinishedEvent(duration)
    }
}

impl DrawTimerFinishedEvent {
    pub fn duration(&self) -> Duration {
        self.0
    }
}

/// TODO
pub fn sync_draw_timer(
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
