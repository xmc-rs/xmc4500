#[doc = "Register `FDR` reader"]
pub struct R(crate::R<FDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDR` writer"]
pub struct W(crate::W<FDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STEP` reader - Step Value"]
pub type STEP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STEP` writer - Step Value"]
pub type STEP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDR_SPEC, u16, u16, 10, O>;
#[doc = "Field `SM` reader - Suspend Mode"]
pub type SM_R = crate::BitReader<bool>;
#[doc = "Field `SM` writer - Suspend Mode"]
pub type SM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDR_SPEC, bool, O>;
#[doc = "Field `SC` reader - Suspend Control"]
pub type SC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SC` writer - Suspend Control"]
pub type SC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DM` reader - Divider Mode"]
pub type DM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DM` writer - Divider Mode"]
pub type DM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESULT` reader - Result Value"]
pub type RESULT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SUSACK` reader - Suspend Mode Acknowledge"]
pub type SUSACK_R = crate::BitReader<bool>;
#[doc = "Field `SUSREQ` reader - Suspend Mode Request"]
pub type SUSREQ_R = crate::BitReader<bool>;
#[doc = "Field `ENHW` reader - Enable Hardware Clock Control"]
pub type ENHW_R = crate::BitReader<bool>;
#[doc = "Field `ENHW` writer - Enable Hardware Clock Control"]
pub type ENHW_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDR_SPEC, bool, O>;
#[doc = "Field `DISCLK` reader - Disable Clock"]
pub type DISCLK_R = crate::BitReader<bool>;
#[doc = "Field `DISCLK` writer - Disable Clock"]
pub type DISCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - Step Value"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 11 - Suspend Mode"]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Suspend Control"]
    #[inline(always)]
    pub fn sc(&self) -> SC_R {
        SC_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Divider Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:25 - Result Value"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 28 - Suspend Mode Acknowledge"]
    #[inline(always)]
    pub fn susack(&self) -> SUSACK_R {
        SUSACK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Suspend Mode Request"]
    #[inline(always)]
    pub fn susreq(&self) -> SUSREQ_R {
        SUSREQ_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Hardware Clock Control"]
    #[inline(always)]
    pub fn enhw(&self) -> ENHW_R {
        ENHW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Disable Clock"]
    #[inline(always)]
    pub fn disclk(&self) -> DISCLK_R {
        DISCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Step Value"]
    #[inline(always)]
    #[must_use]
    pub fn step(&mut self) -> STEP_W<0> {
        STEP_W::new(self)
    }
    #[doc = "Bit 11 - Suspend Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sm(&mut self) -> SM_W<11> {
        SM_W::new(self)
    }
    #[doc = "Bits 12:13 - Suspend Control"]
    #[inline(always)]
    #[must_use]
    pub fn sc(&mut self) -> SC_W<12> {
        SC_W::new(self)
    }
    #[doc = "Bits 14:15 - Divider Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<14> {
        DM_W::new(self)
    }
    #[doc = "Bit 30 - Enable Hardware Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn enhw(&mut self) -> ENHW_W<30> {
        ENHW_W::new(self)
    }
    #[doc = "Bit 31 - Disable Clock"]
    #[inline(always)]
    #[must_use]
    pub fn disclk(&mut self) -> DISCLK_W<31> {
        DISCLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Fractional Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdr](index.html) module"]
pub struct FDR_SPEC;
impl crate::RegisterSpec for FDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdr::R](R) reader structure"]
impl crate::Readable for FDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdr::W](W) writer structure"]
impl crate::Writable for FDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDR to value 0"]
impl crate::Resettable for FDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
