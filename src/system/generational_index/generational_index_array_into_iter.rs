pub struct GenerationalIndexArrayIntoIter<T>(
    iter::Enumerate<vec::IntoIter<Option<ArrayEntry<T>>>>
);

impl<T> Iterator for GenerationalIndexArrayIntoIter<T> {
    type Item = (GenerationalIndex, T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((index, entry)) = self.0.next() {
            if let Some(entry) = entry {
                return Some((
                    GenerationalIndex {
                        index,
                        generation: entry.generation,
                    },
                    entry.value,
                ));
            }
        }
        None
    }
}
