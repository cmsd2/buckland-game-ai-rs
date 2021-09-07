use std::fmt;

use bevy_app::App;
use bevy_ecs::prelude::*;
use bevy_log::LogPlugin;
use miner::MinerPlugin;

mod fsm;
mod log;
mod miner;
// mod timer;

pub struct Person;

pub struct Name(String);

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Location {
    Goldmine,
    Bank,
    Shack,
    Saloon,
}

fn runner(mut app: App) {
    loop {
        app.update();
    }
}

fn main() {
    App::build()
        .add_plugin(LogPlugin)
        .add_plugin(MinerPlugin)
        .set_runner(runner)
        .run();
}
