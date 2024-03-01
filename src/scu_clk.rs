#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clkstat: Clkstat,
    clkset: Clkset,
    clkclr: Clkclr,
    sysclkcr: Sysclkcr,
    cpuclkcr: Cpuclkcr,
    pbclkcr: Pbclkcr,
    usbclkcr: Usbclkcr,
    ebuclkcr: Ebuclkcr,
    ccuclkcr: Ccuclkcr,
    wdtclkcr: Wdtclkcr,
    extclkcr: Extclkcr,
    _reserved11: [u8; 0x04],
    sleepcr: Sleepcr,
    dsleepcr: Dsleepcr,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock Status Register"]
    #[inline(always)]
    pub const fn clkstat(&self) -> &Clkstat {
        &self.clkstat
    }
    #[doc = "0x04 - CLK Set Register"]
    #[inline(always)]
    pub const fn clkset(&self) -> &Clkset {
        &self.clkset
    }
    #[doc = "0x08 - CLK Clear Register"]
    #[inline(always)]
    pub const fn clkclr(&self) -> &Clkclr {
        &self.clkclr
    }
    #[doc = "0x0c - System Clock Control Register"]
    #[inline(always)]
    pub const fn sysclkcr(&self) -> &Sysclkcr {
        &self.sysclkcr
    }
    #[doc = "0x10 - CPU Clock Control Register"]
    #[inline(always)]
    pub const fn cpuclkcr(&self) -> &Cpuclkcr {
        &self.cpuclkcr
    }
    #[doc = "0x14 - Peripheral Bus Clock Control Register"]
    #[inline(always)]
    pub const fn pbclkcr(&self) -> &Pbclkcr {
        &self.pbclkcr
    }
    #[doc = "0x18 - USB Clock Control Register"]
    #[inline(always)]
    pub const fn usbclkcr(&self) -> &Usbclkcr {
        &self.usbclkcr
    }
    #[doc = "0x1c - EBU Clock Control Register"]
    #[inline(always)]
    pub const fn ebuclkcr(&self) -> &Ebuclkcr {
        &self.ebuclkcr
    }
    #[doc = "0x20 - CCU Clock Control Register"]
    #[inline(always)]
    pub const fn ccuclkcr(&self) -> &Ccuclkcr {
        &self.ccuclkcr
    }
    #[doc = "0x24 - WDT Clock Control Register"]
    #[inline(always)]
    pub const fn wdtclkcr(&self) -> &Wdtclkcr {
        &self.wdtclkcr
    }
    #[doc = "0x28 - External Clock Control"]
    #[inline(always)]
    pub const fn extclkcr(&self) -> &Extclkcr {
        &self.extclkcr
    }
    #[doc = "0x30 - Sleep Control Register"]
    #[inline(always)]
    pub const fn sleepcr(&self) -> &Sleepcr {
        &self.sleepcr
    }
    #[doc = "0x34 - Deep Sleep Control Register"]
    #[inline(always)]
    pub const fn dsleepcr(&self) -> &Dsleepcr {
        &self.dsleepcr
    }
}
#[doc = "CLKSTAT (r) register accessor: Clock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkstat`]
module"]
#[doc(alias = "CLKSTAT")]
pub type Clkstat = crate::Reg<clkstat::ClkstatSpec>;
#[doc = "Clock Status Register"]
pub mod clkstat;
#[doc = "CLKSET (w) register accessor: CLK Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkset`]
module"]
#[doc(alias = "CLKSET")]
pub type Clkset = crate::Reg<clkset::ClksetSpec>;
#[doc = "CLK Set Register"]
pub mod clkset;
#[doc = "CLKCLR (w) register accessor: CLK Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkclr`]
module"]
#[doc(alias = "CLKCLR")]
pub type Clkclr = crate::Reg<clkclr::ClkclrSpec>;
#[doc = "CLK Clear Register"]
pub mod clkclr;
#[doc = "SYSCLKCR (rw) register accessor: System Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysclkcr`]
module"]
#[doc(alias = "SYSCLKCR")]
pub type Sysclkcr = crate::Reg<sysclkcr::SysclkcrSpec>;
#[doc = "System Clock Control Register"]
pub mod sysclkcr;
#[doc = "CPUCLKCR (rw) register accessor: CPU Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuclkcr`]
module"]
#[doc(alias = "CPUCLKCR")]
pub type Cpuclkcr = crate::Reg<cpuclkcr::CpuclkcrSpec>;
#[doc = "CPU Clock Control Register"]
pub mod cpuclkcr;
#[doc = "PBCLKCR (rw) register accessor: Peripheral Bus Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pbclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pbclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbclkcr`]
module"]
#[doc(alias = "PBCLKCR")]
pub type Pbclkcr = crate::Reg<pbclkcr::PbclkcrSpec>;
#[doc = "Peripheral Bus Clock Control Register"]
pub mod pbclkcr;
#[doc = "USBCLKCR (rw) register accessor: USB Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbclkcr`]
module"]
#[doc(alias = "USBCLKCR")]
pub type Usbclkcr = crate::Reg<usbclkcr::UsbclkcrSpec>;
#[doc = "USB Clock Control Register"]
pub mod usbclkcr;
#[doc = "EBUCLKCR (rw) register accessor: EBU Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ebuclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ebuclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ebuclkcr`]
module"]
#[doc(alias = "EBUCLKCR")]
pub type Ebuclkcr = crate::Reg<ebuclkcr::EbuclkcrSpec>;
#[doc = "EBU Clock Control Register"]
pub mod ebuclkcr;
#[doc = "CCUCLKCR (rw) register accessor: CCU Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccuclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccuclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccuclkcr`]
module"]
#[doc(alias = "CCUCLKCR")]
pub type Ccuclkcr = crate::Reg<ccuclkcr::CcuclkcrSpec>;
#[doc = "CCU Clock Control Register"]
pub mod ccuclkcr;
#[doc = "WDTCLKCR (rw) register accessor: WDT Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtclkcr`]
module"]
#[doc(alias = "WDTCLKCR")]
pub type Wdtclkcr = crate::Reg<wdtclkcr::WdtclkcrSpec>;
#[doc = "WDT Clock Control Register"]
pub mod wdtclkcr;
#[doc = "EXTCLKCR (rw) register accessor: External Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extclkcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extclkcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extclkcr`]
module"]
#[doc(alias = "EXTCLKCR")]
pub type Extclkcr = crate::Reg<extclkcr::ExtclkcrSpec>;
#[doc = "External Clock Control"]
pub mod extclkcr;
#[doc = "SLEEPCR (rw) register accessor: Sleep Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sleepcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleepcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleepcr`]
module"]
#[doc(alias = "SLEEPCR")]
pub type Sleepcr = crate::Reg<sleepcr::SleepcrSpec>;
#[doc = "Sleep Control Register"]
pub mod sleepcr;
#[doc = "DSLEEPCR (rw) register accessor: Deep Sleep Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsleepcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsleepcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsleepcr`]
module"]
#[doc(alias = "DSLEEPCR")]
pub type Dsleepcr = crate::Reg<dsleepcr::DsleepcrSpec>;
#[doc = "Deep Sleep Control Register"]
pub mod dsleepcr;
