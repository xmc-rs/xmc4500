#[doc = "Reader of register EVFLAG"]
pub type R = crate::R<u32, super::EVFLAG>;
#[doc = "Writer for register EVFLAG"]
pub type W = crate::W<u32, super::EVFLAG>;
#[doc = "Register EVFLAG `reset()`'s with value 0"]
impl crate::ResetValue for super::EVFLAG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESEV0_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: A new result has been stored in register RESMx"]
    VALUE2 = 1,
}
impl From<RESEV0_A> for bool {
    #[inline(always)]
    fn from(variant: RESEV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESEV0`"]
pub type RESEV0_R = crate::R<bool, RESEV0_A>;
impl RESEV0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESEV0_A {
        match self.bits {
            false => RESEV0_A::VALUE1,
            true => RESEV0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESEV0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESEV0_A::VALUE2
    }
}
#[doc = "Write proxy for field `RESEV0`"]
pub struct RESEV0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESEV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESEV0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESEV0_A::VALUE1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESEV0_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESEV1_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: A new result has been stored in register RESMx"]
    VALUE2 = 1,
}
impl From<RESEV1_A> for bool {
    #[inline(always)]
    fn from(variant: RESEV1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESEV1`"]
pub type RESEV1_R = crate::R<bool, RESEV1_A>;
impl RESEV1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESEV1_A {
        match self.bits {
            false => RESEV1_A::VALUE1,
            true => RESEV1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESEV1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESEV1_A::VALUE2
    }
}
#[doc = "Write proxy for field `RESEV1`"]
pub struct RESEV1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESEV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESEV1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESEV1_A::VALUE1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESEV1_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESEV2_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: A new result has been stored in register RESMx"]
    VALUE2 = 1,
}
impl From<RESEV2_A> for bool {
    #[inline(always)]
    fn from(variant: RESEV2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESEV2`"]
pub type RESEV2_R = crate::R<bool, RESEV2_A>;
impl RESEV2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESEV2_A {
        match self.bits {
            false => RESEV2_A::VALUE1,
            true => RESEV2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESEV2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESEV2_A::VALUE2
    }
}
#[doc = "Write proxy for field `RESEV2`"]
pub struct RESEV2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESEV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESEV2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESEV2_A::VALUE1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESEV2_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESEV3_A {
    #[doc = "0: No result event"]
    VALUE1 = 0,
    #[doc = "1: A new result has been stored in register RESMx"]
    VALUE2 = 1,
}
impl From<RESEV3_A> for bool {
    #[inline(always)]
    fn from(variant: RESEV3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESEV3`"]
pub type RESEV3_R = crate::R<bool, RESEV3_A>;
impl RESEV3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESEV3_A {
        match self.bits {
            false => RESEV3_A::VALUE1,
            true => RESEV3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESEV3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESEV3_A::VALUE2
    }
}
#[doc = "Write proxy for field `RESEV3`"]
pub struct RESEV3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESEV3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESEV3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESEV3_A::VALUE1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESEV3_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEV0_A {
    #[doc = "0: No alarm event"]
    VALUE1 = 0,
    #[doc = "1: An alarm event has occurred"]
    VALUE2 = 1,
}
impl From<ALEV0_A> for bool {
    #[inline(always)]
    fn from(variant: ALEV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALEV0`"]
pub type ALEV0_R = crate::R<bool, ALEV0_A>;
impl ALEV0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALEV0_A {
        match self.bits {
            false => ALEV0_A::VALUE1,
            true => ALEV0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV0_A::VALUE2
    }
}
#[doc = "Write proxy for field `ALEV0`"]
pub struct ALEV0_W<'a> {
    w: &'a mut W,
}
impl<'a> ALEV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALEV0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV0_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV0_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEV1_A {
    #[doc = "0: No alarm event"]
    VALUE1 = 0,
    #[doc = "1: An alarm event has occurred"]
    VALUE2 = 1,
}
impl From<ALEV1_A> for bool {
    #[inline(always)]
    fn from(variant: ALEV1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALEV1`"]
pub type ALEV1_R = crate::R<bool, ALEV1_A>;
impl ALEV1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALEV1_A {
        match self.bits {
            false => ALEV1_A::VALUE1,
            true => ALEV1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV1_A::VALUE2
    }
}
#[doc = "Write proxy for field `ALEV1`"]
pub struct ALEV1_W<'a> {
    w: &'a mut W,
}
impl<'a> ALEV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALEV1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV1_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV1_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEV2_A {
    #[doc = "0: No alarm event"]
    VALUE1 = 0,
    #[doc = "1: An alarm event has occurred"]
    VALUE2 = 1,
}
impl From<ALEV2_A> for bool {
    #[inline(always)]
    fn from(variant: ALEV2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALEV2`"]
pub type ALEV2_R = crate::R<bool, ALEV2_A>;
impl ALEV2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALEV2_A {
        match self.bits {
            false => ALEV2_A::VALUE1,
            true => ALEV2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV2_A::VALUE2
    }
}
#[doc = "Write proxy for field `ALEV2`"]
pub struct ALEV2_W<'a> {
    w: &'a mut W,
}
impl<'a> ALEV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALEV2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV2_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV2_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEV3_A {
    #[doc = "0: No alarm event"]
    VALUE1 = 0,
    #[doc = "1: An alarm event has occurred"]
    VALUE2 = 1,
}
impl From<ALEV3_A> for bool {
    #[inline(always)]
    fn from(variant: ALEV3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALEV3`"]
pub type ALEV3_R = crate::R<bool, ALEV3_A>;
impl ALEV3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALEV3_A {
        match self.bits {
            false => ALEV3_A::VALUE1,
            true => ALEV3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV3_A::VALUE2
    }
}
#[doc = "Write proxy for field `ALEV3`"]
pub struct ALEV3_W<'a> {
    w: &'a mut W,
}
impl<'a> ALEV3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALEV3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV3_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV3_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEV4_A {
    #[doc = "0: No alarm event"]
    VALUE1 = 0,
    #[doc = "1: An alarm event has occurred"]
    VALUE2 = 1,
}
impl From<ALEV4_A> for bool {
    #[inline(always)]
    fn from(variant: ALEV4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALEV4`"]
pub type ALEV4_R = crate::R<bool, ALEV4_A>;
impl ALEV4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALEV4_A {
        match self.bits {
            false => ALEV4_A::VALUE1,
            true => ALEV4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV4_A::VALUE2
    }
}
#[doc = "Write proxy for field `ALEV4`"]
pub struct ALEV4_W<'a> {
    w: &'a mut W,
}
impl<'a> ALEV4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALEV4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV4_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV4_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEV5_A {
    #[doc = "0: No alarm event"]
    VALUE1 = 0,
    #[doc = "1: An alarm event has occurred"]
    VALUE2 = 1,
}
impl From<ALEV5_A> for bool {
    #[inline(always)]
    fn from(variant: ALEV5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALEV5`"]
pub type ALEV5_R = crate::R<bool, ALEV5_A>;
impl ALEV5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALEV5_A {
        match self.bits {
            false => ALEV5_A::VALUE1,
            true => ALEV5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV5_A::VALUE2
    }
}
#[doc = "Write proxy for field `ALEV5`"]
pub struct ALEV5_W<'a> {
    w: &'a mut W,
}
impl<'a> ALEV5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALEV5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV5_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV5_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEV6_A {
    #[doc = "0: No alarm event"]
    VALUE1 = 0,
    #[doc = "1: An alarm event has occurred"]
    VALUE2 = 1,
}
impl From<ALEV6_A> for bool {
    #[inline(always)]
    fn from(variant: ALEV6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALEV6`"]
pub type ALEV6_R = crate::R<bool, ALEV6_A>;
impl ALEV6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALEV6_A {
        match self.bits {
            false => ALEV6_A::VALUE1,
            true => ALEV6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV6_A::VALUE2
    }
}
#[doc = "Write proxy for field `ALEV6`"]
pub struct ALEV6_W<'a> {
    w: &'a mut W,
}
impl<'a> ALEV6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALEV6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV6_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV6_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEV7_A {
    #[doc = "0: No alarm event"]
    VALUE1 = 0,
    #[doc = "1: An alarm event has occurred"]
    VALUE2 = 1,
}
impl From<ALEV7_A> for bool {
    #[inline(always)]
    fn from(variant: ALEV7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALEV7`"]
pub type ALEV7_R = crate::R<bool, ALEV7_A>;
impl ALEV7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALEV7_A {
        match self.bits {
            false => ALEV7_A::VALUE1,
            true => ALEV7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV7_A::VALUE2
    }
}
#[doc = "Write proxy for field `ALEV7`"]
pub struct ALEV7_W<'a> {
    w: &'a mut W,
}
impl<'a> ALEV7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALEV7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV7_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV7_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEV8_A {
    #[doc = "0: No alarm event"]
    VALUE1 = 0,
    #[doc = "1: An alarm event has occurred"]
    VALUE2 = 1,
}
impl From<ALEV8_A> for bool {
    #[inline(always)]
    fn from(variant: ALEV8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALEV8`"]
pub type ALEV8_R = crate::R<bool, ALEV8_A>;
impl ALEV8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALEV8_A {
        match self.bits {
            false => ALEV8_A::VALUE1,
            true => ALEV8_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV8_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV8_A::VALUE2
    }
}
#[doc = "Write proxy for field `ALEV8`"]
pub struct ALEV8_W<'a> {
    w: &'a mut W,
}
impl<'a> ALEV8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALEV8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV8_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV8_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEV9_A {
    #[doc = "0: No alarm event"]
    VALUE1 = 0,
    #[doc = "1: An alarm event has occurred"]
    VALUE2 = 1,
}
impl From<ALEV9_A> for bool {
    #[inline(always)]
    fn from(variant: ALEV9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALEV9`"]
pub type ALEV9_R = crate::R<bool, ALEV9_A>;
impl ALEV9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALEV9_A {
        match self.bits {
            false => ALEV9_A::VALUE1,
            true => ALEV9_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV9_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV9_A::VALUE2
    }
}
#[doc = "Write proxy for field `ALEV9`"]
pub struct ALEV9_W<'a> {
    w: &'a mut W,
}
impl<'a> ALEV9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALEV9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV9_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV9_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Result Event"]
    #[inline(always)]
    pub fn resev0(&self) -> RESEV0_R {
        RESEV0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Result Event"]
    #[inline(always)]
    pub fn resev1(&self) -> RESEV1_R {
        RESEV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Result Event"]
    #[inline(always)]
    pub fn resev2(&self) -> RESEV2_R {
        RESEV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Result Event"]
    #[inline(always)]
    pub fn resev3(&self) -> RESEV3_R {
        RESEV3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Alarm Event"]
    #[inline(always)]
    pub fn alev0(&self) -> ALEV0_R {
        ALEV0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Alarm Event"]
    #[inline(always)]
    pub fn alev1(&self) -> ALEV1_R {
        ALEV1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Alarm Event"]
    #[inline(always)]
    pub fn alev2(&self) -> ALEV2_R {
        ALEV2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Alarm Event"]
    #[inline(always)]
    pub fn alev3(&self) -> ALEV3_R {
        ALEV3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Alarm Event"]
    #[inline(always)]
    pub fn alev4(&self) -> ALEV4_R {
        ALEV4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Alarm Event"]
    #[inline(always)]
    pub fn alev5(&self) -> ALEV5_R {
        ALEV5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Alarm Event"]
    #[inline(always)]
    pub fn alev6(&self) -> ALEV6_R {
        ALEV6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Alarm Event"]
    #[inline(always)]
    pub fn alev7(&self) -> ALEV7_R {
        ALEV7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Alarm Event"]
    #[inline(always)]
    pub fn alev8(&self) -> ALEV8_R {
        ALEV8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Alarm Event"]
    #[inline(always)]
    pub fn alev9(&self) -> ALEV9_R {
        ALEV9_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Result Event"]
    #[inline(always)]
    pub fn resev0(&mut self) -> RESEV0_W {
        RESEV0_W { w: self }
    }
    #[doc = "Bit 1 - Result Event"]
    #[inline(always)]
    pub fn resev1(&mut self) -> RESEV1_W {
        RESEV1_W { w: self }
    }
    #[doc = "Bit 2 - Result Event"]
    #[inline(always)]
    pub fn resev2(&mut self) -> RESEV2_W {
        RESEV2_W { w: self }
    }
    #[doc = "Bit 3 - Result Event"]
    #[inline(always)]
    pub fn resev3(&mut self) -> RESEV3_W {
        RESEV3_W { w: self }
    }
    #[doc = "Bit 16 - Alarm Event"]
    #[inline(always)]
    pub fn alev0(&mut self) -> ALEV0_W {
        ALEV0_W { w: self }
    }
    #[doc = "Bit 17 - Alarm Event"]
    #[inline(always)]
    pub fn alev1(&mut self) -> ALEV1_W {
        ALEV1_W { w: self }
    }
    #[doc = "Bit 18 - Alarm Event"]
    #[inline(always)]
    pub fn alev2(&mut self) -> ALEV2_W {
        ALEV2_W { w: self }
    }
    #[doc = "Bit 19 - Alarm Event"]
    #[inline(always)]
    pub fn alev3(&mut self) -> ALEV3_W {
        ALEV3_W { w: self }
    }
    #[doc = "Bit 20 - Alarm Event"]
    #[inline(always)]
    pub fn alev4(&mut self) -> ALEV4_W {
        ALEV4_W { w: self }
    }
    #[doc = "Bit 21 - Alarm Event"]
    #[inline(always)]
    pub fn alev5(&mut self) -> ALEV5_W {
        ALEV5_W { w: self }
    }
    #[doc = "Bit 22 - Alarm Event"]
    #[inline(always)]
    pub fn alev6(&mut self) -> ALEV6_W {
        ALEV6_W { w: self }
    }
    #[doc = "Bit 23 - Alarm Event"]
    #[inline(always)]
    pub fn alev7(&mut self) -> ALEV7_W {
        ALEV7_W { w: self }
    }
    #[doc = "Bit 24 - Alarm Event"]
    #[inline(always)]
    pub fn alev8(&mut self) -> ALEV8_W {
        ALEV8_W { w: self }
    }
    #[doc = "Bit 25 - Alarm Event"]
    #[inline(always)]
    pub fn alev9(&mut self) -> ALEV9_W {
        ALEV9_W { w: self }
    }
}
