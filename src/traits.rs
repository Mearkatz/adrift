use std::collections::HashSet;

use crate::resource::Resource;

pub trait Inputs {
    fn inputs() -> HashSet<Resource>;
}

pub trait Outputs {
    fn outputs() -> HashSet<Resource>;
}
