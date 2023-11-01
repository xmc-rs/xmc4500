#[doc = "Register `HCINTMSK` reader"]
pub type R = crate::R<HCINTMSK_SPEC>;
#[doc = "Register `HCINTMSK` writer"]
pub type W = crate::W<HCINTMSK_SPEC>;
#[doc = "Field `XferComplMsk` reader - Transfer Completed Mask"]
pub type XFER_COMPL_MSK_R = crate::BitReader;
#[doc = "Field `XferComplMsk` writer - Transfer Completed Mask"]
pub type XFER_COMPL_MSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ChHltdMsk` reader - Channel Halted Mask"]
pub type CH_HLTD_MSK_R = crate::BitReader;
#[doc = "Field `ChHltdMsk` writer - Channel Halted Mask"]
pub type CH_HLTD_MSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AHBErrMsk` reader - AHB Error Mask"]
pub type AHBERR_MSK_R = crate::BitReader;
#[doc = "Field `AHBErrMsk` writer - AHB Error Mask"]
pub type AHBERR_MSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `StallMsk` reader - STALL Response Received Interrupt Mask"]
pub type STALL_MSK_R = crate::BitReader;
#[doc = "Field `StallMsk` writer - STALL Response Received Interrupt Mask"]
pub type STALL_MSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NakMsk` reader - NAK Response Received Interrupt Mask"]
pub type NAK_MSK_R = crate::BitReader;
#[doc = "Field `NakMsk` writer - NAK Response Received Interrupt Mask"]
pub type NAK_MSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AckMsk` reader - ACK Response Received/Transmitted Interrupt Mask"]
pub type ACK_MSK_R = crate::BitReader;
#[doc = "Field `AckMsk` writer - ACK Response Received/Transmitted Interrupt Mask"]
pub type ACK_MSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NyetMsk` reader - NYET Response Received Interrupt Mask"]
pub type NYET_MSK_R = crate::BitReader;
#[doc = "Field `NyetMsk` writer - NYET Response Received Interrupt Mask"]
pub type NYET_MSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XactErrMsk` reader - Transaction Error Mask"]
pub type XACT_ERR_MSK_R = crate::BitReader;
#[doc = "Field `XactErrMsk` writer - Transaction Error Mask"]
pub type XACT_ERR_MSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BblErrMsk` reader - Babble Error Mask"]
pub type BBL_ERR_MSK_R = crate::BitReader;
#[doc = "Field `BblErrMsk` writer - Babble Error Mask"]
pub type BBL_ERR_MSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FrmOvrunMsk` reader - Frame Overrun Mask"]
pub type FRM_OVRUN_MSK_R = crate::BitReader;
#[doc = "Field `FrmOvrunMsk` writer - Frame Overrun Mask"]
pub type FRM_OVRUN_MSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DataTglErrMsk` reader - Data Toggle Error Mask"]
pub type DATA_TGL_ERR_MSK_R = crate::BitReader;
#[doc = "Field `DataTglErrMsk` writer - Data Toggle Error Mask"]
pub type DATA_TGL_ERR_MSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BNAIntrMsk` reader - BNA (Buffer Not Available) Interrupt mask register"]
pub type BNAINTR_MSK_R = crate::BitReader;
#[doc = "Field `BNAIntrMsk` writer - BNA (Buffer Not Available) Interrupt mask register"]
pub type BNAINTR_MSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DESC_LST_ROLLIntrMsk` reader - Descriptor rollover interrupt Mask register"]
pub type DESC_LST_ROLLINTR_MSK_R = crate::BitReader;
#[doc = "Field `DESC_LST_ROLLIntrMsk` writer - Descriptor rollover interrupt Mask register"]
pub type DESC_LST_ROLLINTR_MSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Transfer Completed Mask"]
    #[inline(always)]
    pub fn xfer_compl_msk(&self) -> XFER_COMPL_MSK_R {
        XFER_COMPL_MSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Halted Mask"]
    #[inline(always)]
    pub fn ch_hltd_msk(&self) -> CH_HLTD_MSK_R {
        CH_HLTD_MSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    pub fn ahberr_msk(&self) -> AHBERR_MSK_R {
        AHBERR_MSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt Mask"]
    #[inline(always)]
    pub fn stall_msk(&self) -> STALL_MSK_R {
        STALL_MSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt Mask"]
    #[inline(always)]
    pub fn nak_msk(&self) -> NAK_MSK_R {
        NAK_MSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt Mask"]
    #[inline(always)]
    pub fn ack_msk(&self) -> ACK_MSK_R {
        ACK_MSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NYET Response Received Interrupt Mask"]
    #[inline(always)]
    pub fn nyet_msk(&self) -> NYET_MSK_R {
        NYET_MSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction Error Mask"]
    #[inline(always)]
    pub fn xact_err_msk(&self) -> XACT_ERR_MSK_R {
        XACT_ERR_MSK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble Error Mask"]
    #[inline(always)]
    pub fn bbl_err_msk(&self) -> BBL_ERR_MSK_R {
        BBL_ERR_MSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame Overrun Mask"]
    #[inline(always)]
    pub fn frm_ovrun_msk(&self) -> FRM_OVRUN_MSK_R {
        FRM_OVRUN_MSK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Toggle Error Mask"]
    #[inline(always)]
    pub fn data_tgl_err_msk(&self) -> DATA_TGL_ERR_MSK_R {
        DATA_TGL_ERR_MSK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BNA (Buffer Not Available) Interrupt mask register"]
    #[inline(always)]
    pub fn bnaintr_msk(&self) -> BNAINTR_MSK_R {
        BNAINTR_MSK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Descriptor rollover interrupt Mask register"]
    #[inline(always)]
    pub fn desc_lst_rollintr_msk(&self) -> DESC_LST_ROLLINTR_MSK_R {
        DESC_LST_ROLLINTR_MSK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_compl_msk(&mut self) -> XFER_COMPL_MSK_W<HCINTMSK_SPEC, 0> {
        XFER_COMPL_MSK_W::new(self)
    }
    #[doc = "Bit 1 - Channel Halted Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ch_hltd_msk(&mut self) -> CH_HLTD_MSK_W<HCINTMSK_SPEC, 1> {
        CH_HLTD_MSK_W::new(self)
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr_msk(&mut self) -> AHBERR_MSK_W<HCINTMSK_SPEC, 2> {
        AHBERR_MSK_W::new(self)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn stall_msk(&mut self) -> STALL_MSK_W<HCINTMSK_SPEC, 3> {
        STALL_MSK_W::new(self)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nak_msk(&mut self) -> NAK_MSK_W<HCINTMSK_SPEC, 4> {
        NAK_MSK_W::new(self)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ack_msk(&mut self) -> ACK_MSK_W<HCINTMSK_SPEC, 5> {
        ACK_MSK_W::new(self)
    }
    #[doc = "Bit 6 - NYET Response Received Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nyet_msk(&mut self) -> NYET_MSK_W<HCINTMSK_SPEC, 6> {
        NYET_MSK_W::new(self)
    }
    #[doc = "Bit 7 - Transaction Error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn xact_err_msk(&mut self) -> XACT_ERR_MSK_W<HCINTMSK_SPEC, 7> {
        XACT_ERR_MSK_W::new(self)
    }
    #[doc = "Bit 8 - Babble Error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn bbl_err_msk(&mut self) -> BBL_ERR_MSK_W<HCINTMSK_SPEC, 8> {
        BBL_ERR_MSK_W::new(self)
    }
    #[doc = "Bit 9 - Frame Overrun Mask"]
    #[inline(always)]
    #[must_use]
    pub fn frm_ovrun_msk(&mut self) -> FRM_OVRUN_MSK_W<HCINTMSK_SPEC, 9> {
        FRM_OVRUN_MSK_W::new(self)
    }
    #[doc = "Bit 10 - Data Toggle Error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn data_tgl_err_msk(&mut self) -> DATA_TGL_ERR_MSK_W<HCINTMSK_SPEC, 10> {
        DATA_TGL_ERR_MSK_W::new(self)
    }
    #[doc = "Bit 11 - BNA (Buffer Not Available) Interrupt mask register"]
    #[inline(always)]
    #[must_use]
    pub fn bnaintr_msk(&mut self) -> BNAINTR_MSK_W<HCINTMSK_SPEC, 11> {
        BNAINTR_MSK_W::new(self)
    }
    #[doc = "Bit 13 - Descriptor rollover interrupt Mask register"]
    #[inline(always)]
    #[must_use]
    pub fn desc_lst_rollintr_msk(&mut self) -> DESC_LST_ROLLINTR_MSK_W<HCINTMSK_SPEC, 13> {
        DESC_LST_ROLLINTR_MSK_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Host Channel Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCINTMSK_SPEC;
impl crate::RegisterSpec for HCINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcintmsk::R`](R) reader structure"]
impl crate::Readable for HCINTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcintmsk::W`](W) writer structure"]
impl crate::Writable for HCINTMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCINTMSK to value 0"]
impl crate::Resettable for HCINTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
