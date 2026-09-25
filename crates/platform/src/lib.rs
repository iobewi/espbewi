#![no_std]

//! Pure ESP platform descriptors.
//!
//! This crate contains hardware facts only: chip identifiers and memory
//! geometry consumed by boot/image logic. It has no HAL, flash, NVS, OTA,
//! rollback or application dependencies and therefore remains host-testable.

use core::ops::Range;

/// Memory geometry required to validate and load an ESP application image.
#[derive(Clone, Debug)]
pub struct MemoryMap {
    pub chip_id: u16,
    /// Flash-mapped through the MMU.
    pub drom: Range<u32>,
    pub irom: Range<u32>,
    /// Internal SRAM instruction-bus and data-bus aliases.
    pub iram: Range<u32>,
    pub dram: Range<u32>,
    pub rtc: Range<u32>,
    /// `iram - sram_alias_offset == dram`.
    pub sram_alias_offset: u32,
    /// Memory occupied by the second-stage bootloader/ROM while loading.
    pub boot_window: Range<u32>,
    /// MMU page size used for flash-mapped segments.
    pub mmu_page: u32,
}

impl MemoryMap {
    pub fn is_flash_mapped(&self, addr: u32) -> bool {
        self.drom.contains(&addr) || self.irom.contains(&addr)
    }
}

pub mod chips {
    pub mod esp32c3 {
        use crate::MemoryMap;

        /// ESP32-C3 boot memory geometry for the current FiBeWI second-stage
        /// loader RAM window.
        ///
        /// The concrete bootloader linker script must stay consistent with
        /// `boot_window`; espbewi owns both hardware descriptions as the boot
        /// platform is extracted.
        pub const BOOT_MEMORY_MAP: MemoryMap = MemoryMap {
            chip_id: 0x0005,
            drom: 0x3C00_0000..0x3C80_0000,
            irom: 0x4200_0000..0x4280_0000,
            iram: 0x4037_C000..0x403E_0000,
            dram: 0x3FC8_0000..0x3FCE_0000,
            rtc: 0x5000_0000..0x5000_2000,
            sram_alias_offset: 0x0070_0000,
            boot_window: 0x3FCC_B000..0x3FCE_0000,
            mmu_page: 0x1_0000,
        };
    }
}
