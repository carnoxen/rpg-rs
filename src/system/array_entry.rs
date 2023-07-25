#[derive(Clone)]
struct ArrayEntry<T> {
    value: T,
    generation: u64,
}