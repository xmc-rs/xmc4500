#[doc = "Register `BFL` reader"]
pub type R = crate::R<BFL_SPEC>;
#[doc = "Register `BFL` writer"]
pub type W = crate::W<BFL_SPEC>;
#[doc = "Boundar0 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type BFL0_R = crate::BitReader<BFL0_A>;
impl BFL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFL0_A {
        match self.bits {
            false => BFL0_A::VALUE1,
            true => BFL0_A::VALUE2,
        }
    }
    #[doc = "Passive state: result has not yet crossed the activation boundary, or selected gate signal is inactive, or this boundary flag is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFL0_A::VALUE1
    }
    #[doc = "Active state: result has crossed the activation boundary"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFL0_A::VALUE2
    }
}
#[doc = "Boundar1 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type BFL1_R = crate::BitReader<BFL1_A>;
impl BFL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFL1_A {
        match self.bits {
            false => BFL1_A::VALUE1,
            true => BFL1_A::VALUE2,
        }
    }
    #[doc = "Passive state: result has not yet crossed the activation boundary, or selected gate signal is inactive, or this boundary flag is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFL1_A::VALUE1
    }
    #[doc = "Active state: result has crossed the activation boundary"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFL1_A::VALUE2
    }
}
#[doc = "Boundar2 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type BFL2_R = crate::BitReader<BFL2_A>;
impl BFL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFL2_A {
        match self.bits {
            false => BFL2_A::VALUE1,
            true => BFL2_A::VALUE2,
        }
    }
    #[doc = "Passive state: result has not yet crossed the activation boundary, or selected gate signal is inactive, or this boundary flag is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFL2_A::VALUE1
    }
    #[doc = "Active state: result has crossed the activation boundary"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFL2_A::VALUE2
    }
}
#[doc = "Boundar3 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type BFL3_R = crate::BitReader<BFL3_A>;
impl BFL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFL3_A {
        match self.bits {
            false => BFL3_A::VALUE1,
            true => BFL3_A::VALUE2,
        }
    }
    #[doc = "Passive state: result has not yet crossed the activation boundary, or selected gate signal is inactive, or this boundary flag is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFL3_A::VALUE1
    }
    #[doc = "Active state: result has crossed the activation boundary"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFL3_A::VALUE2
    }
}
#[doc = "Enable Bit for Boundar0 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type BFE0_R = crate::BitReader<BFE0_A>;
impl BFE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFE0_A {
        match self.bits {
            false => BFE0_A::VALUE1,
            true => BFE0_A::VALUE2,
        }
    }
    #[doc = "Output 0 on this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFE0_A::VALUE1
    }
    #[doc = "Output BFLy on this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFE0_A::VALUE2
    }
}
#[doc = "Field `BFE0` writer - Enable Bit for Boundar0 Flag y"]
pub type BFE0_W<'a, REG> = crate::BitWriter<'a, REG, BFE0_A>;
impl<'a, REG> BFE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output 0 on this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFE0_A::VALUE1)
    }
    #[doc = "Output BFLy on this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFE0_A::VALUE2)
    }
}
#[doc = "Enable Bit for Boundar1 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type BFE1_R = crate::BitReader<BFE1_A>;
impl BFE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFE1_A {
        match self.bits {
            false => BFE1_A::VALUE1,
            true => BFE1_A::VALUE2,
        }
    }
    #[doc = "Output 0 on this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFE1_A::VALUE1
    }
    #[doc = "Output BFLy on this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFE1_A::VALUE2
    }
}
#[doc = "Field `BFE1` writer - Enable Bit for Boundar1 Flag y"]
pub type BFE1_W<'a, REG> = crate::BitWriter<'a, REG, BFE1_A>;
impl<'a, REG> BFE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output 0 on this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFE1_A::VALUE1)
    }
    #[doc = "Output BFLy on this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFE1_A::VALUE2)
    }
}
#[doc = "Enable Bit for Boundar2 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type BFE2_R = crate::BitReader<BFE2_A>;
impl BFE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFE2_A {
        match self.bits {
            false => BFE2_A::VALUE1,
            true => BFE2_A::VALUE2,
        }
    }
    #[doc = "Output 0 on this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFE2_A::VALUE1
    }
    #[doc = "Output BFLy on this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFE2_A::VALUE2
    }
}
#[doc = "Field `BFE2` writer - Enable Bit for Boundar2 Flag y"]
pub type BFE2_W<'a, REG> = crate::BitWriter<'a, REG, BFE2_A>;
impl<'a, REG> BFE2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output 0 on this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFE2_A::VALUE1)
    }
    #[doc = "Output BFLy on this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFE2_A::VALUE2)
    }
}
#[doc = "Enable Bit for Boundar3 Flag y\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
pub type BFE3_R = crate::BitReader<BFE3_A>;
impl BFE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFE3_A {
        match self.bits {
            false => BFE3_A::VALUE1,
            true => BFE3_A::VALUE2,
        }
    }
    #[doc = "Output 0 on this channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFE3_A::VALUE1
    }
    #[doc = "Output BFLy on this channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFE3_A::VALUE2
    }
}
#[doc = "Field `BFE3` writer - Enable Bit for Boundar3 Flag y"]
pub type BFE3_W<'a, REG> = crate::BitWriter<'a, REG, BFE3_A>;
impl<'a, REG> BFE3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output 0 on this channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BFE3_A::VALUE1)
    }
    #[doc = "Output BFLy on this channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BFE3_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Boundar0 Flag y"]
    #[inline(always)]
    pub fn bfl0(&self) -> BFL0_R {
        BFL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Boundar1 Flag y"]
    #[inline(always)]
    pub fn bfl1(&self) -> BFL1_R {
        BFL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Boundar2 Flag y"]
    #[inline(always)]
    pub fn bfl2(&self) -> BFL2_R {
        BFL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Boundar3 Flag y"]
    #[inline(always)]
    pub fn bfl3(&self) -> BFL3_R {
        BFL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Bit for Boundar0 Flag y"]
    #[inline(always)]
    pub fn bfe0(&self) -> BFE0_R {
        BFE0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Bit for Boundar1 Flag y"]
    #[inline(always)]
    pub fn bfe1(&self) -> BFE1_R {
        BFE1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Bit for Boundar2 Flag y"]
    #[inline(always)]
    pub fn bfe2(&self) -> BFE2_R {
        BFE2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Bit for Boundar3 Flag y"]
    #[inline(always)]
    pub fn bfe3(&self) -> BFE3_R {
        BFE3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Enable Bit for Boundar0 Flag y"]
    #[inline(always)]
    #[must_use]
    pub fn bfe0(&mut self) -> BFE0_W<BFL_SPEC> {
        BFE0_W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Bit for Boundar1 Flag y"]
    #[inline(always)]
    #[must_use]
    pub fn bfe1(&mut self) -> BFE1_W<BFL_SPEC> {
        BFE1_W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Bit for Boundar2 Flag y"]
    #[inline(always)]
    #[must_use]
    pub fn bfe2(&mut self) -> BFE2_W<BFL_SPEC> {
        BFE2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Bit for Boundar3 Flag y"]
    #[inline(always)]
    #[must_use]
    pub fn bfe3(&mut self) -> BFE3_W<BFL_SPEC> {
        BFE3_W::new(self, 19)
    }
}
#[doc = "Boundary Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bfl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bfl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BFL_SPEC;
impl crate::RegisterSpec for BFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bfl::R`](R) reader structure"]
impl crate::Readable for BFL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bfl::W`](W) writer structure"]
impl crate::Writable for BFL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BFL to value 0"]
impl crate::Resettable for BFL_SPEC {
    const RESET_VALUE: u32 = 0;
}
