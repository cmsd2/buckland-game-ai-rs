use crate::{
    location::Location,
    log::{ConsoleLog, Log, Named},
};
use game_state_machine::*;
use rand::distributions::{Distribution, Standard};

enum PartnerChore {
    Mopping,
    Washing,
    BedMaking,
}

impl Distribution<PartnerChore> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> PartnerChore {
        match rng.gen_range(0..2) {
            0 => PartnerChore::Mopping,
            1 => PartnerChore::Washing,
            2 => PartnerChore::BedMaking,
            _ => unreachable!(),
        }
    }
}

pub struct Partner {
    pub name: String,
    location: Location,
}

impl<'a> Named<'a> for Partner {
    fn name(&'a self) -> &'a str {
        &self.name
    }
}

impl Partner {
    pub fn new(name: String) -> Self {
        Partner {
            name,
            location: Location::Shack,
        }
    }

    pub fn log(&self, msg: String) {
        ConsoleLog.log(self, msg);
    }
}

pub struct DoHouseWork;

impl State<Partner> for DoHouseWork {
    fn update(&mut self, partner: &mut Partner) -> StateTransition<Partner> {
        if rand::random::<f32>() < 0.1 {
            return StateTransition::Push(Box::new(VisitBathroom));
        }

        match rand::random() {
            PartnerChore::Mopping => {
                partner.log(format!("Moppin' the floor"));
            }
            PartnerChore::BedMaking => {
                partner.log(format!("Makin' the bed"));
            }
            PartnerChore::Washing => {
                partner.log(format!("Washin' the dishes"));
            }
        }

        StateTransition::None
    }
}

pub struct VisitBathroom;

impl State<Partner> for VisitBathroom {
    fn on_start(&mut self, partner: &mut Partner) {
        partner.log(format!("Walkin' to the can"));
    }

    fn on_resume(&mut self, partner: &mut Partner) {
        self.on_start(partner);
    }

    fn update(&mut self, partner: &mut Partner) -> StateTransition<Partner> {
        partner.log(format!("Ahhhhhh! Sweet relief"));

        StateTransition::Pop
    }

    fn on_stop(&mut self, partner: &mut Partner) {
        partner.log(format!("Leavin' the Jon"));
    }
}
