#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clkstat: CLKSTAT,
    clkset: CLKSET,
    clkclr: CLKCLR,
    sysclkcr: SYSCLKCR,
    cpuclkcr: CPUCLKCR,
    pbclkcr: PBCLKCR,
    usbclkcr: USBCLKCR,
    ebuclkcr: EBUCLKCR,
    ccuclkcr: CCUCLKCR,
    wdtclkcr: WDTCLKCR,
    extclkcr: EXTCLKCR,
    _reserved11: [u8; 0x04],
    sleepcr: SLEEPCR,
    dsleepcr: DSLEEPCR,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock Status Register"]
    #[inline(always)]
    pub const fn clkstat(&self) -> &CLKSTAT {
        &self.clkstat
    }
    #[doc = "0x04 - CLK Set Register"]
    #[inline(always)]
    pub const fn clkset(&self) -> &CLKSET {
        &self.clkset
    }
    #[doc = "0x08 - CLK Clear Register"]
    #[inline(always)]
    pub const fn clkclr(&self) -> &CLKCLR {
        &self.clkclr
    }
    #[doc = "0x0c - System Clock Control Register"]
    #[inline(always)]
    pub const fn sysclkcr(&self) -> &SYSCLKCR {
        &self.sysclkcr
    }
    #[doc = "0x10 - CPU Clock Control Register"]
    #[inline(always)]
    pub const fn cpuclkcr(&self) -> &CPUCLKCR {
        &self.cpuclkcr
    }
    #[doc = "0x14 - Peripheral Bus Clock Control Register"]
    #[inline(always)]
    pub const fn pbclkcr(&self) -> &PBCLKCR {
        &self.pbclkcr
    }
    #[doc = "0x18 - USB Clock Control Register"]
    #[inline(always)]
    pub const fn usbclkcr(&self) -> &USBCLKCR {
        &self.usbclkcr
    }
    #[doc = "0x1c - EBU Clock Control Register"]
    #[inline(always)]
    pub const fn ebuclkcr(&self) -> &EBUCLKCR {
        &self.ebuclkcr
    }
    #[doc = "0x20 - CCU Clock Control Register"]
    #[inline(always)]
    pub const fn ccuclkcr(&self) -> &CCUCLKCR {
        &self.ccuclkcr
    }
    #[doc = "0x24 - WDT Clock Control Register"]
    #[inline(always)]
    pub const fn wdtclkcr(&self) -> &WDTCLKCR {
        &self.wdtclkcr
    }
    #[doc = "0x28 - External Clock Control"]
    #[inline(always)]
    pub const fn extclkcr(&self) -> &EXTCLKCR {
        &self.extclkcr
    }
    #[doc = "0x30 - Sleep Control Register"]
    #[inline(always)]
    pub const fn sleepcr(&self) -> &SLEEPCR {
        &self.sleepcr
    }
    #[doc = "0x34 - Deep Sleep Control Register"]
    #[inline(always)]
    pub const fn dsleepcr(&self) -> &DSLEEPCR {
        &self.dsleepcr
    }
}
#[doc = "CLKSTAT (r) register accessor: Clock Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkstat`] module"]
pub type CLKSTAT = crate::Reg<clkstat::CLKSTAT_SPEC>;
#[doc = "Clock Status Register"]
pub mod clkstat;
#[doc = "CLKSET (w) register accessor: CLK Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkset`] module"]
pub type CLKSET = crate::Reg<clkset::CLKSET_SPEC>;
#[doc = "CLK Set Register"]
pub mod clkset;
#[doc = "CLKCLR (w) register accessor: CLK Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkclr`] module"]
pub type CLKCLR = crate::Reg<clkclr::CLKCLR_SPEC>;
#[doc = "CLK Clear Register"]
pub mod clkclr;
#[doc = "SYSCLKCR (rw) register accessor: System Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysclkcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysclkcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysclkcr`] module"]
pub type SYSCLKCR = crate::Reg<sysclkcr::SYSCLKCR_SPEC>;
#[doc = "System Clock Control Register"]
pub mod sysclkcr;
#[doc = "CPUCLKCR (rw) register accessor: CPU Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuclkcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuclkcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuclkcr`] module"]
pub type CPUCLKCR = crate::Reg<cpuclkcr::CPUCLKCR_SPEC>;
#[doc = "CPU Clock Control Register"]
pub mod cpuclkcr;
#[doc = "PBCLKCR (rw) register accessor: Peripheral Bus Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pbclkcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbclkcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbclkcr`] module"]
pub type PBCLKCR = crate::Reg<pbclkcr::PBCLKCR_SPEC>;
#[doc = "Peripheral Bus Clock Control Register"]
pub mod pbclkcr;
#[doc = "USBCLKCR (rw) register accessor: USB Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbclkcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbclkcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbclkcr`] module"]
pub type USBCLKCR = crate::Reg<usbclkcr::USBCLKCR_SPEC>;
#[doc = "USB Clock Control Register"]
pub mod usbclkcr;
#[doc = "EBUCLKCR (rw) register accessor: EBU Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ebuclkcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ebuclkcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ebuclkcr`] module"]
pub type EBUCLKCR = crate::Reg<ebuclkcr::EBUCLKCR_SPEC>;
#[doc = "EBU Clock Control Register"]
pub mod ebuclkcr;
#[doc = "CCUCLKCR (rw) register accessor: CCU Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccuclkcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccuclkcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccuclkcr`] module"]
pub type CCUCLKCR = crate::Reg<ccuclkcr::CCUCLKCR_SPEC>;
#[doc = "CCU Clock Control Register"]
pub mod ccuclkcr;
#[doc = "WDTCLKCR (rw) register accessor: WDT Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtclkcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtclkcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtclkcr`] module"]
pub type WDTCLKCR = crate::Reg<wdtclkcr::WDTCLKCR_SPEC>;
#[doc = "WDT Clock Control Register"]
pub mod wdtclkcr;
#[doc = "EXTCLKCR (rw) register accessor: External Clock Control\n\nYou can [`read`](crate::Reg::read) this register and get [`extclkcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extclkcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extclkcr`] module"]
pub type EXTCLKCR = crate::Reg<extclkcr::EXTCLKCR_SPEC>;
#[doc = "External Clock Control"]
pub mod extclkcr;
#[doc = "SLEEPCR (rw) register accessor: Sleep Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sleepcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleepcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleepcr`] module"]
pub type SLEEPCR = crate::Reg<sleepcr::SLEEPCR_SPEC>;
#[doc = "Sleep Control Register"]
pub mod sleepcr;
#[doc = "DSLEEPCR (rw) register accessor: Deep Sleep Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsleepcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsleepcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsleepcr`] module"]
pub type DSLEEPCR = crate::Reg<dsleepcr::DSLEEPCR_SPEC>;
#[doc = "Deep Sleep Control Register"]
pub mod dsleepcr;
