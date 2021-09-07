use bevy_app::Plugin;
use bevy_ecs::prelude::*;
use wheel_timer::WheelTimer;

static MAX_INTERVAL: usize = 20;

#[derive(Clone, Debug)]
pub enum Event {
    Message,
}

pub struct Timer {
    wheeltimer: WheelTimer<Event>,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            wheeltimer: WheelTimer::new(MAX_INTERVAL),
        }
    }
    pub fn tick(&mut self) -> Vec<Event> {
        self.wheeltimer.tick()
    }

    pub fn schedule(&mut self, delay: usize, event: Event) {
        self.wheeltimer.schedule(delay, event);
    }
}

fn timer(mut wheeltimer: ResMut<Timer>) {
    for event in wheeltimer.tick() {
        println!("timer event: {:?}", event);
    }
}

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut bevy_app::AppBuilder) {
        app.insert_resource(Timer::new());
        app.add_system(timer.system());
    }
}
