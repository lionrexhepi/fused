pub trait Heap {
    pub type Guard;
    pub type Header;

    pub fn allocate<T>(value: T);

    pub fn allocate_array<T>(value: [T]);
}

pub trait HeapStoreable {}
