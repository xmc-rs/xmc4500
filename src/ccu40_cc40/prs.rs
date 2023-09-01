#[doc = "Register `PRS` reader"]
pub type R = crate::R<PRS_SPEC>;
#[doc = "Register `PRS` writer"]
pub type W = crate::W<PRS_SPEC>;
#[doc = "Field `PRS` reader - Period Register"]
pub type PRS_R = crate::FieldReader<u16>;
#[doc = "Field `PRS` writer - Period Register"]
pub type PRS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Period Register"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Period Register"]
    #[inline(always)]
    #[must_use]
    pub fn prs(&mut self) -> PRS_W<PRS_SPEC, 0> {
        PRS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer Shadow Period Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRS_SPEC;
impl crate::RegisterSpec for PRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prs::R`](R) reader structure"]
impl crate::Readable for PRS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prs::W`](W) writer structure"]
impl crate::Writable for PRS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRS to value 0"]
impl crate::Resettable for PRS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
