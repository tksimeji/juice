use crate::address::PhysicalAddress;
use crate::memory::MemoryRegion;

pub struct BootContext<'a> {
    pub memory_regions: &'a [MemoryRegion],
    pub device_tree: PhysicalAddress,
}

impl<'a> BootContext<'a> {
    pub const fn new(memory_regions: &'a [MemoryRegion], device_tree: PhysicalAddress) -> Self {
        Self {
            memory_regions,
            device_tree,
        }
    }
}
