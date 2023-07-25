#[derive(Clone)]
struct AllocatorEntry {
    is_live: bool,
    generation: u64,
}