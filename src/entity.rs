
struct Entity<T> {
    id: u32,
    gen: u32,
    _phantom_data: [T; 0],
}

struct EntityVec<T> {
    data: Vec<(u32, MaybeUninit<T>)>,
}

