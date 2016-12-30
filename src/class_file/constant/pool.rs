use {Constant, ConstantIndex};

use std::collections::HashMap;

const INITIAL_INDEX: usize = 1;

/// A constant pool.
#[derive(Clone, Debug)]
pub struct ConstantPool
{
    accumulator: usize,
    pool: HashMap<ConstantIndex, Constant>,
}

impl ConstantPool
{
    /// Creates a new constant pool.
    pub fn new() -> Self {
        ConstantPool {
            accumulator: INITIAL_INDEX,
            pool: HashMap::new(),
        }
    }

    /// Inserts a new value into the pool if it isn't there already.
    ///
    /// Returns the index of the newly added constant or the index of an identical
    /// constant that already exists.
    pub fn insert(&mut self, constant: Constant) -> ConstantIndex {
        if let Some((&index,_)) = self.pool.iter().find(|&(_,v)| *v == constant) {
            index // The constant already exists.
        } else {
            // We need to insert a new value.
            let index = ConstantIndex(self.accumulator);
            self.accumulator += 1;

            self.pool.insert(index, constant);
            index
        }
    }

    /// Retrieves an existing item from the pool.
    pub fn get(&self, index: ConstantIndex) -> Option<&Constant> {
        self.pool.get(&index)
    }

    pub fn iter(&self) -> ::std::collections::hash_map::Iter<ConstantIndex, Constant> {
        self.pool.iter()
    }
}
