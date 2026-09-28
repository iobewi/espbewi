//! ESP OTA operations over the process-wide flash owner.
//!
//! The flash mutex is released before the caller accesses ConfigSpace: both
//! backends use the same non-reentrant physical flash capability.

extern crate alloc;

use alloc::boxed::Box;
use embedded_storage::nor_flash::NorFlash;
use espbewi_flash::{EspFlash, SharedFlash};
use fibewi::BackendOutcome;

use crate::{AppPartition, AppSlot, PARTITION_TABLE_BUFFER_SIZE, otadata};

fn table_buffer() -> Box<otadata::TableBuffer> {
    Box::new([0u8; PARTITION_TABLE_BUFFER_SIZE])
}

pub fn write_target_locked(flash: &mut EspFlash) -> Result<AppPartition, otadata::Error> {
    otadata::write_target(flash.storage(), &mut table_buffer())
}

pub async fn write_target(flash: &SharedFlash) -> Result<AppPartition, otadata::Error> {
    let mut guard = flash.lock().await;
    write_target_locked(&mut guard)
}

pub async fn confirm(flash: &SharedFlash) -> Result<(), otadata::Error> {
    let mut guard = flash.lock().await;
    otadata::confirm(guard.storage(), &mut table_buffer())
}

pub async fn reject(flash: &SharedFlash) -> Result<(), otadata::Error> {
    let mut guard = flash.lock().await;
    otadata::reject(guard.storage(), &mut table_buffer())
}

pub async fn activate(flash: &SharedFlash, target: AppSlot) -> Result<(), otadata::Error> {
    let mut guard = flash.lock().await;
    otadata::activate(guard.storage(), &mut table_buffer(), target)
}

/// The partition actually booted, including after an image-header fallback.
pub async fn active_slot(flash: &SharedFlash) -> &'static str {
    let mut guard = flash.lock().await;
    otadata::booted_slot(guard.storage(), &mut table_buffer()).unwrap_or("")
}

pub async fn image_outcome(flash: &SharedFlash) -> BackendOutcome {
    let mut guard = flash.lock().await;
    otadata::read_entries(guard.storage(), &mut table_buffer())
        .map(|entries| otadata::image_outcome(&entries))
        .unwrap_or(BackendOutcome::Other)
}

/// Raw bootloader state for the agent's status endpoint.
pub async fn boot_info(flash: &SharedFlash) -> otadata::BootEntry {
    const UNKNOWN: otadata::BootEntry = otadata::BootEntry {
        slot: "", seq: 0, state: "unknown",
    };
    let mut guard = flash.lock().await;
    otadata::read_entries(guard.storage(), &mut table_buffer())
        .ok()
        .and_then(|entries| otadata::boot_entry(&entries))
        .unwrap_or(UNKNOWN)
}

pub const fn erase_batch_size() -> u64 { 64 * 1024 }

pub const fn erase_size() -> usize { <EspFlash as NorFlash>::ERASE_SIZE }
