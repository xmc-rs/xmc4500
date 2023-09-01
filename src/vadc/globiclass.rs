#[doc = "Register `GLOBICLASS[%s]` reader"]
pub type R = crate::R<GLOBICLASS_SPEC>;
#[doc = "Register `GLOBICLASS[%s]` writer"]
pub type W = crate::W<GLOBICLASS_SPEC>;
#[doc = "Field `STCS` reader - Sample Time Control for Standard Conversions"]
pub type STCS_R = crate::FieldReader;
#[doc = "Field `STCS` writer - Sample Time Control for Standard Conversions"]
pub type STCS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `CMS` reader - Conversion Mode for Standard Conversions"]
pub type CMS_R = crate::FieldReader<CMS_A>;
#[doc = "Conversion Mode for Standard Conversions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMS_A {
    #[doc = "0: 12-bit conversion"]
    VALUE1 = 0,
    #[doc = "1: 10-bit conversion"]
    VALUE2 = 1,
    #[doc = "2: 8-bit conversion"]
    VALUE3 = 2,
    #[doc = "5: 10-bit fast compare mode"]
    VALUE6 = 5,
}
impl From<CMS_A> for u8 {
    #[inline(always)]
    fn from(variant: CMS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMS_A {
    type Ux = u8;
}
impl CMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMS_A> {
        match self.bits {
            0 => Some(CMS_A::VALUE1),
            1 => Some(CMS_A::VALUE2),
            2 => Some(CMS_A::VALUE3),
            5 => Some(CMS_A::VALUE6),
            _ => None,
        }
    }
    #[doc = "12-bit conversion"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMS_A::VALUE1
    }
    #[doc = "10-bit conversion"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMS_A::VALUE2
    }
    #[doc = "8-bit conversion"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CMS_A::VALUE3
    }
    #[doc = "10-bit fast compare mode"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == CMS_A::VALUE6
    }
}
#[doc = "Field `CMS` writer - Conversion Mode for Standard Conversions"]
pub type CMS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CMS_A>;
impl<'a, REG, const O: u8> CMS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit conversion"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CMS_A::VALUE1)
    }
    #[doc = "10-bit conversion"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CMS_A::VALUE2)
    }
    #[doc = "8-bit conversion"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CMS_A::VALUE3)
    }
    #[doc = "10-bit fast compare mode"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(CMS_A::VALUE6)
    }
}
#[doc = "Field `STCE` reader - Sample Time Control for EMUX Conversions"]
pub type STCE_R = crate::FieldReader;
#[doc = "Field `STCE` writer - Sample Time Control for EMUX Conversions"]
pub type STCE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `CME` reader - Conversion Mode for EMUX Conversions"]
pub type CME_R = crate::FieldReader<CME_A>;
#[doc = "Conversion Mode for EMUX Conversions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CME_A {
    #[doc = "0: 12-bit conversion"]
    VALUE1 = 0,
    #[doc = "1: 10-bit conversion"]
    VALUE2 = 1,
    #[doc = "2: 8-bit conversion"]
    VALUE3 = 2,
    #[doc = "5: 10-bit fast compare mode"]
    VALUE6 = 5,
}
impl From<CME_A> for u8 {
    #[inline(always)]
    fn from(variant: CME_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CME_A {
    type Ux = u8;
}
impl CME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CME_A> {
        match self.bits {
            0 => Some(CME_A::VALUE1),
            1 => Some(CME_A::VALUE2),
            2 => Some(CME_A::VALUE3),
            5 => Some(CME_A::VALUE6),
            _ => None,
        }
    }
    #[doc = "12-bit conversion"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CME_A::VALUE1
    }
    #[doc = "10-bit conversion"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CME_A::VALUE2
    }
    #[doc = "8-bit conversion"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CME_A::VALUE3
    }
    #[doc = "10-bit fast compare mode"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == CME_A::VALUE6
    }
}
#[doc = "Field `CME` writer - Conversion Mode for EMUX Conversions"]
pub type CME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CME_A>;
impl<'a, REG, const O: u8> CME_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit conversion"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CME_A::VALUE1)
    }
    #[doc = "10-bit conversion"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CME_A::VALUE2)
    }
    #[doc = "8-bit conversion"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CME_A::VALUE3)
    }
    #[doc = "10-bit fast compare mode"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(CME_A::VALUE6)
    }
}
impl R {
    #[doc = "Bits 0:4 - Sample Time Control for Standard Conversions"]
    #[inline(always)]
    pub fn stcs(&self) -> STCS_R {
        STCS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - Conversion Mode for Standard Conversions"]
    #[inline(always)]
    pub fn cms(&self) -> CMS_R {
        CMS_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:20 - Sample Time Control for EMUX Conversions"]
    #[inline(always)]
    pub fn stce(&self) -> STCE_R {
        STCE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - Conversion Mode for EMUX Conversions"]
    #[inline(always)]
    pub fn cme(&self) -> CME_R {
        CME_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Sample Time Control for Standard Conversions"]
    #[inline(always)]
    #[must_use]
    pub fn stcs(&mut self) -> STCS_W<GLOBICLASS_SPEC, 0> {
        STCS_W::new(self)
    }
    #[doc = "Bits 8:10 - Conversion Mode for Standard Conversions"]
    #[inline(always)]
    #[must_use]
    pub fn cms(&mut self) -> CMS_W<GLOBICLASS_SPEC, 8> {
        CMS_W::new(self)
    }
    #[doc = "Bits 16:20 - Sample Time Control for EMUX Conversions"]
    #[inline(always)]
    #[must_use]
    pub fn stce(&mut self) -> STCE_W<GLOBICLASS_SPEC, 16> {
        STCE_W::new(self)
    }
    #[doc = "Bits 24:26 - Conversion Mode for EMUX Conversions"]
    #[inline(always)]
    #[must_use]
    pub fn cme(&mut self) -> CME_W<GLOBICLASS_SPEC, 24> {
        CME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Input Class Register, Global\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globiclass::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globiclass::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GLOBICLASS_SPEC;
impl crate::RegisterSpec for GLOBICLASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`globiclass::R`](R) reader structure"]
impl crate::Readable for GLOBICLASS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`globiclass::W`](W) writer structure"]
impl crate::Writable for GLOBICLASS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GLOBICLASS[%s]
to value 0"]
impl crate::Resettable for GLOBICLASS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
