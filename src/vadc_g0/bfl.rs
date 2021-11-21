#[doc = "Register `BFL` reader"]
pub struct R(crate::R<BFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BFL` writer"]
pub struct W(crate::W<BFL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BFL_SPEC>;
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
impl From<crate::W<BFL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BFL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Boundar0 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL0_A {
    #[doc = "0: Passive state: result has not yet crossed the activation boundary, or selected gate signal is inactive, or this boundary flag is disabled"]
    VALUE1 = 0,
    #[doc = "1: Active state: result has crossed the activation boundary"]
    VALUE2 = 1,
}
impl From<BFL0_A> for bool {
    #[inline(always)]
    fn from(variant: BFL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFL0` reader - Boundar0 Flag y"]
pub struct BFL0_R(crate::FieldReader<bool, BFL0_A>);
impl BFL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFL0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFL0_A {
        match self.bits {
            false => BFL0_A::VALUE1,
            true => BFL0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFL0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFL0_A::VALUE2
    }
}
impl core::ops::Deref for BFL0_R {
    type Target = crate::FieldReader<bool, BFL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Boundar1 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL1_A {
    #[doc = "0: Passive state: result has not yet crossed the activation boundary, or selected gate signal is inactive, or this boundary flag is disabled"]
    VALUE1 = 0,
    #[doc = "1: Active state: result has crossed the activation boundary"]
    VALUE2 = 1,
}
impl From<BFL1_A> for bool {
    #[inline(always)]
    fn from(variant: BFL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFL1` reader - Boundar1 Flag y"]
pub struct BFL1_R(crate::FieldReader<bool, BFL1_A>);
impl BFL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFL1_A {
        match self.bits {
            false => BFL1_A::VALUE1,
            true => BFL1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFL1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFL1_A::VALUE2
    }
}
impl core::ops::Deref for BFL1_R {
    type Target = crate::FieldReader<bool, BFL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Boundar2 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL2_A {
    #[doc = "0: Passive state: result has not yet crossed the activation boundary, or selected gate signal is inactive, or this boundary flag is disabled"]
    VALUE1 = 0,
    #[doc = "1: Active state: result has crossed the activation boundary"]
    VALUE2 = 1,
}
impl From<BFL2_A> for bool {
    #[inline(always)]
    fn from(variant: BFL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFL2` reader - Boundar2 Flag y"]
pub struct BFL2_R(crate::FieldReader<bool, BFL2_A>);
impl BFL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFL2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFL2_A {
        match self.bits {
            false => BFL2_A::VALUE1,
            true => BFL2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFL2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFL2_A::VALUE2
    }
}
impl core::ops::Deref for BFL2_R {
    type Target = crate::FieldReader<bool, BFL2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Boundar3 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFL3_A {
    #[doc = "0: Passive state: result has not yet crossed the activation boundary, or selected gate signal is inactive, or this boundary flag is disabled"]
    VALUE1 = 0,
    #[doc = "1: Active state: result has crossed the activation boundary"]
    VALUE2 = 1,
}
impl From<BFL3_A> for bool {
    #[inline(always)]
    fn from(variant: BFL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFL3` reader - Boundar3 Flag y"]
pub struct BFL3_R(crate::FieldReader<bool, BFL3_A>);
impl BFL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFL3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFL3_A {
        match self.bits {
            false => BFL3_A::VALUE1,
            true => BFL3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFL3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFL3_A::VALUE2
    }
}
impl core::ops::Deref for BFL3_R {
    type Target = crate::FieldReader<bool, BFL3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable Bit for Boundar0 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFE0_A {
    #[doc = "0: Output 0 on this channel"]
    VALUE1 = 0,
    #[doc = "1: Output BFLy on this channel"]
    VALUE2 = 1,
}
impl From<BFE0_A> for bool {
    #[inline(always)]
    fn from(variant: BFE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFE0` reader - Enable Bit for Boundar0 Flag y"]
pub struct BFE0_R(crate::FieldReader<bool, BFE0_A>);
impl BFE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFE0_A {
        match self.bits {
            false => BFE0_A::VALUE1,
            true => BFE0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFE0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFE0_A::VALUE2
    }
}
impl core::ops::Deref for BFE0_R {
    type Target = crate::FieldReader<bool, BFE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFE0` writer - Enable Bit for Boundar0 Flag y"]
pub struct BFE0_W<'a> {
    w: &'a mut W,
}
impl<'a> BFE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Output 0 on this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFE0_A::VALUE1)
    }
    #[doc = "Output BFLy on this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFE0_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Enable Bit for Boundar1 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFE1_A {
    #[doc = "0: Output 0 on this channel"]
    VALUE1 = 0,
    #[doc = "1: Output BFLy on this channel"]
    VALUE2 = 1,
}
impl From<BFE1_A> for bool {
    #[inline(always)]
    fn from(variant: BFE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFE1` reader - Enable Bit for Boundar1 Flag y"]
pub struct BFE1_R(crate::FieldReader<bool, BFE1_A>);
impl BFE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFE1_A {
        match self.bits {
            false => BFE1_A::VALUE1,
            true => BFE1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFE1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFE1_A::VALUE2
    }
}
impl core::ops::Deref for BFE1_R {
    type Target = crate::FieldReader<bool, BFE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFE1` writer - Enable Bit for Boundar1 Flag y"]
pub struct BFE1_W<'a> {
    w: &'a mut W,
}
impl<'a> BFE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Output 0 on this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFE1_A::VALUE1)
    }
    #[doc = "Output BFLy on this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFE1_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Enable Bit for Boundar2 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFE2_A {
    #[doc = "0: Output 0 on this channel"]
    VALUE1 = 0,
    #[doc = "1: Output BFLy on this channel"]
    VALUE2 = 1,
}
impl From<BFE2_A> for bool {
    #[inline(always)]
    fn from(variant: BFE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFE2` reader - Enable Bit for Boundar2 Flag y"]
pub struct BFE2_R(crate::FieldReader<bool, BFE2_A>);
impl BFE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFE2_A {
        match self.bits {
            false => BFE2_A::VALUE1,
            true => BFE2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFE2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFE2_A::VALUE2
    }
}
impl core::ops::Deref for BFE2_R {
    type Target = crate::FieldReader<bool, BFE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFE2` writer - Enable Bit for Boundar2 Flag y"]
pub struct BFE2_W<'a> {
    w: &'a mut W,
}
impl<'a> BFE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFE2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Output 0 on this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFE2_A::VALUE1)
    }
    #[doc = "Output BFLy on this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFE2_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Enable Bit for Boundar3 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFE3_A {
    #[doc = "0: Output 0 on this channel"]
    VALUE1 = 0,
    #[doc = "1: Output BFLy on this channel"]
    VALUE2 = 1,
}
impl From<BFE3_A> for bool {
    #[inline(always)]
    fn from(variant: BFE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFE3` reader - Enable Bit for Boundar3 Flag y"]
pub struct BFE3_R(crate::FieldReader<bool, BFE3_A>);
impl BFE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFE3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFE3_A {
        match self.bits {
            false => BFE3_A::VALUE1,
            true => BFE3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BFE3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BFE3_A::VALUE2
    }
}
impl core::ops::Deref for BFE3_R {
    type Target = crate::FieldReader<bool, BFE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFE3` writer - Enable Bit for Boundar3 Flag y"]
pub struct BFE3_W<'a> {
    w: &'a mut W,
}
impl<'a> BFE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFE3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Output 0 on this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFE3_A::VALUE1)
    }
    #[doc = "Output BFLy on this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFE3_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Boundar0 Flag y"]
    #[inline(always)]
    pub fn bfl0(&self) -> BFL0_R {
        BFL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Boundar1 Flag y"]
    #[inline(always)]
    pub fn bfl1(&self) -> BFL1_R {
        BFL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Boundar2 Flag y"]
    #[inline(always)]
    pub fn bfl2(&self) -> BFL2_R {
        BFL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Boundar3 Flag y"]
    #[inline(always)]
    pub fn bfl3(&self) -> BFL3_R {
        BFL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable Bit for Boundar0 Flag y"]
    #[inline(always)]
    pub fn bfe0(&self) -> BFE0_R {
        BFE0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable Bit for Boundar1 Flag y"]
    #[inline(always)]
    pub fn bfe1(&self) -> BFE1_R {
        BFE1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable Bit for Boundar2 Flag y"]
    #[inline(always)]
    pub fn bfe2(&self) -> BFE2_R {
        BFE2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable Bit for Boundar3 Flag y"]
    #[inline(always)]
    pub fn bfe3(&self) -> BFE3_R {
        BFE3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Enable Bit for Boundar0 Flag y"]
    #[inline(always)]
    pub fn bfe0(&mut self) -> BFE0_W {
        BFE0_W { w: self }
    }
    #[doc = "Bit 17 - Enable Bit for Boundar1 Flag y"]
    #[inline(always)]
    pub fn bfe1(&mut self) -> BFE1_W {
        BFE1_W { w: self }
    }
    #[doc = "Bit 18 - Enable Bit for Boundar2 Flag y"]
    #[inline(always)]
    pub fn bfe2(&mut self) -> BFE2_W {
        BFE2_W { w: self }
    }
    #[doc = "Bit 19 - Enable Bit for Boundar3 Flag y"]
    #[inline(always)]
    pub fn bfe3(&mut self) -> BFE3_W {
        BFE3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Boundary Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfl](index.html) module"]
pub struct BFL_SPEC;
impl crate::RegisterSpec for BFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bfl::R](R) reader structure"]
impl crate::Readable for BFL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bfl::W](W) writer structure"]
impl crate::Writable for BFL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BFL to value 0"]
impl crate::Resettable for BFL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
