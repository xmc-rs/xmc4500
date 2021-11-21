#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Message Object Function Control Register"]
    pub mofcr: crate::Reg<mofcr::MOFCR_SPEC>,
    #[doc = "0x04 - Message Object FIFO/Gateway Pointer Register"]
    pub mofgpr: crate::Reg<mofgpr::MOFGPR_SPEC>,
    #[doc = "0x08 - Message Object Interrupt Pointer Register"]
    pub moipr: crate::Reg<moipr::MOIPR_SPEC>,
    #[doc = "0x0c - Message Object Acceptance Mask Register"]
    pub moamr: crate::Reg<moamr::MOAMR_SPEC>,
    #[doc = "0x10 - Message Object Data Register Low"]
    pub modatal: crate::Reg<modatal::MODATAL_SPEC>,
    #[doc = "0x14 - Message Object Data Register High"]
    pub modatah: crate::Reg<modatah::MODATAH_SPEC>,
    #[doc = "0x18 - Message Object Arbitration Register"]
    pub moar: crate::Reg<moar::MOAR_SPEC>,
    _reserved_7_moctr: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x1c - Message Object Status Register"]
    #[inline(always)]
    pub fn mostat(&self) -> &crate::Reg<mostat::MOSTAT_SPEC> {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const crate::Reg<mostat::MOSTAT_SPEC>) }
    }
    #[doc = "0x1c - Message Object Control Register"]
    #[inline(always)]
    pub fn moctr(&self) -> &crate::Reg<moctr::MOCTR_SPEC> {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const crate::Reg<moctr::MOCTR_SPEC>) }
    }
}
#[doc = "MOFCR register accessor: an alias for `Reg<MOFCR_SPEC>`"]
pub type MOFCR = crate::Reg<mofcr::MOFCR_SPEC>;
#[doc = "Message Object Function Control Register"]
pub mod mofcr;
#[doc = "MOFGPR register accessor: an alias for `Reg<MOFGPR_SPEC>`"]
pub type MOFGPR = crate::Reg<mofgpr::MOFGPR_SPEC>;
#[doc = "Message Object FIFO/Gateway Pointer Register"]
pub mod mofgpr;
#[doc = "MOIPR register accessor: an alias for `Reg<MOIPR_SPEC>`"]
pub type MOIPR = crate::Reg<moipr::MOIPR_SPEC>;
#[doc = "Message Object Interrupt Pointer Register"]
pub mod moipr;
#[doc = "MOAMR register accessor: an alias for `Reg<MOAMR_SPEC>`"]
pub type MOAMR = crate::Reg<moamr::MOAMR_SPEC>;
#[doc = "Message Object Acceptance Mask Register"]
pub mod moamr;
#[doc = "MODATAL register accessor: an alias for `Reg<MODATAL_SPEC>`"]
pub type MODATAL = crate::Reg<modatal::MODATAL_SPEC>;
#[doc = "Message Object Data Register Low"]
pub mod modatal;
#[doc = "MODATAH register accessor: an alias for `Reg<MODATAH_SPEC>`"]
pub type MODATAH = crate::Reg<modatah::MODATAH_SPEC>;
#[doc = "Message Object Data Register High"]
pub mod modatah;
#[doc = "MOAR register accessor: an alias for `Reg<MOAR_SPEC>`"]
pub type MOAR = crate::Reg<moar::MOAR_SPEC>;
#[doc = "Message Object Arbitration Register"]
pub mod moar;
#[doc = "MOCTR register accessor: an alias for `Reg<MOCTR_SPEC>`"]
pub type MOCTR = crate::Reg<moctr::MOCTR_SPEC>;
#[doc = "Message Object Control Register"]
pub mod moctr;
#[doc = "MOSTAT register accessor: an alias for `Reg<MOSTAT_SPEC>`"]
pub type MOSTAT = crate::Reg<mostat::MOSTAT_SPEC>;
#[doc = "Message Object Status Register"]
pub mod mostat;
