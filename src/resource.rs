
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ResourceKind {
    Water,
    DirtyWater,
    Food,
    Electricity,
    Oxygen,
}

/// Something that a type needs to function properly like Water, or Oxygen.
/// `range` denotes the minimum and maximum amount of this resource that is desired by a
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Resource {
    kind: ResourceKind,
    amount: u8,
}

impl Resource {
    pub fn new(kind: ResourceKind, amount: u8) -> Self {
        Self { kind, amount }
    }
    pub fn water(amount: u8) -> Self {
        Self::new(ResourceKind::Water, amount)
    }
    pub fn dirty_water(amount: u8) -> Self {
        Self::new(ResourceKind::DirtyWater, amount)
    }
    pub fn food(amount: u8) -> Self {
        Self::new(ResourceKind::Food, amount)
    }
    pub fn electricity(amount: u8) -> Self {
        Self::new(ResourceKind::Electricity, amount)
    }
    pub fn oxygen(amount: u8) -> Self {
        Self::new(ResourceKind::Oxygen, amount)
    }
}
