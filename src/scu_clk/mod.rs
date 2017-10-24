use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Status Register"]
    pub clkstat: CLKSTAT,
    #[doc = "0x04 - CLK Set Register"]
    pub clkset: CLKSET,
    #[doc = "0x08 - CLK Clear Register"]
    pub clkclr: CLKCLR,
    #[doc = "0x0c - System Clock Control Register"]
    pub sysclkcr: SYSCLKCR,
    #[doc = "0x10 - CPU Clock Control Register"]
    pub cpuclkcr: CPUCLKCR,
    #[doc = "0x14 - Peripheral Bus Clock Control Register"]
    pub pbclkcr: PBCLKCR,
    #[doc = "0x18 - USB Clock Control Register"]
    pub usbclkcr: USBCLKCR,
    #[doc = "0x1c - EBU Clock Control Register"]
    pub ebuclkcr: EBUCLKCR,
    #[doc = "0x20 - CCU Clock Control Register"]
    pub ccuclkcr: CCUCLKCR,
    #[doc = "0x24 - WDT Clock Control Register"]
    pub wdtclkcr: WDTCLKCR,
    #[doc = "0x28 - External Clock Control"]
    pub extclkcr: EXTCLKCR,
    _reserved0: [u8; 4usize],
    #[doc = "0x30 - Sleep Control Register"]
    pub sleepcr: SLEEPCR,
    #[doc = "0x34 - Deep Sleep Control Register"]
    pub dsleepcr: DSLEEPCR,
}
#[doc = "Clock Status Register"]
pub struct CLKSTAT {
    register: VolatileCell<u32>,
}
#[doc = "Clock Status Register"]
pub mod clkstat;
#[doc = "CLK Set Register"]
pub struct CLKSET {
    register: VolatileCell<u32>,
}
#[doc = "CLK Set Register"]
pub mod clkset;
#[doc = "CLK Clear Register"]
pub struct CLKCLR {
    register: VolatileCell<u32>,
}
#[doc = "CLK Clear Register"]
pub mod clkclr;
#[doc = "System Clock Control Register"]
pub struct SYSCLKCR {
    register: VolatileCell<u32>,
}
#[doc = "System Clock Control Register"]
pub mod sysclkcr;
#[doc = "CPU Clock Control Register"]
pub struct CPUCLKCR {
    register: VolatileCell<u32>,
}
#[doc = "CPU Clock Control Register"]
pub mod cpuclkcr;
#[doc = "Peripheral Bus Clock Control Register"]
pub struct PBCLKCR {
    register: VolatileCell<u32>,
}
#[doc = "Peripheral Bus Clock Control Register"]
pub mod pbclkcr;
#[doc = "USB Clock Control Register"]
pub struct USBCLKCR {
    register: VolatileCell<u32>,
}
#[doc = "USB Clock Control Register"]
pub mod usbclkcr;
#[doc = "EBU Clock Control Register"]
pub struct EBUCLKCR {
    register: VolatileCell<u32>,
}
#[doc = "EBU Clock Control Register"]
pub mod ebuclkcr;
#[doc = "CCU Clock Control Register"]
pub struct CCUCLKCR {
    register: VolatileCell<u32>,
}
#[doc = "CCU Clock Control Register"]
pub mod ccuclkcr;
#[doc = "WDT Clock Control Register"]
pub struct WDTCLKCR {
    register: VolatileCell<u32>,
}
#[doc = "WDT Clock Control Register"]
pub mod wdtclkcr;
#[doc = "External Clock Control"]
pub struct EXTCLKCR {
    register: VolatileCell<u32>,
}
#[doc = "External Clock Control"]
pub mod extclkcr;
#[doc = "Sleep Control Register"]
pub struct SLEEPCR {
    register: VolatileCell<u32>,
}
#[doc = "Sleep Control Register"]
pub mod sleepcr;
#[doc = "Deep Sleep Control Register"]
pub struct DSLEEPCR {
    register: VolatileCell<u32>,
}
#[doc = "Deep Sleep Control Register"]
pub mod dsleepcr;
