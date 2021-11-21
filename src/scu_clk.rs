#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Status Register"]
    pub clkstat: crate::Reg<clkstat::CLKSTAT_SPEC>,
    #[doc = "0x04 - CLK Set Register"]
    pub clkset: crate::Reg<clkset::CLKSET_SPEC>,
    #[doc = "0x08 - CLK Clear Register"]
    pub clkclr: crate::Reg<clkclr::CLKCLR_SPEC>,
    #[doc = "0x0c - System Clock Control Register"]
    pub sysclkcr: crate::Reg<sysclkcr::SYSCLKCR_SPEC>,
    #[doc = "0x10 - CPU Clock Control Register"]
    pub cpuclkcr: crate::Reg<cpuclkcr::CPUCLKCR_SPEC>,
    #[doc = "0x14 - Peripheral Bus Clock Control Register"]
    pub pbclkcr: crate::Reg<pbclkcr::PBCLKCR_SPEC>,
    #[doc = "0x18 - USB Clock Control Register"]
    pub usbclkcr: crate::Reg<usbclkcr::USBCLKCR_SPEC>,
    #[doc = "0x1c - EBU Clock Control Register"]
    pub ebuclkcr: crate::Reg<ebuclkcr::EBUCLKCR_SPEC>,
    #[doc = "0x20 - CCU Clock Control Register"]
    pub ccuclkcr: crate::Reg<ccuclkcr::CCUCLKCR_SPEC>,
    #[doc = "0x24 - WDT Clock Control Register"]
    pub wdtclkcr: crate::Reg<wdtclkcr::WDTCLKCR_SPEC>,
    #[doc = "0x28 - External Clock Control"]
    pub extclkcr: crate::Reg<extclkcr::EXTCLKCR_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x30 - Sleep Control Register"]
    pub sleepcr: crate::Reg<sleepcr::SLEEPCR_SPEC>,
    #[doc = "0x34 - Deep Sleep Control Register"]
    pub dsleepcr: crate::Reg<dsleepcr::DSLEEPCR_SPEC>,
}
#[doc = "CLKSTAT register accessor: an alias for `Reg<CLKSTAT_SPEC>`"]
pub type CLKSTAT = crate::Reg<clkstat::CLKSTAT_SPEC>;
#[doc = "Clock Status Register"]
pub mod clkstat;
#[doc = "CLKSET register accessor: an alias for `Reg<CLKSET_SPEC>`"]
pub type CLKSET = crate::Reg<clkset::CLKSET_SPEC>;
#[doc = "CLK Set Register"]
pub mod clkset;
#[doc = "CLKCLR register accessor: an alias for `Reg<CLKCLR_SPEC>`"]
pub type CLKCLR = crate::Reg<clkclr::CLKCLR_SPEC>;
#[doc = "CLK Clear Register"]
pub mod clkclr;
#[doc = "SYSCLKCR register accessor: an alias for `Reg<SYSCLKCR_SPEC>`"]
pub type SYSCLKCR = crate::Reg<sysclkcr::SYSCLKCR_SPEC>;
#[doc = "System Clock Control Register"]
pub mod sysclkcr;
#[doc = "CPUCLKCR register accessor: an alias for `Reg<CPUCLKCR_SPEC>`"]
pub type CPUCLKCR = crate::Reg<cpuclkcr::CPUCLKCR_SPEC>;
#[doc = "CPU Clock Control Register"]
pub mod cpuclkcr;
#[doc = "PBCLKCR register accessor: an alias for `Reg<PBCLKCR_SPEC>`"]
pub type PBCLKCR = crate::Reg<pbclkcr::PBCLKCR_SPEC>;
#[doc = "Peripheral Bus Clock Control Register"]
pub mod pbclkcr;
#[doc = "USBCLKCR register accessor: an alias for `Reg<USBCLKCR_SPEC>`"]
pub type USBCLKCR = crate::Reg<usbclkcr::USBCLKCR_SPEC>;
#[doc = "USB Clock Control Register"]
pub mod usbclkcr;
#[doc = "EBUCLKCR register accessor: an alias for `Reg<EBUCLKCR_SPEC>`"]
pub type EBUCLKCR = crate::Reg<ebuclkcr::EBUCLKCR_SPEC>;
#[doc = "EBU Clock Control Register"]
pub mod ebuclkcr;
#[doc = "CCUCLKCR register accessor: an alias for `Reg<CCUCLKCR_SPEC>`"]
pub type CCUCLKCR = crate::Reg<ccuclkcr::CCUCLKCR_SPEC>;
#[doc = "CCU Clock Control Register"]
pub mod ccuclkcr;
#[doc = "WDTCLKCR register accessor: an alias for `Reg<WDTCLKCR_SPEC>`"]
pub type WDTCLKCR = crate::Reg<wdtclkcr::WDTCLKCR_SPEC>;
#[doc = "WDT Clock Control Register"]
pub mod wdtclkcr;
#[doc = "EXTCLKCR register accessor: an alias for `Reg<EXTCLKCR_SPEC>`"]
pub type EXTCLKCR = crate::Reg<extclkcr::EXTCLKCR_SPEC>;
#[doc = "External Clock Control"]
pub mod extclkcr;
#[doc = "SLEEPCR register accessor: an alias for `Reg<SLEEPCR_SPEC>`"]
pub type SLEEPCR = crate::Reg<sleepcr::SLEEPCR_SPEC>;
#[doc = "Sleep Control Register"]
pub mod sleepcr;
#[doc = "DSLEEPCR register accessor: an alias for `Reg<DSLEEPCR_SPEC>`"]
pub type DSLEEPCR = crate::Reg<dsleepcr::DSLEEPCR_SPEC>;
#[doc = "Deep Sleep Control Register"]
pub mod dsleepcr;
