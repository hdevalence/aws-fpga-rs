//! Interface to AWS FPGAs (Incomplete, TBD).
//!
//! More specifically, this module provides Rustic interfaces
//! corresponding to the functions in the AWS FPGA SDK's `fpga_pci.c`.
//!
//! The actual FFI definitions are in the `ffi` module; this module
//! wraps those functions in safe Rust wrappers.

use crate::ffi;

bitflags! {
    pub struct AttachFlags: u32 {
        const BURST_CAPABLE = 0x1;
        // Should this be included in the Rust API?
        const FPGA_ATTACH_RESERVED = 0xfffffffe;
    }
}


