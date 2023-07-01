#[doc = "Register `SYST_CSR` reader"]
pub struct R(crate::R<SYST_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYST_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYST_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYST_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYST_CSR` writer"]
pub struct W(crate::W<SYST_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYST_CSR_SPEC>;
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
impl From<crate::W<SYST_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYST_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: counter disabled"]
    VALUE1 = 0,
    #[doc = "1: counter enabled."]
    VALUE2 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::VALUE1,
            true => ENABLE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENABLE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENABLE_A::VALUE2
    }
}
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, SYST_CSR_SPEC, O, ENABLE_A>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "counter disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENABLE_A::VALUE1)
    }
    #[doc = "counter enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENABLE_A::VALUE2)
    }
}
#[doc = "Field `TICKINT` reader - Tick Interrupt Enable"]
pub type TICKINT_R = crate::BitReader<TICKINT_A>;
#[doc = "Tick Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TICKINT_A {
    #[doc = "0: counting down to zero does not assert the SysTick exception request"]
    VALUE1 = 0,
    #[doc = "1: counting down to zero to asserts the SysTick exception request."]
    VALUE2 = 1,
}
impl From<TICKINT_A> for bool {
    #[inline(always)]
    fn from(variant: TICKINT_A) -> Self {
        variant as u8 != 0
    }
}
impl TICKINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TICKINT_A {
        match self.bits {
            false => TICKINT_A::VALUE1,
            true => TICKINT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TICKINT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TICKINT_A::VALUE2
    }
}
#[doc = "Field `TICKINT` writer - Tick Interrupt Enable"]
pub type TICKINT_W<'a, const O: u8> = crate::BitWriter<'a, SYST_CSR_SPEC, O, TICKINT_A>;
impl<'a, const O: u8> TICKINT_W<'a, O> {
    #[doc = "counting down to zero does not assert the SysTick exception request"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TICKINT_A::VALUE1)
    }
    #[doc = "counting down to zero to asserts the SysTick exception request."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TICKINT_A::VALUE2)
    }
}
#[doc = "Field `CLKSOURCE` reader - Indicates the clock source:"]
pub type CLKSOURCE_R = crate::BitReader<CLKSOURCE_A>;
#[doc = "Indicates the clock source:\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKSOURCE_A {
    #[doc = "0: external clock"]
    VALUE1 = 0,
    #[doc = "1: processor clock."]
    VALUE2 = 1,
}
impl From<CLKSOURCE_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSOURCE_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKSOURCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSOURCE_A {
        match self.bits {
            false => CLKSOURCE_A::VALUE1,
            true => CLKSOURCE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLKSOURCE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CLKSOURCE_A::VALUE2
    }
}
#[doc = "Field `CLKSOURCE` writer - Indicates the clock source:"]
pub type CLKSOURCE_W<'a, const O: u8> = crate::BitWriter<'a, SYST_CSR_SPEC, O, CLKSOURCE_A>;
impl<'a, const O: u8> CLKSOURCE_W<'a, O> {
    #[doc = "external clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLKSOURCE_A::VALUE1)
    }
    #[doc = "processor clock."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CLKSOURCE_A::VALUE2)
    }
}
#[doc = "Field `COUNTFLAG` reader - Counter Flag"]
pub type COUNTFLAG_R = crate::BitReader;
#[doc = "Field `COUNTFLAG` writer - Counter Flag"]
pub type COUNTFLAG_W<'a, const O: u8> = crate::BitWriter<'a, SYST_CSR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tick Interrupt Enable"]
    #[inline(always)]
    pub fn tickint(&self) -> TICKINT_R {
        TICKINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates the clock source:"]
    #[inline(always)]
    pub fn clksource(&self) -> CLKSOURCE_R {
        CLKSOURCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Counter Flag"]
    #[inline(always)]
    pub fn countflag(&self) -> COUNTFLAG_R {
        COUNTFLAG_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Tick Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tickint(&mut self) -> TICKINT_W<1> {
        TICKINT_W::new(self)
    }
    #[doc = "Bit 2 - Indicates the clock source:"]
    #[inline(always)]
    #[must_use]
    pub fn clksource(&mut self) -> CLKSOURCE_W<2> {
        CLKSOURCE_W::new(self)
    }
    #[doc = "Bit 16 - Counter Flag"]
    #[inline(always)]
    #[must_use]
    pub fn countflag(&mut self) -> COUNTFLAG_W<16> {
        COUNTFLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SysTick Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syst_csr](index.html) module"]
pub struct SYST_CSR_SPEC;
impl crate::RegisterSpec for SYST_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syst_csr::R](R) reader structure"]
impl crate::Readable for SYST_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syst_csr::W](W) writer structure"]
impl crate::Writable for SYST_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYST_CSR to value 0x04"]
impl crate::Resettable for SYST_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
