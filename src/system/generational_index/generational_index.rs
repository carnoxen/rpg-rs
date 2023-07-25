use std::{iter, slice, vec};
use std::iter::FromIterator;

/// A unique identifier with an associated usize index.  Indexes are valued proportional to the
/// number of indexes allocated, are reused after being freed, and do not grow without bound.  When
/// an index is re-used, an associated "generation" is incremented, so that within the life of a
/// single allocator, no two GenerationalIndex values will ever be equal.  Since the indexes do not
/// grow without bound, GenerationalIndex values are particularly suited to being stored by their
/// index in extremely fast contiguous arrays.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct GenerationalIndex {
    index: usize,
    generation: u64,
}

impl GenerationalIndex {
    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    #[inline]
    pub fn generation(&self) -> u64 {
        self.generation
    }
}