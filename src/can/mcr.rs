#[doc = "Register `MCR` reader"]
pub type R = crate::R<MCR_SPEC>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<MCR_SPEC>;
#[doc = "Field `MPSEL` reader - Message Pending Selector"]
pub type MPSEL_R = crate::FieldReader;
#[doc = "Field `MPSEL` writer - Message Pending Selector"]
pub type MPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 12:15 - Message Pending Selector"]
    #[inline(always)]
    pub fn mpsel(&self) -> MPSEL_R {
        MPSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Message Pending Selector"]
    #[inline(always)]
    #[must_use]
    pub fn mpsel(&mut self) -> MPSEL_W<MCR_SPEC> {
        MPSEL_W::new(self, 12)
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
#[doc = "Module Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for MCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
