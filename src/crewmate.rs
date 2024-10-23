use crate::{
    resource::Resource,
    traits::{Inputs, Outputs},
};
use bounded_ints::BoundedU8;
use std::{collections::HashSet, iter::once};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Crewmate {
    /// What the crewmate likes to be called.
    name: String,

    /// A measure of the crewmate's wellbeing.
    /// Affected by most other stats.
    /// When this reaches zero the crewmate dies.
    health: BoundedU8<0, 10>,

    /// How sane the crewmate is.
    /// Decreases with boredom, lack of food/water, or death of another crewmate.
    /// Whenever this is zero the crewmate takes damage.
    sanity: BoundedU8<0, 10>,

    /// A measure of the crewmate's supply of oxygen.
    /// Whenever this is zero the crewmate takes damage.
    oxygen: BoundedU8<0, 10>,

    /// How hungry the crewmate is.
    /// When they eat `n` food, their food meter increases by `n`.
    /// Whenever this is zero the crewmate takes damage.
    food: BoundedU8<0, 10>,

    /// How thirsty the crewmate is.
    /// When they drink `n` water, their water meter increases by `n`.
    /// Whenever this is empty the crewmate takes damage.
    water: BoundedU8<0, 10>,
}

impl Inputs for Crewmate {
    fn inputs() -> HashSet<Resource> {
        [Resource::water(2), Resource::food(2)]
            .into_iter()
            .collect()
    }
}

impl Outputs for Crewmate {
    fn outputs() -> HashSet<Resource> {
        once(Resource::dirty_water(2)).collect()
    }
}
