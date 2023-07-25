/// Allocates GenerationalIndexes without duplication.
#[derive(Clone, Default)]
pub struct GenerationalIndexAllocator {
    entries: Vec<AllocatorEntry>,
    free: Vec<usize>,
}

impl GenerationalIndexAllocator {
    pub fn new() -> GenerationalIndexAllocator {
        Default::default()
    }

    pub fn allocate(&mut self) -> GenerationalIndex {
        if let Some(index) = self.free.pop() {
            let id_entry = &mut self.entries[index];
            assert!(!id_entry.is_live);
            id_entry.is_live = true;
            GenerationalIndex {
                index: index,
                generation: id_entry.generation,
            }
        } else {
            self.entries.push(AllocatorEntry {
                is_live: true,
                generation: 0,
            });
            GenerationalIndex {
                index: self.entries.len() - 1,
                generation: 0,
            }
        }
    }

    pub fn deallocate(&mut self, gen_index: GenerationalIndex) -> bool {
        if gen_index.index >= self.entries.len() {
            return false;
        }

        let id_entry = &mut self.entries[gen_index.index];
        if !id_entry.is_live {
            return false;
        }

        id_entry.is_live = false;
        id_entry.generation = id_entry
            .generation
            .checked_add(1)
            .expect("GenerationalIndex generation overflow");
        self.free.push(gen_index.index);
        true
    }

    #[inline]
    pub fn is_live(&self, gen_index: GenerationalIndex) -> bool {
        if gen_index.index < self.entries.len() {
            let id_entry = &self.entries[gen_index.index];
            id_entry.is_live && id_entry.generation == gen_index.generation
        } else {
            false
        }
    }

    /// Returns the maximum index ever allocated so far.
    #[inline]
    pub fn max_allocated_index(&self) -> usize {
        self.entries.len()
    }

    /// If there is a live GenerationalIndex for the given index, returns it.  All entries past
    /// max_allocated_index will return None.
    #[inline]
    pub fn live_at_index(&self, index: usize) -> Option<GenerationalIndex> {
        self.entries.get(index).and_then(|entry| {
            if entry.is_live {
                Some(GenerationalIndex {
                    index,
                    generation: self.entries[index].generation,
                })
            } else {
                None
            }
        })
    }
}