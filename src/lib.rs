#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate failure;

pub mod errors;
pub mod ffi;
pub mod fpga_pci;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
