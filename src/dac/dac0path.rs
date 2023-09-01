#[doc = "Register `DAC0PATH` reader"]
pub type R = crate::R<DAC0PATH_SPEC>;
#[doc = "Register `DAC0PATH` writer"]
pub type W = crate::W<DAC0PATH_SPEC>;
#[doc = "Field `PAT6` reader - Pattern Number 6 for PATGEN of DAC0"]
pub type PAT6_R = crate::FieldReader;
#[doc = "Field `PAT6` writer - Pattern Number 6 for PATGEN of DAC0"]
pub type PAT6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `PAT7` reader - Pattern Number 7 for PATGEN of DAC0"]
pub type PAT7_R = crate::FieldReader;
#[doc = "Field `PAT7` writer - Pattern Number 7 for PATGEN of DAC0"]
pub type PAT7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `PAT8` reader - Pattern Number 8 for PATGEN of DAC0"]
pub type PAT8_R = crate::FieldReader;
#[doc = "Field `PAT8` writer - Pattern Number 8 for PATGEN of DAC0"]
pub type PAT8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Pattern Number 6 for PATGEN of DAC0"]
    #[inline(always)]
    pub fn pat6(&self) -> PAT6_R {
        PAT6_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Pattern Number 7 for PATGEN of DAC0"]
    #[inline(always)]
    pub fn pat7(&self) -> PAT7_R {
        PAT7_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Pattern Number 8 for PATGEN of DAC0"]
    #[inline(always)]
    pub fn pat8(&self) -> PAT8_R {
        PAT8_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pattern Number 6 for PATGEN of DAC0"]
    #[inline(always)]
    #[must_use]
    pub fn pat6(&mut self) -> PAT6_W<DAC0PATH_SPEC, 0> {
        PAT6_W::new(self)
    }
    #[doc = "Bits 5:9 - Pattern Number 7 for PATGEN of DAC0"]
    #[inline(always)]
    #[must_use]
    pub fn pat7(&mut self) -> PAT7_W<DAC0PATH_SPEC, 5> {
        PAT7_W::new(self)
    }
    #[doc = "Bits 10:14 - Pattern Number 8 for PATGEN of DAC0"]
    #[inline(always)]
    #[must_use]
    pub fn pat8(&mut self) -> PAT8_W<DAC0PATH_SPEC, 10> {
        PAT8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DAC0 Higher Pattern Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0path::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0path::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC0PATH_SPEC;
impl crate::RegisterSpec for DAC0PATH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac0path::R`](R) reader structure"]
impl crate::Readable for DAC0PATH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dac0path::W`](W) writer structure"]
impl crate::Writable for DAC0PATH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC0PATH to value 0x7fdd"]
impl crate::Resettable for DAC0PATH_SPEC {
    const RESET_VALUE: Self::Ux = 0x7fdd;
}
