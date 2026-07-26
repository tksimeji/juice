use juice_api::boot::BootContext;
use juice_api::memory::{MemoryRegion, MemoryRegionKind};

pub const PAGE_SIZE: usize = 4096;

pub fn first_region(
    boot_context: &BootContext<'_>,
    kind: MemoryRegionKind,
) -> Option<MemoryRegion> {
    boot_context
        .memory_regions
        .iter()
        .find(|region| region.kind == kind)
        .copied()
}

pub fn total_size(boot_context: &BootContext<'_>, kind: MemoryRegionKind) -> usize {
    boot_context
        .memory_regions
        .iter()
        .filter(|region| region.kind == kind)
        .map(MemoryRegion::size)
        .sum()
}

pub fn usable_size(boot_context: &BootContext<'_>) -> usize {
    total_size(boot_context, MemoryRegionKind::Usable)
}
