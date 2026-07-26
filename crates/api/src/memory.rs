use crate::address::PhysicalAddress;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryRegionKind {
    Usable,
    Reserved,
    Kernel,
    DeviceTree,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemoryRegion {
    pub start: PhysicalAddress,
    pub end: PhysicalAddress,
    pub kind: MemoryRegionKind,
}

impl MemoryRegion {
    pub const fn new(start: PhysicalAddress, end: PhysicalAddress, kind: MemoryRegionKind) -> Self {
        Self { start, end, kind }
    }

    pub const fn size(&self) -> usize {
        self.end.as_usize().saturating_sub(self.start.as_usize())
    }

    pub fn contains(&self, address: PhysicalAddress) -> bool {
        address >= self.start && address < self.end
    }

    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }
}
