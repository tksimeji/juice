#![no_std]
#![no_main]

use core::arch::asm;
use juice_api::address::PhysicalAddress;
use juice_api::boot::BootContext;
use juice_api::device_tree::DeviceTreeHeader;
use juice_api::memory::{MemoryRegion, MemoryRegionKind};

core::arch::global_asm!(include_str!("boot.s"));

const DEVICE_TREE_ADDRESS: PhysicalAddress = PhysicalAddress::new(0x4000_0000);

unsafe extern "C" {
    fn __kernel_start();
    fn __kernel_end();
}

#[unsafe(no_mangle)]
pub extern "C" fn boot_main() -> ! {
    let device_tree_header = match read_device_tree_header() {
        Ok(header) => header,
        Err(_) => halt(),
    };

    let device_tree_end = match DEVICE_TREE_ADDRESS
        .as_usize()
        .checked_add(device_tree_header.total_size)
    {
        Some(address) => PhysicalAddress::new(address),
        None => halt(),
    };

    let kernel_start = PhysicalAddress::new(__kernel_start as *const () as usize);

    let kernel_end = PhysicalAddress::new(__kernel_end as *const () as usize);

    let memory_regions = [
        MemoryRegion::new(
            DEVICE_TREE_ADDRESS,
            device_tree_end,
            MemoryRegionKind::DeviceTree,
        ),
        MemoryRegion::new(kernel_start, kernel_end, MemoryRegionKind::Kernel),
    ];

    let boot_info = BootContext::new(&memory_regions, DEVICE_TREE_ADDRESS);

    juice_kernel::start(&boot_info);
}

fn read_device_tree_header() -> Result<DeviceTreeHeader, juice_api::device_tree::Error> {
    let bytes = unsafe {
        core::slice::from_raw_parts(
            DEVICE_TREE_ADDRESS.as_usize() as *const u8,
            DeviceTreeHeader::SIZE,
        )
    };

    DeviceTreeHeader::parse(bytes)
}

fn halt() -> ! {
    loop {
        unsafe {
            asm!("wfe");
        }
    }
}
