pub struct GenerationalIndexArrayIter<'a, T: 'a>(
    iter::Enumerate<slice::Iter<'a, Option<ArrayEntry<T>>>>,
);

impl<'a, T: 'a> Iterator for GenerationalIndexArrayIter<'a, T> {
    type Item = (GenerationalIndex, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((index, entry)) = self.0.next() {
            if let &Some(ref entry) = entry {
                return Some((
                    GenerationalIndex {
                        index,
                        generation: entry.generation,
                    },
                    &entry.value,
                ));
            }
        }
        None
    }
}