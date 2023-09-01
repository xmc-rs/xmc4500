#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Hibernate Domain Status Register"]
    pub hdstat: HDSTAT,
    #[doc = "0x04 - Hibernate Domain Status Clear Register"]
    pub hdclr: HDCLR,
    #[doc = "0x08 - Hibernate Domain Status Set Register"]
    pub hdset: HDSET,
    #[doc = "0x0c - Hibernate Domain Control Register"]
    pub hdcr: HDCR,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - fOSI Control Register"]
    pub oscsictrl: OSCSICTRL,
    #[doc = "0x18 - OSC_ULP Status Register"]
    pub osculstat: OSCULSTAT,
    #[doc = "0x1c - OSC_ULP Control Register"]
    pub osculctrl: OSCULCTRL,
}
#[doc = "HDSTAT (r) register accessor: Hibernate Domain Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hdstat`]
module"]
pub type HDSTAT = crate::Reg<hdstat::HDSTAT_SPEC>;
#[doc = "Hibernate Domain Status Register"]
pub mod hdstat;
#[doc = "HDCLR (w) register accessor: Hibernate Domain Status Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hdclr`]
module"]
pub type HDCLR = crate::Reg<hdclr::HDCLR_SPEC>;
#[doc = "Hibernate Domain Status Clear Register"]
pub mod hdclr;
#[doc = "HDSET (w) register accessor: Hibernate Domain Status Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hdset`]
module"]
pub type HDSET = crate::Reg<hdset::HDSET_SPEC>;
#[doc = "Hibernate Domain Status Set Register"]
pub mod hdset;
#[doc = "HDCR (rw) register accessor: Hibernate Domain Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hdcr`]
module"]
pub type HDCR = crate::Reg<hdcr::HDCR_SPEC>;
#[doc = "Hibernate Domain Control Register"]
pub mod hdcr;
#[doc = "OSCSICTRL (rw) register accessor: fOSI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oscsictrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscsictrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`oscsictrl`]
module"]
pub type OSCSICTRL = crate::Reg<oscsictrl::OSCSICTRL_SPEC>;
#[doc = "fOSI Control Register"]
pub mod oscsictrl;
#[doc = "OSCULSTAT (r) register accessor: OSC_ULP Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osculstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`osculstat`]
module"]
pub type OSCULSTAT = crate::Reg<osculstat::OSCULSTAT_SPEC>;
#[doc = "OSC_ULP Status Register"]
pub mod osculstat;
#[doc = "OSCULCTRL (rw) register accessor: OSC_ULP Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osculctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osculctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`osculctrl`]
module"]
pub type OSCULCTRL = crate::Reg<osculctrl::OSCULCTRL_SPEC>;
#[doc = "OSC_ULP Control Register"]
pub mod osculctrl;
