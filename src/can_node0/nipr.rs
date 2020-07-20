#[doc = "Reader of register NIPR"]
pub type R = crate::R<u32, super::NIPR>;
#[doc = "Writer for register NIPR"]
pub type W = crate::W<u32, super::NIPR>;
#[doc = "Register NIPR `reset()`'s with value 0"]
impl crate::ResetValue for super::NIPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Alert Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ALINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "7: Interrupt output line INT_O7 is selected."]
    VALUE3 = 7,
}
impl From<ALINP_A> for u8 {
    #[inline(always)]
    fn from(variant: ALINP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ALINP`"]
pub type ALINP_R = crate::R<u8, ALINP_A>;
impl ALINP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ALINP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ALINP_A::VALUE1),
            1 => Val(ALINP_A::VALUE2),
            7 => Val(ALINP_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ALINP_A::VALUE3
    }
}
#[doc = "Write proxy for field `ALINP`"]
pub struct ALINP_W<'a> {
    w: &'a mut W,
}
impl<'a> ALINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O7 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ALINP_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Last Error Code Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LECINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "7: Interrupt output line INT_O7 is selected."]
    VALUE3 = 7,
}
impl From<LECINP_A> for u8 {
    #[inline(always)]
    fn from(variant: LECINP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LECINP`"]
pub type LECINP_R = crate::R<u8, LECINP_A>;
impl LECINP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LECINP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LECINP_A::VALUE1),
            1 => Val(LECINP_A::VALUE2),
            7 => Val(LECINP_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LECINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LECINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LECINP_A::VALUE3
    }
}
#[doc = "Write proxy for field `LECINP`"]
pub struct LECINP_W<'a> {
    w: &'a mut W,
}
impl<'a> LECINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LECINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LECINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LECINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O7 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(LECINP_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Transfer OK Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "7: Interrupt output line INT_O7 is selected."]
    VALUE3 = 7,
}
impl From<TRINP_A> for u8 {
    #[inline(always)]
    fn from(variant: TRINP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRINP`"]
pub type TRINP_R = crate::R<u8, TRINP_A>;
impl TRINP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRINP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRINP_A::VALUE1),
            1 => Val(TRINP_A::VALUE2),
            7 => Val(TRINP_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TRINP_A::VALUE3
    }
}
#[doc = "Write proxy for field `TRINP`"]
pub struct TRINP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O7 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TRINP_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Frame Counter Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFCINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "7: Interrupt output line INT_O7 is selected."]
    VALUE3 = 7,
}
impl From<CFCINP_A> for u8 {
    #[inline(always)]
    fn from(variant: CFCINP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CFCINP`"]
pub type CFCINP_R = crate::R<u8, CFCINP_A>;
impl CFCINP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CFCINP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CFCINP_A::VALUE1),
            1 => Val(CFCINP_A::VALUE2),
            7 => Val(CFCINP_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFCINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CFCINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CFCINP_A::VALUE3
    }
}
#[doc = "Write proxy for field `CFCINP`"]
pub struct CFCINP_W<'a> {
    w: &'a mut W,
}
impl<'a> CFCINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFCINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFCINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFCINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O7 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CFCINP_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Alert Interrupt Node Pointer"]
    #[inline(always)]
    pub fn alinp(&self) -> ALINP_R {
        ALINP_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Last Error Code Interrupt Node Pointer"]
    #[inline(always)]
    pub fn lecinp(&self) -> LECINP_R {
        LECINP_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Transfer OK Interrupt Node Pointer"]
    #[inline(always)]
    pub fn trinp(&self) -> TRINP_R {
        TRINP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Frame Counter Interrupt Node Pointer"]
    #[inline(always)]
    pub fn cfcinp(&self) -> CFCINP_R {
        CFCINP_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Alert Interrupt Node Pointer"]
    #[inline(always)]
    pub fn alinp(&mut self) -> ALINP_W {
        ALINP_W { w: self }
    }
    #[doc = "Bits 4:6 - Last Error Code Interrupt Node Pointer"]
    #[inline(always)]
    pub fn lecinp(&mut self) -> LECINP_W {
        LECINP_W { w: self }
    }
    #[doc = "Bits 8:10 - Transfer OK Interrupt Node Pointer"]
    #[inline(always)]
    pub fn trinp(&mut self) -> TRINP_W {
        TRINP_W { w: self }
    }
    #[doc = "Bits 12:14 - Frame Counter Interrupt Node Pointer"]
    #[inline(always)]
    pub fn cfcinp(&mut self) -> CFCINP_W {
        CFCINP_W { w: self }
    }
}
