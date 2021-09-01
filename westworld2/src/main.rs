use game_state_machine::StateMachine;
use std::io::{stdin, stdout, Read, Write};
use std::thread;
use std::time::Duration;

mod location;
mod log;
mod miner;
mod partner;

use miner::{GoHomeAndSleepTilRested, Miner};
use partner::{DoHouseWork, Partner};

fn main() {
    let mut sm = StateMachine::<Miner>::default();
    let mut miner = Miner::new("Miner Bob".into());
    sm.push(Box::new(GoHomeAndSleepTilRested), &mut miner);

    let mut sm2 = StateMachine::<Partner>::default();
    let mut partner = Partner::new("Elsa".into());
    sm2.push(Box::new(DoHouseWork), &mut partner);

    while sm.is_running() || sm2.is_running() {
        if sm.is_running() {
            sm.update(&mut miner);
        }

        if sm2.is_running() {
            sm2.update(&mut partner);
        }

        println!("");

        thread::sleep(Duration::from_millis(800));
    }

    pause();
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
