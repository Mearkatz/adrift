use bounded_ints::BoundedU8;

pub mod crewmate;
pub mod resource;
pub mod traits;

/// Creates a progress bar out of a BoundedU8
fn bar<const MIN: u8, const MAX: u8>(
    value: BoundedU8<MIN, MAX>,
    empty: &str,
    full: &str,
) -> String {
    format!(
        "{}{}",
        full.repeat(value.get().into()),
        empty.repeat((MAX - value.get()).into())
    )
}

pub mod ship {
    use bounded_ints::BoundedU8;

    #[derive(Debug, Clone)]
    pub struct Ship {
        /// Health of the hull.
        /// When this reaches zero the ship is destroyed.
        pub health: BoundedU8<0, 10>,

        pub water: u8,
        pub dirty_water: u8,
        pub food: u8,
        pub electricity: u8,
        pub oxygen: u8,
    }
}

fn main() -> Result<(), std::io::Error> {
    // let ship = Ship::new(BoundedU8::new(10).unwrap(), 10, 10, 10, 10, 10);
    // dbg!(ship);
    let x: BoundedU8<0, 10> = BoundedU8::new(5).unwrap();
    let b = bar(x, ".", "#");
    println!("{b}");
    Ok(())
}
