#[doc = "Register `PWRCLR` writer"]
pub struct W(crate::W<PWRCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCLR_SPEC>;
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
impl From<crate::W<PWRCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear Disable Hibernate Domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIB_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable Hibernate domain"]
    VALUE2 = 1,
}
impl From<HIB_AW> for bool {
    #[inline(always)]
    fn from(variant: HIB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIB` writer - Clear Disable Hibernate Domain"]
pub type HIB_W<'a, const O: u8> = crate::BitWriter<'a, PWRCLR_SPEC, O, HIB_AW>;
impl<'a, const O: u8> HIB_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HIB_AW::VALUE1)
    }
    #[doc = "Disable Hibernate domain"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HIB_AW::VALUE2)
    }
}
#[doc = "Clear USB PHY Transceiver Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBPHYPDQ_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Power-down"]
    VALUE2 = 1,
}
impl From<USBPHYPDQ_AW> for bool {
    #[inline(always)]
    fn from(variant: USBPHYPDQ_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPHYPDQ` writer - Clear USB PHY Transceiver Disable"]
pub type USBPHYPDQ_W<'a, const O: u8> = crate::BitWriter<'a, PWRCLR_SPEC, O, USBPHYPDQ_AW>;
impl<'a, const O: u8> USBPHYPDQ_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(USBPHYPDQ_AW::VALUE1)
    }
    #[doc = "Power-down"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(USBPHYPDQ_AW::VALUE2)
    }
}
#[doc = "Clear USB On-The-Go Comparators Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBOTGEN_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Power-down"]
    VALUE2 = 1,
}
impl From<USBOTGEN_AW> for bool {
    #[inline(always)]
    fn from(variant: USBOTGEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBOTGEN` writer - Clear USB On-The-Go Comparators Enable"]
pub type USBOTGEN_W<'a, const O: u8> = crate::BitWriter<'a, PWRCLR_SPEC, O, USBOTGEN_AW>;
impl<'a, const O: u8> USBOTGEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(USBOTGEN_AW::VALUE1)
    }
    #[doc = "Power-down"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(USBOTGEN_AW::VALUE2)
    }
}
#[doc = "Clear USB Weak Pull-Up at PADN Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBPUWQ_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Pull-up active"]
    VALUE2 = 1,
}
impl From<USBPUWQ_AW> for bool {
    #[inline(always)]
    fn from(variant: USBPUWQ_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPUWQ` writer - Clear USB Weak Pull-Up at PADN Enable"]
pub type USBPUWQ_W<'a, const O: u8> = crate::BitWriter<'a, PWRCLR_SPEC, O, USBPUWQ_AW>;
impl<'a, const O: u8> USBPUWQ_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(USBPUWQ_AW::VALUE1)
    }
    #[doc = "Pull-up active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(USBPUWQ_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Disable Hibernate Domain"]
    #[inline(always)]
    #[must_use]
    pub fn hib(&mut self) -> HIB_W<0> {
        HIB_W::new(self)
    }
    #[doc = "Bit 16 - Clear USB PHY Transceiver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn usbphypdq(&mut self) -> USBPHYPDQ_W<16> {
        USBPHYPDQ_W::new(self)
    }
    #[doc = "Bit 17 - Clear USB On-The-Go Comparators Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbotgen(&mut self) -> USBOTGEN_W<17> {
        USBOTGEN_W::new(self)
    }
    #[doc = "Bit 18 - Clear USB Weak Pull-Up at PADN Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbpuwq(&mut self) -> USBPUWQ_W<18> {
        USBPUWQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCU Clear Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrclr](index.html) module"]
pub struct PWRCLR_SPEC;
impl crate::RegisterSpec for PWRCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pwrclr::W](W) writer structure"]
impl crate::Writable for PWRCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRCLR to value 0"]
impl crate::Resettable for PWRCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
