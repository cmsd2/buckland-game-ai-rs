use crate::log::{ConsoleLog, Log, Named};
use game_state_machine::*;

pub static COMFORT_LEVEL: i32 = 5; // the amount of gold a miner must have before he feels comfortable
pub static MAX_NUGGETS: i32 = 3; // the amount of nuggets a miner can carry
pub static THIRST_LEVEL: i32 = 5; // above this value a miner is thirsty
pub static TIREDNESS_THRESHOLD: i32 = 5; // above this value a miner is sleepy

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Location {
    Goldmine,
    Bank,
    Shack,
    Saloon,
}

pub struct Miner {
    pub name: String,
    pub location: Location,
    gold: i32,
    bank: i32,
    thirst: i32,
    fatigue: i32,
}

impl<'a> Named<'a> for Miner {
    fn name(&'a self) -> &'a str {
        &self.name
    }
}

impl Miner {
    pub fn new(name: String) -> Self {
        Miner {
            name,
            location: Location::Shack,
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
    pub fn log(&self, msg: String) {
        ConsoleLog.log(self, msg);
    }
}

pub struct EnterMineAndDigForNugget;

impl State<Miner> for EnterMineAndDigForNugget {
    fn on_start(&mut self, miner: &mut Miner) {
        if miner.location != Location::Goldmine {
            miner.log(format!("Walkin' to the goldmine"));
            miner.location = Location::Goldmine;
        }
    }

    fn on_resume(&mut self, miner: &mut Miner) {
        self.on_start(miner);
    }

    fn update(&mut self, miner: &mut Miner) -> StateTransition<Miner> {
        miner.increase_thirst();
        miner.add_to_gold_carried(1);
        miner.increase_fatigue();

        miner.log(format!("Pickin' up a nugget"));

        if miner.pockets_full() {
            StateTransition::Switch(Box::new(VisitBankAndDepositGold))
        } else if miner.thirsty() {
            StateTransition::Switch(Box::new(QuenchThirst))
        } else {
            StateTransition::None
        }
    }

    fn on_stop(&mut self, miner: &mut Miner) {
        miner.log(format!(
            "Ah'm leavin' the goldmine with mah pockets full o' sweet gold"
        ));
    }
}

pub struct VisitBankAndDepositGold;

impl State<Miner> for VisitBankAndDepositGold {
    fn on_start(&mut self, miner: &mut Miner) {
        if miner.location != Location::Bank {
            miner.log(format!("Goin' to the bank. Yes siree"));
            miner.location = Location::Bank;
        }
    }

    fn on_resume(&mut self, miner: &mut Miner) {
        self.on_start(miner);
    }

    fn update(&mut self, miner: &mut Miner) -> StateTransition<Miner> {
        miner.increase_thirst();
        miner.move_gold_to_bank();
        miner.log(format!(
            "Depositing gold. Total savings now: {}",
            miner.wealth()
        ));

        if miner.wealth() >= COMFORT_LEVEL {
            miner.log(format!(
                "WooHoo! Rich enough for now. Back home to mah li'lle lady"
            ));
            StateTransition::Switch(Box::new(GoHomeAndSleepTilRested))
        } else {
            StateTransition::Switch(Box::new(EnterMineAndDigForNugget))
        }
    }

    fn on_stop(&mut self, miner: &mut Miner) {
        miner.log(format!("Leavin' the bank"));
    }
}

pub struct GoHomeAndSleepTilRested;

impl State<Miner> for GoHomeAndSleepTilRested {
    fn on_start(&mut self, miner: &mut Miner) {
        if miner.location != Location::Shack {
            miner.log(format!("Walkin' home"));
            miner.location = Location::Shack;
        }
    }

    fn update(&mut self, miner: &mut Miner) -> StateTransition<Miner> {
        miner.increase_thirst();
        if !miner.fatigued() {
            miner.log(format!(
                "What a God darn fantastic nap! Time to find more gold"
            ));
            StateTransition::Switch(Box::new(EnterMineAndDigForNugget))
        } else {
            miner.decrease_fatigue();
            miner.log(format!("ZZZZ... "));
            StateTransition::None
        }
    }

    fn on_stop(&mut self, miner: &mut Miner) {
        miner.log(format!("Leaving the house"));
    }
}

pub struct QuenchThirst;

impl State<Miner> for QuenchThirst {
    fn on_start(&mut self, miner: &mut Miner) {
        if miner.location != Location::Saloon {
            miner.location = Location::Saloon;
            miner.log(format!("Boy, ah sure is thusty! Walking to the saloon"));
        }
    }

    fn update(&mut self, miner: &mut Miner) -> StateTransition<Miner> {
        miner.increase_thirst();
        if miner.thirsty() {
            miner.buy_and_drink_whiskey();
            miner.log(format!("That's mighty fine sippin liquer"));
            StateTransition::Switch(Box::new(EnterMineAndDigForNugget))
        } else {
            println!("ERROR!\nERROR!\nERROR!");
            StateTransition::Quit
        }
    }

    fn on_stop(&mut self, miner: &mut Miner) {
        miner.log(format!("Leaving the saloon, feelin' good"));
    }
}
