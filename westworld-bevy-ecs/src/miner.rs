use std::ops::{Deref, DerefMut};

use crate::fsm::{self, Handler};
use crate::{
    log::{ConsoleLog, Log, Named},
    Location, Name,
};
use bevy_app::{AppBuilder, Plugin};
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;

pub static COMFORT_LEVEL: i32 = 5; // the amount of gold a miner must have before he feels comfortable
pub static MAX_NUGGETS: i32 = 3; // the amount of nuggets a miner can carry
pub static THIRST_LEVEL: i32 = 5; // above this value a miner is thirsty
pub static TIREDNESS_THRESHOLD: i32 = 5; // above this value a miner is sleepy

pub type MinerStateData<'a> = (&'a Name, &'a mut Location, &'a mut Miner);
//pub type MinerStateData = (Name, Location, Miner);

pub struct Miner {
    gold: i32,
    bank: i32,
    thirst: i32,
    fatigue: i32,
}

impl Miner {
    pub fn new() -> Self {
        Miner {
            gold: 0,
            bank: 0,
            thirst: 0,
            fatigue: 0,
        }
    }
    pub fn add_to_gold_carried(&mut self, gold: i32) {
        self.gold += gold;
        if self.gold < 0 {
            self.gold = 0;
        }
    }
    pub fn increase_fatigue(&mut self) {
        self.fatigue += 1;
    }
    pub fn decrease_fatigue(&mut self) {
        self.fatigue -= 1;
    }
    pub fn pockets_full(&self) -> bool {
        self.gold >= MAX_NUGGETS
    }
    pub fn increase_thirst(&mut self) {
        self.thirst += 1;
    }
    pub fn thirsty(&self) -> bool {
        self.thirst > THIRST_LEVEL
    }
    pub fn buy_and_drink_whiskey(&mut self) {
        self.bank -= 2;
        self.thirst = 0;
    }
    pub fn move_gold_to_bank(&mut self) {
        self.bank += self.gold;
        self.gold = 0;
    }
    pub fn wealth(&self) -> i32 {
        self.bank
    }
    pub fn fatigued(&self) -> bool {
        self.fatigue > TIREDNESS_THRESHOLD
    }
}

#[derive(Copy, Clone)]
pub enum MinerState {
    EnterMineAndDigForNugget,
    VisitBankAndDepositGold,
    QuenchThirst,
    GoHomeAndSleepTilRested,
}

pub struct EnterMineAndDigForNugget;

impl<'a> fsm::Handler<MinerState, MinerStateData<'a>> for EnterMineAndDigForNugget {
    fn on_start(&self, state: &MinerState, (name, location, miner): &mut MinerStateData) {
        if **location != Location::Goldmine {
            info!("{}: Walkin' to the goldmine", name);
            **location = Location::Goldmine;
        }
    }

    fn on_resume(&self, state: &MinerState, state_data: &mut MinerStateData) {
        self.on_start(state, state_data);
    }

    fn update(
        &self,
        state: &MinerState,
        (name, location, miner): &mut MinerStateData,
    ) -> fsm::StateTransition<MinerState> {
        miner.increase_thirst();
        miner.add_to_gold_carried(1);
        miner.increase_fatigue();

        info!("{}: Pickin' up a nugget", name);

        if miner.pockets_full() {
            fsm::StateTransition::Switch(MinerState::VisitBankAndDepositGold)
        } else if miner.thirsty() {
            fsm::StateTransition::Switch(MinerState::QuenchThirst)
        } else {
            fsm::StateTransition::None
        }
    }

    fn on_stop(&self, state: &MinerState, (name, _location, _miner): &mut MinerStateData) {
        info!(
            "{}: Ah'm leavin' the goldmine with mah pockets full o' sweet gold",
            name
        );
    }
}

pub struct VisitBankAndDepositGold;

impl<'a> fsm::Handler<MinerState, MinerStateData<'a>> for VisitBankAndDepositGold {
    fn on_start(&self, state: &MinerState, (name, location, miner): &mut MinerStateData) {
        if **location != Location::Bank {
            info!("{}: Goin' to the bank. Yes siree", name);
            **location = Location::Bank;
        }
    }

    fn on_resume(&self, state: &MinerState, miner: &mut MinerStateData) {
        self.on_start(state, miner);
    }

    fn update(
        &self,
        state: &MinerState,
        (name, location, miner): &mut MinerStateData,
    ) -> fsm::StateTransition<MinerState> {
        miner.increase_thirst();
        miner.move_gold_to_bank();
        info!(
            "{}: Depositing gold. Total savings now: {}",
            name,
            miner.wealth()
        );

        if miner.wealth() >= COMFORT_LEVEL {
            info!(
                "{}: WooHoo! Rich enough for now. Back home to mah li'lle lady",
                name
            );
            fsm::StateTransition::Switch(MinerState::GoHomeAndSleepTilRested)
        } else {
            fsm::StateTransition::Switch(MinerState::EnterMineAndDigForNugget)
        }
    }

    fn on_stop(&self, state: &MinerState, (name, _location, _miner): &mut MinerStateData) {
        info!("{}: Leavin' the bank", name);
    }
}

pub struct GoHomeAndSleepTilRested;

impl<'a> fsm::Handler<MinerState, MinerStateData<'a>> for GoHomeAndSleepTilRested {
    fn on_start(&self, state: &MinerState, (name, location, miner): &mut MinerStateData) {
        if **location != Location::Shack {
            info!("{}: Walkin' home", name);
            **location = Location::Shack;
        }
    }

    fn update(
        &self,
        state: &MinerState,
        (name, location, miner): &mut MinerStateData,
    ) -> fsm::StateTransition<MinerState> {
        miner.increase_thirst();
        if !miner.fatigued() {
            info!(
                "{}: What a God darn fantastic nap! Time to find more gold",
                name
            );
            fsm::StateTransition::Switch(MinerState::EnterMineAndDigForNugget)
        } else {
            miner.decrease_fatigue();
            info!("{}: ZZZZ... ", name);
            fsm::StateTransition::None
        }
    }

    fn on_stop(&self, state: &MinerState, (name, _location, _miner): &mut MinerStateData) {
        info!("{}: Leaving the house", name);
    }
}

pub struct QuenchThirst;

impl<'a> fsm::Handler<MinerState, MinerStateData<'a>> for QuenchThirst {
    fn on_start(&self, state: &MinerState, (name, location, miner): &mut MinerStateData) {
        if **location != Location::Saloon {
            **location = Location::Saloon;
            info!("{}: Boy, ah sure is thusty! Walking to the saloon", name);
        }
    }

    fn update(
        &self,
        state: &MinerState,
        (name, location, miner): &mut MinerStateData,
    ) -> fsm::StateTransition<MinerState> {
        miner.increase_thirst();
        if miner.thirsty() {
            miner.buy_and_drink_whiskey();
            info!("{}: That's mighty fine sippin liquer", name);
            fsm::StateTransition::Switch(MinerState::EnterMineAndDigForNugget)
        } else {
            println!("ERROR!\nERROR!\nERROR!");
            fsm::StateTransition::Quit
        }
    }

    fn on_stop(&self, state: &MinerState, (name, _location, _miner): &mut MinerStateData) {
        info!("{}: Leaving the saloon, feelin' good", name);
    }
}

pub struct MinerHandler;

impl<'a> fsm::Handler<MinerState, MinerStateData<'a>> for MinerHandler {
    fn on_start(&self, state: &MinerState, state_data: &mut MinerStateData<'a>) {
        match state {
            MinerState::EnterMineAndDigForNugget => {
                EnterMineAndDigForNugget.on_start(state, state_data)
            }
            MinerState::VisitBankAndDepositGold => {
                VisitBankAndDepositGold.on_start(state, state_data)
            }
            MinerState::GoHomeAndSleepTilRested => {
                GoHomeAndSleepTilRested.on_start(state, state_data)
            }
            MinerState::QuenchThirst => QuenchThirst.on_start(state, state_data),
        }
    }

    fn on_stop(&self, state: &MinerState, state_data: &mut MinerStateData<'a>) {
        match state {
            MinerState::EnterMineAndDigForNugget => {
                EnterMineAndDigForNugget.on_stop(state, state_data)
            }
            MinerState::VisitBankAndDepositGold => {
                VisitBankAndDepositGold.on_stop(state, state_data)
            }
            MinerState::GoHomeAndSleepTilRested => {
                GoHomeAndSleepTilRested.on_stop(state, state_data)
            }
            MinerState::QuenchThirst => QuenchThirst.on_stop(state, state_data),
        }
    }

    fn on_pause(&self, state: &MinerState, state_data: &mut MinerStateData<'a>) {
        match state {
            MinerState::EnterMineAndDigForNugget => {
                EnterMineAndDigForNugget.on_pause(state, state_data)
            }
            MinerState::VisitBankAndDepositGold => {
                VisitBankAndDepositGold.on_pause(state, state_data)
            }
            MinerState::GoHomeAndSleepTilRested => {
                GoHomeAndSleepTilRested.on_pause(state, state_data)
            }
            MinerState::QuenchThirst => QuenchThirst.on_pause(state, state_data),
        }
    }

    fn on_resume(&self, state: &MinerState, state_data: &mut MinerStateData<'a>) {
        match state {
            MinerState::EnterMineAndDigForNugget => {
                EnterMineAndDigForNugget.on_resume(state, state_data)
            }
            MinerState::VisitBankAndDepositGold => {
                VisitBankAndDepositGold.on_resume(state, state_data)
            }
            MinerState::GoHomeAndSleepTilRested => {
                GoHomeAndSleepTilRested.on_resume(state, state_data)
            }
            MinerState::QuenchThirst => QuenchThirst.on_resume(state, state_data),
        }
    }

    fn update(
        &self,
        state: &MinerState,
        state_data: &mut MinerStateData<'a>,
    ) -> fsm::StateTransition<MinerState> {
        match state {
            MinerState::EnterMineAndDigForNugget => {
                EnterMineAndDigForNugget.update(state, state_data)
            }
            MinerState::VisitBankAndDepositGold => {
                VisitBankAndDepositGold.update(state, state_data)
            }
            MinerState::GoHomeAndSleepTilRested => {
                GoHomeAndSleepTilRested.update(state, state_data)
            }
            MinerState::QuenchThirst => QuenchThirst.update(state, state_data),
        }
    }
}

pub struct MinerPlugin;

impl Plugin for MinerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(init_miners.system());
        app.add_system(update_miners.system());
    }
}

pub fn init_miners(mut commands: Commands) {
    info!("initialising miners");
    commands
        .spawn()
        .insert(Name("Miner Bob".to_string()))
        .insert(Location::Shack)
        .insert(Miner::new())
        .insert(fsm::StateStack::<MinerState>::new_initial_state(
            MinerState::GoHomeAndSleepTilRested,
        ));
}

pub fn update_miners(
    mut miners: Query<(
        &Name,
        &mut Location,
        &mut Miner,
        &mut fsm::StateStack<MinerState>,
    )>,
) {
    for (name, mut location, mut miner, mut state_stack) in miners.iter_mut() {
        let mut stack_data = (name, location.deref_mut(), miner.deref_mut());
        fsm::StateMachine::update(&MinerHandler, &mut state_stack, &mut stack_data);
    }
}
