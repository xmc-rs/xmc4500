#[doc = "Register `CLEARBLOCK` writer"]
pub struct W(crate::W<CLEARBLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLEARBLOCK_SPEC>;
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
impl From<crate::W<CLEARBLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLEARBLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH0_AW> for bool {
    #[inline(always)]
    fn from(variant: CH0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0` writer - Clear Interrupt Status and Raw Status for channel 0"]
pub type CH0_W<'a, const O: u8> = crate::BitWriter<'a, CLEARBLOCK_SPEC, O, CH0_AW>;
impl<'a, const O: u8> CH0_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH0_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH0_AW::VALUE2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH1_AW> for bool {
    #[inline(always)]
    fn from(variant: CH1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1` writer - Clear Interrupt Status and Raw Status for channel 1"]
pub type CH1_W<'a, const O: u8> = crate::BitWriter<'a, CLEARBLOCK_SPEC, O, CH1_AW>;
impl<'a, const O: u8> CH1_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH1_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH1_AW::VALUE2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH2_AW> for bool {
    #[inline(always)]
    fn from(variant: CH2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2` writer - Clear Interrupt Status and Raw Status for channel 2"]
pub type CH2_W<'a, const O: u8> = crate::BitWriter<'a, CLEARBLOCK_SPEC, O, CH2_AW>;
impl<'a, const O: u8> CH2_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH2_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH2_AW::VALUE2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3_AW {
    #[doc = "0: no effect"]
    VALUE1 = 0,
    #[doc = "1: clear status"]
    VALUE2 = 1,
}
impl From<CH3_AW> for bool {
    #[inline(always)]
    fn from(variant: CH3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3` writer - Clear Interrupt Status and Raw Status for channel 3"]
pub type CH3_W<'a, const O: u8> = crate::BitWriter<'a, CLEARBLOCK_SPEC, O, CH3_AW>;
impl<'a, const O: u8> CH3_W<'a, O> {
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH3_AW::VALUE1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH3_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Interrupt Status and Raw Status for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Clear Interrupt Status and Raw Status for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - Clear Interrupt Status and Raw Status for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - Clear Interrupt Status and Raw Status for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<3> {
        CH3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IntBlock Status\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clearblock](index.html) module"]
pub struct CLEARBLOCK_SPEC;
impl crate::RegisterSpec for CLEARBLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clearblock::W](W) writer structure"]
impl crate::Writable for CLEARBLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLEARBLOCK to value 0"]
impl crate::Resettable for CLEARBLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
