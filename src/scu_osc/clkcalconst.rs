#[doc = "Register `CLKCALCONST` reader"]
pub type R = crate::R<CLKCALCONST_SPEC>;
#[doc = "Register `CLKCALCONST` writer"]
pub type W = crate::W<CLKCALCONST_SPEC>;
#[doc = "Field `CALIBCONST` reader - Clock Calibration Constant Value"]
pub type CALIBCONST_R = crate::FieldReader;
#[doc = "Field `CALIBCONST` writer - Clock Calibration Constant Value"]
pub type CALIBCONST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Clock Calibration Constant Value"]
    #[inline(always)]
    pub fn calibconst(&self) -> CALIBCONST_R {
        CALIBCONST_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Calibration Constant Value"]
    #[inline(always)]
    #[must_use]
    pub fn calibconst(&mut self) -> CALIBCONST_W<CLKCALCONST_SPEC, 0> {
        CALIBCONST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock Calibration Constant Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcalconst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcalconst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKCALCONST_SPEC;
impl crate::RegisterSpec for CLKCALCONST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkcalconst::R`](R) reader structure"]
impl crate::Readable for CLKCALCONST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkcalconst::W`](W) writer structure"]
impl crate::Writable for CLKCALCONST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKCALCONST to value 0"]
impl crate::Resettable for CLKCALCONST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
