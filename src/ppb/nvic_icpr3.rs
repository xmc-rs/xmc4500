#[doc = "Register `NVIC_ICPR3` reader"]
pub struct R(crate::R<NVIC_ICPR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_ICPR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_ICPR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_ICPR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_ICPR3` writer"]
pub struct W(crate::W<NVIC_ICPR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_ICPR3_SPEC>;
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
impl From<crate::W<NVIC_ICPR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_ICPR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRPEND` reader - Interrupt set-pending bits."]
pub type CLRPEND_R = crate::FieldReader<CLRPEND_A>;
#[doc = "Interrupt set-pending bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum CLRPEND_A {
    #[doc = "0: interrupt is not pending"]
    VALUE3 = 0,
    #[doc = "1: interrupt is pending."]
    VALUE4 = 1,
}
impl From<CLRPEND_A> for u32 {
    #[inline(always)]
    fn from(variant: CLRPEND_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLRPEND_A {
    type Ux = u32;
}
impl CLRPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLRPEND_A> {
        match self.bits {
            0 => Some(CLRPEND_A::VALUE3),
            1 => Some(CLRPEND_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CLRPEND_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CLRPEND_A::VALUE4
    }
}
#[doc = "Field `CLRPEND` writer - Interrupt set-pending bits."]
pub type CLRPEND_W<'a, const O: u8> = crate::FieldWriter<'a, NVIC_ICPR3_SPEC, 32, O, CLRPEND_A>;
impl<'a, const O: u8> CLRPEND_W<'a, O> {
    #[doc = "interrupt is not pending"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CLRPEND_A::VALUE3)
    }
    #[doc = "interrupt is pending."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CLRPEND_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt set-pending bits."]
    #[inline(always)]
    pub fn clrpend(&self) -> CLRPEND_R {
        CLRPEND_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt set-pending bits."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend(&mut self) -> CLRPEND_W<0> {
        CLRPEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear-pending Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_icpr3](index.html) module"]
pub struct NVIC_ICPR3_SPEC;
impl crate::RegisterSpec for NVIC_ICPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_icpr3::R](R) reader structure"]
impl crate::Readable for NVIC_ICPR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_icpr3::W](W) writer structure"]
impl crate::Writable for NVIC_ICPR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_ICPR3 to value 0"]
impl crate::Resettable for NVIC_ICPR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
