#[doc = "Register `CLC` reader"]
pub type R = crate::R<CLC_SPEC>;
#[doc = "Register `CLC` writer"]
pub type W = crate::W<CLC_SPEC>;
#[doc = "Field `DISR` reader - Module Disable Request Bit"]
pub type DISR_R = crate::BitReader;
#[doc = "Field `DISR` writer - Module Disable Request Bit"]
pub type DISR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISS` reader - Module Disable Status Bit"]
pub type DISS_R = crate::BitReader;
#[doc = "Field `EDIS` reader - Sleep Mode Enable Control"]
pub type EDIS_R = crate::BitReader;
#[doc = "Field `EDIS` writer - Sleep Mode Enable Control"]
pub type EDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SBWE` writer - Module Suspend Bit Write Enable for OCDS"]
pub type SBWE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Module Disable Request Bit"]
    #[inline(always)]
    pub fn disr(&self) -> DISR_R {
        DISR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Module Disable Status Bit"]
    #[inline(always)]
    pub fn diss(&self) -> DISS_R {
        DISS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Sleep Mode Enable Control"]
    #[inline(always)]
    pub fn edis(&self) -> EDIS_R {
        EDIS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Module Disable Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn disr(&mut self) -> DISR_W<CLC_SPEC, 0> {
        DISR_W::new(self)
    }
    #[doc = "Bit 3 - Sleep Mode Enable Control"]
    #[inline(always)]
    #[must_use]
    pub fn edis(&mut self) -> EDIS_W<CLC_SPEC, 3> {
        EDIS_W::new(self)
    }
    #[doc = "Bit 4 - Module Suspend Bit Write Enable for OCDS"]
    #[inline(always)]
    #[must_use]
    pub fn sbwe(&mut self) -> SBWE_W<CLC_SPEC, 4> {
        SBWE_W::new(self)
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
#[doc = "CAN Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLC_SPEC;
impl crate::RegisterSpec for CLC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clc::R`](R) reader structure"]
impl crate::Readable for CLC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clc::W`](W) writer structure"]
impl crate::Writable for CLC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLC to value 0x03"]
impl crate::Resettable for CLC_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
