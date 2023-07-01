#[doc = "Register `MOIPR` reader"]
pub struct R(crate::R<MOIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOIPR` writer"]
pub struct W(crate::W<MOIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOIPR_SPEC>;
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
impl From<crate::W<MOIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOIPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXINP` reader - Receive Interrupt Node Pointer"]
pub type RXINP_R = crate::FieldReader<RXINP_A>;
#[doc = "Receive Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "7: Interrupt output line INT_O7 is selected."]
    VALUE3 = 7,
}
impl From<RXINP_A> for u8 {
    #[inline(always)]
    fn from(variant: RXINP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXINP_A {
    type Ux = u8;
}
impl RXINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXINP_A> {
        match self.bits {
            0 => Some(RXINP_A::VALUE1),
            1 => Some(RXINP_A::VALUE2),
            7 => Some(RXINP_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RXINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RXINP_A::VALUE3
    }
}
#[doc = "Field `RXINP` writer - Receive Interrupt Node Pointer"]
pub type RXINP_W<'a, const O: u8> = crate::FieldWriter<'a, MOIPR_SPEC, 3, O, RXINP_A>;
impl<'a, const O: u8> RXINP_W<'a, O> {
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O7 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(RXINP_A::VALUE3)
    }
}
#[doc = "Field `TXINP` reader - Transmit Interrupt Node Pointer"]
pub type TXINP_R = crate::FieldReader<TXINP_A>;
#[doc = "Transmit Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "7: Interrupt output line INT_O7 is selected."]
    VALUE3 = 7,
}
impl From<TXINP_A> for u8 {
    #[inline(always)]
    fn from(variant: TXINP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TXINP_A {
    type Ux = u8;
}
impl TXINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXINP_A> {
        match self.bits {
            0 => Some(TXINP_A::VALUE1),
            1 => Some(TXINP_A::VALUE2),
            7 => Some(TXINP_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TXINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TXINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TXINP_A::VALUE3
    }
}
#[doc = "Field `TXINP` writer - Transmit Interrupt Node Pointer"]
pub type TXINP_W<'a, const O: u8> = crate::FieldWriter<'a, MOIPR_SPEC, 3, O, TXINP_A>;
impl<'a, const O: u8> TXINP_W<'a, O> {
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TXINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O7 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TXINP_A::VALUE3)
    }
}
#[doc = "Field `MPN` reader - Message Pending Number"]
pub type MPN_R = crate::FieldReader;
#[doc = "Field `MPN` writer - Message Pending Number"]
pub type MPN_W<'a, const O: u8> = crate::FieldWriter<'a, MOIPR_SPEC, 8, O>;
#[doc = "Field `CFCVAL` reader - CAN Frame Counter Value"]
pub type CFCVAL_R = crate::FieldReader<u16>;
#[doc = "Field `CFCVAL` writer - CAN Frame Counter Value"]
pub type CFCVAL_W<'a, const O: u8> = crate::FieldWriter<'a, MOIPR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:2 - Receive Interrupt Node Pointer"]
    #[inline(always)]
    pub fn rxinp(&self) -> RXINP_R {
        RXINP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Transmit Interrupt Node Pointer"]
    #[inline(always)]
    pub fn txinp(&self) -> TXINP_R {
        TXINP_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:15 - Message Pending Number"]
    #[inline(always)]
    pub fn mpn(&self) -> MPN_R {
        MPN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - CAN Frame Counter Value"]
    #[inline(always)]
    pub fn cfcval(&self) -> CFCVAL_R {
        CFCVAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn rxinp(&mut self) -> RXINP_W<0> {
        RXINP_W::new(self)
    }
    #[doc = "Bits 4:6 - Transmit Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn txinp(&mut self) -> TXINP_W<4> {
        TXINP_W::new(self)
    }
    #[doc = "Bits 8:15 - Message Pending Number"]
    #[inline(always)]
    #[must_use]
    pub fn mpn(&mut self) -> MPN_W<8> {
        MPN_W::new(self)
    }
    #[doc = "Bits 16:31 - CAN Frame Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn cfcval(&mut self) -> CFCVAL_W<16> {
        CFCVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Object Interrupt Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [moipr](index.html) module"]
pub struct MOIPR_SPEC;
impl crate::RegisterSpec for MOIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [moipr::R](R) reader structure"]
impl crate::Readable for MOIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [moipr::W](W) writer structure"]
impl crate::Writable for MOIPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOIPR to value 0"]
impl crate::Resettable for MOIPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
