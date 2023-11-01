#[doc = "Register `PCON` reader"]
pub type R = crate::R<PCON_SPEC>;
#[doc = "Register `PCON` writer"]
pub type W = crate::W<PCON_SPEC>;
#[doc = "Field `IBYP` reader - Instruction Prefetch Buffer Bypass"]
pub type IBYP_R = crate::BitReader<IBYP_A>;
#[doc = "Instruction Prefetch Buffer Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IBYP_A {
    #[doc = "0: Instruction prefetch buffer not bypassed."]
    VALUE1 = 0,
    #[doc = "1: Instruction prefetch buffer bypassed."]
    VALUE2 = 1,
}
impl From<IBYP_A> for bool {
    #[inline(always)]
    fn from(variant: IBYP_A) -> Self {
        variant as u8 != 0
    }
}
impl IBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IBYP_A {
        match self.bits {
            false => IBYP_A::VALUE1,
            true => IBYP_A::VALUE2,
        }
    }
    #[doc = "Instruction prefetch buffer not bypassed."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IBYP_A::VALUE1
    }
    #[doc = "Instruction prefetch buffer bypassed."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IBYP_A::VALUE2
    }
}
#[doc = "Field `IBYP` writer - Instruction Prefetch Buffer Bypass"]
pub type IBYP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IBYP_A>;
impl<'a, REG, const O: u8> IBYP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Instruction prefetch buffer not bypassed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(IBYP_A::VALUE1)
    }
    #[doc = "Instruction prefetch buffer bypassed."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(IBYP_A::VALUE2)
    }
}
#[doc = "Instruction Prefetch Buffer Invalidate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IINV_AW {
    #[doc = "0: No effect."]
    VALUE1 = 0,
    #[doc = "1: Initiate invalidation of entire instruction cache."]
    VALUE2 = 1,
}
impl From<IINV_AW> for bool {
    #[inline(always)]
    fn from(variant: IINV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IINV` writer - Instruction Prefetch Buffer Invalidate"]
pub type IINV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IINV_AW>;
impl<'a, REG, const O: u8> IINV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(IINV_AW::VALUE1)
    }
    #[doc = "Initiate invalidation of entire instruction cache."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(IINV_AW::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Instruction Prefetch Buffer Bypass"]
    #[inline(always)]
    pub fn ibyp(&self) -> IBYP_R {
        IBYP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Instruction Prefetch Buffer Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn ibyp(&mut self) -> IBYP_W<PCON_SPEC, 0> {
        IBYP_W::new(self)
    }
    #[doc = "Bit 1 - Instruction Prefetch Buffer Invalidate"]
    #[inline(always)]
    #[must_use]
    pub fn iinv(&mut self) -> IINV_W<PCON_SPEC, 1> {
        IINV_W::new(self)
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
#[doc = "Prefetch Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCON_SPEC;
impl crate::RegisterSpec for PCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcon::R`](R) reader structure"]
impl crate::Readable for PCON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcon::W`](W) writer structure"]
impl crate::Writable for PCON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCON to value 0"]
impl crate::Resettable for PCON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
