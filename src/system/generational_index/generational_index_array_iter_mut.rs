pub struct GenerationalIndexArrayIterMut<'a, T: 'a>(
    iter::Enumerate<slice::IterMut<'a, Option<ArrayEntry<T>>>>,
);

impl<'a, T: 'a> Iterator for GenerationalIndexArrayIterMut<'a, T> {
    type Item = (GenerationalIndex, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((index, entry)) = self.0.next() {
            if let &mut Some(ref mut entry) = entry {
                return Some((
                    GenerationalIndex {
                        index,
                        generation: entry.generation,
                    },
                    &mut entry.value,
                ));
            }
        }
        None
    }
}