#[doc = "Register `EBUCLKCR` reader"]
pub struct R(crate::R<EBUCLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EBUCLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EBUCLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EBUCLKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EBUCLKCR` writer"]
pub struct W(crate::W<EBUCLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EBUCLKCR_SPEC>;
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
impl From<crate::W<EBUCLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EBUCLKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EBUDIV` reader - EBU Clock Divider Value"]
pub type EBUDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EBUDIV` writer - EBU Clock Divider Value"]
pub type EBUDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EBUCLKCR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - EBU Clock Divider Value"]
    #[inline(always)]
    pub fn ebudiv(&self) -> EBUDIV_R {
        EBUDIV_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - EBU Clock Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn ebudiv(&mut self) -> EBUDIV_W<0> {
        EBUDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBU Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebuclkcr](index.html) module"]
pub struct EBUCLKCR_SPEC;
impl crate::RegisterSpec for EBUCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ebuclkcr::R](R) reader structure"]
impl crate::Readable for EBUCLKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ebuclkcr::W](W) writer structure"]
impl crate::Writable for EBUCLKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EBUCLKCR to value 0"]
impl crate::Resettable for EBUCLKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
