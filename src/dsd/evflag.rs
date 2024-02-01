#[doc = "Register `EVFLAG` reader"]
pub type R = crate::R<EVFLAG_SPEC>;
#[doc = "Register `EVFLAG` writer"]
pub type W = crate::W<EVFLAG_SPEC>;
#[doc = "Field `RESEV0` reader - Result Event"]
pub type RESEV0_R = crate::BitReader<RESEV0_A>;
#[doc = "Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl RESEV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESEV0_A {
        match self.bits {
            false => RESEV0_A::VALUE1,
            true => RESEV0_A::VALUE2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESEV0_A::VALUE1
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESEV0_A::VALUE2
    }
}
#[doc = "Field `RESEV0` writer - Result Event"]
pub type RESEV0_W<'a, REG> = crate::BitWriter<'a, REG, RESEV0_A>;
impl<'a, REG> RESEV0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RESEV0_A::VALUE1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RESEV0_A::VALUE2)
    }
}
#[doc = "Field `RESEV1` reader - Result Event"]
pub type RESEV1_R = crate::BitReader<RESEV1_A>;
#[doc = "Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl RESEV1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESEV1_A {
        match self.bits {
            false => RESEV1_A::VALUE1,
            true => RESEV1_A::VALUE2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESEV1_A::VALUE1
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESEV1_A::VALUE2
    }
}
#[doc = "Field `RESEV1` writer - Result Event"]
pub type RESEV1_W<'a, REG> = crate::BitWriter<'a, REG, RESEV1_A>;
impl<'a, REG> RESEV1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RESEV1_A::VALUE1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RESEV1_A::VALUE2)
    }
}
#[doc = "Field `RESEV2` reader - Result Event"]
pub type RESEV2_R = crate::BitReader<RESEV2_A>;
#[doc = "Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl RESEV2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESEV2_A {
        match self.bits {
            false => RESEV2_A::VALUE1,
            true => RESEV2_A::VALUE2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESEV2_A::VALUE1
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESEV2_A::VALUE2
    }
}
#[doc = "Field `RESEV2` writer - Result Event"]
pub type RESEV2_W<'a, REG> = crate::BitWriter<'a, REG, RESEV2_A>;
impl<'a, REG> RESEV2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RESEV2_A::VALUE1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RESEV2_A::VALUE2)
    }
}
#[doc = "Field `RESEV3` reader - Result Event"]
pub type RESEV3_R = crate::BitReader<RESEV3_A>;
#[doc = "Result Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl RESEV3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESEV3_A {
        match self.bits {
            false => RESEV3_A::VALUE1,
            true => RESEV3_A::VALUE2,
        }
    }
    #[doc = "No result event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RESEV3_A::VALUE1
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RESEV3_A::VALUE2
    }
}
#[doc = "Field `RESEV3` writer - Result Event"]
pub type RESEV3_W<'a, REG> = crate::BitWriter<'a, REG, RESEV3_A>;
impl<'a, REG> RESEV3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No result event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RESEV3_A::VALUE1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RESEV3_A::VALUE2)
    }
}
#[doc = "Field `ALEV0` reader - Alarm Event"]
pub type ALEV0_R = crate::BitReader<ALEV0_A>;
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ALEV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALEV0_A {
        match self.bits {
            false => ALEV0_A::VALUE1,
            true => ALEV0_A::VALUE2,
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV0_A::VALUE1
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV0_A::VALUE2
    }
}
#[doc = "Field `ALEV0` writer - Alarm Event"]
pub type ALEV0_W<'a, REG> = crate::BitWriter<'a, REG, ALEV0_A>;
impl<'a, REG> ALEV0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV0_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV0_A::VALUE2)
    }
}
#[doc = "Field `ALEV1` reader - Alarm Event"]
pub type ALEV1_R = crate::BitReader<ALEV1_A>;
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ALEV1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALEV1_A {
        match self.bits {
            false => ALEV1_A::VALUE1,
            true => ALEV1_A::VALUE2,
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV1_A::VALUE1
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV1_A::VALUE2
    }
}
#[doc = "Field `ALEV1` writer - Alarm Event"]
pub type ALEV1_W<'a, REG> = crate::BitWriter<'a, REG, ALEV1_A>;
impl<'a, REG> ALEV1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV1_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV1_A::VALUE2)
    }
}
#[doc = "Field `ALEV2` reader - Alarm Event"]
pub type ALEV2_R = crate::BitReader<ALEV2_A>;
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ALEV2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALEV2_A {
        match self.bits {
            false => ALEV2_A::VALUE1,
            true => ALEV2_A::VALUE2,
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV2_A::VALUE1
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV2_A::VALUE2
    }
}
#[doc = "Field `ALEV2` writer - Alarm Event"]
pub type ALEV2_W<'a, REG> = crate::BitWriter<'a, REG, ALEV2_A>;
impl<'a, REG> ALEV2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV2_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV2_A::VALUE2)
    }
}
#[doc = "Field `ALEV3` reader - Alarm Event"]
pub type ALEV3_R = crate::BitReader<ALEV3_A>;
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ALEV3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALEV3_A {
        match self.bits {
            false => ALEV3_A::VALUE1,
            true => ALEV3_A::VALUE2,
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV3_A::VALUE1
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV3_A::VALUE2
    }
}
#[doc = "Field `ALEV3` writer - Alarm Event"]
pub type ALEV3_W<'a, REG> = crate::BitWriter<'a, REG, ALEV3_A>;
impl<'a, REG> ALEV3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV3_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV3_A::VALUE2)
    }
}
#[doc = "Field `ALEV4` reader - Alarm Event"]
pub type ALEV4_R = crate::BitReader<ALEV4_A>;
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ALEV4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALEV4_A {
        match self.bits {
            false => ALEV4_A::VALUE1,
            true => ALEV4_A::VALUE2,
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV4_A::VALUE1
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV4_A::VALUE2
    }
}
#[doc = "Field `ALEV4` writer - Alarm Event"]
pub type ALEV4_W<'a, REG> = crate::BitWriter<'a, REG, ALEV4_A>;
impl<'a, REG> ALEV4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV4_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV4_A::VALUE2)
    }
}
#[doc = "Field `ALEV5` reader - Alarm Event"]
pub type ALEV5_R = crate::BitReader<ALEV5_A>;
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ALEV5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALEV5_A {
        match self.bits {
            false => ALEV5_A::VALUE1,
            true => ALEV5_A::VALUE2,
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV5_A::VALUE1
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV5_A::VALUE2
    }
}
#[doc = "Field `ALEV5` writer - Alarm Event"]
pub type ALEV5_W<'a, REG> = crate::BitWriter<'a, REG, ALEV5_A>;
impl<'a, REG> ALEV5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV5_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV5_A::VALUE2)
    }
}
#[doc = "Field `ALEV6` reader - Alarm Event"]
pub type ALEV6_R = crate::BitReader<ALEV6_A>;
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ALEV6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALEV6_A {
        match self.bits {
            false => ALEV6_A::VALUE1,
            true => ALEV6_A::VALUE2,
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV6_A::VALUE1
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV6_A::VALUE2
    }
}
#[doc = "Field `ALEV6` writer - Alarm Event"]
pub type ALEV6_W<'a, REG> = crate::BitWriter<'a, REG, ALEV6_A>;
impl<'a, REG> ALEV6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV6_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV6_A::VALUE2)
    }
}
#[doc = "Field `ALEV7` reader - Alarm Event"]
pub type ALEV7_R = crate::BitReader<ALEV7_A>;
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ALEV7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALEV7_A {
        match self.bits {
            false => ALEV7_A::VALUE1,
            true => ALEV7_A::VALUE2,
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV7_A::VALUE1
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV7_A::VALUE2
    }
}
#[doc = "Field `ALEV7` writer - Alarm Event"]
pub type ALEV7_W<'a, REG> = crate::BitWriter<'a, REG, ALEV7_A>;
impl<'a, REG> ALEV7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV7_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV7_A::VALUE2)
    }
}
#[doc = "Field `ALEV8` reader - Alarm Event"]
pub type ALEV8_R = crate::BitReader<ALEV8_A>;
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ALEV8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALEV8_A {
        match self.bits {
            false => ALEV8_A::VALUE1,
            true => ALEV8_A::VALUE2,
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV8_A::VALUE1
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV8_A::VALUE2
    }
}
#[doc = "Field `ALEV8` writer - Alarm Event"]
pub type ALEV8_W<'a, REG> = crate::BitWriter<'a, REG, ALEV8_A>;
impl<'a, REG> ALEV8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV8_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV8_A::VALUE2)
    }
}
#[doc = "Field `ALEV9` reader - Alarm Event"]
pub type ALEV9_R = crate::BitReader<ALEV9_A>;
#[doc = "Alarm Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ALEV9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALEV9_A {
        match self.bits {
            false => ALEV9_A::VALUE1,
            true => ALEV9_A::VALUE2,
        }
    }
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALEV9_A::VALUE1
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALEV9_A::VALUE2
    }
}
#[doc = "Field `ALEV9` writer - Alarm Event"]
pub type ALEV9_W<'a, REG> = crate::BitWriter<'a, REG, ALEV9_A>;
impl<'a, REG> ALEV9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No alarm event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV9_A::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ALEV9_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Result Event"]
    #[inline(always)]
    pub fn resev0(&self) -> RESEV0_R {
        RESEV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Result Event"]
    #[inline(always)]
    pub fn resev1(&self) -> RESEV1_R {
        RESEV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Result Event"]
    #[inline(always)]
    pub fn resev2(&self) -> RESEV2_R {
        RESEV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Result Event"]
    #[inline(always)]
    pub fn resev3(&self) -> RESEV3_R {
        RESEV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Alarm Event"]
    #[inline(always)]
    pub fn alev0(&self) -> ALEV0_R {
        ALEV0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Alarm Event"]
    #[inline(always)]
    pub fn alev1(&self) -> ALEV1_R {
        ALEV1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Alarm Event"]
    #[inline(always)]
    pub fn alev2(&self) -> ALEV2_R {
        ALEV2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Alarm Event"]
    #[inline(always)]
    pub fn alev3(&self) -> ALEV3_R {
        ALEV3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Alarm Event"]
    #[inline(always)]
    pub fn alev4(&self) -> ALEV4_R {
        ALEV4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Alarm Event"]
    #[inline(always)]
    pub fn alev5(&self) -> ALEV5_R {
        ALEV5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Alarm Event"]
    #[inline(always)]
    pub fn alev6(&self) -> ALEV6_R {
        ALEV6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Alarm Event"]
    #[inline(always)]
    pub fn alev7(&self) -> ALEV7_R {
        ALEV7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Alarm Event"]
    #[inline(always)]
    pub fn alev8(&self) -> ALEV8_R {
        ALEV8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Alarm Event"]
    #[inline(always)]
    pub fn alev9(&self) -> ALEV9_R {
        ALEV9_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Result Event"]
    #[inline(always)]
    #[must_use]
    pub fn resev0(&mut self) -> RESEV0_W<EVFLAG_SPEC> {
        RESEV0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Result Event"]
    #[inline(always)]
    #[must_use]
    pub fn resev1(&mut self) -> RESEV1_W<EVFLAG_SPEC> {
        RESEV1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Result Event"]
    #[inline(always)]
    #[must_use]
    pub fn resev2(&mut self) -> RESEV2_W<EVFLAG_SPEC> {
        RESEV2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Result Event"]
    #[inline(always)]
    #[must_use]
    pub fn resev3(&mut self) -> RESEV3_W<EVFLAG_SPEC> {
        RESEV3_W::new(self, 3)
    }
    #[doc = "Bit 16 - Alarm Event"]
    #[inline(always)]
    #[must_use]
    pub fn alev0(&mut self) -> ALEV0_W<EVFLAG_SPEC> {
        ALEV0_W::new(self, 16)
    }
    #[doc = "Bit 17 - Alarm Event"]
    #[inline(always)]
    #[must_use]
    pub fn alev1(&mut self) -> ALEV1_W<EVFLAG_SPEC> {
        ALEV1_W::new(self, 17)
    }
    #[doc = "Bit 18 - Alarm Event"]
    #[inline(always)]
    #[must_use]
    pub fn alev2(&mut self) -> ALEV2_W<EVFLAG_SPEC> {
        ALEV2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Alarm Event"]
    #[inline(always)]
    #[must_use]
    pub fn alev3(&mut self) -> ALEV3_W<EVFLAG_SPEC> {
        ALEV3_W::new(self, 19)
    }
    #[doc = "Bit 20 - Alarm Event"]
    #[inline(always)]
    #[must_use]
    pub fn alev4(&mut self) -> ALEV4_W<EVFLAG_SPEC> {
        ALEV4_W::new(self, 20)
    }
    #[doc = "Bit 21 - Alarm Event"]
    #[inline(always)]
    #[must_use]
    pub fn alev5(&mut self) -> ALEV5_W<EVFLAG_SPEC> {
        ALEV5_W::new(self, 21)
    }
    #[doc = "Bit 22 - Alarm Event"]
    #[inline(always)]
    #[must_use]
    pub fn alev6(&mut self) -> ALEV6_W<EVFLAG_SPEC> {
        ALEV6_W::new(self, 22)
    }
    #[doc = "Bit 23 - Alarm Event"]
    #[inline(always)]
    #[must_use]
    pub fn alev7(&mut self) -> ALEV7_W<EVFLAG_SPEC> {
        ALEV7_W::new(self, 23)
    }
    #[doc = "Bit 24 - Alarm Event"]
    #[inline(always)]
    #[must_use]
    pub fn alev8(&mut self) -> ALEV8_W<EVFLAG_SPEC> {
        ALEV8_W::new(self, 24)
    }
    #[doc = "Bit 25 - Alarm Event"]
    #[inline(always)]
    #[must_use]
    pub fn alev9(&mut self) -> ALEV9_W<EVFLAG_SPEC> {
        ALEV9_W::new(self, 25)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Event Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVFLAG_SPEC;
impl crate::RegisterSpec for EVFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evflag::R`](R) reader structure"]
impl crate::Readable for EVFLAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evflag::W`](W) writer structure"]
impl crate::Writable for EVFLAG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVFLAG to value 0"]
impl crate::Resettable for EVFLAG_SPEC {
    const RESET_VALUE: u32 = 0;
}
