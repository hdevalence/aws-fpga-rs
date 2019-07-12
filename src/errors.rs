//! Error types for the FPGA SDK.

use crate::ffi;

/// Represents an error from this library.
#[derive(Fail, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    #[fail(display = "Invalid BAR parameter")]
    InvalidBar,
    #[fail(display = "Internal SDK error: {}", _0)]
    AwsSdkError(SdkError),
}

impl From<SdkError> for Error {
    fn from(e: SdkError) -> Error {
        Error::AwsSdkError(e)
    }
}

/// Represents an error returned by the AWS FPGA SDK.
///
/// Error messages are copied from the SDK header files.
#[derive(Fail, Clone, Debug, Eq, PartialEq)]
pub enum SdkError {
    #[fail(display = "AFI command is in-progress (busy)")]
    AfiCommandBusy = 3,
    #[fail(display = "Invalid AFI ID")]
    AfiIdInvalid = 5,
    #[fail(display = "Invalid AFI_CMD_API_VERSION, see afi_cmd_api.h")]
    AfiCommandApiVersionInvalid = 11,
    #[fail(display = "CL PCI IDs did not match (e.g. between LF and CL reported values")]
    ClIdMismatch = 12,
    #[fail(display = "CL DDR calibration failed")]
    ClDdrCalibrationFailed = 13,
    #[fail(display = "generic/unspecified error")]
    UnspecifiedError = 14,
    #[fail(display = "Not documented in AWS FPGA SDK")]
    ShellMismatch = 16,
    #[fail(display = "Not documented in AWS FPGA SDK")]
    PowerViolation = 17,
    #[fail(
        display = "In some cases it is possible to detect when data retention is not possible. This prevents the loss of data when retention cannot work."
    )]
    DramDataRetentionNotPossible = 18,
    #[fail(display = "Not documented in AWS FPGA SDK")]
    HardwareBusy = 19,
    #[fail(display = "Unable to locate PCI devices/resources")]
    PciMissing = 20,
    #[fail(display = "Not documented in AWS FPGA SDK")]
    AfiCommandMalformed = 21,
    #[fail(
        display = "Data retention was attempted, but failed and data was lost. All efforts are made to avoid this condition."
    )]
    DramDataRetentionFailed = 22,
    #[fail(
        display = "Saving DDR control calibration failed and data retention will not be possible."
    )]
    DramDataRetentionSetupFailed = 23,
    #[fail(
        display = "This error indicates a bug or unhandled external condition in the software. Report occurrences on github."
    )]
    SoftwareProblem = 24,
    #[fail(display = "Cannot communicate with the FPGA")]
    Unresponsive = 25,
}

impl From<ffi::_bindgen_ty_12> for SdkError {
    fn from(e: ffi::_bindgen_ty_12) -> SdkError {
        match e {
            ffi::FPGA_ERR_AFI_CMD_BUSY => SdkError::AfiCommandBusy,
            ffi::FPGA_ERR_AFI_ID_INVALID => SdkError::AfiIdInvalid,
            ffi::FPGA_ERR_AFI_CMD_API_VERSION_INVALID => SdkError::AfiCommandApiVersionInvalid,
            ffi::FPGA_ERR_CL_ID_MISMATCH => SdkError::ClIdMismatch,
            ffi::FPGA_ERR_CL_DDR_CALIB_FAILED => SdkError::ClDdrCalibrationFailed,
            ffi::FPGA_ERR_FAIL => SdkError::UnspecifiedError,
            ffi::FPGA_ERR_SHELL_MISMATCH => SdkError::ShellMismatch,
            ffi::FPGA_ERR_POWER_VIOLATION => SdkError::PowerViolation,
            ffi::FPGA_ERR_DRAM_DATA_RETENTION_NOT_POSSIBLE => {
                SdkError::DramDataRetentionNotPossible
            }
            ffi::FPGA_ERR_HARDWARE_BUSY => SdkError::HardwareBusy,
            ffi::FPGA_ERR_PCI_MISSING => SdkError::PciMissing,
            ffi::FPGA_ERR_AFI_CMD_MALFORMED => SdkError::AfiCommandMalformed,
            ffi::FPGA_ERR_DRAM_DATA_RETENTION_FAILED => SdkError::DramDataRetentionFailed,
            ffi::FPGA_ERR_DRAM_DATA_RETENTION_SETUP_FAILED => {
                SdkError::DramDataRetentionSetupFailed
            }
            ffi::FPGA_ERR_SOFTWARE_PROBLEM => SdkError::SoftwareProblem,
            ffi::FPGA_ERR_UNRESPONSIVE => SdkError::Unresponsive,
            _ => panic!("unknown error code"),
        }
    }
}
