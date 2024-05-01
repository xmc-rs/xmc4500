#[doc = "Register `BFL` reader"]
pub type R = crate::R<BflSpec>;
#[doc = "Register `BFL` writer"]
pub type W = crate::W<BflSpec>;
#[doc = "Boundar0 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfl0 {
    #[doc = "0: Passive state: result has not yet crossed the activation boundary, or selected gate signal is inactive, or this boundary flag is disabled"]
    Value1 = 0,
    #[doc = "1: Active state: result has crossed the activation boundary"]
    Value2 = 1,
}
impl From<Bfl0> for bool {
    #[inline(always)]
    fn from(variant: Bfl0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFL0` reader - Boundar0 Flag y"]
pub type Bfl0R = crate::BitReader<Bfl0>;
impl Bfl0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfl0 {
        match self.bits {
            false => Bfl0::Value1,
            true => Bfl0::Value2,
        }
    }
    #[doc = "Passive state: result has not yet crossed the activation boundary, or selected gate signal is inactive, or this boundary flag is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfl0::Value1
    }
    #[doc = "Active state: result has crossed the activation boundary"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfl0::Value2
    }
}
#[doc = "Boundar1 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfl1 {
    #[doc = "0: Passive state: result has not yet crossed the activation boundary, or selected gate signal is inactive, or this boundary flag is disabled"]
    Value1 = 0,
    #[doc = "1: Active state: result has crossed the activation boundary"]
    Value2 = 1,
}
impl From<Bfl1> for bool {
    #[inline(always)]
    fn from(variant: Bfl1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFL1` reader - Boundar1 Flag y"]
pub type Bfl1R = crate::BitReader<Bfl1>;
impl Bfl1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfl1 {
        match self.bits {
            false => Bfl1::Value1,
            true => Bfl1::Value2,
        }
    }
    #[doc = "Passive state: result has not yet crossed the activation boundary, or selected gate signal is inactive, or this boundary flag is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfl1::Value1
    }
    #[doc = "Active state: result has crossed the activation boundary"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfl1::Value2
    }
}
#[doc = "Boundar2 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfl2 {
    #[doc = "0: Passive state: result has not yet crossed the activation boundary, or selected gate signal is inactive, or this boundary flag is disabled"]
    Value1 = 0,
    #[doc = "1: Active state: result has crossed the activation boundary"]
    Value2 = 1,
}
impl From<Bfl2> for bool {
    #[inline(always)]
    fn from(variant: Bfl2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFL2` reader - Boundar2 Flag y"]
pub type Bfl2R = crate::BitReader<Bfl2>;
impl Bfl2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfl2 {
        match self.bits {
            false => Bfl2::Value1,
            true => Bfl2::Value2,
        }
    }
    #[doc = "Passive state: result has not yet crossed the activation boundary, or selected gate signal is inactive, or this boundary flag is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfl2::Value1
    }
    #[doc = "Active state: result has crossed the activation boundary"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfl2::Value2
    }
}
#[doc = "Boundar3 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfl3 {
    #[doc = "0: Passive state: result has not yet crossed the activation boundary, or selected gate signal is inactive, or this boundary flag is disabled"]
    Value1 = 0,
    #[doc = "1: Active state: result has crossed the activation boundary"]
    Value2 = 1,
}
impl From<Bfl3> for bool {
    #[inline(always)]
    fn from(variant: Bfl3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFL3` reader - Boundar3 Flag y"]
pub type Bfl3R = crate::BitReader<Bfl3>;
impl Bfl3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfl3 {
        match self.bits {
            false => Bfl3::Value1,
            true => Bfl3::Value2,
        }
    }
    #[doc = "Passive state: result has not yet crossed the activation boundary, or selected gate signal is inactive, or this boundary flag is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfl3::Value1
    }
    #[doc = "Active state: result has crossed the activation boundary"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfl3::Value2
    }
}
#[doc = "Enable Bit for Boundar0 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfe0 {
    #[doc = "0: Output 0 on this channel"]
    Value1 = 0,
    #[doc = "1: Output BFLy on this channel"]
    Value2 = 1,
}
impl From<Bfe0> for bool {
    #[inline(always)]
    fn from(variant: Bfe0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFE0` reader - Enable Bit for Boundar0 Flag y"]
pub type Bfe0R = crate::BitReader<Bfe0>;
impl Bfe0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfe0 {
        match self.bits {
            false => Bfe0::Value1,
            true => Bfe0::Value2,
        }
    }
    #[doc = "Output 0 on this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfe0::Value1
    }
    #[doc = "Output BFLy on this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfe0::Value2
    }
}
#[doc = "Field `BFE0` writer - Enable Bit for Boundar0 Flag y"]
pub type Bfe0W<'a, REG> = crate::BitWriter<'a, REG, Bfe0>;
impl<'a, REG> Bfe0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output 0 on this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfe0::Value1)
    }
    #[doc = "Output BFLy on this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfe0::Value2)
    }
}
#[doc = "Enable Bit for Boundar1 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfe1 {
    #[doc = "0: Output 0 on this channel"]
    Value1 = 0,
    #[doc = "1: Output BFLy on this channel"]
    Value2 = 1,
}
impl From<Bfe1> for bool {
    #[inline(always)]
    fn from(variant: Bfe1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFE1` reader - Enable Bit for Boundar1 Flag y"]
pub type Bfe1R = crate::BitReader<Bfe1>;
impl Bfe1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfe1 {
        match self.bits {
            false => Bfe1::Value1,
            true => Bfe1::Value2,
        }
    }
    #[doc = "Output 0 on this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfe1::Value1
    }
    #[doc = "Output BFLy on this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfe1::Value2
    }
}
#[doc = "Field `BFE1` writer - Enable Bit for Boundar1 Flag y"]
pub type Bfe1W<'a, REG> = crate::BitWriter<'a, REG, Bfe1>;
impl<'a, REG> Bfe1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output 0 on this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfe1::Value1)
    }
    #[doc = "Output BFLy on this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfe1::Value2)
    }
}
#[doc = "Enable Bit for Boundar2 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfe2 {
    #[doc = "0: Output 0 on this channel"]
    Value1 = 0,
    #[doc = "1: Output BFLy on this channel"]
    Value2 = 1,
}
impl From<Bfe2> for bool {
    #[inline(always)]
    fn from(variant: Bfe2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFE2` reader - Enable Bit for Boundar2 Flag y"]
pub type Bfe2R = crate::BitReader<Bfe2>;
impl Bfe2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfe2 {
        match self.bits {
            false => Bfe2::Value1,
            true => Bfe2::Value2,
        }
    }
    #[doc = "Output 0 on this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfe2::Value1
    }
    #[doc = "Output BFLy on this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfe2::Value2
    }
}
#[doc = "Field `BFE2` writer - Enable Bit for Boundar2 Flag y"]
pub type Bfe2W<'a, REG> = crate::BitWriter<'a, REG, Bfe2>;
impl<'a, REG> Bfe2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output 0 on this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfe2::Value1)
    }
    #[doc = "Output BFLy on this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfe2::Value2)
    }
}
#[doc = "Enable Bit for Boundar3 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfe3 {
    #[doc = "0: Output 0 on this channel"]
    Value1 = 0,
    #[doc = "1: Output BFLy on this channel"]
    Value2 = 1,
}
impl From<Bfe3> for bool {
    #[inline(always)]
    fn from(variant: Bfe3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFE3` reader - Enable Bit for Boundar3 Flag y"]
pub type Bfe3R = crate::BitReader<Bfe3>;
impl Bfe3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfe3 {
        match self.bits {
            false => Bfe3::Value1,
            true => Bfe3::Value2,
        }
    }
    #[doc = "Output 0 on this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfe3::Value1
    }
    #[doc = "Output BFLy on this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfe3::Value2
    }
}
#[doc = "Field `BFE3` writer - Enable Bit for Boundar3 Flag y"]
pub type Bfe3W<'a, REG> = crate::BitWriter<'a, REG, Bfe3>;
impl<'a, REG> Bfe3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output 0 on this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfe3::Value1)
    }
    #[doc = "Output BFLy on this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfe3::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Boundar0 Flag y"]
    #[inline(always)]
    pub fn bfl0(&self) -> Bfl0R {
        Bfl0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Boundar1 Flag y"]
    #[inline(always)]
    pub fn bfl1(&self) -> Bfl1R {
        Bfl1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Boundar2 Flag y"]
    #[inline(always)]
    pub fn bfl2(&self) -> Bfl2R {
        Bfl2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Boundar3 Flag y"]
    #[inline(always)]
    pub fn bfl3(&self) -> Bfl3R {
        Bfl3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Bit for Boundar0 Flag y"]
    #[inline(always)]
    pub fn bfe0(&self) -> Bfe0R {
        Bfe0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Bit for Boundar1 Flag y"]
    #[inline(always)]
    pub fn bfe1(&self) -> Bfe1R {
        Bfe1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Bit for Boundar2 Flag y"]
    #[inline(always)]
    pub fn bfe2(&self) -> Bfe2R {
        Bfe2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Bit for Boundar3 Flag y"]
    #[inline(always)]
    pub fn bfe3(&self) -> Bfe3R {
        Bfe3R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Enable Bit for Boundar0 Flag y"]
    #[inline(always)]
    #[must_use]
    pub fn bfe0(&mut self) -> Bfe0W<BflSpec> {
        Bfe0W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Bit for Boundar1 Flag y"]
    #[inline(always)]
    #[must_use]
    pub fn bfe1(&mut self) -> Bfe1W<BflSpec> {
        Bfe1W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Bit for Boundar2 Flag y"]
    #[inline(always)]
    #[must_use]
    pub fn bfe2(&mut self) -> Bfe2W<BflSpec> {
        Bfe2W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Bit for Boundar3 Flag y"]
    #[inline(always)]
    #[must_use]
    pub fn bfe3(&mut self) -> Bfe3W<BflSpec> {
        Bfe3W::new(self, 19)
    }
}
#[doc = "Boundary Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bfl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bfl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BflSpec;
impl crate::RegisterSpec for BflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bfl::R`](R) reader structure"]
impl crate::Readable for BflSpec {}
#[doc = "`write(|w| ..)` method takes [`bfl::W`](W) writer structure"]
impl crate::Writable for BflSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BFL to value 0"]
impl crate::Resettable for BflSpec {
    const RESET_VALUE: u32 = 0;
}
