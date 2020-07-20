#[doc = r"Register block"]
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
    _reserved11: [u8; 4usize],
    #[doc = "0x30 - Sleep Control Register"]
    pub sleepcr: SLEEPCR,
    #[doc = "0x34 - Deep Sleep Control Register"]
    pub dsleepcr: DSLEEPCR,
}
#[doc = "Clock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkstat](clkstat) module"]
pub type CLKSTAT = crate::Reg<u32, _CLKSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKSTAT;
#[doc = "`read()` method returns [clkstat::R](clkstat::R) reader structure"]
impl crate::Readable for CLKSTAT {}
#[doc = "Clock Status Register"]
pub mod clkstat;
#[doc = "CLK Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkset](clkset) module"]
pub type CLKSET = crate::Reg<u32, _CLKSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKSET;
#[doc = "`write(|w| ..)` method takes [clkset::W](clkset::W) writer structure"]
impl crate::Writable for CLKSET {}
#[doc = "CLK Set Register"]
pub mod clkset;
#[doc = "CLK Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkclr](clkclr) module"]
pub type CLKCLR = crate::Reg<u32, _CLKCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKCLR;
#[doc = "`write(|w| ..)` method takes [clkclr::W](clkclr::W) writer structure"]
impl crate::Writable for CLKCLR {}
#[doc = "CLK Clear Register"]
pub mod clkclr;
#[doc = "System Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysclkcr](sysclkcr) module"]
pub type SYSCLKCR = crate::Reg<u32, _SYSCLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCLKCR;
#[doc = "`read()` method returns [sysclkcr::R](sysclkcr::R) reader structure"]
impl crate::Readable for SYSCLKCR {}
#[doc = "`write(|w| ..)` method takes [sysclkcr::W](sysclkcr::W) writer structure"]
impl crate::Writable for SYSCLKCR {}
#[doc = "System Clock Control Register"]
pub mod sysclkcr;
#[doc = "CPU Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuclkcr](cpuclkcr) module"]
pub type CPUCLKCR = crate::Reg<u32, _CPUCLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUCLKCR;
#[doc = "`read()` method returns [cpuclkcr::R](cpuclkcr::R) reader structure"]
impl crate::Readable for CPUCLKCR {}
#[doc = "`write(|w| ..)` method takes [cpuclkcr::W](cpuclkcr::W) writer structure"]
impl crate::Writable for CPUCLKCR {}
#[doc = "CPU Clock Control Register"]
pub mod cpuclkcr;
#[doc = "Peripheral Bus Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbclkcr](pbclkcr) module"]
pub type PBCLKCR = crate::Reg<u32, _PBCLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBCLKCR;
#[doc = "`read()` method returns [pbclkcr::R](pbclkcr::R) reader structure"]
impl crate::Readable for PBCLKCR {}
#[doc = "`write(|w| ..)` method takes [pbclkcr::W](pbclkcr::W) writer structure"]
impl crate::Writable for PBCLKCR {}
#[doc = "Peripheral Bus Clock Control Register"]
pub mod pbclkcr;
#[doc = "USB Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbclkcr](usbclkcr) module"]
pub type USBCLKCR = crate::Reg<u32, _USBCLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBCLKCR;
#[doc = "`read()` method returns [usbclkcr::R](usbclkcr::R) reader structure"]
impl crate::Readable for USBCLKCR {}
#[doc = "`write(|w| ..)` method takes [usbclkcr::W](usbclkcr::W) writer structure"]
impl crate::Writable for USBCLKCR {}
#[doc = "USB Clock Control Register"]
pub mod usbclkcr;
#[doc = "EBU Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebuclkcr](ebuclkcr) module"]
pub type EBUCLKCR = crate::Reg<u32, _EBUCLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBUCLKCR;
#[doc = "`read()` method returns [ebuclkcr::R](ebuclkcr::R) reader structure"]
impl crate::Readable for EBUCLKCR {}
#[doc = "`write(|w| ..)` method takes [ebuclkcr::W](ebuclkcr::W) writer structure"]
impl crate::Writable for EBUCLKCR {}
#[doc = "EBU Clock Control Register"]
pub mod ebuclkcr;
#[doc = "CCU Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccuclkcr](ccuclkcr) module"]
pub type CCUCLKCR = crate::Reg<u32, _CCUCLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCUCLKCR;
#[doc = "`read()` method returns [ccuclkcr::R](ccuclkcr::R) reader structure"]
impl crate::Readable for CCUCLKCR {}
#[doc = "`write(|w| ..)` method takes [ccuclkcr::W](ccuclkcr::W) writer structure"]
impl crate::Writable for CCUCLKCR {}
#[doc = "CCU Clock Control Register"]
pub mod ccuclkcr;
#[doc = "WDT Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtclkcr](wdtclkcr) module"]
pub type WDTCLKCR = crate::Reg<u32, _WDTCLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCLKCR;
#[doc = "`read()` method returns [wdtclkcr::R](wdtclkcr::R) reader structure"]
impl crate::Readable for WDTCLKCR {}
#[doc = "`write(|w| ..)` method takes [wdtclkcr::W](wdtclkcr::W) writer structure"]
impl crate::Writable for WDTCLKCR {}
#[doc = "WDT Clock Control Register"]
pub mod wdtclkcr;
#[doc = "External Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extclkcr](extclkcr) module"]
pub type EXTCLKCR = crate::Reg<u32, _EXTCLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTCLKCR;
#[doc = "`read()` method returns [extclkcr::R](extclkcr::R) reader structure"]
impl crate::Readable for EXTCLKCR {}
#[doc = "`write(|w| ..)` method takes [extclkcr::W](extclkcr::W) writer structure"]
impl crate::Writable for EXTCLKCR {}
#[doc = "External Clock Control"]
pub mod extclkcr;
#[doc = "Sleep Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleepcr](sleepcr) module"]
pub type SLEEPCR = crate::Reg<u32, _SLEEPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLEEPCR;
#[doc = "`read()` method returns [sleepcr::R](sleepcr::R) reader structure"]
impl crate::Readable for SLEEPCR {}
#[doc = "`write(|w| ..)` method takes [sleepcr::W](sleepcr::W) writer structure"]
impl crate::Writable for SLEEPCR {}
#[doc = "Sleep Control Register"]
pub mod sleepcr;
#[doc = "Deep Sleep Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsleepcr](dsleepcr) module"]
pub type DSLEEPCR = crate::Reg<u32, _DSLEEPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSLEEPCR;
#[doc = "`read()` method returns [dsleepcr::R](dsleepcr::R) reader structure"]
impl crate::Readable for DSLEEPCR {}
#[doc = "`write(|w| ..)` method takes [dsleepcr::W](dsleepcr::W) writer structure"]
impl crate::Writable for DSLEEPCR {}
#[doc = "Deep Sleep Control Register"]
pub mod dsleepcr;
