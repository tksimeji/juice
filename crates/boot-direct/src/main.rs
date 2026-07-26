#![no_std]
#![no_main]

use core::arch::asm;
use fdt::Fdt;
use juice_api::address::PhysicalAddress;
use juice_api::boot::BootContext;
use juice_api::memory::{MemoryRegion, MemoryRegionKind};

core::arch::global_asm!(include_str!("boot.s"));

const DEVICE_TREE_ADDRESS: PhysicalAddress = PhysicalAddress::new(0x4000_0000);

unsafe extern "C" {
    fn __kernel_start();
    fn __kernel_end();
}

#[unsafe(no_mangle)]
pub extern "C" fn boot_main() -> ! {
    let device_tree = match unsafe { Fdt::from_ptr(DEVICE_TREE_ADDRESS.as_usize() as *const u8) } {
        Ok(device_tree) => device_tree,
        Err(_) => halt(),
    };

    let ram = match device_tree.memory().regions().next() {
        Some(region) => region,
        None => halt(),
    };

    let ram_size = match ram.size {
        Some(size) => size,
        None => halt(),
    };

    let ram_start = PhysicalAddress::new(ram.starting_address as usize);

    let ram_end = match ram_start.as_usize().checked_add(ram_size) {
        Some(address) => PhysicalAddress::new(address),
        None => halt(),
    };

    let device_tree_end = match DEVICE_TREE_ADDRESS
        .as_usize()
        .checked_add(device_tree.total_size())
    {
        Some(address) => PhysicalAddress::new(address),
        None => halt(),
    };

    let kernel_start = PhysicalAddress::new(__kernel_start as *const () as usize);

    let kernel_end = PhysicalAddress::new(__kernel_end as *const () as usize);

    if DEVICE_TREE_ADDRESS < ram_start
        || device_tree_end > kernel_start
        || kernel_start < ram_start
        || kernel_end > ram_end
    {
        halt();
    }

    let memory_regions = [
        MemoryRegion::new(
            DEVICE_TREE_ADDRESS,
            device_tree_end,
            MemoryRegionKind::DeviceTree,
        ),
        MemoryRegion::new(device_tree_end, kernel_start, MemoryRegionKind::Usable),
        MemoryRegion::new(kernel_start, kernel_end, MemoryRegionKind::Kernel),
        MemoryRegion::new(kernel_end, ram_end, MemoryRegionKind::Usable),
    ];

    let boot_context = BootContext::new(&memory_regions, DEVICE_TREE_ADDRESS);

    juice_kernel::start(&boot_context);
}

fn halt() -> ! {
    loop {
        unsafe {
            asm!("wfe");
        }
    }
}
