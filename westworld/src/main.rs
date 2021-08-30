use game_state_machine::StateMachine;
use std::io::{stdin, stdout, Read, Write};
use std::thread;
use std::time::Duration;

mod miner;

use miner::{GoHomeAndSleepTilRested, Miner};

fn main() {
    let mut sm = StateMachine::<Miner>::default();
    let mut miner = Miner::new("Miner Bob".into());

    sm.push(Box::new(GoHomeAndSleepTilRested), &mut miner);

    while sm.is_running() {
        sm.update(&mut miner);
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
