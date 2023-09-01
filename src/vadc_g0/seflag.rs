#[doc = "Register `SEFLAG` reader"]
pub type R = crate::R<SEFLAG_SPEC>;
#[doc = "Register `SEFLAG` writer"]
pub type W = crate::W<SEFLAG_SPEC>;
#[doc = "Field `SEV0` reader - Source Event 0/1"]
pub type SEV0_R = crate::BitReader<SEV0_A>;
#[doc = "Source Event 0/1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEV0_A {
    #[doc = "0: No source event"]
    VALUE1 = 0,
    #[doc = "1: A source event has occurred"]
    VALUE2 = 1,
}
impl From<SEV0_A> for bool {
    #[inline(always)]
    fn from(variant: SEV0_A) -> Self {
        variant as u8 != 0
    }
}
impl SEV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEV0_A {
        match self.bits {
            false => SEV0_A::VALUE1,
            true => SEV0_A::VALUE2,
        }
    }
    #[doc = "No source event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SEV0_A::VALUE1
    }
    #[doc = "A source event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SEV0_A::VALUE2
    }
}
#[doc = "Field `SEV0` writer - Source Event 0/1"]
pub type SEV0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SEV0_A>;
impl<'a, REG, const O: u8> SEV0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No source event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SEV0_A::VALUE1)
    }
    #[doc = "A source event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SEV0_A::VALUE2)
    }
}
#[doc = "Field `SEV1` reader - Source Event 0/1"]
pub type SEV1_R = crate::BitReader<SEV1_A>;
#[doc = "Source Event 0/1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEV1_A {
    #[doc = "0: No source event"]
    VALUE1 = 0,
    #[doc = "1: A source event has occurred"]
    VALUE2 = 1,
}
impl From<SEV1_A> for bool {
    #[inline(always)]
    fn from(variant: SEV1_A) -> Self {
        variant as u8 != 0
    }
}
impl SEV1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEV1_A {
        match self.bits {
            false => SEV1_A::VALUE1,
            true => SEV1_A::VALUE2,
        }
    }
    #[doc = "No source event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SEV1_A::VALUE1
    }
    #[doc = "A source event has occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SEV1_A::VALUE2
    }
}
#[doc = "Field `SEV1` writer - Source Event 0/1"]
pub type SEV1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SEV1_A>;
impl<'a, REG, const O: u8> SEV1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No source event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SEV1_A::VALUE1)
    }
    #[doc = "A source event has occurred"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SEV1_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Source Event 0/1"]
    #[inline(always)]
    pub fn sev0(&self) -> SEV0_R {
        SEV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Source Event 0/1"]
    #[inline(always)]
    pub fn sev1(&self) -> SEV1_R {
        SEV1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source Event 0/1"]
    #[inline(always)]
    #[must_use]
    pub fn sev0(&mut self) -> SEV0_W<SEFLAG_SPEC, 0> {
        SEV0_W::new(self)
    }
    #[doc = "Bit 1 - Source Event 0/1"]
    #[inline(always)]
    #[must_use]
    pub fn sev1(&mut self) -> SEV1_W<SEFLAG_SPEC, 1> {
        SEV1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Source Event Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEFLAG_SPEC;
impl crate::RegisterSpec for SEFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seflag::R`](R) reader structure"]
impl crate::Readable for SEFLAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seflag::W`](W) writer structure"]
impl crate::Writable for SEFLAG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEFLAG to value 0"]
impl crate::Resettable for SEFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
