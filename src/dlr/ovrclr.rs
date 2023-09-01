#[doc = "Register `OVRCLR` writer"]
pub type W = crate::W<OVRCLR_SPEC>;
#[doc = "Field `LN0` writer - Line 0 Overrun Status Clear"]
pub type LN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LN1` writer - Line 1 Overrun Status Clear"]
pub type LN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LN2` writer - Line 2 Overrun Status Clear"]
pub type LN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LN3` writer - Line 3 Overrun Status Clear"]
pub type LN3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LN4` writer - Line 4 Overrun Status Clear"]
pub type LN4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LN5` writer - Line 5 Overrun Status Clear"]
pub type LN5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LN6` writer - Line 6 Overrun Status Clear"]
pub type LN6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LN7` writer - Line 7 Overrun Status Clear"]
pub type LN7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LN8` writer - Line 8 Overrun Status Clear"]
pub type LN8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LN9` writer - Line 9 Overrun Status Clear"]
pub type LN9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LN10` writer - Line 10 Overrun Status Clear"]
pub type LN10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LN11` writer - Line 11 Overrun Status Clear"]
pub type LN11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Line 0 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln0(&mut self) -> LN0_W<OVRCLR_SPEC, 0> {
        LN0_W::new(self)
    }
    #[doc = "Bit 1 - Line 1 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln1(&mut self) -> LN1_W<OVRCLR_SPEC, 1> {
        LN1_W::new(self)
    }
    #[doc = "Bit 2 - Line 2 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln2(&mut self) -> LN2_W<OVRCLR_SPEC, 2> {
        LN2_W::new(self)
    }
    #[doc = "Bit 3 - Line 3 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln3(&mut self) -> LN3_W<OVRCLR_SPEC, 3> {
        LN3_W::new(self)
    }
    #[doc = "Bit 4 - Line 4 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln4(&mut self) -> LN4_W<OVRCLR_SPEC, 4> {
        LN4_W::new(self)
    }
    #[doc = "Bit 5 - Line 5 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln5(&mut self) -> LN5_W<OVRCLR_SPEC, 5> {
        LN5_W::new(self)
    }
    #[doc = "Bit 6 - Line 6 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln6(&mut self) -> LN6_W<OVRCLR_SPEC, 6> {
        LN6_W::new(self)
    }
    #[doc = "Bit 7 - Line 7 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln7(&mut self) -> LN7_W<OVRCLR_SPEC, 7> {
        LN7_W::new(self)
    }
    #[doc = "Bit 8 - Line 8 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln8(&mut self) -> LN8_W<OVRCLR_SPEC, 8> {
        LN8_W::new(self)
    }
    #[doc = "Bit 9 - Line 9 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln9(&mut self) -> LN9_W<OVRCLR_SPEC, 9> {
        LN9_W::new(self)
    }
    #[doc = "Bit 10 - Line 10 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln10(&mut self) -> LN10_W<OVRCLR_SPEC, 10> {
        LN10_W::new(self)
    }
    #[doc = "Bit 11 - Line 11 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln11(&mut self) -> LN11_W<OVRCLR_SPEC, 11> {
        LN11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Overrun Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ovrclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OVRCLR_SPEC;
impl crate::RegisterSpec for OVRCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ovrclr::W`](W) writer structure"]
impl crate::Writable for OVRCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OVRCLR to value 0"]
impl crate::Resettable for OVRCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
