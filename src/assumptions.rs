use std::{collections::HashMap, sync::Arc};

use crate::basic::Thing;

pub struct Assumptions {
    positive: HashMap<Arc<Thing>, bool>,
}

impl Assumptions {
    pub fn new() -> Self {
        Self {
            positive: HashMap::new(),
        }
    }
}