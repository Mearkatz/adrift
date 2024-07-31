use bounded_ints::BoundedU8;
pub mod traits;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Resource {
    Water,
    DirtyWater,
    Electricity,
    Oxygen,
}

/// Represents units
struct Unit {
    resouce: Resource,
    amount: u32,
}

pub mod percent {
    use std::fmt::Display;

    use bounded_ints::BoundedU8;

    #[derive(Debug, Clone, Copy)]
    pub struct Percent(BoundedU8<0, 100>);

    impl Percent {
        pub const fn get(self) -> u8 {
            self.0.get()
        }
    }

    impl Display for Percent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            const FULL: &str = "█";
            const EMPTY: &str = "░";
            let n = self.0.get() as usize / 10;
            write!(f, "[{}{}]", FULL.repeat(n), EMPTY.repeat(10 - n))
        }
    }
}

struct Crewmate {
    name: String,
    health: BoundedU8<0, 10>,
    mood: BoundedU8<0, 10>,
    food: BoundedU8<0, 10>,
    water: BoundedU8<0, 10>,
}

struct Ship {
    crew: Vec<Crewmate>,
    hull_health: BoundedU8<0, 10>,

    oxygen_system: OxygenSystem,
    water_system: WaterSystem,
    greenhouse: Greenhouse,
}

struct WaterSystem {
    dirty_water: BoundedU8<0, 10>,
    power: BoundedU8<0, 10>,
    health: BoundedU8<0, 10>,
}

impl WaterSystem {
    /// The amount of water produced by this system
    fn water_output(&self) {
        (self.dirty_water.get() * self.power.get()) / 10;
    }
}

fn main() {
    let p = Percent(unsafe { BoundedU8::new_unchecked(15) });
    println!("{p}");
}
