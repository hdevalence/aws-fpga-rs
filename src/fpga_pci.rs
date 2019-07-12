//! Interface to AWS FPGAs (Incomplete, TBD).
//!
//! More specifically, this module provides Rustic interfaces
//! corresponding to the functions in the AWS FPGA SDK's `fpga_pci.c`.
//!
//! The actual FFI definitions are in the `ffi` module; this module
//! wraps those functions in safe Rust wrappers.

use std::convert::TryFrom;

use crate::ffi;

use crate::errors::Error;

bitflags! {
    pub struct AttachFlags: u32 {
        const BURST_CAPABLE = 0x1;
        // Should this be included in the Rust API?
        const FPGA_ATTACH_RESERVED = 0xfffffffe;
    }
}

/// Physical function definitions.
pub enum PhysicalFunction {
    App = 0,
    Management = 1,
}

// these could probably be transmutes but that's a bit icky

impl From<ffi::_bindgen_ty_14> for PhysicalFunction {
    fn from(f: ffi::_bindgen_ty_14) -> PhysicalFunction {
        match f {
            ffi::FPGA_APP_PF => PhysicalFunction::App,
            ffi::FPGA_MGMT_PF => PhysicalFunction::Management,
            _ => panic!("unknown PhysicalFunction type"),
        }
    }
}

impl From<PhysicalFunction> for ffi::_bindgen_ty_14 {
    fn from(f: PhysicalFunction) -> Self {
        match f {
            PhysicalFunction::App => ffi::FPGA_APP_PF,
            PhysicalFunction::Management => ffi::FPGA_MGMT_PF,
        }
    }
}

/// Base Address Registers.
///
/// FIXME: in the AWS SDK there are two enums, `APP_PF_BAR*` and
/// `MGMT_PF_BAR*`, which get passed to the same parameter of the
/// attach function, but have overlapping values, and presumably only
/// one kind is valid depending on the PhysicalFunction parameter?
pub enum BaseAddressRegister {
    AppBar0,
    AppBar1,
    AppBar4,
    ManagementBar0,
    ManagementBar2,
    ManagementBar4,
}

impl From<BaseAddressRegister> for ffi::_bindgen_ty_15 {
    fn from(b: BaseAddressRegister) -> Self {
        match b {
            BaseAddressRegister::AppBar0 => ffi::APP_PF_BAR0,
            BaseAddressRegister::AppBar1 => ffi::APP_PF_BAR1,
            BaseAddressRegister::AppBar4 => ffi::APP_PF_BAR4,
            BaseAddressRegister::ManagementBar0 => ffi::MGMT_PF_BAR0,
            BaseAddressRegister::ManagementBar2 => ffi::MGMT_PF_BAR2,
            BaseAddressRegister::ManagementBar4 => ffi::MGMT_PF_BAR4,
        }
    }
}

/// A logical slot (?)
pub struct Slot(::std::os::raw::c_int);

impl From<i32> for Slot {
    fn from(id: i32) -> Slot {
        Slot(id)
    }
}

/// An FPGA context.
pub struct FpgaContext(ffi::pci_bar_handle_t);

impl FpgaContext {
    pub fn new(
        slot: Slot,
        pf: PhysicalFunction,
        bar: BaseAddressRegister,
        flags: AttachFlags,
    ) -> Result<FpgaContext, Error> {
        unsafe {
            // Is it okay to call this multiple times?
            // in the actual sdk code it does nothing
            ffi::fpga_pci_init();

            let mut handle: ffi::pci_bar_handle_t = 0;
            // fails to compile because bindgen derives different ffi
            // types for the enum constants (u32) vs the parameters of
            // fpga_pci_attach (i32).
            //let e = ffi::fpga_pci_attach(slot.0, pf.into(), bar.into(), flags.bits(), &mut handle);
            let pf_u32: u32 = pf.into();
            let bar_u32: u32 = bar.into();
            let e = ffi::fpga_pci_attach(
                slot.0,
                pf_u32 as i32,
                bar_u32 as i32,
                flags.bits(),
                &mut handle,
            );

            if e == 0 {
                Ok(FpgaContext(handle))
            } else {
                // similarly to above
                Err(Error::AwsSdkError((e as u32).into()))
            }
        }
    }
}
