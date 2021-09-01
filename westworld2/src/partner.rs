use crate::location::Location;
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

impl Partner {
    pub fn new(name: String) -> Self {
        Partner {
            name,
            location: Location::Shack,
        }
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
                println!("{}: Moppin' the floor", partner.name);
            }
            PartnerChore::BedMaking => {
                println!("{}: Makin' the bed", partner.name);
            }
            PartnerChore::Washing => {
                println!("{}: Washin' the dishes", partner.name);
            }
        }

        StateTransition::None
    }
}

pub struct VisitBathroom;

impl State<Partner> for VisitBathroom {
    fn on_start(&mut self, partner: &mut Partner) {
        println!("{}: Walkin' to the can", partner.name);
    }

    fn on_resume(&mut self, partner: &mut Partner) {
        self.on_start(partner);
    }

    fn update(&mut self, partner: &mut Partner) -> StateTransition<Partner> {
        println!("{}: Ahhhhhh! Sweet relief", partner.name);

        StateTransition::Pop
    }

    fn on_stop(&mut self, partner: &mut Partner) {
        println!("{}: Leavin' the Jon", partner.name);
    }
}
