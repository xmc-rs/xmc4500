#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    out: Out,
    omr: Omr,
    _reserved2: [u8; 0x08],
    iocr0: Iocr0,
    iocr4: Iocr4,
    _reserved4: [u8; 0x0c],
    in_: In,
    _reserved5: [u8; 0x18],
    pdr0: Pdr0,
    _reserved6: [u8; 0x1c],
    pdisc: Pdisc,
    _reserved7: [u8; 0x0c],
    pps: Pps,
    hwsel: Hwsel,
}
impl RegisterBlock {
    #[doc = "0x00 - Port 4 Output Register"]
    #[inline(always)]
    pub const fn out(&self) -> &Out {
        &self.out
    }
    #[doc = "0x04 - Port 4 Output Modification Register"]
    #[inline(always)]
    pub const fn omr(&self) -> &Omr {
        &self.omr
    }
    #[doc = "0x10 - Port 4 Input/Output Control Register 0"]
    #[inline(always)]
    pub const fn iocr0(&self) -> &Iocr0 {
        &self.iocr0
    }
    #[doc = "0x14 - Port 4 Input/Output Control Register 4"]
    #[inline(always)]
    pub const fn iocr4(&self) -> &Iocr4 {
        &self.iocr4
    }
    #[doc = "0x24 - Port 4 Input Register"]
    #[inline(always)]
    pub const fn in_(&self) -> &In {
        &self.in_
    }
    #[doc = "0x40 - Port 4 Pad Driver Mode 0 Register"]
    #[inline(always)]
    pub const fn pdr0(&self) -> &Pdr0 {
        &self.pdr0
    }
    #[doc = "0x60 - Port 4 Pin Function Decision Control Register"]
    #[inline(always)]
    pub const fn pdisc(&self) -> &Pdisc {
        &self.pdisc
    }
    #[doc = "0x70 - Port 4 Pin Power Save Register"]
    #[inline(always)]
    pub const fn pps(&self) -> &Pps {
        &self.pps
    }
    #[doc = "0x74 - Port 4 Pin Hardware Select Register"]
    #[inline(always)]
    pub const fn hwsel(&self) -> &Hwsel {
        &self.hwsel
    }
}
#[doc = "OUT (rw) register accessor: Port 4 Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`]
module"]
#[doc(alias = "OUT")]
pub type Out = crate::Reg<out::OutSpec>;
#[doc = "Port 4 Output Register"]
pub mod out;
#[doc = "OMR (w) register accessor: Port 4 Output Modification Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`omr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@omr`]
module"]
#[doc(alias = "OMR")]
pub type Omr = crate::Reg<omr::OmrSpec>;
#[doc = "Port 4 Output Modification Register"]
pub mod omr;
#[doc = "IOCR0 (rw) register accessor: Port 4 Input/Output Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocr0`]
module"]
#[doc(alias = "IOCR0")]
pub type Iocr0 = crate::Reg<iocr0::Iocr0Spec>;
#[doc = "Port 4 Input/Output Control Register 0"]
pub mod iocr0;
#[doc = "IOCR4 (rw) register accessor: Port 4 Input/Output Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocr4`]
module"]
#[doc(alias = "IOCR4")]
pub type Iocr4 = crate::Reg<iocr4::Iocr4Spec>;
#[doc = "Port 4 Input/Output Control Register 4"]
pub mod iocr4;
#[doc = "IN (r) register accessor: Port 4 Input Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`]
module"]
#[doc(alias = "IN")]
pub type In = crate::Reg<in_::InSpec>;
#[doc = "Port 4 Input Register"]
pub mod in_;
#[doc = "PDR0 (rw) register accessor: Port 4 Pad Driver Mode 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdr0`]
module"]
#[doc(alias = "PDR0")]
pub type Pdr0 = crate::Reg<pdr0::Pdr0Spec>;
#[doc = "Port 4 Pad Driver Mode 0 Register"]
pub mod pdr0;
#[doc = "PDISC (r) register accessor: Port 4 Pin Function Decision Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdisc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdisc`]
module"]
#[doc(alias = "PDISC")]
pub type Pdisc = crate::Reg<pdisc::PdiscSpec>;
#[doc = "Port 4 Pin Function Decision Control Register"]
pub mod pdisc;
#[doc = "PPS (rw) register accessor: Port 4 Pin Power Save Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pps::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pps::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pps`]
module"]
#[doc(alias = "PPS")]
pub type Pps = crate::Reg<pps::PpsSpec>;
#[doc = "Port 4 Pin Power Save Register"]
pub mod pps;
#[doc = "HWSEL (rw) register accessor: Port 4 Pin Hardware Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwsel`]
module"]
#[doc(alias = "HWSEL")]
pub type Hwsel = crate::Reg<hwsel::HwselSpec>;
#[doc = "Port 4 Pin Hardware Select Register"]
pub mod hwsel;
