pub trait Memory {
    fn read(&self, address: usize) -> i32;
    fn store(&mut self, address: usize, value: i32); // Invalid memory adresses handlede by VM
    fn size(&self) -> usize; // used by vm for ensuring validity of adresses
}

pub struct FixedMemory<const N: usize> {
    data: [i32; N],
}

pub struct DynamicMemory {
    data: Vec<i32>,
}

impl<const N: usize> FixedMemory<N> {
    pub fn new() -> Self {
        Self { data: [0; N] }
    }
}

impl<const N: usize> Memory for FixedMemory<N> {
    fn read(&self, address: usize) -> i32 {
        self.data[address]
    }

    fn store(&mut self, address: usize, value: i32) {
        self.data[address] = value;
    }

    fn size(&self) -> usize {
        N
    }
}

impl DynamicMemory {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }
}

impl Memory for DynamicMemory {
    fn read(&self, address: usize) -> i32 {
        self.data[address]
    }

    fn store(&mut self, address: usize, value: i32) {
        self.data[address] = value;
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}
