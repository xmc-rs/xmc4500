#[doc = "Register `CMTR` reader"]
pub type R = crate::R<CmtrSpec>;
#[doc = "Register `CMTR` writer"]
pub type W = crate::W<CmtrSpec>;
#[doc = "Field `CTV` reader - Captured Timer Value"]
pub type CtvR = crate::FieldReader<u16>;
#[doc = "Field `CTV` writer - Captured Timer Value"]
pub type CtvW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Captured Timer Value"]
    #[inline(always)]
    pub fn ctv(&self) -> CtvR {
        CtvR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Captured Timer Value"]
    #[inline(always)]
    #[must_use]
    pub fn ctv(&mut self) -> CtvW<CmtrSpec> {
        CtvW::new(self, 0)
    }
}
#[doc = "Capture Mode Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmtrSpec;
impl crate::RegisterSpec for CmtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmtr::R`](R) reader structure"]
impl crate::Readable for CmtrSpec {}
#[doc = "`write(|w| ..)` method takes [`cmtr::W`](W) writer structure"]
impl crate::Writable for CmtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMTR to value 0"]
impl crate::Resettable for CmtrSpec {
    const RESET_VALUE: u32 = 0;
}
