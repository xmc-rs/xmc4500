#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global Control Register"]
    pub gctrl: crate::Reg<gctrl::GCTRL_SPEC>,
    #[doc = "0x04 - Global Status Register"]
    pub gstat: crate::Reg<gstat::GSTAT_SPEC>,
    #[doc = "0x08 - Global Idle Set"]
    pub gidls: crate::Reg<gidls::GIDLS_SPEC>,
    #[doc = "0x0c - Global Idle Clear"]
    pub gidlc: crate::Reg<gidlc::GIDLC_SPEC>,
    #[doc = "0x10 - Global Channel Set"]
    pub gcss: crate::Reg<gcss::GCSS_SPEC>,
    #[doc = "0x14 - Global Channel Clear"]
    pub gcsc: crate::Reg<gcsc::GCSC_SPEC>,
    #[doc = "0x18 - Global Channel Status"]
    pub gcst: crate::Reg<gcst::GCST_SPEC>,
    _reserved7: [u8; 0x34],
    #[doc = "0x50 - Extended Capture Mode Read"]
    pub ecrd: crate::Reg<ecrd::ECRD_SPEC>,
    _reserved8: [u8; 0x2c],
    #[doc = "0x80 - Module Identification"]
    pub midr: crate::Reg<midr::MIDR_SPEC>,
}
#[doc = "GCTRL register accessor: an alias for `Reg<GCTRL_SPEC>`"]
pub type GCTRL = crate::Reg<gctrl::GCTRL_SPEC>;
#[doc = "Global Control Register"]
pub mod gctrl;
#[doc = "GSTAT register accessor: an alias for `Reg<GSTAT_SPEC>`"]
pub type GSTAT = crate::Reg<gstat::GSTAT_SPEC>;
#[doc = "Global Status Register"]
pub mod gstat;
#[doc = "GIDLS register accessor: an alias for `Reg<GIDLS_SPEC>`"]
pub type GIDLS = crate::Reg<gidls::GIDLS_SPEC>;
#[doc = "Global Idle Set"]
pub mod gidls;
#[doc = "GIDLC register accessor: an alias for `Reg<GIDLC_SPEC>`"]
pub type GIDLC = crate::Reg<gidlc::GIDLC_SPEC>;
#[doc = "Global Idle Clear"]
pub mod gidlc;
#[doc = "GCSS register accessor: an alias for `Reg<GCSS_SPEC>`"]
pub type GCSS = crate::Reg<gcss::GCSS_SPEC>;
#[doc = "Global Channel Set"]
pub mod gcss;
#[doc = "GCSC register accessor: an alias for `Reg<GCSC_SPEC>`"]
pub type GCSC = crate::Reg<gcsc::GCSC_SPEC>;
#[doc = "Global Channel Clear"]
pub mod gcsc;
#[doc = "GCST register accessor: an alias for `Reg<GCST_SPEC>`"]
pub type GCST = crate::Reg<gcst::GCST_SPEC>;
#[doc = "Global Channel Status"]
pub mod gcst;
#[doc = "ECRD register accessor: an alias for `Reg<ECRD_SPEC>`"]
pub type ECRD = crate::Reg<ecrd::ECRD_SPEC>;
#[doc = "Extended Capture Mode Read"]
pub mod ecrd;
#[doc = "MIDR register accessor: an alias for `Reg<MIDR_SPEC>`"]
pub type MIDR = crate::Reg<midr::MIDR_SPEC>;
#[doc = "Module Identification"]
pub mod midr;
