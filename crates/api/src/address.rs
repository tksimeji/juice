#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysicalAddress(usize);

impl PhysicalAddress {
    pub const fn new(value: usize) -> Self {
        Self(value)
    }

    pub const fn as_usize(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtualAddress(usize);

impl VirtualAddress {
    pub const fn new(value: usize) -> Self {
        Self(value)
    }

    pub const fn as_usize(self) -> usize {
        self.0
    }
}
