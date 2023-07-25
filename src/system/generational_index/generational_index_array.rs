/// An associative array of GenerationalIndex keys to values.  Takes advantage of how
/// GenerationalIndex indexes work to very efficiently map values in a contiguous array.  Generally
/// only efficient when storing lots of entries for a long time, as it has storage requirements
/// proportional to the largest index encountered.
#[derive(Clone, Default)]
pub struct GenerationalIndexArray<T>(Vec<Option<ArrayEntry<T>>>);

impl<T> GenerationalIndexArray<T> {
    pub fn new() -> GenerationalIndexArray<T> {
        GenerationalIndexArray(Vec::new())
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Overwrites any entry with the matching index, returns both the GenerationalIndex and T that
    /// were replaced, which may be a GenerationalIndex from a past generation.
    pub fn insert(
        &mut self,
        gen_index: GenerationalIndex,
        value: T,
    ) -> Option<(GenerationalIndex, T)> {
        if gen_index.index >= self.0.len() {
            for _ in self.0.len()..gen_index.index + 1 {
                self.0.push(None);
            }
        }

        let entry = &mut self.0[gen_index.index];

        let old = entry.take().map(|e| {
            (
                GenerationalIndex {
                    index: gen_index.index,
                    generation: e.generation,
                },
                e.value,
            )
        });
        *entry = Some(ArrayEntry {
            value,
            generation: gen_index.generation,
        });
        old
    }

    pub fn remove(&mut self, gen_index: GenerationalIndex) -> Option<T> {
        if gen_index.index < self.0.len() {
            let entry = &mut self.0[gen_index.index];

            if let Some(e) = entry.take() {
                if e.generation == gen_index.generation {
                    return Some(e.value);
                } else {
                    *entry = Some(e);
                }
            }
        }
        None
    }

    pub fn contains_key(&self, gen_index: GenerationalIndex) -> bool {
        self.get(gen_index).is_some()
    }

    pub fn get(&self, gen_index: GenerationalIndex) -> Option<&T> {
        if gen_index.index < self.0.len() {
            self.0[gen_index.index].as_ref().and_then(|e| {
                if e.generation == gen_index.generation {
                    Some(&e.value)
                } else {
                    None
                }
            })
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, gen_index: GenerationalIndex) -> Option<&mut T> {
        if gen_index.index < self.0.len() {
            self.0[gen_index.index].as_mut().and_then(|e| {
                if e.generation == gen_index.generation {
                    Some(&mut e.value)
                } else {
                    None
                }
            })
        } else {
            None
        }
    }

    pub fn retain<F: FnMut(GenerationalIndex, &mut T) -> bool>(&mut self, mut f: F) {
        for i in 0..self.0.len() {
            let entry = &mut self.0[i];

            let keep = if let Some(entry) = entry.as_mut() {
                f(
                    GenerationalIndex {
                        index: i,
                        generation: entry.generation,
                    },
                    &mut entry.value,
                )
            } else {
                false
            };

            if !keep {
                *entry = None;
            }
        }
    }

    pub fn filter_map<F: FnMut(GenerationalIndex, T) -> Option<T>>(&mut self, mut f: F) {
        for i in 0..self.0.len() {
            let entry = &mut self.0[i];

            if let Some(e) = entry.take() {
                let gen_index = GenerationalIndex {
                    index: i,
                    generation: e.generation,
                };

                if let Some(value) = f(gen_index, e.value) {
                    *entry = Some(ArrayEntry {
                        value,
                        generation: gen_index.generation,
                    })
                }
            }
        }
    }

    pub fn iter<'a>(&'a self) -> GenerationalIndexArrayIter<'a, T> {
        GenerationalIndexArrayIter(self.0.iter().enumerate())
    }

    pub fn iter_mut<'a>(&'a mut self) -> GenerationalIndexArrayIterMut<'a, T> {
        GenerationalIndexArrayIterMut(self.0.iter_mut().enumerate())
    }
}

impl<'a, T: 'a> IntoIterator for &'a GenerationalIndexArray<T> {
    type Item = (GenerationalIndex, &'a T);
    type IntoIter = GenerationalIndexArrayIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: 'a> IntoIterator for &'a mut GenerationalIndexArray<T> {
    type Item = (GenerationalIndex, &'a mut T);
    type IntoIter = GenerationalIndexArrayIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> IntoIterator for GenerationalIndexArray<T> {
    type Item = (GenerationalIndex, T);
    type IntoIter = GenerationalIndexArrayIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        GenerationalIndexArrayIntoIter(self.0.into_iter().enumerate())
    }
}

impl<T> FromIterator<(GenerationalIndex, T)> for GenerationalIndexArray<T> {
    fn from_iter<I: IntoIterator<Item = (GenerationalIndex, T)>>(
        iter: I,
    ) -> GenerationalIndexArray<T> {
        let mut map = GenerationalIndexArray::new();
        for (entity, value) in iter {
            map.insert(entity, value);
        }
        map
    }
}